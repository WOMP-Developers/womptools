use std::sync::Arc;

use axum::{routing::{get, post}, Router};

mod status;
mod register;
mod refresh;

use status::status;
use register::register;
use refresh::refresh;

use crate::services::Services;

#[tracing::instrument(skip(state))]
async fn serve_routes(service_port: u16, state: Arc<Services>) -> anyhow::Result<()> {
    let router = Router::new()
        .route("/v1/sso/status", get(status))
        .route("/v1/sso/register", post(register))
        .route("/v1/sso/refresh", post(refresh))
        .with_state(state);

    let bind_address = format!("0.0.0.0:{}", service_port);
    let listener = tokio::net::TcpListener::bind(&bind_address).await?;

    tracing::info!("🚀 started listening on {}", listener.local_addr()?);

    axum::serve(listener, router)
        .await
        .map_err(|err| anyhow::Error::new(err))
}

#[tracing::instrument(skip(state))]
pub async fn run_service_routes(service_port: u16, state: Arc<Services>) -> anyhow::Result<()> {
    let join_handle = tokio::spawn(async move {
        serve_routes(service_port, state).await
    });

    join_handle.await?
}