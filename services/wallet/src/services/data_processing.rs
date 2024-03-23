use std::{sync::Arc, time::Duration};

use chrono::Utc;
use eve_esi::api::{get_wallet, get_wallet_journal};
use message_bus::ServiceEventProducer;
use tokio::time::sleep;

use crate::database::Database;

use super::Services;

async fn update_character_wallet_journal(
    character_id: u64,
    access_token: &str,
    database: &Database,
) -> anyhow::Result<()> {
    let last_journal_id = database.select_last_journal_id(character_id).await?;
    let wallet_journal = get_wallet_journal(character_id, access_token, last_journal_id).await?;

    if wallet_journal.len() > 0 {
        if let Err(err) = database
            .update_wallet_journal(character_id, &wallet_journal)
            .await
        {
            tracing::error!(?err, character_id, "error updating wallet journal");
            return Err(err);
        }

        tracing::info!(character_id, "added {} wallet journal entries", wallet_journal.len());
    }

    Ok(())
}

#[tracing::instrument(skip(access_token, database))]
async fn update_character_wallet_balance(
    character_id: u64,
    access_token: &str,
    database: &Database,
) -> anyhow::Result<()> {
    match get_wallet(character_id, &access_token).await {
        Ok(wallet_balance) => {
            tracing::info!(wallet_balance, character_id, "fetched wallet balance");
            database
                .update_wallet_balance(character_id, wallet_balance as i64)
                .await?;
        }
        Err(err) => tracing::error!(?err, character_id, "couldn't fetch wallet balance"),
    }

    Ok(())
}

async fn update_character_wallet(
    character_id: u64,
    access_token: &str,
    database: &Database,
) -> anyhow::Result<()> {
    update_character_wallet_balance(character_id, access_token, database).await?;
    update_character_wallet_journal(character_id, access_token, database).await?;

    Ok(())
}

async fn process_characters(
    database: &Database,
    producer: &ServiceEventProducer,
) -> anyhow::Result<bool> {
    let update_characters = database
        .select_characters_for_processing(20)
        .await?;

    let tokens_requested = if update_characters.len() > 0 {
        let now = Utc::now();
        let (update_characters, expired_characters): (_, Vec<_>) = update_characters
            .into_iter()
            .partition(|c| c.access_token_expire_at > now);

        let refresh_futures: Vec<_> = expired_characters
            .iter()
            .map(|character| {
                producer.send_event(message_bus::StreamEvent::RequestCharacterAccessToken {
                    character_id: character.id,
                })
            })
            .collect();

        let results = futures::future::try_join_all(refresh_futures).await?;
        let tokens_requested = results.len();

        tracing::info!("requested tokens for {} characters", tokens_requested);

        let update_futures: Vec<_> = update_characters
            .iter()
            .map(|character| {
                update_character_wallet(character.id, &character.access_token, database)
            })
            .collect();

        let _ = futures::future::join_all(update_futures).await;

        tokens_requested
    } else {
        0
    };

    Ok(tokens_requested > 0)
}

pub async fn run_data_processing(services: Arc<Services>) -> anyhow::Result<()> {
    let join_handle = tokio::spawn(async move {
        loop {
            let process_more = process_characters(&services.database, &services.producer).await;

            if let Err(err) = &process_more {
                tracing::error!(?err, "error processing characters");
            }

            if process_more.unwrap() {
                sleep(Duration::from_secs(10)).await;
            } else {
                sleep(Duration::from_secs(60)).await;
            }
        }
    });

    join_handle.await?
}
