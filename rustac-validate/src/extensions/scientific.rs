use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Publication {
    pub doi: String,
    pub citation: String,
}

#[derive(Serialize, Deserialize)]
pub struct ScientificCollectionProperties {
    #[serde(rename = "sci:doi")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doi: Option<String>,
    #[serde(rename = "sci:citation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub citation: Option<String>,
    #[serde(rename = "sci:publications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publications: Option<Vec<Publication>>
}

#[derive(Serialize, Deserialize)]
pub struct ScientificItemProperties {
    #[serde(rename = "sci:doi")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doi: Option<String>,
    #[serde(rename = "sci:citation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub citation: Option<String>,
    #[serde(rename = "sci:publications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publications: Option<Vec<Publication>>
}

#[derive(Serialize, Deserialize)]
pub struct ScientificAssetProperties {}
