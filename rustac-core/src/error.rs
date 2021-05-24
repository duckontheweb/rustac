//! Defines custom error `enum` and [`Result`] types.
use std::error;
use std::fmt;
use std::result;

/// Alias for [`result::Result`] that uses a [`Error`]
pub type Result<T> = result::Result<T, Error>;

/// Covers errors that may be encountered when working with STAC objects in this package. Each variant has an
/// external error associated with it, except for `Other`, which simply has a [`String`]. Implementation of
/// [`fmt::Display`] is delegated to the source error, except for the `Other` variant, which simply prints the
/// associated [`String`].
#[derive(Debug)]
pub enum Error {
    /// Errors resulting from failed serialization/deserialization of types using the [`serde_json`]
    /// package
    JSONParse(serde_json::Error),

    /// Errors resulting from trying to parse a semantic version string with [`semver`]
    SemVer(semver::SemVerError),

    /// Other errors not covered by the variants above.
    Other(String),
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Error::JSONParse(source) => source.fmt(f),
            Error::SemVer(source) => source.fmt(f),
            Error::Other(message) => write!(f, "{}", message.as_str()),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::JSONParse(err)
    }
}

impl From<semver::SemVerError> for Error {
    fn from(err: semver::SemVerError) -> Error {
        Error::SemVer(err)
    }
}
