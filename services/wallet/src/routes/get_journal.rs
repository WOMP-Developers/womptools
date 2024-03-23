use std::{sync::Arc, vec};

use auth_token::AccessToken;
use axum::{
    extract::{Path, State},
    Json,
};
use bigdecimal::ToPrimitive;
use reqwest::StatusCode;
use serde::Serialize;

use crate::{dto::JournalEntry, services::Services};

const ENTRIES_PER_PAGE: usize = 20;

#[derive(Debug, Serialize)]
pub struct JournalResponse {
    successful: bool,
    page_count: usize,
    entries: Vec<JournalEntry>,
}

pub async fn get_journal(
    State(services): State<Arc<Services>>,
    _access_token: AccessToken,
    Path((character_id, page)): Path<(u64, usize)>,
) -> (StatusCode, Json<JournalResponse>) {
    let entry_count = services
        .database
        .select_journal_entry_count(character_id)
        .await;

    if let Err(err) = entry_count {
        tracing::error!(?err, "couldn't select entry count");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(JournalResponse {
                successful: false,
                page_count: 0,
                entries: vec![],
            }),
        );
    }

    let entry_count = entry_count.unwrap();
    let page_count = (entry_count as usize).div_ceil(ENTRIES_PER_PAGE);

    let page = page.max(1);
    let offset = (page - 1) * ENTRIES_PER_PAGE;

    let entries = services
        .database
        .select_journal_entries(character_id, ENTRIES_PER_PAGE, offset)
        .await;

    if let Err(err) = entries {
        tracing::error!(?err, "couldn't select entries");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(JournalResponse {
                successful: false,
                page_count: 0,
                entries: vec![],
            }),
        );
    }

    let entries = entries
        .unwrap()
        .into_iter()
        .map(|entry| JournalEntry {
            id: entry.id,
            character_id: entry.character_id,
            date: entry.date,
            description: entry.description.clone(),
            ref_type: entry.ref_type.clone(),
            amount: entry.amount.map(|am| am.to_f64()).unwrap_or(None),
            tax: entry.tax.map(|ta| ta.to_f64()).unwrap_or(None),
        })
        .collect();

    (
        StatusCode::OK,
        Json(JournalResponse {
            successful: true,
            page_count,
            entries,
        }),
    )
}
