use serde::{Deserialize, Serialize};
use serde_json;

use crate::extensions::AssetExtensionProperties;

#[derive(Serialize, Deserialize)]
pub struct Asset {
    pub href: String,
    pub title: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub roles: Option<Vec<String>>,
    #[serde(flatten)]
    pub extensions: AssetExtensionProperties,
}

impl Asset {
    /// Serializes the instance to a JSON string
    pub fn to_json(&self) -> serde_json::Result<String> {
        Ok(serde_json::to_string(&self)?)
    }

    /// Creates a new instance from a JSON string
    pub fn from_json(data: &str) -> serde_json::Result<Self> {
        Ok(serde_json::from_str(&data)?)
    }
}