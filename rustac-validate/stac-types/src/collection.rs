use std::collections::HashMap;

use geojson::Bbox;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::link::Link;
use super::asset::Asset;
use super::provider::Provider;
use super::extensions::CollectionExtensionProperties;

#[derive(Serialize, Deserialize)]
pub struct SpatialExtent {
    bbox: Vec<Bbox>,
}

#[derive(Serialize, Deserialize)]
pub struct TemporalExtent {
    interval: Vec<Vec<Option<String>>>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    stac_extensions: Option<Vec<String>>,
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    keywords: Option<Vec<String>>,
    license: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    providers: Option<Vec<Provider>>,
    extent: Extent,
    #[serde(skip_serializing_if = "Option::is_none")]
    summaries: Option<HashMap<String, Value>>,
    links: Vec<Link>,
    #[serde(skip_serializing_if = "Option::is_none")]
    assets: Option<HashMap<String, Asset>>,
    #[serde(flatten)]
    pub extensions: CollectionExtensionProperties,
    #[serde(flatten)]
    pub extra_fields: Value,
}

#[cfg(test)]
mod tests {
    use std::fs;
    use serde_json;
    use crate::Collection;

    fn get_test_example(filename: &str) -> String {
        let path = format!("./tests-data/{}", filename);
        fs::read_to_string(path).unwrap()
    }

    #[test]
    fn test_collection_example() {
        let data = get_test_example("collection.json");
        let collection: Collection = serde_json::from_str(data.as_str()).unwrap();

        assert_eq!(collection.id, String::from("simple-collection"));
    }

    #[test]
    fn test_sentinel_2_collection_example() {
        let data = get_test_example("sentinel-2-collection.json");
        let collection: Collection = serde_json::from_str(data.as_str()).unwrap();

        assert_eq!(collection.id, String::from("sentinel-2"))
    }

    #[test]
    fn test_scientific_extension_collection() {
        let data = get_test_example("extensions/scientific/collection.json");
        let collection: Collection = serde_json::from_str(data.as_str()).unwrap();

        assert_eq!(
            collection.extensions.sci.citation, 
            Some(String::from("Vega GC, Pertierra LR, Olalla-Tárraga MÁ (2017) \
            Data from: MERRAclim, a high-resolution global dataset of remotely \
            sensed bioclimatic variables for ecological modelling. Dryad Digital Repository."))
        )
    }
}
