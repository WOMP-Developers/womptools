use std::{collections::HashMap, sync::Arc};

use eve_sso::parse::parse_access_token;
use message_bus::{ServiceEventConsumer, StreamEvent, StreamMessage};
use opentelemetry::global as otel;
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::database::Database;

use super::Services;

#[tracing::instrument(skip(access_token, database))]
async fn handle_character_token(
    user_id: u64,
    access_token: &str,
    database: &Database,
) {
    let parsed_token = parse_access_token(access_token).await;
    
    if let Err(err) = parsed_token {
        tracing::error!(?err, "couldn't parse access token");

        return;
    }

    if let Err(err) = database.update_credentials(user_id, &parsed_token.unwrap()).await {
        tracing::error!(?err, "couldn't update credentials");
    }
}

#[tracing::instrument(skip(stream_event, database))]
async fn process_stream_event(stream_event: &StreamMessage, database: &Database) {
    if let Some(ctx) = &stream_event.ctx {
        if let Ok(carrier) = serde_json::from_str::<HashMap<String, String>>(ctx) {
            let parent_context =
                otel::get_text_map_propagator(|propagator| propagator.extract(&carrier));

            Span::current().set_parent(parent_context);
        }
    }

    // TODO: Handle CharacterRegistered events and update their wallet
    // immediately after being registered + receiving access token. This
    // way users would not have to wait for the next refresh before seeing
    // their wallet balance.

    match &stream_event.evt {
        Some(StreamEvent::CharacterAccessToken {
            user_id,
            access_token,
            ..
        }) => handle_character_token(*user_id, access_token, database).await,
        _ => {}
    }
}

async fn read_and_process_events(
    consumer: &ServiceEventConsumer,
    database: &Database,
) -> anyhow::Result<()> {
    let stream_events = consumer.read_events().await?;

    for stream_event in &stream_events {
        process_stream_event(stream_event, database).await;
    }

    consumer.ack_events(&stream_events).await?;

    Ok(())
}

async fn process_event_loop(services: Arc<Services>) -> anyhow::Result<()> {
    loop {
        read_and_process_events(&services.consumer, &services.database).await?;
    }
}

pub async fn run_message_processing(services: Arc<Services>) -> anyhow::Result<()> {
    tokio::spawn(async move { process_event_loop(services).await }).await?
}
