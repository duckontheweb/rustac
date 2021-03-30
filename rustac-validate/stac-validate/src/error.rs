#![allow(clippy::module_name_repetitions)]
//! Possible errors when validating with STAC objects.
use std::error;
use std::result;
use std::fmt;
use std::str::FromStr;

/// Alias for [`result::Result`] that uses a [`STACValidateError`]
pub type STACValidateResult<T> = result::Result<T, STACValidateError>;

/// All errors that may be encountered when working with STAC objects in this package
#[derive(Debug)]
pub enum STACValidateError {
    /// Errors resulting from failed HTTP requests in the [`reqwest`] package
    HTTP(reqwest::Error),

    /// Errors resulting from failed JSON Schema validation in the [`jsonschema`] package
    Validation(String),

    /// Errors resulting from failed JSON Schema compilation in the [`jsonschema`] package
    Compilation(jsonschema::CompilationError),

    /// Errors resulting from failed serialization/deserialization of types using the [`serde_json`] package
    JSONParse(serde_json::Error),

    /// Errors resulting from trying to parse a semantic version string with [`semver`]
    SemVer(semver::SemVerError),

    /// Other errors not covered by the variants above.
    Other(String),
}

impl error::Error for STACValidateError {}

impl fmt::Display for STACValidateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            STACValidateError::HTTP(source) => source.fmt(f),
            STACValidateError::Validation(message) | STACValidateError::Other(message) => {
                write!(f, "{}", message.as_str())
            },
            STACValidateError::JSONParse(source) => source.fmt(f),
            STACValidateError::Compilation(source) => source.fmt(f),
            STACValidateError::SemVer(source) => source.fmt(f),
        }
    }
}

impl From<reqwest::Error> for STACValidateError {
    fn from (err: reqwest::Error) -> STACValidateError {
        STACValidateError::HTTP(err)
    }
}

impl From<serde_json::Error> for STACValidateError {
    fn from(err: serde_json::Error) -> STACValidateError {
        STACValidateError::JSONParse(err)
    }
}

impl FromStr for STACValidateError {
    type Err = Self;

    fn from_str(s: &str) -> STACValidateResult<Self> {
        Ok(STACValidateError::Other(String::from(s)))
    }
}

impl From<jsonschema::CompilationError> for STACValidateError {
    fn from (err: jsonschema::CompilationError) -> STACValidateError {
        STACValidateError::Compilation(err)
    }
}

impl From<semver::SemVerError> for STACValidateError {
    fn from(err: semver::SemVerError) -> STACValidateError {
        STACValidateError::SemVer(err)
    }
}