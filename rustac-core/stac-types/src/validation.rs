use jsonschema::{JSONSchema, Draft};
use serde::Serialize;
use serde_json::Value;
use reqwest::blocking::get;

use crate::error::{Result, StacError};

pub fn is_valid<T>(instance: &T, schema_uri: &str) -> Result<bool> 
where
    T: Serialize
{
    let schema: Value = get(schema_uri)?.json()?;
    Ok(JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema)?
        .is_valid(&serde_json::to_value(instance)?))
    
}

pub fn validate<T>(instance: &T, schema_uri: &str) -> Result<()>
where
    T: Serialize
{
    let schema: Value = get(schema_uri)?.json()?;
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
        Err(StacError::ValidationError(message))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use serde_json;
    use super::is_valid;
    use crate::{Item, Collection, Catalog};

    fn get_test_example(filename: &str) -> String {
        let path = format!("./tests-data/{}", filename);
        fs::read_to_string(path).unwrap()
    }

    #[test]
    fn test_valid_item() {
        let data = get_test_example("core-item.json");
        let item: Item = serde_json::from_str(data.as_str()).unwrap();

        let result = is_valid(
            &item, 
            "https://schemas.stacspec.org/v1.0.0-rc.1/item-spec/json-schema/item.json"
        )
            .unwrap();

        assert_eq!(result, true);
    }

    #[test]
    fn test_valid_collection() {
        let data = get_test_example("collection.json");
        let collection: Collection = serde_json::from_str(data.as_str()).unwrap();

        let result = is_valid(
            &collection, 
            "https://schemas.stacspec.org/v1.0.0-rc.1/collection-spec/json-schema/collection.json"
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
        let data = get_test_example("catalog.json");
        let catalog: Catalog = serde_json::from_str(data.as_str()).unwrap();

        let result = is_valid(
            &catalog, 
            "https://schemas.stacspec.org/v1.0.0-rc.1/catalog-spec/json-schema/catalog.json"
        )
            .unwrap();
        
        if !result {
            let catalog: Catalog = serde_json::from_str(data.as_str()).unwrap();
            println!("{:#?}", serde_json::to_value(catalog).unwrap());
        }
        assert_eq!(result, true);
    }
}