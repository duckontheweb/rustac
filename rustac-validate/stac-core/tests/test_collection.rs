mod helpers;

use helpers::get_test_example;
use stac_core::Collection;

#[test]
fn test_core_collection() {
    let data = get_test_example("core/collection.json");
    let collection: Collection = serde_json::from_str(data.as_str()).unwrap();

    assert_eq!(
        collection.description,
        String::from(
            "A simple collection demonstrating core catalog fields with links to a couple of items"
        )
    );

    let providers = &collection.providers;
    assert!(providers.is_some());
    let providers = providers.as_ref().unwrap();
    assert_eq!(providers.len(), 1);

    let first_provider = &providers[0];
    assert_eq!(first_provider.name, String::from("Remote Data, Inc"))
}

#[test]
fn test_collection_only() {
    let data = get_test_example("core/collection-only/collection.json");
    let collection: Collection = serde_json::from_str(data.as_str()).unwrap();

    assert_eq!(collection.id, String::from("sentinel-2"));
}
