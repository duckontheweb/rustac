use serde::{Serialize, Deserialize};

use view::ViewItemProperties;
use eo::EoItemProperties;
use projection::ProjectionItemProperties;
use scientific::ScientificItemProperties;

// pub mod eo;
mod projection;
mod view;
mod eo;
mod scientific;

#[derive(Serialize, Deserialize)]
pub struct ItemExtensionProperties {
    #[serde(flatten)]
    pub view: ViewItemProperties,
    #[serde(flatten)]
    pub eo: EoItemProperties,
    #[serde(flatten)]
    pub proj: ProjectionItemProperties,
    #[serde(flatten)]
    pub sci: ScientificItemProperties,
}
