use std::{collections::HashSet, fmt::Display};

use axum::{async_trait, extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use chrono::{DateTime, Utc};
use jsonwebtoken::{decode, Algorithm, Validation};
use serde::{Deserialize, Serialize};

use crate::{error::TokenError, keys::DECODING_KEY};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct AccessTokenClaims {
    pub sub: u64,
    pub iat: usize,
    pub exp: usize,
    pub sid: String,
}

#[derive(Debug)]
pub struct AccessToken {
    pub user_id: u64,
    pub session_id: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub raw: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AccessToken
where
    S: Send + Sync,
{
    type Rejection = TokenError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|err| {
                tracing::error!(?err, "extraction error");
                TokenError::TokenInvalid
            })?;

        let res = decode_access_token(bearer.token());

        if let Err(err) = &res {
            tracing::error!(?err, "couldn't decode access token");
        }

        res
    }
}

impl Display for AccessToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "User ID: {}\nCreated At: {}\nExpires At: {}",
            self.user_id, self.created_at, self.expires_at
        )
    }
}

pub fn decode_access_token(access_token: &str) -> Result<AccessToken, TokenError> {
    let mut validation = Validation::new(Algorithm::RS512);
    validation.validate_exp = true;
    validation.required_spec_claims =
        HashSet::from(["iat".to_string(), "exp".to_string(), "sid".to_string()]);

    // TODO: Validate more properties
    // validation.set_issuer(&[JWT_ISSUER_HOSTNAME, JWT_ISSUER_URI]);
    // validation.set_audience(&[self.client_id.as_str(), JWT_AUDIENCE]);

    let decoded =
        decode::<AccessTokenClaims>(access_token, &DECODING_KEY, &validation).map_err(|err| {
            tracing::error!(?err, "token validation error");
            TokenError::TokenInvalid
        })?;

    let created_at =
        DateTime::from_timestamp(decoded.claims.iat as i64, 0).ok_or(TokenError::TokenInvalid)?;

    let expires_at =
        DateTime::from_timestamp(decoded.claims.exp as i64, 0).ok_or(TokenError::TokenInvalid)?;

    Ok(AccessToken {
        user_id: decoded.claims.sub,
        session_id: decoded.claims.sid,
        created_at,
        expires_at,
        raw: access_token.to_string(),
    })
}
