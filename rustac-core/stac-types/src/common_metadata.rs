//! Attributes described by the [STAC Common Metadata spec](https://github.com/radiantearth/stac-spec/blob/v1.0.0-rc.1/item-spec/common-metadata.md)

use serde::{Serialize, Deserialize};

use super::provider::Provider;

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