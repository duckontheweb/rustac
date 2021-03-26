//! Possible errors when working with STAC objects.
use std::error;
use std::result;
use std::fmt;
use serde_json;
#[cfg(feature = "validate")]
use reqwest;
#[cfg(feature = "validate")]
use jsonschema;

/// Alias for [`result::Result`] that uses a [`StacError`]
pub type Result<T> = result::Result<T, StacError>;

/// All errors that may be encountered when working with STAC objects in this package
#[derive(Debug)]
pub enum StacError {
    /// Errors resulting from failed HTTP requests in the [`reqwest`] package
    #[cfg(feature = "validate")]
    HttpError(reqwest::Error),

    /// Errors resulting from failed JSON Schema validation in the [`jsonschema`] package
    #[cfg(feature = "validate")]
    ValidationError(String),

    /// Errors resulting from failed JSON Schema compilation in the [`jsonschema`] package
    #[cfg(feature = "validate")]
    CompilationError(jsonschema::CompilationError),

    /// Errors resulting from failed serialization/deserialization of types using the [`serde_json`] pacage
    JsonParseError(serde_json::Error),
}

impl error::Error for StacError {}

impl fmt::Display for StacError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            #[cfg(feature = "validate")]
            StacError::HttpError(source) => source.fmt(f),
            #[cfg(feature = "validate")]
            StacError::ValidationError(message) => write!(f, "{}", message.as_str()),
            StacError::JsonParseError(source) => source.fmt(f),
            #[cfg(feature = "validate")]
            StacError::CompilationError(source) => source.fmt(f),
        }
    }
}

#[cfg(feature = "validate")]
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

#[cfg(feature = "validate")]
impl From<jsonschema::CompilationError> for StacError {
    fn from (err: jsonschema::CompilationError) -> StacError {
        StacError::CompilationError(err)
    }
}