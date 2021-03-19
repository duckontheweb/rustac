use serde::{Deserialize, Serialize};
use serde_json;

/// Implementation of a STAC Link.
#[derive(Serialize, Deserialize)]
pub struct Link {
    pub href: String,
    pub rel: String,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub title: Option<String>,
    #[serde(flatten)]
    pub other: serde_json::Value,
}

impl Link {
    /// Serializes the instance to a JSON string
    pub fn to_json(&self) -> serde_json::Result<String> {
        Ok(serde_json::to_string(&self)?)
    }

    /// Creates a new instance from a JSON string
    pub fn from_json(data: &str) -> serde_json::Result<Self> {
        Ok(serde_json::from_str(&data)?)
    }
}
