mod character_list;
mod character_main;
mod character_by_id;
mod status;

use std::sync::Arc;

use axum::{routing::get, Router};

use crate::Service;

use status::status;
use character_main::character_main;
use character_list::character_list;
use character_by_id::character_by_id;

async fn serve_routes(service_port: u16, service: Arc<Service>) -> anyhow::Result<()> {
    let router = Router::new()
        .route("/v1/characters/status", get(status))
        .route("/v1/characters/list", get(character_list))
        .route("/v1/characters/main", get(character_main))
        .route("/v1/characters/:id", get(character_by_id))
        .with_state(service);

    let bind_address = format!("0.0.0.0:{}", service_port);
    let listener = tokio::net::TcpListener::bind(&bind_address).await?;

    tracing::info!("🚀 started listening on {}", listener.local_addr()?);

    axum::serve(listener, router)
        .await
        .map_err(|err| anyhow::Error::new(err))
}

pub async fn run_service_routes(service_port: u16, service: Arc<Service>) -> anyhow::Result<()> {
    let join_handle = tokio::spawn(async move {
        serve_routes(service_port, service).await
    });

    join_handle.await?
}

