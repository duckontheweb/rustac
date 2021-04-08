#![warn(missing_docs)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions, clippy::doc_markdown)]
//! Tools for validating STAC objects
use serde::Serialize;

use stac_core::{Catalog, Collection, Item};
use std::convert::From;
use reqwest::blocking::get;
use semver::{Version, VersionReq};
use serde_json::Value;

use error::STACResult;

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

fn is_valid_for_schema_type(target: &ValidationTarget, schema_uri: &str) -> STACResult<bool>
{
    let instance = &target.serialized_object();
    let schema = get(schema_uri)?.json()?;
    Ok(jsonschema::is_valid(&schema, instance))
}

/// Represents a valid target for validating against a STAC spec. Implements [`From`] for the
/// [`Item`], [`Catalog`], and [`Collection`] structs which allows us to use
/// `Into<ValidationTarget>` as a trait bound in [`is_valid`].
pub struct ValidationTarget<'a> {
    object: STACObject<'a>,
}

impl <'a> ValidationTarget<'a> {

    /// Gets the internal struct as a serialized [`Value`]
    #[must_use]
    pub(crate) fn serialized_object(&self) -> Value {
        serde_json::to_value(&self.object).unwrap()
    }

    /// Gets the STAC spec version associated with this target
    #[must_use]
    pub(crate) fn stac_version(&self) -> &'a Version {
        match self.object {
            STACObject::Item(item) => &item.stac_version,
            STACObject::Collection(collection) => &collection.stac_version,
            STACObject::Catalog(catalog) => &catalog.stac_version,
        }
    }

    /// Gets the type of this target as a string slice.
    #[must_use]
    pub(crate) fn stac_type(&self) -> STACType {
        match self.object {
            STACObject::Item(_) => STACType::Item,
            STACObject::Collection(_) => STACType::Collection,
            STACObject::Catalog(_) => STACType::Catalog,
        }
    }

    /// Gets all of the schema types for this target by combining the "core" schema type with any
    /// extension IDs for extensions implemented on the target.
    #[must_use]
    pub(crate) fn schema_uris(&self) -> Vec<String> {
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
                    schema_uris.push(ext.as_str().into());
                } else if let Some(extension_uri) = self.uri_from_extension_id(ext.as_str()) {
                    schema_uris.push(extension_uri);
                }
            }
        }
        schema_uris
    }

    fn core_schema_uri(&self) -> String {
        let schema_path = match self.stac_type() {
            STACType::Item => "item-spec/json-schema/item.json",
            STACType::Collection => "collection-spec/json-schema/collection.json",
            STACType::Catalog => "catalog-spec/json-schema/catalog.json",
        };
        format!("{}/{}", self.schema_root(), schema_path)
    }

    fn schema_root(&self) -> String
    {
        let stac_version = match self.object {
            STACObject::Item(item) => &item.stac_version,
            STACObject::Collection(collection) => &collection.stac_version,
            STACObject::Catalog(catalog) => &catalog.stac_version,
        };

        let at_least_v1 = VersionReq::parse(">=1.0.0-beta.1").unwrap();

        if at_least_v1.matches(&stac_version) {
            format!("https://schemas.stacspec.org/v{}", stac_version.to_string())
        } else {
            format!("https://raw.githubusercontent.com/radiantearth/stac-spec/v{}", stac_version.to_string())
        }
    }

    fn uri_from_extension_id(&self, extension_id: &str) -> Option<String>
    {
        let stac_type = self.stac_type();
        let schema_path = match extension_id {
            
            "eo" => match stac_type {
                STACType::Item => Some(String::from("extensions/eo/json-schema/schema.json")),
                _ => None,
            },
            "projection" => match stac_type {
                STACType::Item => Some(String::from("extensions/projection/json-schema/schema.json")),
                _ => None,
            },
            "scientific" => match stac_type {
                STACType::Item | STACType::Collection => Some(String::from("extensions/scientific/json-schema/schema.json")),
                _ => None,
            },
            "view" => match stac_type {
                STACType::Item => Some(String::from("extensions/view/json-schema/schema.json")),
                _ => None,
            },
            _ => None,
        };

        if let Some(schema_path) = schema_path {
            Some(format!("{}/{}", self.schema_root(), schema_path))
        } else {
            None
        }
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

pub(crate) enum STACType {
    Catalog,
    Collection,
    Item,
}

pub mod error;

/// Enumerates the top-level STAC objects
#[derive(Serialize)]
#[serde(untagged)]
enum STACObject<'a> {
    Catalog(&'a Catalog),
    Collection(&'a Collection),
    Item(&'a Item),
}

#[cfg(test)]
mod tests {
    use std::fs;
    use stac_core::Collection;
    use serde_json::Value;

    use super::*;

    fn get_test_example(filename: &str) -> String {
        let path = format!("../stac-examples/{}", filename);
        fs::read_to_string(path).unwrap()
    }

    fn test_example(example_path: &str, version: &str) {
        let data = get_test_example(format!("{}/{}", version, example_path).as_str());
        let value: Value = serde_json::from_str(data.as_str()).unwrap();
        let stac_type = value["type"].as_str().unwrap();

        match stac_type {
            "Feature" => {
                let item: Item = serde_json::from_str(data.as_str()).unwrap();
                assert!(is_valid(&item).unwrap());
            },
            "Collection" => {
                let collection: Collection = serde_json::from_str(data.as_str()).unwrap();
                assert!(is_valid(&collection).unwrap());
            },
            "Catalog" => {
                let catalog: Catalog = serde_json::from_str(data.as_str()).unwrap();
                assert!(is_valid(&catalog).unwrap());
            },
            _ => {}
        }
    }

    #[test] 
    fn test_core_item() { test_example("core/core-item.json", "1.0.0-rc.2") }

    #[test] 
    fn test_simple_item() { test_example("core/simple-item.json", "1.0.0-rc.2") }

    #[test]
    fn test_extended_item() { test_example("core/extended-item.json", "1.0.0-rc.2") }

    #[test]
    fn test_collectionless_item() { test_example("core/collectionless-item.json", "1.0.0-rc.2")}

    #[test]
    fn test_collection() { test_example("core/collection.json", "1.0.0-rc.2")}

    #[test]
    fn test_catalog() { test_example("core/catalog.json", "1.0.0-rc.2")}


    #[test]
    fn test_collection_only() { test_example("core/collection-only/collection.json", "1.0.0-rc.2")}

    #[test]
    fn test_extensions_collection() { test_example("core/extensions-collection/proj-example/proj-example.json", "1.0.0-rc.2")}


}