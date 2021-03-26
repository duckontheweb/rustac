use serde::{Serialize, Deserialize};
use serde_json::Value;
use stac_types::{Catalog, Collection, Item};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum LinkTarget {
    Catalog(Catalog),
    Collection(Collection),
    Item(Item),
    Other(Value)
}