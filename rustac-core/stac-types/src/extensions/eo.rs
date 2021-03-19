use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct BandsObject {
    pub name: Option<String>,
    pub common_name: Option<String>,
    pub description: Option<String>,
    pub center_wavelength: Option<f32>,
    pub full_width_half_max: Option<f32>
}

#[derive(Serialize, Deserialize)]
pub struct EoItemProperties {
    #[serde(rename = "eo:bands")]
    pub bands: Option<Vec<BandsObject>>,
    #[serde(rename = "eo:cloud_cover")]
    pub cloud_cover: Option<f32>
}