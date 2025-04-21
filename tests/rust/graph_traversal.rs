use hexchess::{Color, Hexchess};
use hexchess::hexchess::utils::{index, walk};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct GraphEntry {
    from: String,
    direction: u8,
    result: Vec<String>,
}

#[test]
fn test_graph_traversal() {
    let path = env::current_dir().unwrap().join("./tests/graph-traversal.json");
    let json = fs::read_to_string(path).expect("Failed to read graph.json file");
    let entries: Vec<GraphEntry> = serde_json::from_str(&json)
        .expect("Failed to parse graph.json into Graph structs");

    let hexchess = Hexchess::new();

    for entry in entries {
        let i = index(entry.from.as_str()).unwrap();
        let direction = entry.direction;
        let result = entry.result
            .iter()
            .map(|s| index(s.as_str()).unwrap())
            .collect::<Vec<u8>>();

        assert_eq!(
            walk(&hexchess, i, direction, &Color::White),
            result
        );
    }
}
