use hexchess::{Color, Hexchess};
use hexchess::hexchess::utils::{index, walk};
use serde::{Deserialize, Serialize};
use crate::json;

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    from: String,
    results: [Vec<String>; 12],
}

#[test]
fn test_graph_traversal() {
    let tests = json::<Test>("graph-traversal.json");

    let hexchess = Hexchess::new();

    for test in tests {
        let from = index(test.from.as_str()).unwrap();

        for direction in 0u8..12u8 {
            let result = test.results[direction as usize]
                .iter()
                .map(|s| index(s.as_str()).unwrap())
                .collect::<Vec<u8>>();

            assert_eq!(
                walk(&hexchess, from, direction, &Color::White),
                result
            );
        }
    }
}
