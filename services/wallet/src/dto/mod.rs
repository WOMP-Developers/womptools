use chrono::{DateTime, NaiveDate, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CharacterBalance {
    pub character_id: u64,
    pub balance: i64,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct BountySummary {
    pub date: NaiveDate,
    pub character_id: Option<u64>,
    pub sum_bounties: Option<f64>,
    pub sum_taxes: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct BalanceSummary {
    pub date: NaiveDate,
    pub amount: Option<f64>,
    pub balance: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct JournalEntry {
    pub id: i64,
    pub character_id: u64,
    pub date: DateTime<Utc>,
    pub description: String,
    pub ref_type: String,
    pub amount: Option<f64>,
    pub tax: Option<f64>,
}
