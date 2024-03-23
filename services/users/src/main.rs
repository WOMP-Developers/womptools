mod database;
mod dto;
mod users;
mod routes;

use auth_token::AccessTokenBuilder;
use axum::{
    routing::{get, post},
    Router,
};
use clap::Parser;
use database::Database;
use eve_sso::EveSSO;
use message_bus::ServiceEventProducer;
use service_sso_api::ServiceSSO;
use std::sync::Arc;
use users::Users;

use crate::routes::{character, login, logout, refresh};

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

    #[clap(env)]
    esi_client_id: String,

    #[clap(env)]
    esi_secret_key: String,

    #[clap(env, default_value_t = 8080)]
    service_port: u16,
}

struct Service {
    sso: EveSSO,
    users: Users,
    message_producer: ServiceEventProducer,
    service_sso: ServiceSSO,
}

// TODO: Select this file in some better way to avoid hardcoding path.
const ACCESS_TOKEN_PRIVATE_KEY: &'static str =
    include_str!("../../../keys/access_token.private.pem");
const JWKS: &'static str = include_str!("../../../keys/jwks.json");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    
    let environment = Environment::parse();

    logging::setup_tracing(env!("CARGO_PKG_NAME"), &environment.otel_metrics_url);

    let sso = EveSSO::new(&environment.esi_client_id, &environment.esi_secret_key);
    let database = Database::connect(&environment.database_url).await?;
    let message_producer = ServiceEventProducer::create(&environment.redis_url).await?;
    let token_builder = AccessTokenBuilder::new(ACCESS_TOKEN_PRIVATE_KEY);
    let users = Users::new(database, message_producer.clone(), token_builder);
    let service_sso = ServiceSSO::new(&environment.service_sso_url);

    let service_state: Arc<Service> = Arc::new(Service {
        sso,
        users,
        message_producer,
        service_sso,
    });

    let service_builder = Router::new()
        .route("/v1/users/status", get(status))
        .route("/v1/users/login", post(login))
        .route("/v1/users/refresh", post(refresh))
        .route("/v1/users/logout", post(logout))
        .route("/v1/users/character", post(character))
        .route("/v1/users/jwks", get(jwks))
        .with_state(service_state);

    let bind_address = format!("0.0.0.0:{}", environment.service_port);
    let listener = tokio::net::TcpListener::bind(&bind_address).await?;

    tracing::info!("🚀 started listening on {}", listener.local_addr()?);

    axum::serve(listener, service_builder).await?;

    Ok(())
}

#[tracing::instrument]
async fn jwks() -> &'static str {
    JWKS
}

#[tracing::instrument]
async fn status() -> &'static str {
    "OK"
}
