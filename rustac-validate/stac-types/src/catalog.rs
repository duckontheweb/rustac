use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::link::Link;

#[derive(Serialize, Deserialize)]
pub struct Catalog {
    stac_version: String,
    #[serde(rename = "type")]
    type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    stac_extensions: Option<Vec<String>>,
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    summaries: Option<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    links: Option<Vec<Link>>,
    #[serde(flatten)]
    pub extra_fields: Value,
}

#[cfg(test)]
mod tests {
    use std::fs;
    use serde_json;
    use crate::Catalog;

    fn get_test_example(filename: &str) -> String {
        let path = format!("./tests-data/{}", filename);
        fs::read_to_string(path).unwrap()
    }

    #[test]
    fn test_catalog_example() {
        let data = get_test_example("catalog.json");
        let collection: Catalog = serde_json::from_str(data.as_str()).unwrap();

        assert_eq!(collection.id, String::from("examples"));
    }

}
