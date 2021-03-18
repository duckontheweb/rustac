use std::fs;
use std::error::Error;
use crate::{Link, Item};
use super::link::UsesLinks;

fn get_test_example(filename: &str) -> Result<String, Box<dyn Error>> {
    let path = format!("./tests-data/{}", filename);
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}

#[test]
fn test_full_link_example()  {
    let data = get_test_example("full-link.json").unwrap();
    let link = Link::from_json(&data[..]).unwrap();

    assert_eq!(link.href, "./collection.json");
    assert_eq!(link.rel, "collection");
    assert_eq!(link.media_type, Some(String::from("application/json")));
    assert_eq!(link.title, Some(String::from("Simple Example Collection")));
}

#[test]
fn test_link_to_json() {
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
fn test_link_extra_fields() {
    let json_string = "{\"href\":\"https://some-domain.com/path\",\
    \"rel\":\"child\",\
    \"type\":\"application/json\",\
    \"title\":\"A test link\",\
    \"another-field\":\"here\"}";
    let link = Link::from_json(json_string).unwrap();

    assert_eq!(link.extra_fields["another-field"], "here");
    assert_eq!(link.to_json().unwrap(), json_string);
}

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

#[test]
fn test_simple_item() {
    let data = get_test_example("simple-item.json").unwrap();
    let item = Item::from_json(&data[..]).unwrap();

    assert_eq!(item.id, String::from("20201211_223832_CS2"));
}