#![allow(clippy::module_name_repetitions)]
//! Possible errors when validating with STAC objects.
use std::error;
use std::result;
use std::fmt;
use std::str::FromStr;

/// Alias for [`result::Result`] that uses a [`STACValidateError`]
pub type Result<T> = result::Result<T, Error>;

/// All errors that may be encountered when working with STAC objects in this package
#[derive(Debug)]
pub enum Error {
    /// Errors resulting from failed HTTP requests in the [`reqwest`] package
    HTTP(reqwest::Error),

    /// Errors resulting from failed JSON Schema validation in the [`jsonschema`] package
    Validation(String),

    /// Errors resulting from failed JSON Schema compilation in the [`jsonschema`] package
    Compilation(jsonschema::CompilationError),

    /// Errors resulting from failed serialization/deserialization of types using the [`serde_json`] pacage
    JSONParse(serde_json::Error),

    /// Other errors not covered by the variants above.
    Other(String),
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Error::HTTP(source) => source.fmt(f),
            Error::Validation(message) | Error::Other(message) => {
                write!(f, "{}", message.as_str())
            },
            Error::JSONParse(source) => source.fmt(f),
            Error::Compilation(source) => source.fmt(f),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from (err: reqwest::Error) -> Error {
        Error::HTTP(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::JSONParse(err)
    }
}

impl FromStr for Error {
    type Err = Self;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Error::Other(String::from(s)))
    }
}

impl From<jsonschema::CompilationError> for Error {
    fn from (err: jsonschema::CompilationError) -> Error {
        Error::Compilation(err)
    }
}