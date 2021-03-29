use reqwest::blocking::get;
use semver::{Version, VersionReq};
use serde_json::Value;
use std::str::FromStr;

use crate::error::{Result, StacValidateError};

/// Possible STAC object types
pub enum STACType {
    /// STAC Catalog
    Catalog,

    /// STAC Collection
    Collection,
    // ItemCollection,

    /// STAC Item
    Item,
}

/// Possible types of schemas
pub enum SchemaType {
    /// Core JSON schemas
    Core,

    /// Electro-Optical extension schemas
    EOExtension,

    /// Projection extension schemas
    ProjectionExtension,

    /// Scientific Extension schemas
    ScientificExtension,

    /// View extension schemas
    ViewExtension,
}

/// Exposes metadata about a STAC object for determining the STAC version and the type of object (e.g. Item).
pub trait STACMeta {
    /// Gets the type of the STAC object (e.g. Item)
    fn get_type(&self) -> STACType;

    /// Gets the STAC version for this object.
    fn get_stac_version(&self) -> Version;
}

/// Gets the URL for the published JSON schema correspnding to the given object and schema type 
/// (e.g. core, EO extension, etc.)
pub fn get_schema_url<T>(obj: &T, schema_type: SchemaType) -> Option<String>
where
    T: STACMeta
{
    let root_url = root_url_from_object(obj);
    let path = path_from_object_and_type(obj, schema_type);
    
    path.and_then(|path| {
        Some(format!("{}/{}", root_url.as_str(), path.as_str()))
    })
}

/// Gets the JSON schema correspnding to the given object and schema type 
/// (e.g. core, EO extension, etc.) as a [`Value`].
pub fn get_schema<T>(obj: &T, schema_type: SchemaType) -> Result<Value>
where 
    T: STACMeta
{
    let schema_url = match get_schema_url(obj, schema_type) {
        Some(url) => url,
        None => { return Err(StacValidateError::from_str("Could not find schema.").unwrap())}
    };

    Ok(get(schema_url)?.json()?)
}

fn root_url_from_object<T>(obj: &T) -> String 
where
    T: STACMeta
{
    let at_least_v1 = VersionReq::parse(">=1.0.0-beta.1").unwrap();
    let version = obj.get_stac_version();

    if at_least_v1.matches(&version) {
        format!("https://schemas.stacspec.org/v{}", version.to_string())
    } else {
        format!("https://raw.githubusercontent.com/radiantearth/stac-spec/v{}", version.to_string())
    }
}

fn path_from_object_and_type<T>(object: &T, schema_type: SchemaType) -> Option<String> 
where
    T: STACMeta
{
    let stac_type = object.get_type();
    match schema_type {
        SchemaType::Core => match stac_type {
            STACType::Item => Some(String::from("item-spec/json-schema/item.json")),
            STACType::Collection => Some(String::from("collection-spec/json-schema/collection.json")),
            STACType::Catalog => Some(String::from("catalog-spec/json-schema/catalog.json")),
        },
        SchemaType::EOExtension => match stac_type {
            STACType::Item => Some(String::from("extensions/eo/json-schema/schema.json")),
            STACType::Collection => None,
            STACType::Catalog => None,
        },
        SchemaType::ProjectionExtension => match stac_type {
            STACType::Item => Some(String::from("extensions/projection/json-schema/schema.json")),
            STACType::Collection => None,
            STACType::Catalog => None,
        },
        SchemaType::ScientificExtension => match stac_type {
            STACType::Item => Some(String::from("extensions/scientific/json-schema/schema.json")),
            STACType::Collection => Some(String::from("extensions/scientific/json-schema/schema.json")),
            STACType::Catalog => None,
        },
        SchemaType::ViewExtension => match stac_type {
            STACType::Item => Some(String::from("extensions/view/json-schema/schema.json")),
            STACType::Collection => None,
            STACType::Catalog => None,
        },
    }
}

#[cfg(test)]
mod tests {
    use semver::Version;
    use super::*;

