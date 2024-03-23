use std::sync::Arc;

use auth_token::AccessToken;
use axum::{
    extract::{Path, State},
    Json,
};
use bigdecimal::ToPrimitive;
use chrono::{DateTime, Utc};
use reqwest::StatusCode;
use serde::Serialize;

use crate::{dto::CharacterBalance, services::Services};

#[derive(Debug, Serialize)]
pub struct CharacterBalanceResponse {
    successful: bool,
    balance: Option<CharacterBalance>,
}

impl CharacterBalanceResponse {
    pub fn successful(character_id: u64, balance: i64, updated_at: DateTime<Utc>) -> Self {
        CharacterBalanceResponse {
            successful: true,
            balance: Some(CharacterBalance {
                character_id,
                balance,
                updated_at,
            }),
        }
    }

    pub fn failure() -> Self {
        CharacterBalanceResponse {
            successful: false,
            balance: None,
        }
    }
}

#[tracing::instrument(skip(services, access_token))]
pub async fn get_balance(
    State(services): State<Arc<Services>>,
    access_token: AccessToken,
    Path(character_id): Path<u64>,
) -> (StatusCode, Json<CharacterBalanceResponse>) {
    let wallet = services
        .database
        .select_balance(access_token.user_id, character_id)
        .await;

    if let Err(err) = wallet {
        tracing::error!(
            ?err,
            access_token.user_id,
            character_id,
            "couldn't get wallet balance"
        );
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(CharacterBalanceResponse::failure()),
        );
    }

    match wallet.unwrap() {
        Some(wallet) => (
            StatusCode::OK,
            Json(CharacterBalanceResponse::successful(
                character_id,
                wallet.balance.to_i64().unwrap_or(0),
                wallet.date,
            )),
        ),
        None => (
            StatusCode::NOT_FOUND,
            Json(CharacterBalanceResponse::failure()),
        ),
    }
}
