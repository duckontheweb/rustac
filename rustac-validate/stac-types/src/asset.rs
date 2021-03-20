use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::extensions::AssetExtensionProperties;

#[derive(Serialize, Deserialize)]
pub struct Asset {
    pub href: String,
    pub title: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub roles: Option<Vec<String>>,
    #[serde(flatten)]
    pub extensions: AssetExtensionProperties,
    #[serde(flatten)]
    pub other: Value,
}
