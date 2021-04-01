//! Tools for validating STAC objects
use std::convert::From;

use semver::Version;
use serde_json::Value;

use crate::{Item, Collection, types::STACObject, Catalog};
pub use util::{get_schema, get_schema_url};
pub use validate::{is_valid};

mod util;
mod validate;

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
    pub fn schema_types(&self) -> Vec<String> {
        let mut schema_types = vec![String::from("core")];
        let stac_extensions = match self.object {
            STACObject::Item(item) => &item.stac_extensions,
            STACObject::Collection(collection) => &collection.stac_extensions,
            STACObject::Catalog(catalog) => &catalog.stac_extensions,
        };
        if let Some(stac_extensions) = stac_extensions {
            for ext in stac_extensions {
                schema_types.push(ext.clone())
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

// impl From<&ItemCollection> for ValidationTarget {
//     fn from(item_collection: &ItemCollection) -> Self {
//         ValidationTarget {
//             stac_version: item_collection.stac_version.clone(),
//             stac_type: item_collection.type_.as_str(),
//             object: STACObject::ItemCollection(item_collection),
//         }
//     }
// }
//
// impl From<&Collection> for ValidationTarget {
//     fn from(collection: &Collection) -> Self {
//         ValidationTarget {
//             stac_version: collection.stac_version.clone(),
//             stac_type: collection.type_.as_str(),
//             object: STACObject::Collection(collection),
//         }
//     }
// }
//
// impl From<&Catalog> for ValidationTarget {
//     fn from(catalog: &Catalog) -> Self {
//         ValidationTarget {
//             stac_version: catalog.stac_version.clone(),
//             stac_type: catalog.type_.as_str(),
//             object: STACObject::Catalog(catalog),
//         }
//     }
// }