use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Provider {
    name: String,
    description: Option<String>,
    roles: Option<Vec<String>>,
    url: Option<String>
}