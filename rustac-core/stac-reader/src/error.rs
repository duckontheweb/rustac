use std::error::Error as StdError;
use std::fmt;
use reqwest;
use serde_json;

pub(crate) type Result<T> = std::result::Result<T, Error>;
pub(crate) type BoxError = Box<dyn StdError + Send + Sync>;

#[derive(Debug)]
pub struct Error {
    source: Option<BoxError>,
}

impl Error {
    pub fn new(source: Option<BoxError>) -> Error {
        Error { source }
    }
}

impl StdError for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(source) = &self.source {
            source.fmt(f)
        } else {
            write!(f, "Unknown STAC error occurred.")
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from (err: reqwest::Error) -> Error {
        Error::new(Some(Box::new(err)))
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::new(Some(Box::new(err)))
    }
}