use crate::yaml;
use hexchess::Hexchess;
use hexchess::Position;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Test {
    description: String,
    expected: Vec<String>,
    hexchess: String,
    position: String,
}

#[test]
fn test_moves_from_unsafe() {
    let tests = yaml::<Test>("moves-from-unsafe.yaml");

    for test in tests {
        let position = Position::from_string(&test.position).unwrap();

            let result = Hexchess::parse(&test.hexchess)
                .unwrap()
                .get_moves_unsafe(position)
                .into_iter()
                .map(|san| san.to_string())
                .collect::<Vec<String>>();

            assert_eq!(result, test.expected, "{}", test.description);
    }
}
