//! Implementation of the [STAC Asset Object spec](https://github.com/radiantearth/stac-spec/blob/v1.0.0-rc.1/item-spec/item-spec.md#asset-object) 
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::extensions::AssetExtensionProperties;
use super::common_metadata::CommonMetadata;

/// Represents a [STAC Asset Object](https://github.com/radiantearth/stac-spec/blob/master/item-spec/item-spec.md#asset-object) 
/// that may be used by either an Item or a Collection.
#[derive(Serialize, Deserialize)]
pub struct Asset {
    /// URI to the asset object. Relative and absolute URI are both allowed.
    pub href: String,
    
    /// The displayed title for clients and users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    
    /// A description of the Asset providing additional details, such as how it was processed or created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// [Media type](https://github.com/radiantearth/stac-spec/blob/master/item-spec/item-spec.md#asset-media-type) of the asset. 
    /// See the [common media types](https://github.com/radiantearth/stac-spec/blob/master/best-practices.md#common-media-types-in-stac) 
    /// in the best practice doc for commonly used asset types.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,

    /// The [semantic roles](https://github.com/radiantearth/stac-spec/blob/master/item-spec/item-spec.md#asset-role-types) of the asset, 
    /// similar to the use of rel in links.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,

    /// Attributes included in the [STAC Common Metadata](https://github.com/radiantearth/stac-spec/blob/master/item-spec/common-metadata.md) 
    /// spec.
    #[serde(flatten)]
    pub common: CommonMetadata,
    
    /// Attributes included in a STAC extensions applying to Asset objects.
    #[serde(flatten)]
    pub extensions: AssetExtensionProperties,

    /// Additional fields not covered by the STAC spec.
    #[serde(flatten)]
    pub extra_fields: Value,
}
