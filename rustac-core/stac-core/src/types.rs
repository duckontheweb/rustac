use serde::{Serialize};

use item::Item;
use catalog::Catalog;
use collection::Collection;
// use item_collection::ItemCollection;

pub mod catalog;
pub mod collection;
pub mod item;
pub mod item_collection;
pub mod common;

/// Enumerates the top-level STAC objects
#[derive(Serialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum STACObject<'a> {
    Catalog(&'a Catalog),
    Collection(&'a Collection),
    Item(&'a Item),
    // ItemCollection(&'a ItemCollection),
}
