use hexchess::hexchess::color::Color;
use hexchess::hexchess::hexchess::Hexchess;
use hexchess::hexchess::utils::walk;
use hexchess::hexchess::position::Position;
use serde::{Deserialize, Serialize};
use crate::json;
use smallvec::SmallVec;

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    from: String,
    results: [Vec<String>; 12],
}

#[test]
#[ignore]
fn test_graph_traversal() {
    let tests = json::<Test>("graph-traversal.json");

    let hexchess = Hexchess::new();

    for test in tests {
        let from = Position::from_string(&test.from);

        for direction in 0u8..12u8 {
            let result: SmallVec<[Position; 11]> = test.results[direction as usize]
                .iter()
                .map(|s| Position::from_string(s))
                .collect();

            assert_eq!(
                walk(&hexchess, from, direction, &Color::White),
                result
            );
        }
    }
}
