use std::{error::Error as StdError, sync::Arc};
use serde_urlencoded::ser::Error as SerError;
use serde_json::Error as JsonDesError;

pub type Result<T> = core::result::Result<T, Error>;

pub(crate) fn create_error(kind: ErrorKind) -> Error {
    Error(Box::new(kind))
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Error(pub Box<ErrorKind>);

impl Error {
    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }

    pub fn into_kind(self) -> ErrorKind {
        *self.0
    }
}

#[derive(Clone, Debug)]
pub enum ErrorKind {
    AuthorizationError,
    ParseSubjectError,
    ParseExpiredError,
    UrlEncodingError(SerError),
    JsonDecodingError(Arc<JsonDesError>),
    TokenDecodingError(jsonwebtoken::errors::Error),
    WebRequestError(Arc<reqwest::Error>),
}

impl StdError for Error {
    fn cause(&self) -> Option<&dyn StdError> {
        match &*self.0 {
            ErrorKind::ParseSubjectError
            | ErrorKind::ParseExpiredError
            | ErrorKind::AuthorizationError => None,
            ErrorKind::UrlEncodingError(inner) => Some(inner),
            ErrorKind::JsonDecodingError(inner) => Some(inner.as_ref()),
            ErrorKind::TokenDecodingError(inner) => Some(inner),
            ErrorKind::WebRequestError(inner) => Some(inner.as_ref()),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &*self.0 {
            ErrorKind::ParseSubjectError
            | ErrorKind::ParseExpiredError
            | ErrorKind::AuthorizationError => write!(f, "{:?}", self.0),
            ErrorKind::UrlEncodingError(err) => write!(f, "Url Encoding error: {}", err),
            ErrorKind::JsonDecodingError(err) => write!(f, "Json Decoding error: {}", err),
            ErrorKind::TokenDecodingError(err) => write!(f, "Token Decoding error: {}", err),
            ErrorKind::WebRequestError(err) => write!(f, "Web Request error: {}", err)
        }
    }
}

impl PartialEq for ErrorKind {
    fn eq(&self, other: &Self) -> bool {
        format!("{:?}", self) == format!("{:?}", other)
    }
}

impl Eq for ErrorKind {}

impl From<SerError> for Error {
    fn from(err: SerError) -> Error {
        create_error(ErrorKind::UrlEncodingError(err))
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(err: jsonwebtoken::errors::Error) -> Error {
        create_error(ErrorKind::TokenDecodingError(err))
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        create_error(ErrorKind::WebRequestError(Arc::new(err)))
    }
}

impl From<JsonDesError> for Error {
    fn from(err: JsonDesError) -> Error {
        create_error(ErrorKind::JsonDecodingError(Arc::new(err)))
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        create_error(kind)
    }
}
