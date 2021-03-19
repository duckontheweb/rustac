use serde::{Serialize, Deserialize};

use view::{ViewItemProperties, ViewAssetProperties, ViewCollectionProperties};
use eo::{EoItemProperties, EoAssetProperties, EoCollectionProperties};
use projection::{ProjectionItemProperties, ProjectionAssetProperties, ProjectionCollectionProperties};
use scientific::{ScientificItemProperties, ScientificAssetProperties, ScientificCollectionProperties};

// pub mod eo;
mod projection;
mod view;
mod eo;
mod scientific;

#[derive(Serialize, Deserialize)]
pub struct CollectionExtensionProperties {
    #[serde(flatten)]
    pub view: ViewCollectionProperties,
    #[serde(flatten)]
    pub eo: EoCollectionProperties,
    #[serde(flatten)]
    pub proj: ProjectionCollectionProperties,
    #[serde(flatten)]
    pub sci: ScientificCollectionProperties,
}

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

#[derive(Serialize, Deserialize)]
pub struct AssetExtensionProperties {
    #[serde(flatten)]
    pub view: ViewAssetProperties,
    #[serde(flatten)]
    pub eo: EoAssetProperties,
    #[serde(flatten)]
    pub proj: ProjectionAssetProperties,
    #[serde(flatten)]
    pub sci: ScientificAssetProperties,
}


