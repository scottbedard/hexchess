use hexchess::{Color, Hexchess};
use hexchess::hexchess::utils::{index, walk};
use serde::{Deserialize, Serialize};
use crate::json;

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

        let from = index(test.from.as_str()).unwrap();

        let result = test.result
            .iter()
            .map(|s| index(s.as_str()).unwrap())
            .collect::<Vec<u8>>();

        assert_eq!(
            walk(&hexchess, from, test.direction, &test.color),
            result,
            "{}", test.description
        );        
    }
}
