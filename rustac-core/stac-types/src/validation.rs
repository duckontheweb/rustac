//! Tools for validating STAC Objects using JSON Schema. *Requires the `"validate"` feature.*
mod validate;

pub use validate::{is_valid, validate_from_uri};
