# `rustac-validate`

Tools for validation [SpatioTemporal Asset Catalog (STAC)] objects in Rust using [`jsonschema`].

> **NOTE:** This project was mostly a way for me to learn Rust in the context of a real project. I
> hope it ends up being useful to others, and I welcome any feedback as a Rust newbie! I have not
> published this to [crates.io], but if that would be useful to you, please open an issue and let me
> know.

## Known Issues

### Passes Invalid Extended Object

The STAC extension JSON schemas use look-behind assertions in their `"patternProperties"` and the `jsonschema` crate parses regular 
expressions using [`regex`], which [does not support look-around assertions](https://github.com/rust-lang/regex/issues/127). There 
*may* be [a bug](https://github.com/Stranger6667/jsonschema-rs/issues/183) in `jsonschema` that allows the the schema to compile 
but then ignores the `"properties"` validations, which means that objects that should be invalid according to an extension schema are 
passed as valid.


[SpatioTemporal Asset Catalog (STAC)]: https://stacspec.org/
[`jsonschema`]: https://docs.rs/jsonschema
[`regex`]: https://docs.rs/regex/1.4.5/regex/
[`crates.io`]: https://crates.io/