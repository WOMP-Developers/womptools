use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Character {
    pub character_id: u64,
    pub name: String,
    pub alliance_id: Option<i32>,
    pub corporation_id: i32,
    pub is_main: bool,
    pub requires_authorization: bool,
}

#[derive(Debug, Serialize)]
pub struct CharacterResponse {
    pub successful: bool,
    pub character: Option<Character>,
}

#[derive(Debug, Serialize)]
pub struct CharactersResponse {
    pub successful: bool,
    pub characters: Vec<Character>,
}