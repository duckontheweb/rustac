#![warn(missing_docs)]
#![deny(clippy::all, clippy::pedantic)]
//! Tools for validating STAC objects

extern crate semver;
extern crate serde_json;
extern crate reqwest;
extern crate jsonschema;

// use semver::Version;
// use serde::{Serialize};
// use serde_json::Value;
// use reqwest::blocking::get;
// use std::str::FromStr;
// use jsonschema::{JSONSchema, Draft};

pub use util::{get_schema_url, get_schema};
pub use validation::{is_valid_for_schema_type};

mod util;
mod validation;
pub mod error;

// /// Adds functionality for getting JSON schemas and validating against them.
// pub trait Validate: Serialize {
//     /// Gets the type of the STAC object (e.g. Item)
//     fn get_type(&self) -> &String;
//
//     /// Gets the STAC version for this object.
//     fn get_stac_version(&self) -> &Version;
//
//     /// Gets the schema for this object
//     fn get_schema(&self, schema_type: &str) -> Result<Value> {
//         let schema_url = match self.get_schema_url(schema_type) {
//             Some(url) => url,
//             None => { return Err(Error::from_str("Could not find schema.").unwrap())}
//         };
//
//         Ok(get(schema_url)?.json()?)
//     }
//
//     /// Gets the URL for the published JSON schema corresponding to the given object and schema type
//     /// (e.g. core, EO extension, etc.)
//     fn get_schema_url(&self, schema_type: &str) -> Option<String>
//     {
//         let root_url = root_url_from_version(self.get_stac_version());
//         let path = path_from_stac_type(&self.get_type(), schema_type);
//         path.map(|path| format!("{}/{}", root_url.as_str(), path.as_str()))
//     }
//
//     /// Checks if the instance if valid when checked against the given schema.
//     fn is_valid_for_schema(&self, schema_type: &str) -> Result<bool>
//     {
//         let schema = self.get_schema(schema_type)?;
//         Ok(JSONSchema::options()
//             .with_draft(Draft::Draft7)
//             .compile(&schema)?
//             .is_valid(&serde_json::to_value(self)?))
//
//     }
// }


// #[cfg(test)]
// mod tests {
//     use semver::Version;
//     use super::*;
//
//     #[derive(Serialize)]
//     struct STACObject {
//         stac_version: Version,
//         #[serde(rename = "type")]
//         type_: String
//     }
//     impl STACObject {
//         pub fn new(stac_version: &str, type_: &str) -> STACObject {
//             let stac_version = Version::parse(stac_version).unwrap();
//             let type_ = String::from(type_);
//             STACObject {
//                 stac_version,
//                 type_
//             }
//         }
//     }
//     impl Validate for STACObject {
//         fn get_type(&self) -> &String {
//             &self.type_
//         }
//         fn get_stac_version(&self) -> &Version { &self.stac_version }
//     }
//
//     #[test]
//     fn test_item_schemas_urls() {
//         // v1.0.0-rc.1 Item
//         let obj = STACObject::new("1.0.0-rc.1", "Feature");
//
//         // Core
//         assert_eq!(
//             obj.get_schema_url("core").unwrap(),
//             String::from("https://schemas.stacspec.org/v1.0.0-rc.1/item-spec/json-schema/item.json"),
//         );
//
//         // EO Extension
//         assert_eq!(
//             obj.get_schema_url("eo").unwrap(),
//             String::from("https://schemas.stacspec.org/v1.0.0-rc.1/extensions/eo/json-schema/schema.json"),
//         );
//
//         // Projection Extension
//         assert_eq!(
//             obj.get_schema_url("projection").unwrap(),
//             String::from("https://schemas.stacspec.org/v1.0.0-rc.1/extensions/projection/json-schema/schema.json"),
//         );
//
//         // Scientific Extension
//         assert_eq!(
//             obj.get_schema_url("scientific").unwrap(),
//             String::from("https://schemas.stacspec.org/v1.0.0-rc.1/extensions/scientific/json-schema/schema.json"),
//         );
//
//         // View Extension
//         assert_eq!(
//             obj.get_schema_url("view").unwrap(),
//             String::from("https://schemas.stacspec.org/v1.0.0-rc.1/extensions/view/json-schema/schema.json"),
//         );
//     }
//
//     #[test]
//     fn test_collection_schema_urls() {
//         // v1.0.0-rc.1 Collection
//         let obj = STACObject::new("1.0.0-rc.1", "Collection");
//
//         // Core
//         assert_eq!(
//             obj.get_schema_url("core").unwrap(),
//             String::from("https://schemas.stacspec.org/v1.0.0-rc.1/collection-spec/json-schema/collection.json"),
//         );
//
//         // EO Extension
//         assert!(obj.get_schema_url("eo").is_none());
//
//         // Projection Extension
//         assert!(obj.get_schema_url("projection").is_none());
//
//         // Scientific Extension
//         assert_eq!(
//             obj.get_schema_url("scientific").unwrap(),
//             String::from("https://schemas.stacspec.org/v1.0.0-rc.1/extensions/scientific/json-schema/schema.json"),
//         );
//
//         // View Extension
//         assert!(obj.get_schema_url("view").is_none());
//     }
//
//     #[test]
//     fn test_catalog_schema_urls() {
//         // v1.0.0-rc.1 Catalog
//         let obj = STACObject::new("1.0.0-rc.1", "Catalog");
//
//         // Core
//         assert_eq!(
//             obj.get_schema_url("core").unwrap(),
//             String::from("https://schemas.stacspec.org/v1.0.0-rc.1/catalog-spec/json-schema/catalog.json"),
//         );
//
//         // EO Extension
//         assert!(obj.get_schema_url("eo").is_none());
//
//         // Projection Extension
//         assert!(obj.get_schema_url("projection").is_none());
//
//         // Scientific Extension
//         assert!(obj.get_schema_url("scientific").is_none());
//
//         // View Extension
//         assert!(obj.get_schema_url("view").is_none());
//     }
//
//     #[test]
//     fn test_get_schema() {
//         let obj = STACObject::new("1.0.0-rc.1", "Catalog");
//         let schema = obj.get_schema("core").unwrap();
//
//         assert_eq!(schema["title"], "STAC Catalog Specification");
//     }
// }
