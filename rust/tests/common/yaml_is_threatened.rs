use crate::yaml;
use hexchess::hexchess::game::Game;
use hexchess::hexchess::position::Position;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Test {
    description: String,
    hexchess: String,
    position: String,
    expected: bool,
}

#[test]
fn test_is_threatened() {
    let tests = yaml::<Test>("is-threatened.yaml");

    for test in tests {
        let hexchess = match Game::parse(&test.hexchess) {
            Ok(hexchess) => hexchess,
            Err(e) => {
                panic!("{}", e);
            }
        };

        let position = Position::from_string(&test.position).unwrap();

        assert_eq!(hexchess.is_threatened(position), test.expected, "{}", test.description);
    }
}
