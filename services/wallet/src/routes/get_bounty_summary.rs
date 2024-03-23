use std::sync::Arc;

use auth_token::AccessToken;
use axum::{extract::State, Json};
use bigdecimal::ToPrimitive;
use reqwest::StatusCode;
use serde::Serialize;

use crate::{dto::BountySummary, services::Services};

#[derive(Debug, Serialize)]
pub struct BountySummaryResponse {
    successful: bool,
    bounty_summary: Vec<BountySummary>,
}

pub async fn get_bounty_summary(
    State(services): State<Arc<Services>>,
    access_token: AccessToken,
) -> (StatusCode, Json<BountySummaryResponse>) {
    let bounty_summary = services
        .database
        .select_monthly_bounty_summary(access_token.user_id)
        .await;

    if let Err(err) = bounty_summary {
        tracing::error!(?err, "couldn't fetch bounty summary");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(BountySummaryResponse {
                successful: false,
                bounty_summary: vec![],
            }),
        );
    }

    let bounty_summary = bounty_summary
        .unwrap()
        .into_iter()
        .map(|summary| BountySummary {
            date: summary.date,
            character_id: summary.character_id,
            sum_bounties: summary.sum_bounties.map(|bd| bd.to_f64().unwrap_or(0.0)),
            sum_taxes: summary.sum_taxes.map(|st| st.to_f64().unwrap_or(0.0)),
        })
        .collect();

    (
        StatusCode::OK,
        Json(BountySummaryResponse {
            successful: true,
            bounty_summary,
        }),
    )
}
