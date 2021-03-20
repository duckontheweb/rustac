use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json;

use super::link::Link;
use super::asset::Asset;
use super::common_metadata::CommonMetadata;
use crate::extensions::ItemExtensionProperties;


#[derive(Serialize, Deserialize)]
pub struct ItemProperties {
    #[serde(flatten)]
    pub common: CommonMetadata,
    #[serde(flatten)]
    pub extension: ItemExtensionProperties,
    #[serde(flatten)]
    pub extra_fields: serde_json::Value,
}

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
    pub properties: ItemProperties,
    pub links: Vec<Link>,
    pub assets: HashMap<String, Asset>,
    pub collection: Option<String>,
    #[serde(flatten)]
    pub extra_fields: serde_json::Value,
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
        let item: Item = serde_json::from_str(data.as_str()).unwrap();

        assert_eq!(item.id, String::from("20201211_223832_CS2"));
    }

    #[test]
    fn test_extended_item() {
        let data = get_test_example("extended-item.json");
        let item: Item = serde_json::from_str(data.as_str()).unwrap();

        assert_eq!(item.properties.common.platform, Some(String::from("cool_sat2")));
        assert_eq!(item.properties.extension.view.sun_elevation, Some(54.9));
        assert_eq!(item.properties.extension.proj.shape, Some(vec![5558, 9559]));
        assert_eq!(item.properties.extension.sci.doi, Some(String::from("10.5061/dryad.s2v81.2/27.2")));
    }

    #[test]
    fn test_core_item() {
        let data = get_test_example("core-item.json");
        let item: Item = serde_json::from_str(data.as_str()).unwrap();
        
        assert_eq!(item.properties.common.constellation, Some(String::from("ion")));
    }

    #[test]
    fn test_scientific_extension_item() {
        let data = get_test_example("extensions/scientific/item.json");
        let item: Item = serde_json::from_str(data.as_str()).unwrap();

        assert_eq!(item.links[2].rel, String::from("cite-as"));
    }
}