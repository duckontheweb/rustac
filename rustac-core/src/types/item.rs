//! Implementation of the [STAC Item spec](https://github.com/radiantearth/stac-spec/blob/v1.0.0-rc.1/item-spec/item-spec.md)
use std::collections::HashMap;

use geojson::{Bbox, Geometry};
use semver::Version;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::common::Asset;
use super::common::CommonMetadata;
use super::common::Link;

/// Representation of a [STAC Item](https://github.com/radiantearth/stac-spec/blob/v1.0.0-rc.1/item-spec/item-spec.md).
#[derive(Serialize, Deserialize)]
pub struct Item {
    /// The STAC version the Item implements.
    pub stac_version: Version,

    /// A list of extensions the Item implements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stac_extensions: Option<Vec<String>>,

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

    /// Additional fields not covered by the core STAC spec.
    #[serde(flatten)]
    pub extra_fields: Value,
}

/// Additional metadata fields associated with the [`Item`] as described in the [Properties Object
/// spec](https://github.com/radiantearth/stac-spec/blob/v1.0.0-rc.1/item-spec/item-spec.md#properties-object)
#[derive(Serialize, Deserialize)]
pub struct ItemProperties {
    /// Fields that are part of the [Common Metadata](https://github.com/radiantearth/stac-spec/blob/v1.0.0-rc.1/item-spec/common-metadata.md#date-and-time-range)
    #[serde(flatten)]
    pub common: CommonMetadata,

    /// Additional property fields not covered by the core STAC spec.
    #[serde(flatten)]
    pub extra_fields: Value,
}
