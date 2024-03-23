use std::collections::HashSet;

use jsonwebtoken::{decode, Algorithm, Validation};
use serde::{Deserialize, Serialize};

use crate::keys::DECODING_KEY;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct RefreshTokenClaims {
    pub sub: String,
}

#[derive(Debug)]
pub struct RefreshToken {
    pub session_id: String,
}

pub fn decode_refresh_token(refresh_token: &str) -> anyhow::Result<RefreshToken> {
    let mut validation = Validation::new(Algorithm::RS512);
    validation.validate_exp = false;
    validation.required_spec_claims = HashSet::from(["sub".to_string()]);

    // TODO: Validate more properties
    // validation.set_issuer(&[JWT_ISSUER_HOSTNAME, JWT_ISSUER_URI]);
    // validation.set_audience(&[self.client_id.as_str(), JWT_AUDIENCE]);

    let decoded = decode::<RefreshTokenClaims>(refresh_token, &DECODING_KEY, &validation)?;

    Ok(RefreshToken {
        session_id: decoded.claims.sub,
    })
}
