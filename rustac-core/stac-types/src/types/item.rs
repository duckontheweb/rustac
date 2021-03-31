//! Implementation of the [STAC Item spec](https://github.com/radiantearth/stac-spec/blob/v1.0.0-rc.1/item-spec/item-spec.md)
use std::collections::HashMap;

use geojson::{Geometry, Bbox};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use semver::Version;

use super::fragments::Link;
use super::fragments::Asset;
use super::fragments::CommonMetadata;
use crate::extensions::ItemExtensionProperties;

/// Represents a [STAC Item](https://github.com/radiantearth/stac-spec/blob/v1.0.0-rc.1/item-spec/item-spec.md).
#[derive(Serialize, Deserialize)]
pub struct Item {
    /// The STAC version the Item implements.
    pub stac_version: Version,

    /// A list of extensions the Item implements.
    pub stac_extensions: Vec<String>,

    /// Provider identifier. The ID should be unique within the Collection that contains the Item.
    pub id: String,

    /// Type of the GeoJSON Object. MUST be set to `"Feature"`.
    #[serde(rename = "type")]
    pub type_: String,

    /// Defines the full footprint of the asset represented by this item. Coordinates are specified in Longitude/Latitude or Longitude/Latitude/Elevation based on WGS 84.
    pub geometry: Geometry,

    /// Bounding Box of the asset represented by this Item.
    pub bbox: Bbox,

    /// A dictionary of additional metadata for the Item.
    pub properties: ItemProperties,

    /// List of link objects to resources and related URLs.
    pub links: Vec<Link>,

    /// Mapping of asset objects that can be downloaded, each with a unique key.
    pub assets: HashMap<String, Asset>,

    /// The `id` of the STAC Collection this Item references to (see [`collection` relation 
    /// type](https://github.com/radiantearth/stac-spec/blob/v1.0.0-rc.1/item-spec/item-spec.md#relation-types)). This field is required if such a relation type is present. 
    /// This field provides an easy way to search for any Items that belong in a specified Collection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection: Option<String>,

    /// Additional fields not covered by the STAC spec.
    #[serde(flatten)]
    pub extra_fields: Value,
}

/// Additional metadata fields associated with the [`Item`] as described in the [Properties Object 
/// spec](https://github.com/radiantearth/stac-spec/blob/v1.0.0-rc.1/item-spec/item-spec.md#properties-object)
#[derive(Serialize, Deserialize)]
pub struct ItemProperties {
    /// Attributes that are part of the [Common Metadata](https://github.com/radiantearth/stac-spec/blob/v1.0.0-rc.1/item-spec/common-metadata.md#date-and-time-range)
    #[serde(flatten)]
    pub common: CommonMetadata,

    /// Attributes associated with a supported [STAC Extension](https://github.com/radiantearth/stac-spec/tree/v1.0.0-rc.1/extensions)
    #[serde(flatten)]
    pub extension: ItemExtensionProperties,

    /// Additional fields not covered by the STAC spec.
    #[serde(flatten)]
    pub extra_fields: Value,
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