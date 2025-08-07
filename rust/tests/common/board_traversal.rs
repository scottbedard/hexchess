use crate::json;
use hexchess::hexchess::color::Color;
use hexchess::hexchess::hexchess::Hexchess;
use hexchess::hexchess::position::Position;
use hexchess::hexchess::utils::walk;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    color: Color,
    description: String,
    direction: u8,
    from: String,
    hexchess: String,
    result: Vec<String>,
}


#[test]
#[ignore]
fn test_board_traversal() {
    // board traversal tests no longer apply when using bitmasks.
    // a new test suite will be needed for traversal assertions.
    let tests = json::<Test>("board-traversal.json");

    for test in tests {
        let hexchess = Hexchess::parse(&test.hexchess).unwrap();

        let from = Position::from_string(&test.from);

        let result: SmallVec<[Position; 11]> = test.result
            .iter()
            .map(|s| Position::from_string(s))
            .collect();

        assert_eq!(
            walk(&hexchess, from, test.direction, &test.color),
            result,
            "{}", test.description
        );        
    }
}
