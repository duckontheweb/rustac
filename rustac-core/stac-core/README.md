# `stac-core`

The `stac-core` crate provides primitive types for working with SpatioTemporal Asset Catalog (STAC)
objects, including:

* [Items](https://github.com/radiantearth/stac-spec/blob/master/item-spec/item-spec.md)
* [Collections](https://github.com/radiantearth/stac-spec/blob/master/collection-spec/collection-spec.md)
* [Catalogs](https://github.com/radiantearth/stac-spec/blob/master/catalog-spec/catalog-spec.md)

All objects can be serialized from JSON using the [`serde_json`] crate.

## Features

* `validate` -  Enables validation of STAC objects using [`jsonschema`]. This feature is enabled by default.

## Caveat

> This is my first Rust project. Feedback and input is more than welcome in the form of Issues and/or PRs!

[`serde_json`]: https://docs.serde.rs/serde_json/ 
[`jsonschema`]: https://docs.rs/jsonschema