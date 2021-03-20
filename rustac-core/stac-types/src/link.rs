use serde::{Deserialize, Serialize};
use serde_json;

/// Implementation of a STAC Link.
#[derive(Serialize, Deserialize)]
pub struct Link {
    pub href: String,
    pub rel: String,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub title: Option<String>,
    #[serde(flatten)]
    pub other: serde_json::Value,
}
