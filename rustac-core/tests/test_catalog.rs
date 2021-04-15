mod helpers;

use helpers::get_test_example;
use rustac_core::Catalog;

#[test]
fn test_core_catalog() {
    let data = get_test_example("core/catalog.json");
    let catalog: Catalog = serde_json::from_str(data.as_str()).unwrap();

    assert_eq!(catalog.id, String::from("examples"));
}
