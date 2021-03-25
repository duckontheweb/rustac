use geojson::{Geometry, Bbox};
use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct ProjectionCollectionProperties {}

#[derive(Serialize, Deserialize)]
pub struct ProjectionItemProperties {
    #[serde(rename = "proj:epsg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epsg: Option<u32>,
    #[serde(rename = "proj:wkt2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wkt2: Option<String>,
    #[serde(rename = "proj:projjson")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projjson: Option<Value>,
    #[serde(rename = "proj:geometry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry: Option<Geometry>,
    #[serde(rename = "proj:bbox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox: Option<Bbox>,
    #[serde(rename = "proj:centroid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub centroid: Option<Geometry>,
    #[serde(rename = "proj:shape")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shape: Option<Vec<i32>>,
    #[serde(rename = "proj:transform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform: Option<Vec<f32>>
}

#[derive(Serialize, Deserialize)]
pub struct ProjectionAssetProperties {
    #[serde(rename = "proj:epsg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epsg: Option<u16>,
    #[serde(rename = "proj:wkt2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wkt2: Option<String>,
    #[serde(rename = "proj:projjson")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projjson: Option<Value>,
    #[serde(rename = "proj:geometry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry: Option<Value>,
    #[serde(rename = "proj:bbox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox: Option<Vec<f32>>,
    #[serde(rename = "proj:centroid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub centroid: Option<Vec<f32>>,
    #[serde(rename = "proj:shape")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shape: Option<Vec<i32>>,
    #[serde(rename = "proj:transform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform: Option<Vec<f32>>
}
