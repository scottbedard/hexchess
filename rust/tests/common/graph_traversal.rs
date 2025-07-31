use hexchess::{Color, Hexchess};
use hexchess::hexchess::utils::{fen_index, walk};
use serde::{Deserialize, Serialize};
use crate::json;
use smallvec::SmallVec;

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
        let from = fen_index(test.from.as_str()).unwrap();

        for direction in 0u8..12u8 {
            let result: SmallVec<[u8; 11]> = test.results[direction as usize]
                .iter()
                .map(|s| fen_index(s.as_str()).unwrap())
                .collect();

            assert_eq!(
                walk(&hexchess, from, direction, &Color::White),
                result
            );
        }
    }
}
