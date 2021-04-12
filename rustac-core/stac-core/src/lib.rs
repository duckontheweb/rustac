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
//! For validation of STAC entities using JSON Schema, see the `stac-validate` crate.
//! 
//! # Data Types
//! 
//! All objects can be serialized from JSON using [`serde`] and [`serde_json`].
//! 
//! This library tries to be consistent in how it maps a STAC type to a Rust type. Any STAC attributes that are 
//! required are represented as one of the Rust types listed below. Any optional STAC attributes 
//! are represented as an [`Option`] with a value that is one of the Rust types listed below.
//! 
//! | STAC Type  | Rust Type             |
//! |------------|-----------------------|
//! | `string`   | [`String`]            |
//! | `number`   | [`f32`]               |
//! | `array`    | [`Vec`]               |
//! | `datetime` | [`chrono::DateTime`] with [`chrono::FixedOffset`] |
//! | `geometry` | [`geojson::Geometry`] |
//! | `bbox`     | [`geojson::Bbox`]     |
//! 
//! STAC objects may contain fields not included in the STAC spec itself. These fields are not serialized into "primitive" 
//! Rust types, but instead are flattened into a [`serde_json::Value`] in the `extra_fields` field on the `struct`.
//! 
pub use types::{
    catalog::Catalog,
    collection::{Collection, Extent, SpatialExtent, TemporalExtent},
    common::{Asset, Link, Provider},
    item::Item,
};

mod types;
pub mod error;