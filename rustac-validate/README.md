# `rustac-validate`

Tools for validation [SpatioTemporal Asset Catalog (STAC)] objects in Rust using [`jsonschema`].

## Tests

The integration tests validate examples from the core [STAC Spec] and the [Scientific Extension].
The latest releases of each of these repos are included as sub-modules within the `tests` 
directory. Be sure to run the following before attempting to run tests:

```
git submodule init
git submodule update
```

[STAC Spec]: https://github.com/radiantearth/stac-spec
[Scientific Extension]: https://github.com/stac-extensions/scientific
[SpatioTemporal Asset Catalog (STAC)]: https://stacspec.org/
[`jsonschema`]: https://docs.rs/jsonschema
[`crates.io`]: https://crates.io/