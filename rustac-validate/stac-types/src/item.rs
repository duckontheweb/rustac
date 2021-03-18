use serde::{Deserialize, Serialize};
use serde_json;

use super::link::{Link, UsesLinks};
use std::collections::HashMap;

/// Implementation of a STAC Item.
#[derive(Serialize, Deserialize)]
pub struct Item {
    pub stac_version: String,
    pub stac_extensions: Vec<String>,
    pub id: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub geometry: serde_json::Value,
    pub bbox: Vec<f32>,
    pub properties: serde_json::Value,
    pub links: Vec<Link>,
    pub assets: HashMap<String, serde_json::Value>,
    pub collection: Option<String>,
}

impl Item {
    /// Serializes the instance to a JSON string
    pub fn to_json(&self) -> serde_json::Result<String> {
        Ok(serde_json::to_string(&self)?)
    }

    /// Creates a new instance from a JSON string
    pub fn from_json(data: &str) -> serde_json::Result<Self> {
        Ok(serde_json::from_str(&data)?)
    }
}

impl UsesLinks for Item {    
    fn get_links(&self) -> Vec<&Link> {
        self.links.iter().collect()
    }
}