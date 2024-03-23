use std::sync::Arc;

use auth_token::AccessToken;
use axum::{extract::State, Json};

use crate::{dto::{Character, CharacterResponse}, Service};

#[tracing::instrument(skip(service, token))]
pub async fn character_main(
    State(service): State<Arc<Service>>,
    token: AccessToken,
) -> Json<CharacterResponse> {
    let character = service.db.select_main(token.user_id).await;

    if let Err(err) = &character {
        tracing::error!(?err, "couldn't select main character");
        return Json(CharacterResponse {
            successful: false,
            character: None,
        });
    }

    Json(CharacterResponse {
        successful: true,
        character: character.unwrap().map(|c| Character {
            character_id: c.character_id,
            name: c.name,
            alliance_id: c.alliance_id,
            corporation_id: c.corporation_id,
            requires_authorization: c.requires_authorization,
            is_main: c.is_main,
        }),
    })
}
