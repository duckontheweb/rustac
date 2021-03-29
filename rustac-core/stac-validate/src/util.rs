use semver::{VersionReq, Version};

use crate::SchemaType;

pub fn root_url_from_version(version: &Version) -> String
{
    let at_least_v1 = VersionReq::parse(">=1.0.0-beta.1").unwrap();

    if at_least_v1.matches(&version) {
        format!("https://schemas.stacspec.org/v{}", version.to_string())
    } else {
        format!("https://raw.githubusercontent.com/radiantearth/stac-spec/v{}", version.to_string())
    }
}

pub fn path_from_stac_type(stac_type: &str, schema_type: &SchemaType) -> Option<String>
{
    match schema_type {
        SchemaType::Core => match stac_type {
            "Feature" => Some(String::from("item-spec/json-schema/item.json")),
            "Collection" => Some(String::from("collection-spec/json-schema/collection.json")),
            "Catalog" => Some(String::from("catalog-spec/json-schema/catalog.json")),
            _ => None,
        },
        SchemaType::EOExtension => match stac_type {
            "Feature" => Some(String::from("extensions/eo/json-schema/schema.json")),
            _ => None,
        },
        SchemaType::ProjectionExtension => match stac_type {
            "Feature" => Some(String::from("extensions/projection/json-schema/schema.json")),
            _ => None,
        },
        SchemaType::ScientificExtension => match stac_type {
            "Feature" | "Collection" => Some(String::from("extensions/scientific/json-schema/schema.json")),
            _ => None,
        },
        SchemaType::ViewExtension => match stac_type {
            "Feature" => Some(String::from("extensions/view/json-schema/schema.json")),
            _ => None,
        },
    }
}
