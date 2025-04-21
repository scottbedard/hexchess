use hexchess::{Color, Hexchess};
use hexchess::hexchess::utils::{index, walk};
use serde::{Deserialize, Serialize};
use crate::json;

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    from: String,
    direction: u8,
    result: Vec<String>,
}

#[test]
fn test_graph_traversal() {
    let tests = json::<Test>("graph-traversal.json");

    let hexchess = Hexchess::new();

    for test in tests {
        let i = index(test.from.as_str()).unwrap();
        
        let direction = test.direction;

        let result = test.result
            .iter()
            .map(|s| index(s.as_str()).unwrap())
            .collect::<Vec<u8>>();

        assert_eq!(
            walk(&hexchess, i, direction, &Color::White),
            result
        );
    }
}
