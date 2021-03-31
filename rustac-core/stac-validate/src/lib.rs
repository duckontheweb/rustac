#![warn(missing_docs)]
#![deny(clippy::all, clippy::pedantic)]
//! Tools for validating STAC objects

extern crate semver;
extern crate serde_json;
extern crate reqwest;
extern crate jsonschema;

pub use util::{get_schema_url, get_schema};
pub use validation::{is_valid_for_schema_type, is_valid};

mod util;
mod validation;
pub mod error;
