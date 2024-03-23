use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde_json::json;

#[derive(Debug)]
pub enum TokenError {
    TokenExpired,
    TokenInvalid,
}

impl IntoResponse for TokenError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            TokenError::TokenExpired => (StatusCode::UNAUTHORIZED, "Expired token"),
            TokenError::TokenInvalid => (StatusCode::UNAUTHORIZED, "Invalid token"),
        };

        let body = Json(json!({
            "successful": false,
            "message": error_message,
        }));

        (status, body).into_response()
    }
}