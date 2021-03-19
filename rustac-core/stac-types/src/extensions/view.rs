use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ViewItemProperties {
    #[serde(rename = "view:off_nadir")]
    pub off_nadir: Option<f32>,
    #[serde(rename = "view:incidence_angle")]
    pub incidence_angle: Option<f32>,
    #[serde(rename = "view:azimuth")]
    pub azimuth: Option<f32>,
    #[serde(rename = "view:sun_azimuth")]
    pub sun_azimuth: Option<f32>,
    #[serde(rename = "view:sun_elevation")]
    pub sun_elevation: Option<f32>,
}