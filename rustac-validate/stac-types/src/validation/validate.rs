use serde::Serialize;

use crate::error::{STACResult, STACError};
use super::get_schema;

/// Checks if the given instance is valid for all schema types associated with it. This will always
/// check against the "core" schema for this object and will additionally check against schemas
/// for any extensions that the object implements.
///
/// # Arguments
///
/// * `instance` - A serializable struct that has `stac_version` and `type` fields.
///
/// # Errors
///
/// In addition to any errors resulting from calls to the [`is_valid_for_schema_type`] function,
/// this function will return [`STACError::Other`] if the `stac_extensions` field is not
/// an array or does not contains strings.
///
pub fn is_valid<T: Serialize>(instance: &T) -> STACResult<bool> {
    let instance = serde_json::to_value(instance)?;
    let mut schema_types = Vec::new();

    // Always test against the core spec
    schema_types.push("core");

    let stac_extensions = instance.get("stac_extensions");
    if let Some(stac_extensions) = stac_extensions {
        let stac_extensions = stac_extensions
            .as_array()
            .ok_or_else(|| STACError::Other(String::from("Invalid stac_extensions field")))?;
        for stac_extension in stac_extensions {
            let stac_extension = stac_extension
                .as_str()
                .ok_or_else(|| STACError::Other(String::from("Invalid stac_extension value")))?;
            schema_types.push(stac_extension);
        }
    };

    for schema_type in schema_types {
        if !is_valid_for_schema_type(&instance, schema_type)? {
            return Ok(false)
        }
    }

    Ok(true)
}

/// Checks if the given instance is valid for the given schema type.
///
/// # Arguments
///
/// * `instance` - A [`Value`] representing the STAC object to validate
/// * `schema_type` - This must be either a STAC extension ID (e.g. `"eo"`) or the value `"core"`
///    (to validate against the core spec).
///
/// # Errors
///
/// This function may return the following errors:
///
/// * [`STACError::Other`] if instance does not contain the required fields (`"stac_version"` and
///    `"type"`) or if no schema URL can be found for this instance and schema type.
/// * [`STACError::JSONParse`] if there is a problem parsing the schema from the JSON string.
///
/// [`Value`]: serde_json::Value
pub fn is_valid_for_schema_type<T: Serialize>(instance: &T, schema_type: &str) -> STACResult<bool>
{
    let instance = serde_json::to_value(instance)?;
    let schema = get_schema(&instance, schema_type)?;
    Ok(jsonschema::is_valid(&schema, &instance))
}

// /// Checks if the the given instance is valid and returns the validation errors if not.
// pub fn validate<T: Serialize>(instance: &T, schema: Value) -> Result<()>
// {
//     let schema: Value = get_schema(instance, schema_type)?;
//     let value = serde_json::to_value(instance)?;
//     let compiled = JSONSchema::options()
//         .with_draft(Draft::Draft7)
//         .compile(&schema)?;
//     let result = compiled.validate(&value);
//
//     if let Err(errors) = result {
//         let mut message = String::new();
//         for error in errors {
//             message.push_str(format!("{}\n", error).as_str())
//         }
//         Err(Error::ValidationError(message))
//     } else {
//         Ok(())
//     }
// }

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

        assert!(is_valid_for_schema_type(&item, "core").unwrap());
        assert!(is_valid(&item).unwrap());
    }
}