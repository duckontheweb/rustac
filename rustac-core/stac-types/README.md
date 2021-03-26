The `stac-types` crate provides primitive types for working with SpatioTemporal Asset Catalog (STAC)
objects, including:

* [Items](https://github.com/radiantearth/stac-spec/blob/master/item-spec/item-spec.md)
* [Collections](https://github.com/radiantearth/stac-spec/blob/master/collection-spec/collection-spec.md)
* [Catalogs](https://github.com/radiantearth/stac-spec/blob/master/catalog-spec/catalog-spec.md)

All objects can be serialized from JSON using the [`serde`] crate.

# Features

The following [Cargo features](https://doc.rust-lang.org/cargo/reference/features.html) are available

* `validate`: Enables validation of STAC objects using [`jsonschema`].
