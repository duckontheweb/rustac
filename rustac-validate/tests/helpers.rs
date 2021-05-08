use std::fs;

#[allow(dead_code)]
pub(crate) fn get_example(repo: &str, filename: &str) -> String {
    let path = format!("./tests/{}/examples/{}", repo, filename);
    fs::read_to_string(&path).unwrap_or_else(|_| panic!("Could not open {}", &path.as_str()))
}
