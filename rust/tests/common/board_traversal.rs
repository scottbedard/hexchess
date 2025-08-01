use hexchess::hexchess::color::Color;
use hexchess::hexchess::hexchess::Hexchess;
use hexchess::hexchess::utils::{fen_index, walk};
use serde::{Deserialize, Serialize};
use crate::json;
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
fn test_board_traversal() {
    let tests = json::<Test>("board-traversal.json");

    for test in tests {
        let hexchess = Hexchess::parse(&test.hexchess).unwrap();

        let from = fen_index(test.from.as_str()).unwrap();

        let result: SmallVec<[u8; 11]> = test.result
            .iter()
            .map(|s| fen_index(s.as_str()).unwrap())
            .collect();

        assert_eq!(
            walk(&hexchess, from, test.direction, &test.color),
            result,
            "{}", test.description
        );        
    }
}
