use std::sync::Arc;

use auth_token::AccessToken;
use axum::{extract::State, Json};
use bigdecimal::ToPrimitive;
use reqwest::StatusCode;
use serde::Serialize;

use crate::{dto::CharacterBalance, services::Services};

#[derive(Debug, Serialize)]
pub struct UserBalanceResponse {
    successful: bool,
    characters: Vec<CharacterBalance>,
}

pub async fn get_user_balance(
    State(services): State<Arc<Services>>,
    access_token: AccessToken,
) -> (StatusCode, Json<UserBalanceResponse>) {
    let wallets = services
        .database
        .select_user_balance(access_token.user_id)
        .await;

    if let Err(err) = wallets {
        tracing::error!(?err, "couldn't fetch user wallets");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(UserBalanceResponse {
                successful: false,
                characters: vec![],
            }),
        );
    }

    let wallets = wallets
        .unwrap()
        .iter()
        .map(|wallet| CharacterBalance {
            character_id: wallet.character_id,
            balance: wallet.balance.to_i64().unwrap_or(0),
            updated_at: wallet.date,
        })
        .collect();

    (
        StatusCode::OK,
        Json(UserBalanceResponse {
            successful: true,
            characters: wallets,
        }),
    )
}
