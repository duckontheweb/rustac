use jsonschema::is_valid;
use serde_json::{self, Value};

use crate::error::STACValidateResult;
use crate::get_schema;

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
/// * [`STACValidateError::Other`] if instance does not contain the required fields (`"stac_version"` and
///    `"type"`) or if no schema URL can be found for this instance and schema type.
/// * [`STACValidateError::JSONParse`] if there is a problem parsing the schema from the JSON string.
///
/// [`STACValidateError::Other`]: crate::error::STACValidateError::Other
/// [`STACValidateError::JSONParse`]: crate::error::STACValidateError::JSONParse
pub fn is_valid_for_schema_type(instance: &Value, schema_type: &str) -> STACValidateResult<bool>
{
    let schema = get_schema(instance, schema_type)?;
    Ok(is_valid(&schema, instance))
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

// #[cfg(test)]
// mod tests {
//     use std::fs;
//     use serde_json;
//     use semver::Version;
//     use stac_types::{Item, Collection, Catalog};
//
//     use crate::SchemaType;
//     use super::*;
//
//     fn get_test_example(filename: &str) -> String {
//         let path = format!("./tests-data/{}", filename);
//         fs::read_to_string(path).unwrap()
//     }
//
//     #[test]
//     fn test_valid_item() {
//         impl Validate for Item {
//             fn get_type(&self) -> &String { &self.type_ }
//             fn get_stac_version(&self) -> &Version { &self.stac_version }
//         }
//         let data = get_test_example("core-item.json");
//         let item: Item = serde_json::from_str(data.as_str()).unwrap();
//
//         let result = is_valid(
//             &item,
//             SchemaType::Core
//         )
//             .unwrap();
//
//         assert_eq!(result, true);
//     }
//
//     #[test]
//     fn test_valid_collection() {
//         impl Validate for Collection {
//             fn get_type(&self) -> STACType { STACType::Collection }
//             fn get_stac_version(&self) -> Version { self.stac_version.clone() }
//         }
//
//         let data = get_test_example("collection.json");
//         let collection: Collection = serde_json::from_str(data.as_str()).unwrap();
//
//         let result = is_valid(
//             &collection,
//             SchemaType::Core
//         )
//             .unwrap();
//
//         if !result {
//             let collection: Collection = serde_json::from_str(data.as_str()).unwrap();
//             println!("{:#?}", serde_json::to_value(collection).unwrap());
//         }
//         assert_eq!(result, true);
//     }
//
//     #[test]
//     fn test_valid_catalog() {
//         impl Validate for Catalog {
//             fn get_type(&self) -> STACType { STACType::Catalog }
//             fn get_stac_version(&self) -> Version { self.stac_version.clone() }
//         }
//
//         let data = get_test_example("catalog.json");
//         let catalog: Catalog = serde_json::from_str(data.as_str()).unwrap();
//
//         let result = is_valid(
//             &catalog,
//             SchemaType::Core
//         )
//             .unwrap();
//
//         if !result {
//             let catalog: Catalog = serde_json::from_str(data.as_str()).unwrap();
//             println!("{:#?}", serde_json::to_value(catalog).unwrap());
//         }
//         assert_eq!(result, true);
//     }
// }