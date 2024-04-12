use db_utils::create_database;
use sqlx::{
    mysql::MySqlPoolOptions,
    types::chrono::{DateTime, Utc},
    MySqlPool,
};

use self::model::Credentials;

pub mod model;

#[derive(Debug)]
pub struct Database {
    db_pool: MySqlPool,
}

impl Database {
    #[tracing::instrument(skip(connection_string))]
    pub async fn connect(connection_string: &str) -> anyhow::Result<Database> {
        create_database(connection_string, env!("CARGO_PKG_NAME")).await?;

        let connection_string = format!("{}/{}", connection_string, env!("CARGO_PKG_NAME"));

        let db_pool = MySqlPoolOptions::new()
            .max_connections(8)
            .connect(&connection_string)
            .await?;

        sqlx::migrate!("./migrations").run(&db_pool).await?;

        Ok(Database { db_pool })
    }

    #[tracing::instrument(skip(self, access_token, refresh_token))]
    pub async fn insert_credentials(
        &self,
        character_id: u64,
        user_id: u64,
        access_token: &str,
        refresh_token: &str,
        expires_at: &DateTime<Utc>,
    ) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO `credentials` (character_id, user_id, access_token, refresh_token, expires_at)
                VALUES (?, ?, ?, ?, ?)
            ON DUPLICATE KEY UPDATE
                access_token=VALUES(access_token),
                refresh_token=VALUES(refresh_token),
                expires_at=VALUES(expires_at),
                updated_at=CURRENT_TIMESTAMP()
            "#,
        )
        .bind(character_id)
        .bind(user_id)
        .bind(access_token)
        .bind(refresh_token)
        .bind(expires_at)
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }

    pub async fn update_credentials(
        &self,
        character_id: u64,
        access_token: &str,
        refresh_token: &str,
        expires_at: &DateTime<Utc>,
    ) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            UPDATE `credentials` SET access_token=?, refresh_token=?, expires_at=?, is_stale=FALSE WHERE character_id=?
            "#,
        )
        .bind(access_token)
        .bind(refresh_token)
        .bind(expires_at)
        .bind(character_id)
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }

    #[tracing::instrument]
    pub async fn select_credentials(
        &self,
        character_id: u64,
    ) -> anyhow::Result<Option<Credentials>> {
        let credentials: Option<Credentials> =
            sqlx::query_as("SELECT * FROM `credentials` WHERE character_id=?")
                .bind(character_id)
                .fetch_optional(&self.db_pool)
                .await?;

        Ok(credentials)
    }

    #[tracing::instrument]
    pub async fn set_is_stale(&self, character_id: u64, is_stale: bool) -> anyhow::Result<()> {
        sqlx::query("UPDATE `credentials` SET is_stale=? WHERE character_id=?")
            .bind(is_stale)
            .bind(character_id)
            .execute(&self.db_pool)
            .await?;

        Ok(())
    }
}
