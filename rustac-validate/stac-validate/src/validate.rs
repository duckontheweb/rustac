use crate::error::{STACResult};
use super::{get_schema, ValidationTarget};
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