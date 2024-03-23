use std::sync::Arc;

use auth_token::AccessToken;
use axum::{extract::State, Json};
use bigdecimal::ToPrimitive;
use reqwest::StatusCode;
use serde::Serialize;

use crate::{dto::BalanceSummary, services::Services};

#[derive(Debug, Serialize)]
pub struct BalanceSummaryResponse {
    successful: bool,
    balance_summary: Vec<BalanceSummary>,
}

pub async fn get_balance_summary(
    State(services): State<Arc<Services>>,
    access_token: AccessToken,
) -> (StatusCode, Json<BalanceSummaryResponse>) {
    let balance_summary = services
        .database
        .select_monthly_balance(access_token.user_id)
        .await;

    if let Err(err) = balance_summary {
        tracing::error!(?err, "couldn't fetch balance summary");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(BalanceSummaryResponse {
                successful: false,
                balance_summary: vec![],
            }),
        );
    }

    let balance_summary = balance_summary
        .unwrap()
        .into_iter()
        .map(|summary| BalanceSummary {
            date: summary.date,
            amount: summary.amount.map(|bd| bd.to_f64().unwrap_or(0.0)),
            balance: summary.balance.map(|st| st.to_f64().unwrap_or(0.0)),
        })
        .collect();

    (
        StatusCode::OK,
        Json(BalanceSummaryResponse {
            successful: true,
            balance_summary,
        }),
    )
}
