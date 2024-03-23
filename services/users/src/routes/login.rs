use axum::{extract::State, http::StatusCode, Json};
use message_bus::StreamEvent;
use service_sso_api::dto::RegisterRequest;
use std::sync::Arc;

use crate::{
    dto::login::{LoginRequest, LoginResponse},
    Service,
};

#[tracing::instrument(skip(service))]
pub async fn login(
    State(service): State<Arc<Service>>,
    Json(payload): Json<LoginRequest>,
) -> (StatusCode, Json<LoginResponse>) {
    let tokens = service
        .sso
        .oauth_authorize(&payload.authorization_code)
        .await;

    if let Err(err) = tokens {
        tracing::error!(?err, "unable to authorize using oauth code");
        return (StatusCode::UNAUTHORIZED, Json(LoginResponse::failure()));
    }

    let tokens = tokens.unwrap();
    let character_id = tokens.access_token.character_id;

    tracing::info!(character_id, "authorized using eve sso");

    let user_id = service.users.find_user_by_character(character_id).await;

    if let Err(err) = user_id {
        tracing::error!(?err, "could not lookup character user");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(LoginResponse::failure()),
        );
    }

    let user_id = match user_id.unwrap() {
        Some(user_id) => Ok(user_id),
        None => service.users.create_user(character_id).await,
    };

    if let Err(err) = user_id {
        tracing::error!(?err, "could not create user");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(LoginResponse::failure()),
        );
    }

    let user_id = user_id.unwrap();

    if let Err(err) = service
        .message_producer
        .send_event(StreamEvent::UserAuthenticated { user_id })
        .await
    {
        tracing::warn!(?err, "could not send UserAuthenticated event");
    }

    if let Err(err) = service.service_sso.register(RegisterRequest { 
        user_id,
        refresh_token: tokens.refresh_token,
        access_token: tokens.access_token.access_token,
    }).await {
        tracing::error!(?err, "couldn't send sso register request");
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(LoginResponse::failure()));
    }

    match service
        .users
        .create_user_session(user_id, &payload.client_ip)
        .await
    {
        Ok(session) => {
            tracing::info!(user_id, "created user session");
            (StatusCode::OK, Json(LoginResponse::success(session)))
        }
        Err(err) => {
            tracing::error!(?err, "could not create session");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(LoginResponse::failure()),
            )
        }
    }
}
