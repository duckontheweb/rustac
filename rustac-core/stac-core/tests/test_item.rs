mod helpers;

use helpers::get_test_example;
use stac_core::Item;

#[test]
fn test_core_item() {
    let data = get_test_example("core/core-item.json");
    let item: Item = serde_json::from_str(data.as_str()).unwrap();
    
    assert_eq!(
        item.properties.common.description, 
        Some(String::from("A sample STAC Item that includes examples of all common metadata"))
    );
    assert_eq!(item.collection, Some(String::from("simple-collection")));
}

#[test]
fn test_simple_item() {
    let data = get_test_example("core/simple-item.json");
    let item: Item = serde_json::from_str(data.as_str()).unwrap();
    
    assert_eq!(
        item.properties.common.datetime, 
        Some(String::from("2020-12-11T22:38:32.125000Z"))
    );
    assert!(item.properties.common.description.is_none());
    assert_eq!(item.collection, Some(String::from("simple-collection")));
}

#[test]
#[ignore]
fn test_collectionless_item() {
    let data = get_test_example("core/collectionless-item.json");
    let item: Item = serde_json::from_str(data.as_str()).unwrap();
    
    assert!(item.collection.is_none());
}

#[test]
fn test_extended_item() {
    let data = get_test_example("core/extended-item.json");
    let item: Item = serde_json::from_str(data.as_str()).unwrap();
    
    println!("{:#?}", &item.properties.extra_fields);

    let epsg = &item.properties.extra_fields["proj:epsg"];
    assert!(epsg.is_number());

    let epsg = epsg.as_u64();
    let expected: u64 = 32659;
    assert_eq!(epsg, Some(expected));}
