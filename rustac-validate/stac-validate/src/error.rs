//! Possible errors when validating with STAC objects.
use std::error;
use std::result;
use std::fmt;
use std::str::FromStr;
use serde_json;
use reqwest;
use jsonschema;

/// Alias for [`result::Result`] that uses a [`StacValidateError`]
pub type Result<T> = result::Result<T, StacValidateError>;

/// All errors that may be encountered when working with STAC objects in this package
#[derive(Debug)]
pub enum StacValidateError {
    /// Errors resulting from failed HTTP requests in the [`reqwest`] package
    HttpError(reqwest::Error),

    /// Errors resulting from failed JSON Schema validation in the [`jsonschema`] package
    ValidationError(String),

    /// Errors resulting from failed JSON Schema compilation in the [`jsonschema`] package
    CompilationError(jsonschema::CompilationError),

    /// Errors resulting from failed serialization/deserialization of types using the [`serde_json`] pacage
    JsonParseError(serde_json::Error),

    /// Other errors not covered by the variants above.
    OtherError(String),
}

impl error::Error for StacValidateError {}

impl fmt::Display for StacValidateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            StacValidateError::HttpError(source) => source.fmt(f),
            StacValidateError::ValidationError(message) => write!(f, "{}", message.as_str()),
            StacValidateError::JsonParseError(source) => source.fmt(f),
            StacValidateError::CompilationError(source) => source.fmt(f),
            StacValidateError::OtherError(message) => write!(f, "{}", message.as_str()),
        }
    }
}

impl From<reqwest::Error> for StacValidateError {
    fn from (err: reqwest::Error) -> StacValidateError {
        StacValidateError::HttpError(err)
    }
}

impl From<serde_json::Error> for StacValidateError {
    fn from(err: serde_json::Error) -> StacValidateError {
        StacValidateError::JsonParseError(err)
    }
}

impl FromStr for StacValidateError {
    type Err = Self;

    fn from_str(s: &str) -> Result<Self> {
        Ok(StacValidateError::OtherError(String::from(s)))
    }
}

impl From<jsonschema::CompilationError> for StacValidateError {
    fn from (err: jsonschema::CompilationError) -> StacValidateError {
        StacValidateError::CompilationError(err)
    }
}