use std::sync::Arc;

use auth_token::AccessToken;
use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    dto::{Character, CharacterResponse},
    Service,
};

#[tracing::instrument(skip(service, token))]
pub async fn character_by_id(
    State(service): State<Arc<Service>>,
    token: AccessToken,
    Path(character_id): Path<u64>,
) -> Json<CharacterResponse> {
    let character = service.db.select_character(token.user_id, character_id).await;

    if let Err(err) = &character {
        tracing::error!(?err, character_id, "couldn't select character");
        return Json(CharacterResponse {
            successful: false,
            character: None,
        });
    }

    Json(CharacterResponse {
        successful: true,
        character: character.map(|c| Character {
            character_id: c.character_id,
            name: c.name,
            alliance_id: c.alliance_id,
            corporation_id: c.corporation_id,
            requires_authorization: c.requires_authorization,
            is_main: c.is_main,
        }).ok(),
    })
}
