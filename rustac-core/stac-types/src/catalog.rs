use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    summaries: Option<HashMap<String, Value>>,
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
