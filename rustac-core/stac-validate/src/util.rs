use reqwest::blocking::get;
use semver::{VersionReq, Version};
use serde::{Serialize};
use stac_core::{Item, Collection, Catalog};
use crate::{
    error::STACResult,
    ValidationTarget
};

pub fn is_valid_for_schema_type(target: &ValidationTarget, schema_uri: &str) -> STACResult<bool>
{
    let instance = &target.serialized_object();
    let schema = get(schema_uri)?.json()?;
    Ok(jsonschema::is_valid(&schema, instance))
}

pub fn get_schema_root(stac_version: &Version) -> String
{
    let at_least_v1 = VersionReq::parse(">=1.0.0-beta.1").unwrap();

    if at_least_v1.matches(&stac_version) {
        format!("https://schemas.stacspec.org/v{}", stac_version.to_string())
    } else {
        format!("https://raw.githubusercontent.com/radiantearth/stac-spec/v{}", stac_version.to_string())
    }
}

pub fn get_extension_path(extension_id: &str, stac_type: &STACObject) -> Option<String>
{
    match extension_id {
        
        "eo" => match stac_type {
            STACObject::Item(_) => Some("extensions/eo/json-schema/schema.json".into()),
            _ => None,
        },
        "projection" => match stac_type {
            STACObject::Item(_) => Some("extensions/projection/json-schema/schema.json".into()),
            _ => None,
        },
        "scientific" => match stac_type {
            STACObject::Item(_) | STACObject::Collection(_) => Some("extensions/scientific/json-schema/schema.json".into()),
            _ => None,
        },
        "view" => match stac_type {
            STACObject::Item(_) => Some("extensions/view/json-schema/schema.json".into()),
            _ => None,
        },
        _ => None,
    }
}

/// Enumerates the top-level STAC objects
#[derive(Serialize)]
#[serde(untagged)]
pub enum STACObject<'a> {
    Catalog(&'a Catalog),
    Collection(&'a Collection),
    Item(&'a Item),
}