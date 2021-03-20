use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::extensions::AssetExtensionProperties;
use super::common_metadata::CommonMetadata;

#[derive(Serialize, Deserialize)]
pub struct Asset {
    pub href: String,
    pub title: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub roles: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonMetadata,
    #[serde(flatten)]
    pub extensions: AssetExtensionProperties,
    #[serde(flatten)]
    pub extra_fields: Value,
}
