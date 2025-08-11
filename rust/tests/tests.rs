mod common;

use serde::Deserialize;
use std::env;
use std::fs;

/// read and parse yaml test fixture
pub fn yaml<T: for<'de> Deserialize<'de>>(file: &str) -> Vec<T> {
    let mut path = env::current_dir().expect("Failed to get current directory");
    path.pop();
    path.push(format!("tests/{}", file));

    let yaml = fs::read_to_string(path)
        .expect(format!("Failed to read yaml file: {}", file).as_str());

    let result: Vec<T> = serde_yaml::from_str(&yaml)
        .expect(format!("Failed to parse yaml file: {}", file).as_str());

    result
}