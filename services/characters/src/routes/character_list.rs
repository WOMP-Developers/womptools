use std::sync::Arc;

use auth_token::AccessToken;
use axum::{extract::State, Json};

use crate::{dto::{Character, CharactersResponse}, Service};

#[tracing::instrument(skip(service, token))]
pub async fn character_list(
    State(service): State<Arc<Service>>,
    token: AccessToken,
) -> Json<CharactersResponse> {
    let characters = service.db.select_characters(token.user_id).await;

    if let Err(err) = &characters {
        tracing::error!(?err, "couldn't select characters");
        return Json(CharactersResponse {
            successful: false,
            characters: vec![],
        });
    }

    Json(CharactersResponse {
        successful: true,
        characters: characters
            .unwrap()
            .iter()
            .map(|c| Character {
                character_id: c.character_id,
                name: c.name.to_string(),
                alliance_id: c.alliance_id,
                corporation_id: c.corporation_id,
                requires_authorization: c.requires_authorization,
                is_main: c.is_main,
            })
            .collect(),
    })
}
