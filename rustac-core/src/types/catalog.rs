//! Implementation of the [STAC Catalog spec](https://github.com/radiantearth/stac-spec/blob/v1.0.0-rc.1/catalog-spec/catalog-spec.md)
use semver::Version;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use super::common::Link;

/// Representation of a[STAC Catalog](https://github.com/radiantearth/stac-spec/blob/v1.0.0-rc.1/catalog-spec/catalog-spec.md).
#[derive(Serialize, Deserialize, Debug)]
pub struct Catalog {
    /// The STAC version the Catalog implements.
    pub stac_version: Version,

    /// Set to Catalog if this Catalog only implements the Catalog spec.
    /// **This maps to the STAC `"type"` attribute, which is a reserved keyword.**
    pub r#type: String,

    /// A list of extension identifiers the Catalog implements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stac_extensions: Option<Vec<String>>,

    /// Identifier for the Catalog.
    pub id: String,

    /// A short descriptive one-line title for the Catalog.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Detailed multi-line description to fully explain the Catalog.
    pub description: String,

    /// A map of property summaries, either a set of values or statistics such as a range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summaries: Option<HashMap<String, Value>>,

    /// A list of references to other documents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,

    /// Additional fields not covered by the core STAC spec.
    #[serde(flatten)]
    pub extra_fields: Value,
}
