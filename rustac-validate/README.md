# `rustac-validate`

Tools for validation [SpatioTemporal Asset Catalog (STAC)] objects in Rust using [`jsonschema`].

> **NOTE:** This project was mostly a way for me to learn Rust in the context of a real project. I
> hope it ends up being useful to others, and I welcome any feedback as a Rust newbie! I have not
> published this to [crates.io], but if that would be useful to you, please open an issue and let me
> know.


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