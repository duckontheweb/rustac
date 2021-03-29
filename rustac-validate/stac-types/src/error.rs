//! Possible errors when working with STAC objects.
use std::error;
use std::result;
use std::fmt;

/// Alias for [`result::Result`] that uses a [`StacError`]
pub type Result<T> = result::Result<T, StacError>;

/// All errors that may be encountered when working with STAC objects in this package
#[derive(Debug)]
pub enum StacError {
    /// Errors resulting from failed serialization/deserialization of types using the [`serde_json`]
    /// package
    JsonParseError(serde_json::Error),
}

impl error::Error for StacError {}

impl fmt::Display for StacError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            StacError::JsonParseError(source) => source.fmt(f),
        }
    }
}

impl From<serde_json::Error> for StacError {
    fn from(err: serde_json::Error) -> StacError {
        StacError::JsonParseError(err)
    }
}
