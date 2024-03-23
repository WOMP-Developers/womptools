use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug)]
pub struct AccessToken {
    pub character_id: u64,
    pub access_token: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct AccessTokenClaims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Debug)]
pub struct Tokens {
    pub access_token: AccessToken,
    pub refresh_token: String,
}
