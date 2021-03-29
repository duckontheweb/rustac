use jsonschema::{JSONSchema, Draft};
use serde::Serialize;
use serde_json::Value;

use crate::error::{Result, StacValidateError};
use crate::{SchemaType, Validate};

/// Checks if the given instance is valid. 
pub fn is_valid<T>(schema: Value) -> Result<bool>
where
    T: Serialize + Validate
{
    Ok(JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema)?
        .is_valid(&serde_json::to_value(instance)?))
    
}

/// Checks if the the given instance is valid and returns the validation errors if not.
pub fn validate<T>(instance: &T, schema_type: SchemaType) -> Result<()>
where
    T: Serialize + Validate
{
    let schema: Value = get_schema(instance, schema_type)?;
    let value = serde_json::to_value(instance)?;
    let compiled = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema)?;
    let result = compiled.validate(&value);
    
    if let Err(errors) = result {
        let mut message = String::new();
        for error in errors {
            message.push_str(format!("{}\n", error).as_str())
        }
        Err(StacValidateError::ValidationError(message))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use serde_json;
    use semver::Version;
    use stac_types::{Item, Collection, Catalog};

    use crate::{STACType, SchemaType};
    use super::*;

    fn get_test_example(filename: &str) -> String {
        let path = format!("./tests-data/{}", filename);
        fs::read_to_string(path).unwrap()
    }

    #[test]
    fn test_valid_item() {
        impl Validate for Item {
            fn get_type(&self) -> STACType { STACType::Item }
            fn get_stac_version(&self) -> Version { self.stac_version.clone() }
        }
        let data = get_test_example("core-item.json");
        let item: Item = serde_json::from_str(data.as_str()).unwrap();

        let result = is_valid(
            &item, 
            SchemaType::Core
        )
            .unwrap();

        assert_eq!(result, true);
    }

    #[test]
    fn test_valid_collection() {
        impl Validate for Collection {
            fn get_type(&self) -> STACType { STACType::Collection }
            fn get_stac_version(&self) -> Version { self.stac_version.clone() }
        }

        let data = get_test_example("collection.json");
        let collection: Collection = serde_json::from_str(data.as_str()).unwrap();

        let result = is_valid(
            &collection, 
            SchemaType::Core
        )
            .unwrap();
        
        if !result {
            let collection: Collection = serde_json::from_str(data.as_str()).unwrap();
            println!("{:#?}", serde_json::to_value(collection).unwrap());
        }
        assert_eq!(result, true);
    }

    #[test]
    fn test_valid_catalog() {
        impl Validate for Catalog {
            fn get_type(&self) -> STACType { STACType::Catalog }
            fn get_stac_version(&self) -> Version { self.stac_version.clone() }
        }

        let data = get_test_example("catalog.json");
        let catalog: Catalog = serde_json::from_str(data.as_str()).unwrap();

        let result = is_valid(
            &catalog, 
            SchemaType::Core
        )
            .unwrap();
        
        if !result {
            let catalog: Catalog = serde_json::from_str(data.as_str()).unwrap();
            println!("{:#?}", serde_json::to_value(catalog).unwrap());
        }
        assert_eq!(result, true);
    }
}