use reqwest::blocking::get;
use semver::{VersionReq, Version};
use serde_json::Value;

use crate::error::{STACResult, STACError};

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
pub fn get_schema_url(stac_version: &Version, stac_type: &str, schema_type: &str) -> STACResult<String> {
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
pub fn get_schema(instance: &Value, schema_type: &str) -> STACResult<Value> {
    let stac_version = instance.get("stac_version")
        .ok_or_else(|| STACError::Other(String::from("Could not get stac_version from instance.")))?
        .as_str()
        .ok_or_else(|| STACError::Other(String::from("Could not get stac_version from instance.")))?;
    let stac_version= Version::parse(stac_version)?;
    let stac_type = instance.get("type")
        .ok_or_else(|| STACError::Other(String::from("Could not get type from instance.")))?
        .as_str()
        .ok_or_else(|| STACError::Other(String::from("Could not get type from instance.")))?;
    let schema_url = get_schema_url(&stac_version, stac_type, schema_type)?;

    Ok(get(schema_url)?.json()?)
}

fn root_url_from_version(stac_version: &Version) -> String
{
    let at_least_v1 = VersionReq::parse(">=1.0.0-beta.1").unwrap();

    if at_least_v1.matches(&stac_version) {
        format!("https://schemas.stacspec.org/v{}", stac_version.to_string())
    } else {
        format!("https://raw.githubusercontent.com/radiantearth/stac-spec/v{}", stac_version.to_string())
    }
}

fn path_from_stac_type(stac_type: &str, schema_type: &str) -> Option<String>
{
    match schema_type {
        "core" => match stac_type {
            "Feature" => Some(String::from("item-spec/json-schema/item.json")),
            "Collection" => Some(String::from("collection-spec/json-schema/collection.json")),
            "Catalog" => Some(String::from("catalog-spec/json-schema/catalog.json")),
            _ => None,
        },
        "eo" => match stac_type {
            "Feature" => Some(String::from("extensions/eo/json-schema/schema.json")),
            _ => None,
        },
        "projection" => match stac_type {
            "Feature" => Some(String::from("extensions/projection/json-schema/schema.json")),
            _ => None,
        },
        "scientific" => match stac_type {
            "Feature" | "Collection" => Some(String::from("extensions/scientific/json-schema/schema.json")),
            _ => None,
        },
        "view" => match stac_type {
            "Feature" => Some(String::from("extensions/view/json-schema/schema.json")),
            _ => None,
        },
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use super::*;

    #[test]
    fn test_get_item_core_schema_url() {
        let version = Version::parse("1.0.0-rc.1").unwrap();
        let url = get_schema_url(&version, "Feature", "core");

        assert!(url.is_ok());
        assert_eq!(url.unwrap(), "https://schemas.stacspec.org/v1.0.0-rc.1/item-spec/json-schema/item.json");
    }

    #[test]
    fn test_get_item_core_schema() {
        let instance = json!({
            "type": "Feature",
            "stac_version": "1.0.0-rc.1",
        });
        let schema = get_schema(&instance, "core");

        assert!(schema.is_ok());
        assert_eq!(schema.unwrap()["title"].as_str(), Some("STAC Item"));
    }
}
