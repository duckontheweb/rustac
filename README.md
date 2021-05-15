# rustac
Rust(y) implementations of the SpatioTemporal Asset Catalog (STAC) spec

## Disclaimer

This project was mostly a way for me to learn Rust in the context of a real project. I hope it ends
up being useful to others, and I welcome any feedback as a Rust newbie! 

Because the project is still so rough, I have not published this to [crates.io]. I plan to publish
it if it gets to a more stable and useful state, but if it would be useful to you to have a
published version before that, please open up an issue.

## Crates

The project contains the following crates, organized under a single [Cargo workspace].

* **[rustac-core]** -  Core types for working with STAC objects in Rust
* **[rustac-validate]** -  Tools for validating STAC objects using [JSON Schema]
* **rustac-io** (*PLANNED*) - Tools for reading & writing STAC objects, including
  resolving links
* **rustac-extensions** (*PLANNED*) - Tools for working with [STAC Extensions]
* **rustac** (*PLANNED*) - A consolidated entry point for the crates listed above

## Contributing

Contributions are welcome! Take a look at the [issues], add one of your own, or open a PR. 

The "roadmap" for this project is currently guided by my own interests, skill level, and sense of 
what might be useful. If you have suggestions for useful features or directions for the project, please open up an issue to discuss.

[crates.io]: https://crates.io
[Cargo workspace]: https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html
[JSON Schema]: https://json-schema.org/
[STAC Extensions]: https://stac-extensions.github.io/
[issues]: https://github.com/duckontheweb/rustac/issues

[rustac-core]: ./rustac-core
[rustac-validate]: ./rustac-validate
