use jsonwebtoken::{jwk::JwkSet, DecodingKey};
use serde::Deserialize;
use tokio::sync::OnceCell;

const SSO_METADATA_URL: &'static str =
    "https://login.eveonline.com/.well-known/oauth-authorization-server";

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub token_endpoint: String,
    jwks_uri: String,
}

#[tracing::instrument]
async fn fetch_metadata() -> anyhow::Result<Metadata> {
    let client = reqwest::Client::new();

    let response = client.get(SSO_METADATA_URL).send().await?;
    let metadata = response.json::<Metadata>().await?;

    Ok(metadata)
}

#[tracing::instrument]
async fn fetch_jwks(jwks_uri: &str) -> anyhow::Result<JwkSet> {
    let client = reqwest::Client::new();

    let response = client.get(jwks_uri).send().await?;
    let jwks = response.json().await?;

    Ok(jwks)
}

static METADATA: OnceCell<Metadata> = OnceCell::const_new();

pub async fn get_metadata() -> &'static Metadata {
    METADATA
        .get_or_init(|| async { fetch_metadata().await.expect("couldn't fetch metadata") })
        .await
}

static DECODING_KEY: OnceCell<DecodingKey> = OnceCell::const_new();

pub async fn get_decoding_key() -> &'static DecodingKey {
    DECODING_KEY
        .get_or_init(|| async {
            let metadata = get_metadata().await;
            let jwks = fetch_jwks(&metadata.jwks_uri).await.expect("fetch jwks");
            let jwk = jwks.keys.first().expect("at least one jwk");
            DecodingKey::from_jwk(jwk).expect("decode jwk")
        })
        .await
}
