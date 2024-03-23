use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct OAuthLoginRequestBody {
    pub grant_type: String,
    pub code: String,
}

#[derive(Debug, Serialize)]
pub struct OAuthRefreshRequestBody {
    pub grant_type: String,
    pub refresh_token: String,
}

#[derive(Debug, Deserialize)]
pub struct OAuthTokensResponse {
    pub access_token: String,
    pub refresh_token: String,
}


