use auth_token::AccessTokenBuilder;
use message_bus::{ServiceEventProducer, StreamEvent};
use rand::{thread_rng, Rng};
use base64::{engine::general_purpose, Engine as _};
use serde::Serialize;

use crate::database::Database;

#[derive(Debug, Serialize)]
pub struct UserSession {
    pub access_token: String,
    pub refresh_token: String,
}

pub struct Users {
    db: Database,
    messages: ServiceEventProducer,
    token_builder: AccessTokenBuilder,
}

#[derive(Debug, Serialize)]
struct AccessTokenClaims {
    sub: u64,
    exp: usize,
    iat: usize,
}

#[derive(Debug, Serialize)]
struct RefreshTokenClaims {
    sub: String,
}

impl Users {
    #[tracing::instrument(skip(db, messages, token_builder))]
    pub fn new(db: Database, messages: ServiceEventProducer, token_builder: AccessTokenBuilder) -> Self {
        Users { db, messages, token_builder }
    }

    #[tracing::instrument(skip(self))]
    pub async fn find_user_by_character(&self, character_id: u64) -> anyhow::Result<Option<u64>> {
        let user = self.db.select_user_by_character(character_id).await?;

        Ok(user.map(|u| u.id))
    }

    #[tracing::instrument(skip(self))]
    pub async fn create_user(&self, character_id: u64) -> anyhow::Result<u64> {
        let user = self.db.create_user_with_character(character_id).await?;

        self.messages.send_event(StreamEvent::UserRegistered { user_id: user.id }).await?;
        self.messages.send_event(StreamEvent::CharacterRegistered { user_id: user.id, character_id }).await?;

        Ok(user.id)
    }

    #[tracing::instrument(skip(self))]
    pub async fn register_character(&self, user_id: u64, character_id: u64) -> anyhow::Result<()> {
        self.db.insert_registered_character(user_id, character_id).await?;

        self.messages.send_event(StreamEvent::CharacterRegistered { user_id, character_id }).await?;

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    fn create_session_id(&self) -> String {
        let mut buf = [0u8; 32];
        thread_rng().fill(&mut buf[..]);

        general_purpose::STANDARD.encode(&buf)
    }

    #[tracing::instrument(skip(self))]
    pub async fn create_user_session(&self, user_id: u64, ip: &str) -> anyhow::Result<UserSession> {
        let session_id = self.create_session_id();
        let refresh_token = self.token_builder.build_refresh_token(&session_id)?;
        let access_token = self.token_builder.build_access_token(user_id, &session_id)?;

        self.db.insert_session(&session_id, user_id, ip).await?;

        self.messages.send_event(StreamEvent::UserAuthenticated { user_id }).await?;

        Ok(UserSession {
            access_token,
            refresh_token,
        })
    }

    #[tracing::instrument(skip(self))]
    pub async fn refresh_user_session(&self, session_id: &str) -> anyhow::Result<UserSession> {
        let session = self.db.get_session(session_id).await?;

        let refresh_token = self.token_builder.build_refresh_token(&session_id)?;
        let access_token = self.token_builder.build_access_token(session.user_id, session_id)?;

        self.db.update_session(session_id).await?;

        Ok(UserSession {
            access_token,
            refresh_token
        })
    }

    #[tracing::instrument(skip(self))]
    pub async fn delete_user_session(&self, user_id: u64, session_id: &str) -> anyhow::Result<()> {
        self.db.delete_session(session_id).await?;

        Ok(())
    }

    
}
