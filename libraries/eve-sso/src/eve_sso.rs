use base64::{engine::general_purpose, Engine as _};

use crate::{
    dto::{
        oauth::{OAuthLoginRequestBody, OAuthRefreshRequestBody, OAuthTokensResponse},
        token::Tokens,
    },
    error::{ErrorKind, Result},
    keys::get_metadata, parse::parse_access_token,
};

#[derive(Debug)]
pub struct EveSSO {
    client_id: String,
    secret_key: String,
}

impl EveSSO {
    pub fn new(client_id: &str, secret_key: &str) -> Self {
        EveSSO {
            client_id: client_id.to_string(),
            secret_key: secret_key.to_string(),
        }
    }

    #[tracing::instrument]
    fn create_sso_authentication_string(&self) -> String {
        let authorization = format!("{}:{}", self.client_id, self.secret_key);
        let authorization = general_purpose::STANDARD.encode(&authorization);

        format!("Basic {}", authorization)
    }

    #[tracing::instrument(skip(authorization_code))]
    pub async fn oauth_authorize(&self, authorization_code: &str) -> Result<Tokens> {
        let metadata = get_metadata().await;
        let token_endpoint = &metadata.token_endpoint;

        let request_body = OAuthLoginRequestBody {
            grant_type: "authorization_code".to_string(),
            code: authorization_code.to_string(),
        };

        let request_body = serde_urlencoded::to_string(&request_body)?;
        let authorization = self.create_sso_authentication_string();

        let client = reqwest::Client::new();
        let response = client
            .post(token_endpoint)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", authorization)
            .header("Host", "login.eveonline.com")
            .body(request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            if let Ok(response) = response.text().await {
                tracing::error!(?response, "authorization request failed");
            } else {
                tracing::error!("authorization request failed");
            }

            // TODO: Parse the response text further to better know what went wrong.

            return Err(ErrorKind::AuthorizationError.into())
        }

        let tokens_response = response.json::<OAuthTokensResponse>().await?;

        let access_token = parse_access_token(&tokens_response.access_token)
            .await?;

        Ok(Tokens {
            access_token,
            refresh_token: tokens_response.refresh_token.to_string(),
        })
    }

    #[tracing::instrument(skip(refresh_token))]
    pub async fn oauth_refresh(&self, refresh_token: &str) -> Result<Tokens> {
        let metadata = get_metadata().await;
        let token_endpoint = &metadata.token_endpoint;

        let request_body = OAuthRefreshRequestBody {
            grant_type: "refresh_token".to_string(),
            refresh_token: refresh_token.to_string(),
        };

        let request_body = serde_urlencoded::to_string(&request_body)?;
        let authorization = self.create_sso_authentication_string();

        let client = reqwest::Client::new();
        let response = client
            .post(token_endpoint)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", authorization)
            .header("Host", "login.eveonline.com")
            .body(request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            if let Ok(response) = response.text().await {
                tracing::error!(?response, "refresh request failed");
            } else {
                tracing::error!("refresh request failed");
            }

            // TODO: Parse the response text further to better know what went wrong.

            return Err(ErrorKind::AuthorizationError.into())
        }

        let tokens_response = response.json::<OAuthTokensResponse>().await?;
        let access_token = parse_access_token(&tokens_response.access_token)
            .await?;

        Ok(Tokens {
            access_token,
            refresh_token: tokens_response.refresh_token.to_string(),
        })
    }
}
