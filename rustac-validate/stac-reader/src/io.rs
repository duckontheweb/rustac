use reqwest;
use stac_types::Link;

use crate::error::Result;
use crate::types::LinkTarget;

pub(crate) async fn get_link_target(link: &Link) -> Result<reqwest::Response> {
    Ok(reqwest::get(link.href.as_str()).await?)
}

pub(crate) async fn resolve_link(link: &Link) -> Result<LinkTarget>
{
    let resolved = get_link_target(link).await?.json().await?;
    Ok(resolved)
}

#[cfg(test)]
mod tests {
    use tokio;
    use stac_types::Link;
    use serde_json::json;

    use super::{get_link_target, resolve_link, LinkTarget};

    #[tokio::test]
    async fn test_get_target() {
        let link = Link {
            href: String::from("https://earth-search.aws.element84.com/v0/collections/sentinel-s2-l2a/items/S2A_19HEU_20210323_0_L2A"),
            rel: String::from("item"),
            title: None,
            type_: None,
            extra_fields: json!({}),
        };

        let response = get_link_target(&link).await;

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_resolve_target() {
        let link = Link {
            href: String::from("https://earth-search.aws.element84.com/v0/collections/sentinel-s2-l2a/items/S2A_19HEU_20210323_0_L2A"),
            rel: String::from("item"),
            title: None,
            type_: None,
            extra_fields: json!({}),
        };

        let item = if let LinkTarget::Item(item) = resolve_link(&link).await.unwrap() {
            item
        } else {
            panic!("Not an item")
        };

        assert_eq!(item.id, String::from("S2A_19HEU_20210323_0_L2A"));
    }
}