use crate::yaml;
use hexchess::hexchess::position::Position;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Test {
    position: String,
    expected: Vec<Vec<String>>,
}

#[test]
fn test_board_traversal() {
    let tests = yaml::<Test>("board-traversal.yaml");

    for test in tests {
        let position = Position::from_string(&test.position).unwrap();

        for direction in 0u8..12 {
            let actual: Vec<String> = position
                .walk(direction)
                .iter()
                .map(|p| p.to_string())
                .collect();

            let expected = test.expected[direction as usize].clone();

            assert_eq!(actual, expected);
        }
    }
}
