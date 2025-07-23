mod common;

use serde::Deserialize;
use std::env;
use std::fs;

/// read and parse json test fixture
pub fn json<T: for<'de> Deserialize<'de>>(file: &str) -> Vec<T> {
    let mut path = env::current_dir().expect("Failed to get current directory");
    path.pop();
    path.push(format!("tests/{}", file));

    let json = fs::read_to_string(path)
        .expect(format!("Failed to read json file: {}", file).as_str());

    serde_json::from_str(&json)
        .expect(format!("Failed to parse json file: {}", file).as_str())
}
