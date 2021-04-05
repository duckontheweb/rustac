use std::convert::From;
use semver::Version;
use serde_json::Value;
use stac_core::{Catalog, Collection, Item};

use crate::error::STACResult;
use crate::STACObject;

use super::get_schema;
use super::schema::SchemaType;

/// Checks if the given instance is valid for all schema types associated with it. This will always
/// check against the "core" schema for this object and will additionally check against schemas
/// for any extensions that the object implements.
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
    let schema_types = &target.schema_types();

    for schema_type in schema_types {
        match schema_type {
            SchemaType::Invalid => {},
            _ => {
                if !is_valid_for_schema_type(&target, schema_type)? {
                    return Ok(false)
                }
            }
        }
    }

    Ok(true)
}

fn is_valid_for_schema_type(target: &ValidationTarget, schema_type: &SchemaType) -> STACResult<bool>
{
    let instance = &target.serialized_object();
    let schema = get_schema(&target, schema_type)?;
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
    pub fn serialized_object(&self) -> Value {
        serde_json::to_value(&self.object).unwrap()
    }

    /// Gets the STAC spec version associated with this target
    #[must_use]
    pub fn stac_version(&self) -> &'a Version {
        match self.object {
            STACObject::Item(item) => &item.stac_version,
            STACObject::Collection(collection) => &collection.stac_version,
            STACObject::Catalog(catalog) => &catalog.stac_version,
        }
    }

    /// Gets the type of this target as a string slice.
    #[must_use]
    pub fn stac_type(&self) -> &str {
        match self.object {
            STACObject::Item(item) => item.type_.as_str(),
            STACObject::Collection(collection) => collection.type_.as_str(),
            STACObject::Catalog(catalog) => catalog.type_.as_str(),
        }
    }

    /// Gets all of the schema types for this target by combining the "core" schema type with any
    /// extension IDs for extensions implemented on the target.
    #[must_use]
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

#[cfg(test)]
mod tests {
    use std::fs;

    use serde_json;

    use crate::Item;

    use super::*;

    fn get_test_example(filename: &str) -> String {
        let path = format!("./tests-data/{}", filename);
        fs::read_to_string(path).unwrap()
    }

    #[test]
    fn test_valid_item() {
        let data = get_test_example("core-item.json");
        let item: Item = serde_json::from_str(data.as_str()).unwrap();

        // assert!(is_valid_for_schema_type(&item, "core").unwrap());
        assert!(is_valid(&item).unwrap());
    }
}
