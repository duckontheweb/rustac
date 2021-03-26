//! Implementation of [STAC Link Objects](https://github.com/radiantearth/stac-spec/blob/v1.0.0-rc.1/item-spec/item-spec.md#link-object).
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents a [STAC Link Object](https://github.com/radiantearth/stac-spec/blob/v1.0.0-rc.1/item-spec/item-spec.md#link-object). This type 
/// may be used for Collection, Catalog, Item, or ItemCollection links.
#[derive(Serialize, Deserialize)]
pub struct Link {
    /// The actual link in the format of an URL. Relative and absolute links are both allowed.
    pub href: String,

    /// Relationship between the current document and the linked document. See chapter [Relation 
    /// types](https://github.com/radiantearth/stac-spec/blob/v1.0.0-rc.1/item-spec/item-spec.md#relation-types) docs for more information.
    pub rel: String,

    /// [Media type](https://github.com/radiantearth/stac-spec/blob/v1.0.0-rc.1/catalog-spec/catalog-spec.md#media-types) of the referenced entity.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,

    /// A human readable title to be used in rendered displays of the link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Additional fields not covered by the STAC spec.
    #[serde(flatten)]
    pub extra_fields: Value,
}
