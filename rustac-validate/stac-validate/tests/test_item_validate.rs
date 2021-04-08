mod helpers;
use helpers::test_example;

#[test] 
fn test_core_item() { test_example("core/core-item.json") }

#[test] 
fn test_simple_item() { test_example("core/simple-item.json") }

#[test]
fn test_extended_item() { test_example("core/extended-item.json") }

#[test]
fn test_collectionless_item() { test_example("core/collectionless-item.json") }
