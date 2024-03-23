use jsonwebtoken::{jwk::JwkSet, DecodingKey};
use once_cell::sync::Lazy;

const JWKS: &'static str = include_str!("../../../keys/jwks.json");

pub static DECODING_KEY: Lazy<DecodingKey> = Lazy::new(|| {
    let jwks = serde_json::from_str::<JwkSet>(JWKS).expect("invalid jwks");
    let jwk = jwks.keys.first().expect("missing jwk");
    DecodingKey::from_jwk(jwk).expect("valid jwk")
});
