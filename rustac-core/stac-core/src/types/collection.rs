//! Implementation of [STAC Collection](https://github.com/radiantearth/stac-spec/blob/v1.0.0-rc.1/collection-spec/collection-spec.md)
use std::collections::HashMap;

use geojson::Bbox;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use semver::Version;

use crate::types::common::Link;
use crate::types::common::Asset;
use crate::types::common::Provider;

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
