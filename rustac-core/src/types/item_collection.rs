//! Implementation of a [STAC Item 
//! Collection](https://github.com/radiantearth/stac-api-spec/tree/v1.0.0-beta.1/fragments/itemcollection) 
use serde::{Deserialize, Serialize};
use serde_json::Value;
use semver::Version;

use crate::types::common::Link;
use crate::types::item::Item;

/// Implements the [STAC Item Collection 
/// spec](https://github.com/radiantearth/stac-api-spec/tree/v1.0.0-beta.1/fragments/itemcollection)
#[derive(Serialize, Deserialize)]
pub struct ItemCollection {
    /// The STAC version the ItemCollection implements.
    pub stac_version: Version,

    /// A list of extensions the ItemCollection implements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stac_extensions: Option<Vec<String>>,

    ///  Always "FeatureCollection" to provide compatibility with GeoJSON.
    #[serde(rename = "type")]
    pub type_: String,

    /// A possibly-empty array of Items.
    pub features: Vec<Item>,

    /// An array of Links related to this ItemCollection.
    pub links: Option<Vec<Link>>,

    /// Additional fields not covered by the STAC spec.
    #[serde(flatten)]
    pub extra_fields: Value,
}

#[cfg(test)]
mod tests {
    use std::fs;
    use serde_json;
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
