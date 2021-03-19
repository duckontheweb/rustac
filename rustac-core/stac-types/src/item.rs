use serde::{Deserialize, Serialize};
use serde_json;

use super::link::Link;
use super::asset::Asset;
use std::collections::HashMap;

/// Implementation of a STAC Item.
#[derive(Serialize, Deserialize)]
pub struct Item {
    pub stac_version: String,
    pub stac_extensions: Vec<String>,
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub geometry: serde_json::Value,
    pub bbox: Vec<f32>,
    pub properties: serde_json::Value,
    pub links: Vec<Link>,
    pub assets: HashMap<String, Asset>,
    pub collection: Option<String>,
    #[serde(flatten)]
    pub extra_fields: serde_json::Value,
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

#[cfg(test)]
mod test {
    use std::fs;
    use crate::Item;

    fn get_test_example(filename: &str) -> String {
        let path = format!("./tests-data/{}", filename);
        fs::read_to_string(path).unwrap()
    }

    #[test]
    fn test_simple_item() {
        let data = get_test_example("simple-item.json");
        let item = Item::from_json(data.as_str()).unwrap();

        assert_eq!(item.id, String::from("20201211_223832_CS2"));
    }

    #[test]
    fn test_extended_item() {
        let data = get_test_example("extended-item.json");
        let item = Item::from_json(data.as_str()).unwrap();

        assert_eq!(item.properties["platform"].as_str(), Some("cool_sat2"));
    }

    #[test]
    fn test_core_item() {
        let data = get_test_example("core-item.json");
        let item = Item::from_json(data.as_str()).unwrap();

        assert_eq!(item.properties["constellation"].as_str(), Some("ion"));
    }
}