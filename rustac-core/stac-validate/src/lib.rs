#![warn(missing_docs)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions, clippy::doc_markdown)]
//! Tools for validating STAC objects
use serde::Serialize;

pub use schema::{get_schema, get_schema_url, SchemaType};
use stac_core::{Catalog, Collection, Item};
pub use validate::{is_valid, ValidationTarget};

extern crate stac_core;
extern crate reqwest;
extern crate jsonschema;
extern crate serde_json;
extern crate serde;

mod schema;
mod validate;
pub mod error;

/// Enumerates the top-level STAC objects
#[derive(Serialize)]
#[serde(untagged)]
enum STACObject<'a> {
    Catalog(&'a Catalog),
    Collection(&'a Collection),
    Item(&'a Item),
}
