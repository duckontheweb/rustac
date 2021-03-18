use serde::{Deserialize, Serialize};
use serde_json;

/// Implementation of a STAC Link.
#[derive(Serialize, Deserialize)]
pub struct Link {
    pub href: String,
    pub rel: String,
    #[serde(rename = "type")]
    pub media_type: Option<String>,
    pub title: Option<String>,
    #[serde(flatten)]
    pub extra_fields: serde_json::Value,
}

impl Link {
    /// Serializes the instance to a JSON string
    pub fn to_json(&self) -> serde_json::Result<String> {
        Ok(serde_json::to_string(&self)?)
    }

    /// Creates a new instance from a JSON string
    pub fn from_json(data: &str) -> serde_json::Result<Self> {
        Ok(serde_json::from_str(&data)?)
    }
}

pub trait UsesLinks {
    /// Gets the complete collection of links for this instance.
    fn get_links(&self) -> Vec<&Link>;

    /// Gets all links with the given "rel" type
    fn links_of_rel(&self, rel: &str) -> Vec<&Link> {
        self.get_links()
            .into_iter()
            .filter(|link| link.rel == rel)
            .collect()
    }
}
