use chrono::DateTime;
use jsonwebtoken::{Algorithm, Validation};

use crate::{dto::token::{AccessToken, AccessTokenClaims}, error::{Result, ErrorKind}, keys::get_decoding_key};

const JWT_ISSUER_HOSTNAME: &'static str = "login.eveonline.com";
const JWT_ISSUER_URI: &'static str = "https://login.eveonline.com";
const JWT_AUDIENCE: &'static str = "EVE Online";

#[tracing::instrument(skip(access_token))]
async fn validate_and_decode_access_token_claims(access_token: &str) -> Result<AccessTokenClaims> {
    let key = get_decoding_key().await;

    let mut validation: Validation = Validation::new(Algorithm::RS256);
    validation.set_issuer(&[JWT_ISSUER_HOSTNAME, JWT_ISSUER_URI]);
    validation.set_audience(&[JWT_AUDIENCE]);

    let decoded_token = jsonwebtoken::decode::<AccessTokenClaims>(access_token, &key, &validation)?;

    Ok(decoded_token.claims)
}

#[tracing::instrument(skip(access_token))]
pub async fn parse_access_token(access_token: &str) -> Result<AccessToken> {
    let claims = validate_and_decode_access_token_claims(access_token)
        .await?;

    let character_id = claims
        .sub
        .split(":")
        .nth(2)
        .and_then(|cid| cid.parse::<u64>().ok())
        .ok_or(ErrorKind::ParseSubjectError)?;

    let expires_at =
        DateTime::from_timestamp(claims.exp as i64, 0).ok_or(ErrorKind::ParseExpiredError)?;

    Ok(AccessToken {
        character_id,
        expires_at,
        access_token: access_token.to_string(),
    })
}
