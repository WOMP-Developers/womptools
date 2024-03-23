use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct RegisterCharacterRequest {
    pub authorization_code: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterCharacterResponse {
    pub successful: bool,
}

impl RegisterCharacterResponse {
    pub fn success() -> Self {
        RegisterCharacterResponse { successful: true }
    }

    pub fn failure() -> Self {
        RegisterCharacterResponse { successful: false }
    }
}
