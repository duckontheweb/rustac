use serde::{Deserialize, Serialize};
use serde_json;

/// Implementation of a STAC Link.
#[derive(Serialize, Deserialize)]
pub struct Link {
    pub href: String,
    pub rel: String,
    #[serde(rename = "type")]
    pub media_type: Option<String>,
    pub title: Option<String>,
    #[serde(flatten)]
    pub extra_fields: serde_json::Value,
}

impl Link {
    /// Serializes the instance to a JSON string
    pub fn to_json(&self) -> serde_json::Result<String> {
        Ok(serde_json::to_string(&self)?)
    }

    /// Creates a new instance from a JSON string
    pub fn from_json(data: &str) -> serde_json::Result<Self> {
        Ok(serde_json::from_str(&data)?)
    }
}

pub trait UsesLinks {
    /// Gets the complete collection of links for this instance.
    fn get_links(&self) -> Vec<&Link>;

    /// Gets all links with the given "rel" type
    fn links_of_rel(&self, rel: &str) -> Vec<&Link> {
        self.get_links()
            .into_iter()
            .filter(|link| link.rel == rel)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct UsesLinksTester {
        links: Vec<Link>,
    }
    impl UsesLinks for UsesLinksTester {
        fn get_links(&self) -> Vec<&Link> {
            self.links.iter().collect()
        }
    }

    fn get_links_tester() -> impl UsesLinks {
        let link1 = Link {
            href: String::from("https://www.some-domain.com/"),
            rel: String::from("parent"),
            media_type: None,
            title: None,
            extra_fields: serde_json::Value::Null,
        };
        let link2 = Link {
            href: String::from("https://www.some-other-domain.com/path"),
            rel: String::from("derived_from"),
            media_type: None,
            title: None,
            extra_fields: serde_json::Value::Null,
        };

        UsesLinksTester {
            links: vec![link1, link2],
        }
    }

    #[test]
    fn test_from_json_string() {
        let data = r#"
            {
                "href": "https://some-domain.com/path",
                "rel": "child",
                "type": "application/json",
                "title": "A test link"
            }
            "#;
        let link = Link::from_json(data).unwrap();

        assert_eq!(link.href, "https://some-domain.com/path");
        assert_eq!(link.rel, "child");
        assert_eq!(link.media_type, Some(String::from("application/json")));
        assert_eq!(link.title, Some(String::from("A test link")));
    }

    #[test]
    fn test_from_json_string_with_nulls() {
        let data = r#"
            {
                "href": "https://some-domain.com/path",
                "rel": "child"
            }
            "#;
        let link = Link::from_json(data).unwrap();

        assert_eq!(link.href, "https://some-domain.com/path");
        assert_eq!(link.rel, "child");
        assert_eq!(link.media_type, None);
        assert_eq!(link.title, None);
    }

    #[test]
    fn test_to_json_string() {
        let link = Link {
            href: String::from("https://some-domain.com/path"),
            rel: String::from("child"),
            media_type: Some(String::from("application/json")),
            title: Some(String::from("A test link")),
            extra_fields: serde_json::Value::Null,
        };

        let json_string = "{\"href\":\"https://some-domain.com/path\",\
        \"rel\":\"child\",\
        \"type\":\"application/json\",\
        \"title\":\"A test link\"}";
        assert_eq!(link.to_json().unwrap(), json_string);
    }

    #[test]
    fn test_extra_fields() {
        let json_string = "{\"href\":\"https://some-domain.com/path\",\
        \"rel\":\"child\",\
        \"type\":\"application/json\",\
        \"title\":\"A test link\",\
        \"another-field\":\"here\"}";
        let link = Link::from_json(json_string).unwrap();

        assert_eq!(link.extra_fields["another-field"], "here");
        assert_eq!(link.to_json().unwrap(), json_string);
    }

    #[test]
    fn test_get_links() {
        let uses_links = get_links_tester();
        let links = uses_links.get_links();

        assert_eq!(links.len(), 2);
    }

    #[test]
    fn test_links_of_rel() {
        let uses_links = get_links_tester();
        let parent_links = uses_links.links_of_rel("parent");

        assert_eq!(parent_links.len(), 1);
    }
}
