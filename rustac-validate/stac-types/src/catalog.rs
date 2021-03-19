use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json;

use super::link::Link;

#[derive(Serialize, Deserialize)]
pub struct Catalog {
    stac_version: String,
    #[serde(rename = "type")]
    type_: String,
    stac_extensions: Option<Vec<String>>,
    id: String,
    title: Option<String>,
    description: String,
    summaries: Option<HashMap<String, serde_json::Value>>,
    links: Option<Vec<Link>>,
    #[serde(flatten)]
    pub extra_fields: serde_json::Value,
}

impl Catalog {
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
mod tests {
    use std::fs;
    use crate::Catalog;

    fn get_test_example(filename: &str) -> String {
        let path = format!("./tests-data/{}", filename);
        fs::read_to_string(path).unwrap()
    }

    #[test]
    fn test_catalog_example() {
        let data = get_test_example("catalog.json");
        let collection = Catalog::from_json(data.as_str()).unwrap();

        assert_eq!(collection.id, String::from("examples"));
    }

}
