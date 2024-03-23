use sqlx::types::chrono::{DateTime, Utc};

#[derive(sqlx::FromRow)]
pub struct Credentials {
    pub character_id: u64,
    pub user_id: u64,
    pub refresh_token: String,
    pub access_token: String,
    pub updated_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}