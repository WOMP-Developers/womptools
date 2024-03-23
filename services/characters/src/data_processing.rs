use std::{sync::Arc, time::Duration};

use eve_esi::api::get_character;
use tokio::time::sleep;

use crate::{database::Database, Service};

#[tracing::instrument(skip(db))]
async fn update_character(character_id: u64, db: &Database) -> anyhow::Result<()> {
    match get_character(character_id).await {
        Ok(character) => db.insert_character_data(character_id, &character).await?,
        Err(err) => tracing::error!(?err, character_id, "couldn't get character data"),
    }

    Ok(())
}

async fn data_processing_loop(services: Arc<Service>) -> anyhow::Result<()> {
    loop {
        let update_ids = services
            .db
            .select_characters_for_data_processing(20)
            .await?;

        if update_ids.len() > 0 {
            let update_futures: Vec<_> = update_ids
                .iter()
                .map(|character_id| update_character(*character_id, &services.db))
                .collect();

            let results = futures::future::join_all(update_futures).await;

            tracing::info!("updated character data for {} characters", results.len());
        }

        sleep(Duration::from_secs(240)).await;
    }
}

pub async fn run_data_processing(services: Arc<Service>) -> anyhow::Result<()> {
    let join_handle = tokio::spawn(async {
        data_processing_loop(services).await?;

        Ok(())
    });

    join_handle.await?
}
