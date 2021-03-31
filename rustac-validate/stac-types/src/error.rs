//! Possible errors when working with STAC objects.
use std::error;
use std::result;
use std::fmt;
use std::str::FromStr;

/// Alias for [`result::Result`] that uses a [`STACError`]
pub type STACResult<T> = result::Result<T, STACError>;

/// All errors that may be encountered when working with STAC objects in this package
#[derive(Debug)]
pub enum STACError {
    /// Errors resulting from failed serialization/deserialization of types using the [`serde_json`]
    /// package
    JSONParse(serde_json::Error),

    /// Errors resulting from failed HTTP requests in the [`reqwest`] package
    HTTP(reqwest::Error),

    /// Errors resulting from failed JSON Schema validation in the [`jsonschema`] package
    Validation(String),

    /// Errors resulting from failed JSON Schema compilation in the [`jsonschema`] package
    Compilation(jsonschema::CompilationError),

    /// Errors resulting from trying to parse a semantic version string with [`semver`]
    SemVer(semver::SemVerError),

    /// Other errors not covered by the variants above.
    Other(String),
}

impl error::Error for STACError {}

impl fmt::Display for STACError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            STACError::HTTP(source) => source.fmt(f),
            STACError::Validation(message) | STACError::Other(message) => {
                write!(f, "{}", message.as_str())
            },
            STACError::JSONParse(source) => source.fmt(f),
            STACError::Compilation(source) => source.fmt(f),
            STACError::SemVer(source) => source.fmt(f),
        }
    }
}

impl From<serde_json::Error> for STACError {
    fn from(err: serde_json::Error) -> STACError {
        STACError::JSONParse(err)
    }
}

impl From<reqwest::Error> for STACError {
    fn from (err: reqwest::Error) -> STACError {
        STACError::HTTP(err)
    }
}

impl FromStr for STACError {
    type Err = Self;

    fn from_str(s: &str) -> STACResult<Self> {
        Ok(STACError::Other(String::from(s)))
    }
}

impl From<jsonschema::CompilationError> for STACError {
    fn from (err: jsonschema::CompilationError) -> STACError {
        STACError::Compilation(err)
    }
}

impl From<semver::SemVerError> for STACError {
    fn from(err: semver::SemVerError) -> STACError {
        STACError::SemVer(err)
    }
}
