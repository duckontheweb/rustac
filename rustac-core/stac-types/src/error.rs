use std::error;
use std::fmt;
use serde_json;
#[cfg(feature = "validate")]
use reqwest;
#[cfg(feature = "validate")]
use jsonschema;

pub(crate) type Result<T> = std::result::Result<T, StacError>;

#[derive(Debug)]
pub enum StacError {
    HttpError(reqwest::Error),
    ValidationError(String),
    CompilationError(jsonschema::CompilationError),
    JsonParseError(serde_json::Error),
}

impl error::Error for StacError {}

impl fmt::Display for StacError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            StacError::HttpError(source) => source.fmt(f),
            StacError::ValidationError(message) => write!(f, "{}", message.as_str()),
            StacError::JsonParseError(source) => source.fmt(f),
            StacError::CompilationError(source) => source.fmt(f),
        }
    }
}

impl From<reqwest::Error> for StacError {
    fn from (err: reqwest::Error) -> StacError {
        StacError::HttpError(err)
    }
}

impl From<serde_json::Error> for StacError {
    fn from(err: serde_json::Error) -> StacError {
        StacError::JsonParseError(err)
    }
}

impl From<jsonschema::CompilationError> for StacError {
    fn from (err: jsonschema::CompilationError) -> StacError {
        StacError::CompilationError(err)
    }
}