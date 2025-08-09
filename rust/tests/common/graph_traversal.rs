use crate::json;
use hexchess::hexchess::position::Position;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    from: String,
    results: [Vec<String>; 12],
}

#[test]
fn test_graph_traversal() {
    let tests = json::<Test>("graph-traversal.json");

    for test in tests {
        let position = Position::from_string(&test.from).unwrap();

        for direction in 0u8..12 {
            let actual: Vec<String> = position
                .walk(direction)
                .iter()
                .map(|p| p.to_string())
                .collect();

            let expected = test.results[direction as usize].clone();

            assert_eq!(actual, expected);
        }
    }
}
