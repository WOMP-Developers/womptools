use std::fmt::Display;

use serde::{Deserialize, Serialize};

pub const MESSAGE_STREAM_KEY: &'static str = "wt-service-messages";

#[derive(Debug)]
pub struct StreamMessage {
    pub id: String,
    pub evt: Option<StreamEvent>,
    pub ctx: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StreamEvent {
    UserRegistered {
        user_id: u64,
    },
    UserAuthenticated {
        user_id: u64,
    },
    UserLogout {
        user_id: u64,
    },
    CharacterRegistered {
        user_id: u64,
        character_id: u64,
    },
    RequestCharacterAccessToken {
        character_id: u64,
    },
    CharacterAccessToken {
        user_id: u64,
        access_token: String,
    },
    CharacterTokenInvalid {
        character_id: u64,
    }
}

impl Display for StreamEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StreamEvent::UserRegistered { user_id } => write!(f, "UserRegistered {{ user_id: {} }}", user_id),
            StreamEvent::UserAuthenticated { user_id } => write!(f, "UserAuthenticated {{ user_id: {} }}", user_id),
            StreamEvent::UserLogout { user_id } => write!(f, "UserLogout {{ user_id: {} }}", user_id),
            StreamEvent::CharacterRegistered { user_id, character_id } => write!(f, "CharacterRegistered {{ user_id: {}, character_id: {} }}", user_id, character_id),
            StreamEvent::RequestCharacterAccessToken { character_id } => write!(f, "RequestCharacterAccessToken {{ character_id: {} }}", character_id),
            StreamEvent::CharacterAccessToken { user_id, .. } => write!(f, "CharacterAccessToken {{ user_id: {}, access_token: <REDACTED> }}", user_id),
            StreamEvent::CharacterTokenInvalid { character_id } => write!(f, "CharacterTokenInvalid {{ character_id: {} }}", character_id),
        }
    }
}
