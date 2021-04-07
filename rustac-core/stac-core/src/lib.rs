#![warn(missing_docs)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions, clippy::doc_markdown)]
//! The `stac-types` crate provides primitive types for working with the core SpatioTemporal Asset
//! Catalog (STAC) spec. This includes types for working with:
//! 
//! * [Items](https://github.com/radiantearth/stac-spec/blob/master/item-spec/item-spec.md)
//! * [Collections](https://github.com/radiantearth/stac-spec/blob/master/collection-spec/collection-spec.md)
//! * [Catalogs](https://github.com/radiantearth/stac-spec/blob/master/catalog-spec/catalog-spec.md)
//! 
//! All objects can be serialized from JSON using the [`serde`] crate.
extern crate serde;
extern crate serde_json;
extern crate geojson;
extern crate semver;
extern crate chrono;

pub use types::{
    catalog::Catalog,
    collection::{Collection, Extent, SpatialExtent, TemporalExtent},
    common::{Asset, Link, Provider},
    item::Item,
};

mod types;
pub mod error;