use serde::{Deserialize, Serialize};

use crate::users::UserSession;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub authorization_code: String,
    pub client_ip: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub successful: bool,

    #[serde(flatten)]
    pub session: Option<UserSession>,
}

impl LoginResponse {
    pub fn success(session: UserSession) -> Self {
        LoginResponse { successful: true, session: Some(session) }
    }

    pub fn failure() -> Self {
        LoginResponse { successful: false, session: None }
    }
}
