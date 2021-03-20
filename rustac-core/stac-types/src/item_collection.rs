use serde::{Deserialize, Serialize};
use serde_json;

use super::link::Link;
use super::item::Item;

#[derive(Serialize, Deserialize)]
pub struct ItemCollection {
    stac_version: String,
    stac_extensions: Vec<String>,
    #[serde(rename = "type")]
    type_: String,
    features: Vec<Item>,
    links: Option<Vec<Link>>,
    #[serde(flatten)]
    pub extra_fields: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::ItemCollection;

    fn get_test_example(filename: &str) -> String {
        let path = format!("./tests-data/{}", filename);
        fs::read_to_string(path).unwrap()
    }

    #[test]
    fn test_minimal_item_collection_example() {
        let data = get_test_example("itemcollection-sample-minimal.json");
        let item_collection: ItemCollection = serde_json::from_str(data.as_str()).unwrap();

        assert_eq!(item_collection.type_, String::from("FeatureCollection"));
    }
    
    #[test]
    fn test_ful_item_collection_example() {
        let data = get_test_example("itemcollection-sample-full.json");
        let item_collection: ItemCollection = serde_json::from_str(data.as_str()).unwrap();

        assert_eq!(item_collection.features.len(), 1);

        let links = if let Some(links_) = item_collection.links {
            links_
        } else {
            vec![]
        };
        assert_eq!(links.len(), 1);
    }
}
