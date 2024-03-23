
use std::sync::Arc;

use auth_token::decode_access_token;
use axum::{extract::State, http::StatusCode, Json};
use service_sso_api::dto::{RefreshRequest, RefreshResponse};

use crate::services::Services;

#[tracing::instrument(skip(services, payload))]
pub async fn refresh(
    State(services): State<Arc<Services>>,
    Json(payload): Json<RefreshRequest>,
) -> (StatusCode, Json<RefreshResponse>) {

    let response = if let Ok(user_token) = decode_access_token(&payload.user_access_token) {
        match services.credentials_manager.refresh_character_credentials_on_demand(payload.character_id, user_token.user_id).await {
            Ok(Some(access_token)) => (StatusCode::OK, Json(RefreshResponse::successful(&access_token.access_token))),
            Ok(None) => (StatusCode::NOT_FOUND, Json(RefreshResponse::failure("credentials not found"))),
            Err(err) => {
                tracing::error!(?err, "error refreshing credentials on demand");
                (StatusCode::INTERNAL_SERVER_ERROR, Json(RefreshResponse::failure("couldn't refresh credentials")))
            }
        }
    } else {
        (StatusCode::UNAUTHORIZED, Json(RefreshResponse::failure("invalid access token")))
    };

    response
}
