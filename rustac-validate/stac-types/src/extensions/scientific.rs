use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Publication {
    pub doi: String,
    pub citation: String,
}

#[derive(Serialize, Deserialize)]
pub struct ScientificCollectionProperties {
    #[serde(rename = "sci:doi")]
    pub doi: Option<String>,
    #[serde(rename = "sci:citation")]
    pub citation: Option<String>,
    #[serde(rename = "sci:publications")]
    pub publications: Option<Vec<Publication>>
}

#[derive(Serialize, Deserialize)]
pub struct ScientificItemProperties {
    #[serde(rename = "sci:doi")]
    pub doi: Option<String>,
    #[serde(rename = "sci:citation")]
    pub citation: Option<String>,
    #[serde(rename = "sci:publications")]
    pub publications: Option<Vec<Publication>>
}

#[derive(Serialize, Deserialize)]
pub struct ScientificAssetProperties {}
