#![warn(missing_docs)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions, clippy::doc_markdown)]
//! The `stac-core` crate provides primitive types for working with SpatioTemporal Asset Catalog (STAC) entities,
//! including:
//! 
//! * [Items](https://github.com/radiantearth/stac-spec/blob/master/item-spec/item-spec.md)
//! * [Collections](https://github.com/radiantearth/stac-spec/blob/master/collection-spec/collection-spec.md)
//! * [Catalogs](https://github.com/radiantearth/stac-spec/blob/master/catalog-spec/catalog-spec.md)
//! 
//! All of these structs (and their internal structs) can be serialized from JSON using the [`serde`] crate.
//! 
//! # Features
//! 
//! The following [Cargo features](https://doc.rust-lang.org/cargo/reference/features.html) are available
//! 
//! * `validate`: Enables validation of STAC objects against the appropriate JSON schemas using [`jsonschema`]. This
//! feature is enabled by default, use `default-features = false` in the dependency declaration if you do not need this.
//!
//! # Caveat
//!
//! > This is my first Rust project. Feedback and input is more than welcome!
//!
//! [`jsonschema`](https://docs.rs/jsonschema)
extern crate serde;
extern crate serde_json;
extern crate geojson;
extern crate semver;

#[cfg(feature = "validate")]
extern crate jsonschema;

pub use types::{
    catalog::Catalog,
    collection::{Collection, Extent, SpatialExtent, TemporalExtent},
    common::{Asset, Link, Provider},
    item::Item,
    item_collection::ItemCollection
};

mod types;
mod extensions;
pub mod error;
#[cfg(feature = "validate")]
pub mod validation;
