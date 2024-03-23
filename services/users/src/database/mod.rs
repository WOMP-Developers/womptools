use chrono::{DateTime, Utc};
use db_utils::create_database;
use secrecy::Secret;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};

#[derive(Debug)]
pub struct Database {
    db_pool: MySqlPool,
}

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: u64,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Session {
    pub session_id: String,
    pub user_id: u64,
    pub ip: String,
    pub created_at: DateTime<Utc>,
    pub last_used_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct AuthorizedToon {
    pub user_id: u64,
    pub character_id: u64,
    pub refresh_token: Secret<String>,
    pub access_token: Secret<String>,
}

impl Database {
    #[tracing::instrument]
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

    #[tracing::instrument]
    pub async fn select_user_by_character(
        &self,
        character_id: u64,
    ) -> anyhow::Result<Option<User>> {
        let user: Option<User> =
            sqlx::query_as("SELECT u.* FROM users u LEFT JOIN characters aut ON aut.user_id=u.id WHERE aut.character_id=?")
                .bind(character_id)
                .fetch_optional(&self.db_pool)
                .await?;

        Ok(user)
    }

    #[tracing::instrument]
    pub async fn create_user_with_character(&self, character_id: u64) -> anyhow::Result<User> {
        let mut tx = self.db_pool.begin().await?;

        let result = sqlx::query("INSERT INTO users VALUES (NULL)")
            .execute(&mut *tx)
            .await?;

        let user = User {
            id: result.last_insert_id(),
        };

        sqlx::query("INSERT INTO characters (user_id, character_id) VALUES (?, ?)")
            .bind(user.id)
            .bind(character_id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(user)
    }

    pub async fn insert_registered_character(&self, user_id: u64, character_id: u64) -> anyhow::Result<()> {
        sqlx::query("INSERT INTO characters (user_id, character_id) VALUES (?, ?)")
            .bind(user_id)
            .bind(character_id)
            .execute(&self.db_pool)
            .await?;

        Ok(())
    }

    #[tracing::instrument]
    pub async fn insert_session(
        &self,
        session_id: &str,
        user_id: u64,
        ip: &str,
    ) -> anyhow::Result<()> {
        let mut tx = self.db_pool.begin().await?;

        sqlx::query("DELETE FROM sessions WHERE user_id=? AND ip=?")
            .bind(user_id)
            .bind(ip)
            .execute(&mut *tx)
            .await?;

        sqlx::query("INSERT INTO sessions (session_id, user_id, ip) VALUES (?, ?, ?)")
            .bind(session_id)
            .bind(user_id)
            .bind(ip)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(())
    }

    #[tracing::instrument]
    pub async fn delete_session(&self, session_id: &str) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM sessions WHERE session_id=?")
            .bind(session_id)
            .execute(&self.db_pool)
            .await?;

        Ok(())
    }

    pub async fn get_session(&self, session_id: &str) -> anyhow::Result<Session> {
        let session = sqlx::query_as("SELECT * FROM sessions WHERE session_id=?")
            .bind(session_id)
            .fetch_one(&self.db_pool)
            .await?;

        Ok(session)
    }

    pub async fn update_session(&self, session_id: &str) -> anyhow::Result<()> {
        sqlx::query("UPDATE sessions SET last_used_at=CURRENT_TIMESTAMP() WHERE session_id=?")
            .bind(session_id)
            .execute(&self.db_pool)
            .await?;

        Ok(())
    }
}
