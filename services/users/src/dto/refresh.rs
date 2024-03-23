use secrecy::Secret;
use serde::{Deserialize, Serialize};

use crate::users::UserSession;

#[derive(Debug, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: Secret<String>,
}

#[derive(Debug, Serialize)]
pub struct RefreshResponse {
    pub successful: bool,

    #[serde(flatten)]
    pub session: Option<UserSession>,
}

impl RefreshResponse {
    pub fn success(session: UserSession) -> Self {
        RefreshResponse { successful: true, session: Some(session) }
    }

    pub fn failure() -> Self {
        RefreshResponse { successful: false, session: None }
    }
}
