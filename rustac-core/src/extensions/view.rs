use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ViewCollectionProperties {}

#[derive(Serialize, Deserialize)]
pub struct ViewItemProperties {
    #[serde(rename = "view:off_nadir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_nadir: Option<f32>,
    #[serde(rename = "view:incidence_angle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incidence_angle: Option<f32>,
    #[serde(rename = "view:azimuth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azimuth: Option<f32>,
    #[serde(rename = "view:sun_azimuth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sun_azimuth: Option<f32>,
    #[serde(rename = "view:sun_elevation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sun_elevation: Option<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct ViewAssetProperties {
    #[serde(rename = "view:off_nadir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_nadir: Option<f32>,
    #[serde(rename = "view:incidence_angle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incidence_angle: Option<f32>,
    #[serde(rename = "view:azimuth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azimuth: Option<f32>,
    #[serde(rename = "view:sun_azimuth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sun_azimuth: Option<f32>,
    #[serde(rename = "view:sun_elevation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sun_elevation: Option<f32>,
}
