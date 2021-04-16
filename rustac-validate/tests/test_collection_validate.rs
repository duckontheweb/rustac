mod helpers;
use helpers::test_example;

#[test]
fn test_collection() {
    test_example("core/collection.json")
}

#[test]
fn test_collection_only() {
    test_example("core/collection-only/collection.json")
}

#[test]
fn test_extensions_collection() {
    test_example("core/extensions-collection/proj-example/proj-example.json")
}
