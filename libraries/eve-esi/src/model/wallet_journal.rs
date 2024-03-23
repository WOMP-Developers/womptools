use chrono::{DateTime, Utc};
use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct WalletJournal {
    pub id: i64,
    pub date: DateTime<Utc>,
    pub description: String,
    pub ref_type: String,
    pub reason: Option<String>,
    pub amount: Option<f64>,
    pub balance: Option<f64>,
    pub context_id: Option<i64>,
    pub context_id_type: Option<String>,
    pub first_party_id: Option<i32>,
    pub second_party_id: Option<i32>,
    pub tax: Option<f64>,
    pub tax_receiver_id: Option<i32>,
}