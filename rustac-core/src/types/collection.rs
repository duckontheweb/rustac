//! Implementation of [STAC Collection](https://github.com/radiantearth/stac-spec/blob/v1.0.0-rc.1/collection-spec/collection-spec.md)
use std::collections::HashMap;

use geojson::Bbox;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use semver::Version;

use crate::types::common::Link;
use crate::types::common::Asset;
use crate::types::common::Provider;
use crate::extensions::CollectionExtensionProperties;

/// Implements the [STAC Collection spec](https://github.com/radiantearth/stac-spec/blob/v1.0.0-rc.1/collection-spec/collection-spec.md).
#[derive(Serialize, Deserialize)]
pub struct Collection {
    /// The STAC version the Collection implements.
    pub stac_version: Version,
    
    /// Must be set to `"Collection"` to be a valid Collection.
    /// **This maps to the STAC `"type"` attribute, which is a reserved keyword.**
    #[serde(rename = "type")]
    pub type_: String,

    /// A list of extension identifiers the Collection implements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stac_extensions: Option<Vec<String>>,

    /// Identifier for the Collection that is unique across the provider.
    pub id: String,

    /// A short descriptive one-line title for the Collection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Detailed multi-line description to fully explain the Collection.
    pub description: String,

    /// List of keywords describing the Collection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,

    /// Collection's license(s), either a SPDX [License identifier](https://spdx.org/licenses/), `"various"` if multiple licenses apply 
    /// or `"proprietary"` for all other cases.
    pub license: String,

    /// A list of providers, which may include all organizations capturing or processing the data or the hosting provider. Providers 
    /// should be listed in chronological order with the most recent provider being the last element of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub providers: Option<Vec<Provider>>,

    /// Spatial and temporal extents.
    pub extent: Extent,

    /// A map of property summaries, either a set of values or statistics such as a range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summaries: Option<HashMap<String, Value>>,

    /// A list of references to other documents.
    pub links: Vec<Link>,

    /// Dictionary of asset objects that can be downloaded, each with a unique key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<HashMap<String, Asset>>,

    /// Attributes included in a STAC extensions applying to Collection objects.
    #[serde(flatten)]
    pub extensions: CollectionExtensionProperties,

    /// Additional fields not covered by the STAC spec.
    #[serde(flatten)]
    pub extra_fields: Value,
}

/// Implementation of [Extent Object](https://github.com/radiantearth/stac-spec/blob/v1.0.0-rc.1/collection-spec/collection-spec.md#extent-object)
#[derive(Serialize, Deserialize)]
pub struct Extent {
    /// Potential spatial extents covered by the Collection.
    pub spatial: SpatialExtent,
    /// Potential temporal extents covered by the Collection.
    pub temporal: TemporalExtent,
}

/// Implementation of [Spatial Extent Object](https://github.com/radiantearth/stac-spec/blob/v1.0.0-rc.1/collection-spec/collection-spec.md#spatial-extent-object)
#[derive(Serialize, Deserialize)]
pub struct SpatialExtent {
    /// Potential spatial extents covered by the Collection.
    pub bbox: Vec<Bbox>,
}

/// Implementation of [Temporal Extent Object](https://github.com/radiantearth/stac-spec/blob/master/collection-spec/collection-spec.md#temporal-extent-object)
#[derive(Serialize, Deserialize)]
pub struct TemporalExtent {
    /// Potential temporal extents covered by the Collection. 
    pub interval: Vec<Vec<Option<String>>>,
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
