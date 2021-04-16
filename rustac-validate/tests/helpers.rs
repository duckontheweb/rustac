use std::fs;

use rustac_core::{Catalog, Collection, Item};
use rustac_validate::is_valid;
use serde_json::Value;

#[allow(dead_code)]
pub(crate) fn get_example(filename: &str) -> String {
    let path = format!("./stac-examples/{}", filename);
    fs::read_to_string(&path).unwrap_or_else(|_| panic!("Could not open {}", &path.as_str()))
}

#[allow(dead_code)]
pub(crate) fn test_example(example_path: &str) {
    let data = get_example(example_path);
    let value: Value = serde_json::from_str(data.as_str()).unwrap();
    let stac_type = value["type"].as_str().unwrap();

    match stac_type {
        "Feature" => {
            let item: Item = serde_json::from_str(data.as_str()).unwrap();
            assert!(is_valid(&item).unwrap());
        }
        "Collection" => {
            let collection: Collection = serde_json::from_str(data.as_str()).unwrap();
            assert!(is_valid(&collection).unwrap());
        }
        "Catalog" => {
            let catalog: Catalog = serde_json::from_str(data.as_str()).unwrap();
            assert!(is_valid(&catalog).unwrap());
        }
        _ => {}
    }
}
