//! Implementation of the [STAC Provider Object](https://github.com/radiantearth/stac-spec/blob/v1.0.0-rc.1/collection-spec/collection-spec.md#provider-object)
use serde::{Serialize, Deserialize};

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