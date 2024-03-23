mod database;
mod routes;
mod services;
mod dto;

use clap::Parser;
use database::Database;
use message_bus::{ServiceEventConsumer, ServiceEventProducer};
use routes::run_service_routes;
use service_sso_api::ServiceSSO;
use services::Services;
use std::sync::Arc;

use crate::services::{
    data_processing::run_data_processing, message_processing::run_message_processing,
};

#[derive(Debug, Parser)]
struct Environment {
    #[clap(env)]
    database_url: String,

    #[clap(env)]
    redis_url: String,

    #[clap(env)]
    otel_metrics_url: String,

    #[clap(env)]
    service_sso_url: String,

    #[clap(env, default_value_t = 8080)]
    service_port: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let environment = Environment::parse();

    logging::setup_tracing(env!("CARGO_PKG_NAME"), &environment.otel_metrics_url);

    let database = Database::connect(&environment.database_url).await?;

    let consumer =
        ServiceEventConsumer::create(&environment.redis_url, "wallet-service").await?;

    let producer = ServiceEventProducer::create(&environment.redis_url).await?;

    let service_sso = ServiceSSO::new(&environment.service_sso_url);

    let service_state: Arc<Services> = Arc::new(Services {
        database,
        consumer,
        producer,
        service_sso,
    });

    let service_port = environment.service_port;

    tokio::try_join!(
        run_service_routes(service_port, service_state.clone()),
        run_message_processing(service_state.clone()),
        run_data_processing(service_state.clone()),
    )?;

    Ok(())
}
