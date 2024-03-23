use chrono::{DateTime, NaiveDate, Utc};
use sqlx::types::BigDecimal;


#[derive(Debug, sqlx::FromRow)]
pub struct Character {
    pub id: u64,
    pub access_token: String,
    pub access_token_expire_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct CharacterBalance {
    pub character_id: u64,
    pub balance: BigDecimal,
    pub date: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct BountySummary {
    pub date: NaiveDate,
    pub character_id: Option<u64>,
    pub sum_bounties: Option<BigDecimal>,
    pub sum_taxes: Option<BigDecimal>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct BalanceSummary {
    pub date: NaiveDate,
    pub balance: Option<BigDecimal>,
    pub amount: Option<BigDecimal>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct JournalEntry {
    pub id: i64,
    pub character_id: u64,
    pub date: DateTime<Utc>,
    pub description: String,
    pub ref_type: String, // TODO: make enum
    pub reason: Option<String>,
    pub amount: Option<BigDecimal>,
    pub balance: Option<BigDecimal>,
    pub context_id: Option<i64>,
    pub context_id_type: Option<String>, // TODO: make enum
    pub first_party_id: Option<i32>,
    pub second_party_id: Option<i32>,
    pub tax: Option<BigDecimal>,
    pub tax_receiver_id: Option<i32>,
}
