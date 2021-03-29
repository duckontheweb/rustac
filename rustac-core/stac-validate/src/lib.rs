#![warn(missing_docs)]
//! Tools for validating STAC objects

extern crate semver;
extern crate serde_json;
extern crate reqwest;
extern crate jsonschema;

pub use schema::{get_schema_url, get_schema, STACMeta, STACType, SchemaType};
pub use validation::{is_valid, validate_from_uri};

mod schema;
mod validation;
pub mod error;
