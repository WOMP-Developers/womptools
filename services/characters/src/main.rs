mod database;
mod dto;
mod message_processing;
mod data_processing;
mod routes;

use clap::Parser;
use message_bus::ServiceEventConsumer;
use routes::run_service_routes;
use std::sync::Arc;

use crate::{data_processing::run_data_processing, database::Database, message_processing::run_message_processing};

#[derive(Debug, Parser)]
struct Environment {
    #[clap(env)]
    database_url: String,

    #[clap(env)]
    redis_url: String,

    #[clap(env)]
    otel_metrics_url: String,

    #[clap(env)]
    esi_client_id: String,

    #[clap(env)]
    esi_secret_key: String,

    #[clap(env, default_value_t = 8080)]
    service_port: u16,
}

#[derive(Debug)]
struct Service {
    db: Database,
    event_consumer: ServiceEventConsumer,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let environment = Environment::parse();

    logging::setup_tracing(env!("CARGO_PKG_NAME"), &environment.otel_metrics_url);

    let database = Database::connect(&environment.database_url).await?;

    let event_consumer =
        ServiceEventConsumer::create(&environment.redis_url, "characters-service").await?;

    let service_state: Arc<Service> = Arc::new(Service {
        db: database,
        event_consumer,
    });

    let service_port = environment.service_port;

    tokio::try_join!(
        run_service_routes(service_port, service_state.clone()),
        run_message_processing(service_state.clone()),
        run_data_processing(service_state.clone()),
    )?;

    Ok(())
}
