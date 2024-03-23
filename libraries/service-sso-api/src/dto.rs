use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterRequest {
    pub user_id: u64,
    pub refresh_token: String,
    pub access_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterResponse {
    pub successful: bool,
    pub message: Option<String>,
}

impl RegisterResponse {
    pub fn successful() -> Self {
        RegisterResponse {
            successful: true,
            message: None,
        }
    }

    pub fn failure(msg: &str) -> Self {
        RegisterResponse {
            successful: false,
            message: Some(msg.to_string()),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RefreshRequest {
    pub user_access_token: String,
    pub character_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshResponse {
    pub successful: bool,
    pub message: Option<String>,
    pub access_token: Option<String>,
}

impl RefreshResponse {
    pub fn successful(access_token: &str) -> Self {
        RefreshResponse {
            successful: true,
            access_token: Some(access_token.to_string()),
            message: None,
        }
    }

    pub fn failure(msg: &str) -> Self {
        RefreshResponse {
            successful: false,
            message: Some(msg.to_string()),
            access_token: None,
        }
    }
}