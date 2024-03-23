use reqwest::StatusCode;

use crate::model::{character::Character, wallet_journal::WalletJournal};

const EVE_ESI_ENDPOINT: &'static str = "https://esi.evetech.net/latest";

#[tracing::instrument]
pub async fn get_character(character_id: u64) -> anyhow::Result<Character> {
    let client = reqwest::Client::new();

    let url = format!("{}/characters/{}", EVE_ESI_ENDPOINT, character_id);

    let response = client
        .get(url)
        .query(&[("datasource", "tranquility")])
        .header("Accept", "application/json")
        .header("Cache-Control", "no-cache")
        .send()
        .await?;

    Ok(response.json::<Character>().await?)
}

#[tracing::instrument(skip(access_token))]
pub async fn get_wallet(character_id: u64, access_token: &str) -> anyhow::Result<i64> {
    let client = reqwest::Client::new();

    let url = format!("{}/characters/{}/wallet", EVE_ESI_ENDPOINT, character_id);
    let authorization = format!("Bearer {}", access_token);

    let response = client
        .get(url)
        .query(&[("datasource", "tranquility")])
        .header("Authorization", authorization)
        .send()
        .await?;

    if response.status() != StatusCode::OK {
        return Err(anyhow::Error::msg("couldn't get character wallet"));
    }

    let balance = response.text().await?.parse::<f64>()?;

    Ok(balance as i64)
}

// TODO(optimization): allow caller passing the desired page to fetch, this could prevent
// function from having to fetch all the pages every time. If the caller know they only need
// one specific page.

#[tracing::instrument(skip(access_token))]
pub async fn get_wallet_journal(
    character_id: u64,
    access_token: &str,
    last_journal_id: Option<i64>,
) -> anyhow::Result<Vec<WalletJournal>> {
    let client = reqwest::Client::new();

    let url = format!(
        "{}/characters/{}/wallet/journal",
        EVE_ESI_ENDPOINT, character_id
    );
    let authorization = format!("Bearer {}", access_token);

    let mut page = 1;
    let mut all_journal_entries = vec![];

    loop {
        let response = client
            .get(&url)
            .query(&[("datasource", "tranquility"), ("page", &page.to_string())])
            .header("Authorization", &authorization)
            .send()
            .await?;

        if response.status() != StatusCode::OK {
            return Err(anyhow::Error::msg("couldn't get character wallet journal"));
        }

        let pages = match response.headers().get("x-pages") {
            Some(pages) => pages.to_str()?.parse::<i32>()?,
            None => 1,
        };

        let mut journal_entries: Vec<WalletJournal> = response.json().await?;

        let page_included_last_id = last_journal_id.map_or(false, |id| {
            journal_entries.iter().any(|entry| entry.id == id)
        });

        tracing::info!(character_id, page_included_last_id, "fetched page {} of {}", page, pages);

        all_journal_entries.append(&mut journal_entries);

        if page_included_last_id || page >= pages {
            break;
        }

        page = page + 1;
    }

    let result = if let Some(last_journal_id) = last_journal_id {
        all_journal_entries
            .into_iter()
            .filter(|entry| entry.id > last_journal_id)
            .collect()
    } else {
        all_journal_entries
    };

    Ok(result)
}
