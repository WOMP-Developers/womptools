
use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use service_sso_api::dto::{RegisterRequest, RegisterResponse};

use crate::services::Services;

#[tracing::instrument(skip(services, payload))]
pub async fn register(
    State(services): State<Arc<Services>>,
    Json(payload): Json<RegisterRequest>,
) -> (StatusCode, Json<RegisterResponse>) {

    if let Err(err) = services.credentials_manager.register_credentials(payload.user_id, &payload.access_token, &payload.refresh_token).await {
        tracing::error!(?err, "couldn't register credentials");

        return (StatusCode::INTERNAL_SERVER_ERROR, Json(RegisterResponse::failure("register credentials error")));
    }
    
    (StatusCode::OK, Json(RegisterResponse::successful()))
}
