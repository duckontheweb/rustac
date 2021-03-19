use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json;

use super::link::Link;
use super::asset::Asset;
use super::provider::Provider;


#[derive(Serialize, Deserialize)]
pub struct SpatialExtent {
    bbox: Vec<Vec<f32>>
}

#[derive(Serialize, Deserialize)]
pub struct TemporalExtent {
    interval: Vec<Vec<Option<String>>>
}

#[derive(Serialize, Deserialize)]
pub struct Extent {
    spatial: SpatialExtent,
    temporal: TemporalExtent,
}

#[derive(Serialize, Deserialize)]
pub struct Collection {
    stac_version: String,
    #[serde(rename = "type")]
    type_: String,
    stac_extensions: Option<Vec<String>>,
    id: String,
    title: Option<String>,
    description: String,
    keywords: Option<Vec<String>>,
    license: String,
    providers: Option<Vec<Provider>>,
    extent: Extent,
    summaries: Option<HashMap<String, serde_json::Value>>,
    links: Vec<Link>,
    assets: Option<HashMap<String, Asset>>,
    #[serde(flatten)]
    pub extra_fields: serde_json::Value,
}

impl Collection {
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
    use crate::Collection;

    fn get_test_example(filename: &str) -> String {
        let path = format!("./tests-data/{}", filename);
        fs::read_to_string(path).unwrap()
    }

    #[test]
    fn test_collection_example() {
        let data = get_test_example("collection.json");
        let collection = Collection::from_json(data.as_str()).unwrap();

        assert_eq!(collection.id, String::from("simple-collection"));
    }

    #[test]
    fn test_sentinel_2_collection_example() {
        let data = get_test_example("sentinel-2-collection.json");
        let collection = Collection::from_json(data.as_str()).unwrap();

        assert_eq!(collection.id, String::from("sentinel-2"))
    }
}
