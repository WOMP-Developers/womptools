use dto::{RefreshRequest, RefreshResponse, RegisterRequest, RegisterResponse};

pub mod dto;

pub struct ServiceSSO {
    endpoint: String,
    client: reqwest::Client,
}

impl ServiceSSO {
    pub fn new(endpoint: &str) -> Self {
        let client = reqwest::Client::new();

        ServiceSSO {
            endpoint: endpoint.to_string(),
            client,
        }
    }

    pub async fn register(&self, request: RegisterRequest) -> anyhow::Result<RegisterResponse> {
        let url = format!("{}/v1/sso/register", self.endpoint);
        let body = serde_json::to_string(&request)?;

        let response = self
            .client
            .post(&url)
            .body(body)
            .header("Content-Type", "application/json")
            .send()
            .await?;

        let response = response.json::<RegisterResponse>().await?;

        Ok(response)
    }

    pub async fn refresh(&self, request: RefreshRequest) -> anyhow::Result<RefreshResponse> {
        let url = format!("{}/v1/sso/refresh", self.endpoint);
        let body = serde_json::to_string(&request)?;

        let response = self
            .client
            .post(&url)
            .body(body)
            .header("Content-Type", "application/json")
            .send()
            .await?;

        let response = response.json::<RefreshResponse>().await?;

        Ok(response)
    }
}
