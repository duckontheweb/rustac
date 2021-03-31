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
//! * `validate`: Enables validation of STAC objects using [`jsonschema`](https://docs.rs/jsonschema).
//! 

extern crate serde;
extern crate serde_json;
extern crate geojson;
extern crate semver;

#[cfg(feature = "validate")]
extern crate jsonschema;

pub use types::{
    item::Item,
    catalog::Catalog,
    collection::{Collection, Extent, SpatialExtent, TemporalExtent},
    item_collection::ItemCollection,
    fragments::{Link, Asset, Provider}
};

mod types;
mod extensions;
pub mod error;
#[cfg(feature = "validate")]
pub mod validation;
