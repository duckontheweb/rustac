use reqwest::blocking::get;
use semver::{Version, VersionReq};
use serde_json::Value;

use crate::error::{STACError, STACResult};

use super::ValidationTarget;

/// Gets the URL for the schema associated with the given STAC version, STAC object type, and schema
/// type.
///
/// # Arguments
///
/// * `stac_version` - The STAC Spec version for the schema you want to fetch.
/// * `stac_type` - The value of the `"type"` field for the STAC object you want to validate. This is
///    used per the guidelines in [How to Differentiate STAC Files] to determine the type of the STAC
///    object. This must be one of `"Catalog"`, `"Collection"`, or `"Feature"` (for Items).
/// * `schema_type` - This must be either a STAC extension ID (e.g. `"eo"`) or the value `"core"`
///    (to validate against the core spec).
///
/// # Errors
///
/// This function will return [`STACError::Other`] if there is no schema URL associated with the given
/// inputs.
///
/// [How to Differentiate STAC Files]: https://github.com/radiantearth/stac-spec/blob/master/best-practices.md#how-to-differentiate-stac-files
pub fn get_schema_url(stac_version: &Version, stac_type: &str, schema_type: &SchemaType) -> STACResult<String> {
    let root_url = root_url_from_version(stac_version);
    let path = path_from_stac_type(stac_type, schema_type);
    let url = path.map(|path| {
        format!("{}/{}", root_url.as_str(), path.as_str())
    });
    url.ok_or_else(|| STACError::Other(String::from("Could not find schema URL.")))
}

/// Gets the JSON schema for the given STAC object and schema type as a
/// [`Value`].
///
/// # Arguments
///
/// * `instance` - A [`Value`] representing the STAC object to validate
/// * `schema_type` - This must be either a STAC extension ID (e.g. `"eo"`) or the value `"core"`
///    (to validate against the core spec).
///
/// # Errors
///
/// In addition to the errors that may result from calling [`get_schema_url`], this function may
/// return [`STACError::JSONParse`] if there is an error parsing the schema from the JSON string.
pub fn get_schema(target: &ValidationTarget, schema_type: &SchemaType) -> STACResult<Value> {
    let stac_version = target.stac_version();

    let instance = target.serialized_object();
    let stac_type = instance.get("type")
        .ok_or_else(|| STACError::Other(String::from("Could not get type from instance.")))?
        .as_str()
        .ok_or_else(|| STACError::Other(String::from("Could not get type from instance.")))?;
    let schema_url = get_schema_url(stac_version, stac_type, schema_type)?;

    Ok(get(schema_url)?.json()?)
}

/// Gets the root schema URL based on the STAC spec version
fn root_url_from_version(stac_version: &Version) -> String
{
    let at_least_v1 = VersionReq::parse(">=1.0.0-beta.1").unwrap();

    if at_least_v1.matches(&stac_version) {
        format!("https://schemas.stacspec.org/v{}", stac_version.to_string())
    } else {
        format!("https://raw.githubusercontent.com/radiantearth/stac-spec/v{}", stac_version.to_string())
    }
}

/// Gets the relative path to the schema based on the schema type and type of STAC object.
fn path_from_stac_type(stac_type: &str, schema_type: &SchemaType) -> Option<String>
{
    match schema_type {
        SchemaType::Core => match stac_type {
            "Feature" => Some(String::from("item-spec/json-schema/item.json")),
            "Collection" => Some(String::from("collection-spec/json-schema/collection.json")),
            "Catalog" => Some(String::from("catalog-spec/json-schema/catalog.json")),
            _ => None,
        },
        SchemaType::EO => match stac_type {
            "Feature" => Some(String::from("extensions/eo/json-schema/schema.json")),
            _ => None,
        },
        SchemaType::Projection => match stac_type {
            "Feature" => Some(String::from("extensions/projection/json-schema/schema.json")),
            _ => None,
        },
        SchemaType::Scientific => match stac_type {
            "Feature" | "Collection" => Some(String::from("extensions/scientific/json-schema/schema.json")),
            _ => None,
        },
        SchemaType::View => match stac_type {
            "Feature" => Some(String::from("extensions/view/json-schema/schema.json")),
            _ => None,
        },
        _ => None,
    }
}

/// Enumerates the schema types that can be validated, catching all other values in
/// the Invalid variant.
#[allow(missing_docs)]
pub enum SchemaType {
    Core,
    EO,
    Projection,
    View,
    Scientific,
    Invalid
}

impl From<&str> for SchemaType {
    fn from(s: &str) -> Self {
        match s {
            "core" => SchemaType::Core,
            "eo" => SchemaType::EO,
            "projection" => SchemaType::Projection,
            "view" => SchemaType::View,
            "scientific" => SchemaType::Scientific,
            _ => SchemaType::Invalid,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::Item;

    use super::*;

    fn get_test_example(filename: &str) -> String {
        let path = format!("./tests-data/{}", filename);
        fs::read_to_string(path).unwrap()
    }

    #[test]
    fn test_get_item_core_schema_url() {
        let version = Version::parse("1.0.0-rc.1").unwrap();
        let schema_type = SchemaType::Core;
        let url = get_schema_url(&version, "Feature", &schema_type);

        assert!(url.is_ok());
        assert_eq!(url.unwrap(), "https://schemas.stacspec.org/v1.0.0-rc.1/item-spec/json-schema/item.json");
    }

    #[test]
    fn test_get_item_core_schema() {
        let data = get_test_example("simple-item.json");
        let item: Item = serde_json::from_str(data.as_str()).unwrap();
        let target = ValidationTarget::from(&item);
        let schema_type = SchemaType::Core;
        let schema = get_schema(&target, &schema_type);

        assert!(schema.is_ok());
        assert_eq!(schema.unwrap()["title"].as_str(), Some("STAC Item"));
    }
}
