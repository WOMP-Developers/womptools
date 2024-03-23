use std::sync::Arc;

use clap::Parser;
use database::Database;
use eve_sso::EveSSO;
use message_bus::{ServiceEventConsumer, ServiceEventProducer};
use services::{credentials_manager::CredentialsManager, Services};

use crate::{routes::run_service_routes, services::message_processing::run_message_processing};

mod database;
mod routes;
mod services;

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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let environment = Environment::parse();

    logging::setup_tracing(env!("CARGO_PKG_NAME"), &environment.otel_metrics_url);

    let database = Database::connect(&environment.database_url).await?;

    let consumer = ServiceEventConsumer::create(&environment.redis_url, "sso-service").await?;
    let producer = ServiceEventProducer::create(&environment.redis_url).await?;

    let sso = EveSSO::new(&environment.esi_client_id, &environment.esi_secret_key);

    let credentials_manager = CredentialsManager::new(database, producer, sso);

    let services = Arc::new(Services {
        credentials_manager,
        consumer,
    });

    tokio::try_join!(
        run_service_routes(environment.service_port, services.clone()),
        run_message_processing(services.clone())
    )?;

    Ok(())
}
