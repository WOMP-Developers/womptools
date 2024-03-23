use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

use crate::{access_token::AccessTokenClaims, refresh_token::RefreshTokenClaims};

pub struct AccessTokenBuilder {
    encoding_key: EncodingKey,
}

impl AccessTokenBuilder {
    pub fn new(rsa_private_key: &str) -> Self {
        let encoding_key =
            EncodingKey::from_rsa_pem(rsa_private_key.as_bytes()).expect("invalid rsa private key");

        AccessTokenBuilder { encoding_key }
    }

    #[tracing::instrument(skip(self))]
    pub fn build_access_token(&self, user_id: u64, session_id: &str) -> anyhow::Result<String> {
        // let expire_duration =
        //     Duration::try_hours(2).ok_or(anyhow::Error::msg("couldn't create expiration time"))?;

        let expire_duration =
            Duration::try_minutes(20).ok_or(anyhow::Error::msg("couldn't create expiration time"))?;

        let expire_at = Utc::now()
            .checked_add_signed(expire_duration)
            .and_then(|t| Some(t.timestamp()))
            .ok_or(anyhow::Error::msg("couldn't create expiration time"))?;
        let issued_at = Utc::now().timestamp();

        let claims = AccessTokenClaims {
            sub: user_id,
            exp: expire_at as usize,
            iat: issued_at as usize,
            sid: session_id.to_string(),
        };

        let header = Header::new(Algorithm::RS512);

        let jwt = encode(&header, &claims, &self.encoding_key)?;

        Ok(jwt)
    }

    #[tracing::instrument(skip(self))]
    pub fn build_refresh_token(&self, session_id: &str) -> anyhow::Result<String> {
        let claims = RefreshTokenClaims {
            sub: session_id.to_string(),
        };

        let header = Header::new(Algorithm::RS512);

        let jwt = encode(&header, &claims, &self.encoding_key)?;

        Ok(jwt)
    }
}
