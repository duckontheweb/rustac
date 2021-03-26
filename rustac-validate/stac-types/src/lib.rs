#![warn(missing_docs)]
//! The `stac-types` crate provides primitive types for working with SpatioTemporal Asset Catalog (STAC)
//! objects, including:
//! 
//! * [Items](https://github.com/radiantearth/stac-spec/blob/master/item-spec/item-spec.md)
//! * [Collections](https://github.com/radiantearth/stac-spec/blob/master/collection-spec/collection-spec.md)
//! * [Catalogs](https://github.com/radiantearth/stac-spec/blob/master/catalog-spec/catalog-spec.md)
//! 
//! All objects can be serialized from JSON using the [`serde`] crate.
//! 
//! # Features
//! 
//! The following [Cargo features](https://doc.rust-lang.org/cargo/reference/features.html) are available
//! 
//! * `validate`: Enables validation of STAC objects using [`jsonschema`].
//! 

extern crate serde;
extern crate serde_json;
extern crate geojson;
#[cfg(feature = "validate")]
extern crate reqwest;
#[cfg(features = "jsonschema")]

pub use link::Link;
pub use item::Item;
pub use asset::Asset;
pub use collection::{Collection, Extent, SpatialExtent, TemporalExtent};
pub use provider::Provider;
pub use catalog::Catalog;
pub use item_collection::ItemCollection;

pub mod link;
pub mod item;
pub mod asset;
pub mod collection;
pub mod catalog;
pub mod item_collection;
pub mod provider;
pub mod common_metadata;
mod extensions;

#[cfg(feature = "validate")]
pub mod validation;
pub mod error;
