use std::sync::Arc;

use axum::{routing::get, Router};

mod status;
mod get_balance;
mod get_user_balance;
mod get_bounty_summary;
mod get_balance_summary;
mod get_journal;

use status::status;
use get_balance::get_balance;
use get_user_balance::get_user_balance;
use get_bounty_summary::get_bounty_summary;
use get_balance_summary::get_balance_summary;
use get_journal::get_journal;

use crate::services::Services;

#[tracing::instrument(skip(state))]
async fn serve_routes(service_port: u16, state: Arc<Services>) -> anyhow::Result<()> {
    let router = Router::new()
        .route("/v1/wallet/status", get(status))
        .route("/v1/wallet/balance", get(get_user_balance))
        .route("/v1/wallet/:character_id/balance", get(get_balance))
        .route("/v1/wallet/hist/bounty", get(get_bounty_summary))
        .route("/v1/wallet/hist/balance", get(get_balance_summary))
        .route("/v1/wallet/:character_id/journal/:page", get(get_journal))
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