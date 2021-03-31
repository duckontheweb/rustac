//! Tools for validating STAC objects
pub use util::{get_schema, get_schema_url};
pub use validate::{is_valid, is_valid_for_schema_type};

mod util;
mod validate;