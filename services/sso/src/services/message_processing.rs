use std::{collections::HashMap, sync::Arc};

use message_bus::{ServiceEventConsumer, StreamEvent, StreamMessage};
use opentelemetry::global as otel;
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;

use super::{credentials_manager::CredentialsManager, Services};

#[tracing::instrument(skip(credentials_manager))]
async fn handle_request_character_access_token(
    character_id: u64,
    credentials_manager: &CredentialsManager,
) {
    if let Err(err) = credentials_manager
        .refresh_character_credentials(character_id)
        .await
    {
        tracing::error!(?err, "couldn't refresh character credentials");

        // TODO: Depending on what kind of error it is it might be good to mark the credentials
        // requiring re-authorization in the database. This information could be important for
        // the users of this service.
    }
}

#[tracing::instrument(skip(stream_event, credentials_manager))]
async fn process_stream_event(stream_event: &StreamMessage, credentials_manager: &CredentialsManager) {
    if let Some(ctx) = &stream_event.ctx {
        if let Ok(carrier) = serde_json::from_str::<HashMap<String, String>>(ctx) {
            let parent_context =
                otel::get_text_map_propagator(|propagator| propagator.extract(&carrier));

            Span::current().set_parent(parent_context);
        }
    }

    match &stream_event.evt {
        Some(StreamEvent::RequestCharacterAccessToken { character_id }) => {
            handle_request_character_access_token(*character_id, &credentials_manager).await
        }
        _ => {}
    }
}

async fn read_and_process_events(
    consumer: &ServiceEventConsumer,
    credentials_manager: &CredentialsManager,
) -> anyhow::Result<()> {

    let stream_events = consumer.read_events().await?;

    for stream_event in &stream_events {
        process_stream_event(stream_event, credentials_manager).await;
    }

    consumer.ack_events(&stream_events).await?;

    Ok(())
}

async fn process_event_loop(services: Arc<Services>) -> anyhow::Result<()> {
    loop {
        read_and_process_events(&services.consumer, &services.credentials_manager).await?;
    }
}

pub async fn run_message_processing(services: Arc<Services>) -> anyhow::Result<()> {
    tokio::spawn(async move { process_event_loop(services).await }).await?
}
