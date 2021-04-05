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

    /// Errors resulting from trying to parse a semantic version string with [`semver`]
    SemVer(semver::SemVerError),

    /// Other errors not covered by the variants above.
    Other(String),
}

impl error::Error for STACError {}

impl fmt::Display for STACError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            STACError::JSONParse(source) => source.fmt(f),
            STACError::SemVer(source) => source.fmt(f),
            STACError::Other(message) => write!(f, "{}", message.as_str())
        }
    }
}

impl From<serde_json::Error> for STACError {
    fn from(err: serde_json::Error) -> STACError {
        STACError::JSONParse(err)
    }
}

impl FromStr for STACError {
    type Err = Self;

    fn from_str(s: &str) -> STACResult<Self> {
        Ok(STACError::Other(String::from(s)))
    }
}

impl From<semver::SemVerError> for STACError {
    fn from(err: semver::SemVerError) -> STACError {
        STACError::SemVer(err)
    }
}
