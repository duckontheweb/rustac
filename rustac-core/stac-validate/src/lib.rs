#![warn(missing_docs)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions, clippy::doc_markdown)]
//! Tools for validating STAC objects

use stac_core::{Catalog, Collection, Item};
use std::convert::From;
use semver::{Version, VersionReq};
use serde_json::Value;

use error::STACResult;
use util::{get_schema_root, get_extension_path, STACObject, is_valid_for_schema_type};

// pub use validate::{is_valid, ValidationTarget};

extern crate stac_core;
extern crate reqwest;
extern crate jsonschema;
extern crate serde_json;
extern crate serde;

/// Checks if the given instance is valid for all schema types associated with it. This will always
/// check against the "core" schema for this object and will additionally check against schemas
/// for any extensions that the object implements and that are supported by this package.
///
/// # Arguments
///
/// * `instance` - This can be any struct that can be converted into a [`ValidationTarget`]. This
///    currently applies to the [`Item`], [`Collection`], and [`Catalog`] types.
///
/// # Errors
///
/// This function may return any of the following errors:
///
/// * [`STACError::Other`] if no schema URL can be found for this instance and schema type.
/// * [`STACError::JSONParse`] if there is a problem parsing a schema from the JSON string.
///
/// [`Value`]: serde_json::Value
/// [`Item`]: crate::Item
/// [`Collection`]: crate::Collection
/// [`Catalog`]: crate::Catalog
/// [`STACError::Other`]: crate::error::STACError::Other
/// [`STACError::JSONParse`]: crate::error::STACError::JSONParse
pub fn is_valid<'a, T>(instance: &'a T) -> STACResult<bool>
where
    &'a T: 'a + Into<ValidationTarget<'a>>
{
    let target: ValidationTarget = instance.into();
    let schema_uris = &target.schema_uris();

    for schema_uri in schema_uris {
        if !is_valid_for_schema_type(&target, schema_uri)? {
            return Ok(false)
        }
    }

    Ok(true)
}

/// Represents a target for validating against a STAC spec. Implements [`From`] for the
/// [`Item`], [`Catalog`], and [`Collection`] structs which allows us to use
/// `Into<ValidationTarget>` as a trait bound in [`is_valid`].
pub struct ValidationTarget<'a> {
    object: STACObject<'a>,
}

impl <'a> ValidationTarget<'a> {

    /// Gets the internal struct as a serialized [`Value`]
    fn serialized_object(&self) -> Value {
        serde_json::to_value(&self.object).unwrap()
    }

    /// Gets the STAC spec version associated with this target
    fn stac_version(&self) -> &'a Version {
        match self.object {
            STACObject::Item(item) => &item.stac_version,
            STACObject::Collection(collection) => &collection.stac_version,
            STACObject::Catalog(catalog) => &catalog.stac_version,
        }
    }

    /// Gets all of the schema types for this target by combining the "core" schema type with any
    /// extension IDs for extensions implemented on the target.
    fn schema_uris(&self) -> Vec<String> {
        let mut schema_uris = vec![self.core_schema_uri()];
        let at_least_rc2 = VersionReq::parse(">=1.0.0-rc.2").unwrap();
        let stac_extensions = match self.object {
            STACObject::Item(item) => &item.stac_extensions,
            STACObject::Collection(collection) => &collection.stac_extensions,
            STACObject::Catalog(catalog) => &catalog.stac_extensions,
        };
        if let Some(stac_extensions) = stac_extensions {
            for ext in stac_extensions {
                if at_least_rc2.matches(&self.stac_version()) {
                    // Starting with STAC v1.0.0-rc.2 extensions are identified by their full conformance (schema) URI rather than using 
                    // an extension ID.
                    schema_uris.push(ext.as_str().into());
                } else if let Some(extension_uri) = get_extension_path(ext.as_str(), &self.object) {
                    // Prior to v1.0.0-rc.2 extensions had a short ID and we need to look up the schema based on the ID. This may return 
                    // None since we only support a pre-defined set of extensions in this package.
                    schema_uris.push(extension_uri);
                }
            }
        }
        schema_uris
    }

    /// Gets the schema URI for the core schema associated with this STAC type.
    fn core_schema_uri(&self) -> String {
        let schema_path = match self.object {
            STACObject::Item(_) => "item-spec/json-schema/item.json",
            STACObject::Collection(_) => "collection-spec/json-schema/collection.json",
            STACObject::Catalog(_) => "catalog-spec/json-schema/catalog.json",
        };
        format!("{}/{}", get_schema_root(&self.stac_version()), schema_path)
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

mod util;
pub mod error;
