#![warn(missing_docs)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(
    clippy::module_name_repetitions,
    clippy::doc_markdown,
    clippy::upper_case_acronyms
)]
//! The `rustac-core` crate provides primitive types for working with SpatioTemporal Asset Catalog (STAC) entities,
//! including:
//!
//! * [Items]
//! * [Collections]
//! * [Catalogs]
//!
//! ## Data Types
//!
//! All objects can be serialized from JSON using [`serde`] and [`serde_json`].
//!
//! This library tries to be consistent in how it maps a STAC type to a Rust type. Any STAC attributes that are
//! required are represented as one of the Rust types listed below. Any optional STAC attributes
//! are represented as an [`Option`] with a value that is one of the Rust types listed below.
//!
//! | STAC Type      | Rust Type             |
//! |----------------|-----------------------|
//! | `string`       | [`String`]            |
//! | `number`       | [`f32`]               |
//! | `array`        | [`Vec`]               |
//! | `datetime`     | [`chrono::DateTime`] with [`chrono::FixedOffset`] |
//! | `geometry`     | [`geojson::Geometry`] |
//! | `bbox`         | [`geojson::Bbox`]     |
//! | `stac_version` | [`semver::Version`] |
//!
//! ## Additional Fields & Extensions
//!
//! STAC objects may contain fields not included in the core STAC spec itself. These fields are not serialized into
//! "primitive" Rust types, but instead are flattened into a [`serde_json::Value`] in the `extra_fields` field
//! on the `struct`. *This includes fields that may be defined as part of a [STAC Extension].* Support for accessing extension
//! fields as Rust types will be added in a separate `stac-extensions` crate that is currently under development.
//!
//! ## Validation
//!
//! This crate does not do any validation of STAC objects beyond ensuring that they can be properly deserialized. For validation
//! of STAC entities using JSON Schema, see the `stac-validate` crate.
//!
//! # Examples
//!
//! Create an [`Item`] by deserializing one of the example JSON files.
//!
//! ```
//! use rustac_core::Item;
//! use rustac_core::error::{STACError, STACResult};
//! use semver::Version;
//! use serde_json::json;
//!
//! fn main() -> STACResult<()> {
//!     let example = json!({
//!     "stac_version": "1.0.0-rc.2",
//!     "stac_extensions": [],
//!     "type": "Feature",
//!     "id": "20201211_223832_CS2",
//!     "bbox": [172.91173669923782, 1.3438851951615003, 172.95469614953714, 1.3690476620161975],
//!     "geometry": {
//!       "type": "Polygon",
//!       "coordinates": [
//!         [
//!           [172.91173669923782, 1.3438851951615003],
//!           [172.95469614953714, 1.3438851951615003],
//!           [172.95469614953714, 1.3690476620161975],
//!           [172.91173669923782, 1.3690476620161975],
//!           [172.91173669923782, 1.3438851951615003]
//!         ]
//!       ]
//!     },
//!     "properties": {
//!       "datetime": "2020-12-11T22:38:32.125000Z"
//!     },
//!     "collection": "simple-collection",
//!     "links": [
//!       {
//!         "rel": "collection",
//!         "href": "./collection.json",
//!         "type": "application/json",
//!         "title": "Simple Example Collection"
//!       },
//!       {
//!         "rel": "root",
//!         "href": "./collection.json",
//!         "type": "application/json"
//!       }
//!     ],
//!     "assets": {
//!       "visual": {
//!         "href": "https://storage.googleapis.com/open-cogs/stac-examples/20201211_223832_CS2.tif",
//!         "type": "image/tiff; application=geotiff; profile=cloud-optimized",
//!         "title": "3-Band Visual",
//!         "roles": [
//!           "visual"
//!         ]
//!       },
//!       "thumbnail": {
//!         "href": "https://storage.googleapis.com/open-cogs/stac-examples/20201211_223832_CS2.jpg",
//!         "title": "Thumbnail",
//!         "type": "image/jpeg",
//!         "roles": [
//!           "thumbnail"
//!         ]
//!       }
//!     }
//!   });
//!
//!   let item: Item = serde_json::from_value(example)?;
//!
//!   assert_eq!(item.stac_version, Version::parse("1.0.0-rc.2")?);
//!   assert_eq!(item.type_, String::from("Feature"));
//!
//!   Ok(())
//! }
//! ```
//!
//! [STAC Extension]: https://stac-extensions.github.io/
//! [Items]: https://github.com/radiantearth/stac-spec/blob/master/item-spec/item-spec.md
//! [Collections]: https://github.com/radiantearth/stac-spec/blob/master/collection-spec/collection-spec.md
//! [Catalogs]: https://github.com/radiantearth/stac-spec/blob/master/catalog-spec/catalog-spec.md
pub use types::{
    catalog::Catalog,
    collection::{Collection, Extent, SpatialExtent, TemporalExtent},
    common::{Asset, Link, Provider},
    item::Item,
};

pub mod error;
mod types;
