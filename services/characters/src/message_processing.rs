use std::{collections::HashMap, sync::Arc};

use eve_esi::api::get_character;
use eve_sso::parse::parse_access_token;
use message_bus::StreamMessage;
use opentelemetry::global as otel;
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::{database::Database, Service};

#[tracing::instrument(skip(db))]
async fn registered_character(
    user_id: u64,
    character_id: u64,
    db: &Database,
) -> anyhow::Result<()> {
    db.insert_character(user_id, character_id).await?;

    match get_character(character_id).await {
        Ok(character) => db.insert_character_data(character_id, &character).await,
        Err(err) => Err(err),
    }
}

#[tracing::instrument(skip(db))]
async fn character_token_invalid(character_id: u64, db: &Database) -> anyhow::Result<()> {
    db.update_character_token_valid(character_id, false).await?;

    Ok(())
}

#[tracing::instrument(skip(stream_event, db))]
async fn process_stream_event(stream_event: &StreamMessage, db: &Database) {
    if let Some(ctx) = &stream_event.ctx {
        if let Ok(carrier) = serde_json::from_str::<HashMap<String, String>>(ctx) {
            let parent_context =
                otel::get_text_map_propagator(|propagator| propagator.extract(&carrier));

            Span::current().set_parent(parent_context);
        }
    }

    let result = match &stream_event.evt {
        Some(message_bus::StreamEvent::CharacterRegistered {
            user_id,
            character_id,
        }) => registered_character(*user_id, *character_id, db).await,
        Some(message_bus::StreamEvent::CharacterAccessToken { access_token, .. }) => {
            if let Ok(parsed) = parse_access_token(&access_token).await {
                db.update_character_token_valid(parsed.character_id, true).await
            } else {
                tracing::warn!("received invalid access token, skipping");
                Ok(())
            }
        }
        Some(message_bus::StreamEvent::CharacterTokenInvalid { character_id }) => {
            db.update_character_token_valid(*character_id, false).await
        },
        _ => { Ok(()) }
    };

    if let Err(err) = result {
        tracing::error!(?err, "error processing service message");
    }
}

async fn process_service_messages(service: Arc<Service>) -> anyhow::Result<()> {
    loop {
        let stream_events = service.event_consumer.read_events().await?;

        for stream_event in &stream_events {
            process_stream_event(stream_event, &service.db).await;
        }

        service.event_consumer.ack_events(&stream_events).await?;
    }
}

pub async fn run_message_processing(services: Arc<Service>) -> anyhow::Result<()> {
    tokio::spawn(async { process_service_messages(services).await }).await?
}
