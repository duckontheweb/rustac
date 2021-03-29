/// Possible STAC object types
pub enum STACType {
    /// STAC Catalog
    Catalog,

    /// STAC Collection
    Collection,

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