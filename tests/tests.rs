mod rust;

use std::env;
use std::fs;
use serde::Deserialize;

/// read and parse json test fixture
pub fn json<T: for<'de> Deserialize<'de>>(file: &str) -> Vec<T> {
    let path = env::current_dir().unwrap().join(format!("./tests/{}", file));

    let json = fs::read_to_string(path)
        .expect(format!("Failed to read json file: {}", file).as_str());

    serde_json::from_str(&json)
        .expect(format!("Failed to parse json file: {}", file).as_str())
}
