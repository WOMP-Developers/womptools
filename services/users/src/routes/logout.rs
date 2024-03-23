use auth_token::AccessToken;
use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use message_bus::StreamEvent;
use std::sync::Arc;

use crate::{dto::logout::LogoutResponse, Service};

#[tracing::instrument(skip(service))]
pub async fn logout(
    State(service): State<Arc<Service>>,
    access_token: AccessToken,
) -> (StatusCode, Json<LogoutResponse>) {

    if let Err(err) = service.users.delete_user_session(access_token.user_id, &access_token.session_id).await {
        tracing::error!(?err, "couldn't delete user session");
    }

    if let Err(err) = service.message_producer.send_event(StreamEvent::UserLogout { user_id: access_token.user_id }).await {
        tracing::error!(?err, "couldn't send UserLogout message");
    }

    (StatusCode::OK, Json(LogoutResponse::success()))
}