use std::fs;

#[allow(dead_code)]
pub(crate) fn get_test_example(filename: &str) -> String {
    let path = format!("./stac-examples/{}", filename);
    fs::read_to_string(&path).unwrap_or_else(|_| panic!("Could not open {}", &path.as_str()))
}
