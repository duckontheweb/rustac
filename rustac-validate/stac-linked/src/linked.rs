use stac_types::Link;

pub trait Linked {
    fn get_links(&self) -> Vec<&Link>;

    fn links_of_rel(&self, rel_type: &str) -> Vec<&Link> {
        self.get_links()
            .into_iter()
            .filter(|link| link.rel == rel_type)
            .collect()
    }

    fn first_link_of_rel(&self, rel_type: &str) -> Option<&Link> {
        self.links_of_rel(rel_type).into_iter().next()
    }
}