mod helpers;
use helpers::get_example;
use rustac_core::{Catalog, Collection, Item};
use rustac_validate::is_valid;
use serde_json::Value;
use test_case::test_case;

#[test_case("collection.json")]
#[test_case("item.json")]
fn validate_core_example(path: &str) {
    let data = get_example("scientific-extension", path);
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
