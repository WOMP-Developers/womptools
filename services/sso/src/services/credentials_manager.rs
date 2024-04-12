use eve_sso::{error::ErrorKind, parse::parse_access_token, AccessToken, EveSSO, Tokens};
use message_bus::{ServiceEventProducer, StreamEvent};
use sqlx::types::chrono::Utc;

use crate::database::Database;

pub struct CredentialsManager {
    database: Database,
    producer: ServiceEventProducer,
    sso: EveSSO,
}

impl CredentialsManager {
    pub fn new(database: Database, producer: ServiceEventProducer, sso: EveSSO) -> Self {
        CredentialsManager {
            database,
            producer,
            sso,
        }
    }

    #[tracing::instrument(skip(self, access_token, refresh_token))]
    pub async fn register_credentials(
        &self,
        user_id: u64,
        access_token: &str,
        refresh_token: &str,
    ) -> anyhow::Result<()> {
        // TODO: Ensure only tokens issued to our app can be registered

        let access_token = parse_access_token(access_token).await?;
        tracing::info!(access_token.character_id, user_id, "register credentials");

        let credentials = self
            .database
            .select_credentials(access_token.character_id)
            .await?;

        if let Some(credentials) = credentials {
            if credentials.user_id != user_id {
                tracing::error!(
                    credentials.user_id,
                    user_id,
                    "provided user_id doesn't match stored user_id"
                );
            }

            self.database
                .update_credentials(
                    access_token.character_id,
                    &access_token.access_token,
                    refresh_token,
                    &access_token.expires_at,
                )
                .await?;
        } else {
            self.database
                .insert_credentials(
                    access_token.character_id,
                    user_id,
                    &access_token.access_token,
                    refresh_token,
                    &access_token.expires_at,
                )
                .await?;
        }

        self.producer
            .send_event(StreamEvent::CharacterAccessToken {
                user_id,
                access_token: access_token.access_token,
            })
            .await?;

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub async fn refresh_character_credentials_on_demand(
        &self,
        character_id: u64,
        on_behalf_of: u64,
    ) -> anyhow::Result<Option<AccessToken>> {
        let result =
            if let Some(credentials) = self.database.select_credentials(character_id).await? {
                if credentials.user_id == on_behalf_of {
                    if credentials.expires_at > Utc::now() {
                        Ok(Some(AccessToken {
                            character_id: credentials.character_id,
                            access_token: credentials.access_token.to_string(),
                            expires_at: credentials.expires_at.clone(),
                        }))
                    } else {
                        let tokens = refresh_credentials(
                            credentials.user_id,
                            character_id,
                            &credentials.refresh_token,
                            &self.sso,
                            &self.database,
                            &self.producer,
                        )
                        .await?;

                        Ok(Some(AccessToken {
                            character_id,
                            access_token: tokens.access_token.access_token.to_string(),
                            expires_at: tokens.access_token.expires_at.clone(),
                        }))
                    }
                } else {
                    tracing::warn!(
                        credentials.user_id,
                        on_behalf_of,
                        "mismatching user id during credentials refresh"
                    );
                    Err(anyhow::Error::msg("credentials belong to other user"))
                }
            } else {
                tracing::warn!(?character_id, "no credentials for character");

                self.producer
                    .send_event(StreamEvent::CharacterTokenInvalid { character_id })
                    .await?;

                Ok(None)
            };

        result
    }

    #[tracing::instrument(skip(self))]
    pub async fn refresh_character_credentials(&self, character_id: u64) -> anyhow::Result<()> {
        let credentials = self.database.select_credentials(character_id).await?;

        if let Some(credentials) = credentials {
            if credentials.is_stale {
                tracing::warn!(character_id, "credentials for character is stale");

                // TODO: evaluate if it's good idea to send this message every time.
                self.producer
                    .send_event(StreamEvent::CharacterTokenInvalid { character_id })
                    .await?;
            } else if credentials.expires_at > Utc::now() {
                tracing::info!(character_id, "token not expired skip refresh");

                self.producer
                    .send_event(StreamEvent::CharacterAccessToken {
                        user_id: credentials.user_id,
                        access_token: credentials.access_token,
                    })
                    .await?;
            } else {
                refresh_credentials(
                    credentials.user_id,
                    character_id,
                    &credentials.refresh_token,
                    &self.sso,
                    &self.database,
                    &self.producer,
                )
                .await?;
            }
        } else {
            tracing::warn!(?character_id, "no credentials for character");

            self.producer
                .send_event(StreamEvent::CharacterTokenInvalid { character_id })
                .await?;
        }

        Ok(())
    }
}

#[tracing::instrument(skip(refresh_token, eve_sso, db, producer))]
async fn refresh_credentials(
    user_id: u64,
    character_id: u64,
    refresh_token: &str,
    eve_sso: &EveSSO,
    db: &Database,
    producer: &ServiceEventProducer,
) -> anyhow::Result<Tokens> {
    tracing::info!(character_id, "refresh character credentials");

    match eve_sso.oauth_refresh(refresh_token).await {
        Ok(tokens) => {
            // TODO: Encrypt the credentials before storing them in the database or sending them to redis.
            db.update_credentials(
                character_id,
                &tokens.access_token.access_token,
                &tokens.refresh_token,
                &tokens.access_token.expires_at,
            )
            .await?;

            producer
                .send_event(StreamEvent::CharacterAccessToken {
                    user_id,
                    access_token: tokens.access_token.access_token.clone(),
                })
                .await?;

            Ok(tokens)
        }
        Err(error) => {
            handle_oauth_error(error.kind(), character_id, db, producer).await?;

            Err(anyhow::Error::new(error))
        }
    }
}

async fn handle_oauth_error(
    kind: &ErrorKind,
    character_id: u64,
    db: &Database,
    producer: &ServiceEventProducer,
) -> anyhow::Result<()> {
    match kind {
        ErrorKind::AuthorizationError => {
            db.set_is_stale(character_id, true).await?;

            producer
                .send_event(StreamEvent::CharacterTokenInvalid { character_id })
                .await?;
        }
        _ => {}
    }

    Ok(())
}