    #[test]
    fn test_item_schemas_urls() {
        // v1.0.0-rc.1 Item
        struct Object {}
        impl STACMeta for Object {
            fn get_type(&self) -> STACType { STACType::Item }
            fn get_stac_version(&self) -> Version { Version::parse("1.0.0-rc.1").unwrap() }
        }
        let obj = Object {};
        
        // Core
        assert_eq!(
            get_schema_url(&obj, SchemaType::Core).unwrap(), 
            String::from("https://schemas.stacspec.org/v1.0.0-rc.1/item-spec/json-schema/item.json"),
        );

        // EO Extension
        assert_eq!(
            get_schema_url(&obj, SchemaType::EOExtension).unwrap(), 
            String::from("https://schemas.stacspec.org/v1.0.0-rc.1/extensions/eo/json-schema/schema.json"),
        );

        // Projection Extension
        assert_eq!(
            get_schema_url(&obj, SchemaType::ProjectionExtension).unwrap(), 
            String::from("https://schemas.stacspec.org/v1.0.0-rc.1/extensions/projection/json-schema/schema.json"),
        );

        // Scientific Extension
        assert_eq!(
            get_schema_url(&obj, SchemaType::ScientificExtension).unwrap(), 
            String::from("https://schemas.stacspec.org/v1.0.0-rc.1/extensions/scientific/json-schema/schema.json"),
        );

        // View Extension
        assert_eq!(
            get_schema_url(&obj, SchemaType::ViewExtension).unwrap(), 
            String::from("https://schemas.stacspec.org/v1.0.0-rc.1/extensions/view/json-schema/schema.json"),
        );
    }

    #[test]
    fn test_collection_schema_urls() {
        // v1.0.0-rc.1 Item
        struct Object {}
        impl STACMeta for Object {
            fn get_type(&self) -> STACType { STACType::Collection }
            fn get_stac_version(&self) -> Version { Version::parse("1.0.0-rc.1").unwrap() }
        }
        let obj = Object {};
        
        // Core
        assert_eq!(
            get_schema_url(&obj, SchemaType::Core).unwrap(), 
            String::from("https://schemas.stacspec.org/v1.0.0-rc.1/collection-spec/json-schema/collection.json"),
        );

        // EO Extension
        assert!(get_schema_url(&obj, SchemaType::EOExtension).is_none());

        // Projection Extension
        assert!(get_schema_url(&obj, SchemaType::ProjectionExtension).is_none());

        // Scientific Extension
        assert_eq!(
            get_schema_url(&obj, SchemaType::ScientificExtension).unwrap(), 
            String::from("https://schemas.stacspec.org/v1.0.0-rc.1/extensions/scientific/json-schema/schema.json"),
        );

        // View Extension
        assert!(get_schema_url(&obj, SchemaType::ViewExtension).is_none());
    }

    #[test]
    fn test_catalog_schema_urls() {
        // v1.0.0-rc.1 Item
        struct Object {}
        impl STACMeta for Object {
            fn get_type(&self) -> STACType { STACType::Catalog }
            fn get_stac_version(&self) -> Version { Version::parse("1.0.0-rc.1").unwrap() }
        }
        let obj = Object {};
        
        // Core
        assert_eq!(
            get_schema_url(&obj, SchemaType::Core).unwrap(), 
            String::from("https://schemas.stacspec.org/v1.0.0-rc.1/catalog-spec/json-schema/catalog.json"),
        );

        // EO Extension
        assert!(get_schema_url(&obj, SchemaType::EOExtension).is_none());

        // Projection Extension
        assert!(get_schema_url(&obj, SchemaType::ProjectionExtension).is_none());

        // Scientific Extension
        assert!(get_schema_url(&obj, SchemaType::ScientificExtension).is_none());

        // View Extension
        assert!(get_schema_url(&obj, SchemaType::ViewExtension).is_none());
    }

    #[test]
    fn test_get_schema() {
        struct Object {}
        impl STACMeta for Object {
            fn get_type(&self) -> STACType { STACType::Catalog }
            fn get_stac_version(&self) -> Version { Version::parse("1.0.0-rc.1").unwrap() }
        }
        let obj = Object {};

        let schema = get_schema(&obj, SchemaType::Core).unwrap();

        assert_eq!(schema["title"], "STAC Catalog Specification");
    }
}
