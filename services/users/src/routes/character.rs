use std::{sync::Arc, time::Duration};

use auth_token::AccessToken;
use axum::{extract::State, Json};
use reqwest::StatusCode;
use service_sso_api::dto::RegisterRequest;
use tokio::time::sleep;

use crate::{
    dto::character::{RegisterCharacterRequest, RegisterCharacterResponse},
    Service,
};

#[tracing::instrument(skip(service, token, payload))]
pub async fn character(
    State(service): State<Arc<Service>>,
    token: AccessToken,
    Json(payload): Json<RegisterCharacterRequest>,
) -> (StatusCode, Json<RegisterCharacterResponse>) {
    let tokens = service
        .sso
        .oauth_authorize(&payload.authorization_code)
        .await;

    if let Err(err) = tokens {
        tracing::error!(?err, "unable to authorize using oauth code");
        return (
            StatusCode::UNAUTHORIZED,
            Json(RegisterCharacterResponse::failure()),
        );
    }

    let tokens = tokens.unwrap();
    let character_id = tokens.access_token.character_id;

    tracing::info!(character_id, "authorized using eve sso");

    if let Ok(Some(existing_user_id)) = service
        .users
        .find_user_by_character(character_id)
        .await
    {
        if existing_user_id != token.user_id {
            tracing::warn!(?token.user_id, character_id, "tried to register a character which was already registered to another user");
            return (StatusCode::FORBIDDEN, Json(RegisterCharacterResponse::failure()));
        }
    } else {
        if let Err(err) = service.users.register_character(token.user_id, character_id).await
        {
            tracing::error!(?err, "unable to register character");
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(RegisterCharacterResponse::failure()));
        }
    }

    if let Err(err) = service.service_sso.register(RegisterRequest { 
        user_id: token.user_id,
        refresh_token: tokens.refresh_token,
        access_token: tokens.access_token.access_token,
    }).await {
        tracing::error!(?err, "couldn't send sso register request");
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(RegisterCharacterResponse::failure()));
    }

    tracing::info!(?token.user_id, character_id, "registered character");

    // TODO: Wait a small amount of time for changes to propagate. Change this later
    // and instead solve this issue on the client side (eventual consistency). Or
    // implement some sort of receipt of crucial data being propagated.
    sleep(Duration::from_secs(1)).await;

    (StatusCode::OK, Json(RegisterCharacterResponse::success()))
}
