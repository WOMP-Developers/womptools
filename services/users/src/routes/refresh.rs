use auth_token::decode_refresh_token;
use axum::{extract::State, http::StatusCode, Json};
use secrecy::ExposeSecret;
use std::sync::Arc;

use crate::{
    dto::refresh::{RefreshRequest, RefreshResponse},
    Service,
};

#[tracing::instrument(skip(service))]
pub async fn refresh(
    State(service): State<Arc<Service>>,
    Json(payload): Json<RefreshRequest>,
) -> (StatusCode, Json<RefreshResponse>) {
    let refresh_token = decode_refresh_token(&payload.refresh_token.expose_secret());

    if let Err(err) = refresh_token {
        tracing::error!(?err, "invalid refresh token");
        return (StatusCode::UNAUTHORIZED, Json(RefreshResponse::failure()));
    }

    let refresh_token = refresh_token.unwrap();

    let user_session = service
        .users
        .refresh_user_session(&refresh_token.session_id)
        .await;

    if let Err(err) = user_session {
        tracing::error!(?err, "invalid session id");
        return (StatusCode::UNAUTHORIZED, Json(RefreshResponse::failure()));
    }

    (StatusCode::OK, Json(RefreshResponse::success(user_session.unwrap())))
}
