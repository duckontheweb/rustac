use serde::{Serialize, Deserialize};

use super::provider::Provider;

#[derive(Serialize, Deserialize)]
pub struct CommonMetadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub datetime: Option<String>,
    pub created: Option<String>,
    pub updated: Option<String>,
    pub start_datetime: Option<String>,
    pub end_datetime: Option<String>,
    pub license: Option<String>,
    pub provider: Option<Provider>,
    pub platform: Option<String>,
    pub instruments: Option<Vec<String>>,
    pub constellation: Option<String>,
    pub mission: Option<String>,
    pub gsd: Option<f32>,
}