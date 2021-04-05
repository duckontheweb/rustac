#![warn(missing_docs)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions, clippy::doc_markdown)]
//! Tools for validating STAC objects
use semver::Version;
use serde::Serialize;
use serde_json::Value;

extern crate stac_core;
extern crate reqwest;
extern crate jsonschema;
extern crate serde_json;
extern crate serde;

use stac_core::{Catalog, Collection, Item};
pub use schema::{get_schema, get_schema_url, SchemaType};
pub use validate::is_valid;

mod schema;
mod validate;
pub mod error;

use std::convert::From;


/// Represents a valid target for validating against a STAC spec. Implements [`From`] for the
/// [`Item`], [`Catalog`], and [`Collection`] structs which allows us to use
/// `Into<ValidationTarget>` as a trait bound in [`is_valid`].
pub struct ValidationTarget<'a> {
    object: STACObject<'a>,
}

impl <'a> ValidationTarget<'a> {

    /// Gets the internal struct as a serialized [`Value`]
    pub fn serialized_object(&self) -> Value {
        serde_json::to_value(&self.object).unwrap()
    }

    /// Gets the STAC spec version associated with this target
    pub fn stac_version(&self) -> &'a Version {
        match self.object {
            STACObject::Item(item) => &item.stac_version,
            STACObject::Collection(collection) => &collection.stac_version,
            STACObject::Catalog(catalog) => &catalog.stac_version,
        }
    }

    /// Gets the type of this target as a string slice.
    pub fn stac_type(&self) -> &str {
        match self.object {
            STACObject::Item(item) => item.type_.as_str(),
            STACObject::Collection(collection) => collection.type_.as_str(),
            STACObject::Catalog(catalog) => catalog.type_.as_str(),
        }
    }

    /// Gets all of the schema types for this target by combining the "core" schema type with any
    /// extension IDs for extensions implemented on the target.
    pub fn schema_types(&self) -> Vec<SchemaType> {
        let mut schema_types = vec![SchemaType::from("core")];
        let stac_extensions = match self.object {
            STACObject::Item(item) => &item.stac_extensions,
            STACObject::Collection(collection) => &collection.stac_extensions,
            STACObject::Catalog(catalog) => &catalog.stac_extensions,
        };
        if let Some(stac_extensions) = stac_extensions {
            for ext in stac_extensions {
                schema_types.push(ext.as_str().into())
            }
        }
        schema_types
    }
}

impl <'a> From<&'a Item> for ValidationTarget<'a> {
    fn from(item: &'a Item) -> ValidationTarget<'a> {
        ValidationTarget {
            object: STACObject::Item(&item)
        }
    }
}

impl <'a> From<&'a Collection> for ValidationTarget<'a> {
    fn from(collection: &'a Collection) -> ValidationTarget<'a> {
        ValidationTarget {
            object: STACObject::Collection(&collection)
        }
    }
}

impl <'a> From<&'a Catalog> for ValidationTarget<'a> {
    fn from(catalog: &'a Catalog) -> ValidationTarget<'a> {
        ValidationTarget {
            object: STACObject::Catalog(&catalog)
        }
    }
}

/// Enumerates the top-level STAC objects
#[derive(Serialize)]
#[serde(untagged)]
enum STACObject<'a> {
    Catalog(&'a Catalog),
    Collection(&'a Collection),
    Item(&'a Item),
}
