use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::extensions::AssetExtensionProperties;

/// Attributes described by the [STAC Common Metadata spec](https://github.com/radiantearth/stac-spec/blob/v1.0.0-rc.1/item-spec/common-metadata.md).
/// These attributes may apply to a STAC Item or Asset.
#[derive(Serialize, Deserialize)]
pub struct CommonMetadata {
    /// A human readable title describing the Item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Detailed multi-line description to fully explain the Item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The searchable date and time of the assets, in UTC. It is formatted according to [RFC 3339, section 5.6](https://tools.ietf.org/html/rfc3339#section-5.6).
    /// `null` ([`None`]) is allowed, but requires `start_datetime` and `end_datetime` to be set.
    pub datetime: Option<String>,

    /// Creation date and time of the corresponding data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,

    /// Date and time the corresponding data was updated last.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,

    /// The first or start date and time for the Item, in UTC. It is formatted as date-time according to [RFC 3339, section
    /// 5.6](https://tools.ietf.org/html/rfc3339#section-5.6).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_datetime: Option<String>,

    /// The last or end date and time for the Item, in UTC. It is formatted as date-time according to [RFC 3339, section
    /// 5.6](https://tools.ietf.org/html/rfc3339#section-5.6).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_datetime: Option<String>,

    /// Item's license(s), either a SPDX [License identifier](https://spdx.org/licenses/), `"various"` if multiple licenses apply or `"proprietary"` for all
    /// other cases. Should be defined at the Collection level if possible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,

    /// A list of providers, which may include all organizations capturing or processing the data or the hosting provider. Providers should be listed
    /// in chronological order with the most recent provider being the last element of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<Provider>,

    /// Unique name of the specific platform to which the instrument is attached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,

    /// Name of instrument or sensor used (e.g., MODIS, ASTER, OLI, Canon F-1).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instruments: Option<Vec<String>>,

    /// Name of the constellation to which the platform belongs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constellation: Option<String>,

    /// Name of the mission for which data is collected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mission: Option<String>,

    /// Ground Sample Distance at the sensor, in meters (m).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gsd: Option<f32>,
}

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

/// Represents a [Provider Object](https://github.com/radiantearth/stac-spec/blob/v1.0.0-rc.1/collection-spec/collection-spec.md#provider-object). This object
/// may be used in the `"providers"` attribute of a Collection and the Common Metadata of an Item.
#[derive(Serialize, Deserialize)]
pub struct Provider {
    /// The name of the organization or the individual.
    pub name: String,

    /// Multi-line description to add further provider information such as processing details for processors and producers, hosting details for hosts or basic contact information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Roles of the provider. Any of `licensor`, `producer`, `processor` or `host`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,

    /// Homepage on which the provider describes the dataset and publishes contact information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>
}

/// Allowed `"roles"` values for a [`Provider`] object.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProviderRoles {
    /// Maps to the `"licensor"` value
    #[serde(rename = "licensor")]
    Licensor,

    /// Maps to the `"producer"` value
    #[serde(rename = "producer")]
    Producer,

    /// Maps to the `"processor"` value
    #[serde(rename = "processor")]
    Processor,

    /// Maps to the `"host"` value
    #[serde(rename = "host")]
    Host,
}
