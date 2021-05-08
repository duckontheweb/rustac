mod helpers;
use helpers::{get_example, test_example};
use rustac_core::Item;
use rustac_validate::is_valid;
use serde_json::Value;

#[test]
fn test_core_item() {
    test_example("core/core-item.json")
}

#[test]
fn test_simple_item() {
    test_example("core/simple-item.json")
}

#[test]
fn test_extended_item() {
    test_example("core/extended-item.json")
}

#[test]
fn test_collectionless_item() {
    test_example("core/collectionless-item.json")
}

#[test]
fn test_eo_extended_item() {
    test_example("extensions/eo/item.json")
}

#[test]
// jsonschema is saying it is valid this even though it is not
// See https://github.com/Stranger6667/jsonschema-rs/issues/183 for details
fn test_invalid_eo_extended_item() {
    let data = get_example("extensions/eo/item.json");
    let mut value: Value = serde_json::from_str(data.as_str()).unwrap();
    value["properties"]["eo:cloud_cover"] = "a lot".into();
    let item: Item = serde_json::from_value(value).unwrap();

    println!("{}", item.properties.extra_fields);

    assert!(!is_valid(&item).unwrap());
}
