use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::extensions::AssetExtensionProperties;
use super::common_metadata::CommonMetadata;

#[derive(Serialize, Deserialize)]
pub struct Asset {
    pub href: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonMetadata,
    #[serde(flatten)]
    pub extensions: AssetExtensionProperties,
    #[serde(flatten)]
    pub extra_fields: Value,
}
