use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct ProjectionCollectionProperties {}

#[derive(Serialize, Deserialize)]
pub struct ProjectionItemProperties {
    #[serde(rename = "proj:epsg")]
    pub epsg: Option<u16>,
    #[serde(rename = "proj:wkt2")]
    pub wkt2: Option<String>,
    #[serde(rename = "proj:projjson")]
    pub projjson: Option<Value>,
    #[serde(rename = "proj:geometry")]
    pub geometry: Option<Value>,
    #[serde(rename = "proj:bbox")]
    pub bbox: Option<Vec<f32>>,
    #[serde(rename = "proj:centroid")]
    pub centroid: Option<Vec<f32>>,
    #[serde(rename = "proj:shape")]
    pub shape: Option<Vec<i32>>,
    #[serde(rename = "proj:transform")]
    pub transform: Option<Vec<f32>>
}

#[derive(Serialize, Deserialize)]
pub struct ProjectionAssetProperties {
    #[serde(rename = "proj:epsg")]
    pub epsg: Option<u16>,
    #[serde(rename = "proj:wkt2")]
    pub wkt2: Option<String>,
    #[serde(rename = "proj:projjson")]
    pub projjson: Option<Value>,
    #[serde(rename = "proj:geometry")]
    pub geometry: Option<Value>,
    #[serde(rename = "proj:bbox")]
    pub bbox: Option<Vec<f32>>,
    #[serde(rename = "proj:centroid")]
    pub centroid: Option<Vec<f32>>,
    #[serde(rename = "proj:shape")]
    pub shape: Option<Vec<i32>>,
    #[serde(rename = "proj:transform")]
    pub transform: Option<Vec<f32>>
}
