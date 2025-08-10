use crate::json;
use hexchess::hexchess::game::Game;
use hexchess::hexchess::position::Position;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    description: String,
    from: String,
    position: String,
    expect: bool,
}

#[test]
fn test_hexchess_is_threatened() {
    let tests = json::<Test>("hexchess-is-threatened.json");

    for test in tests {
        let hexchess = match Game::parse(&test.from) {
            Ok(hexchess) => hexchess,
            Err(e) => {
                panic!("{}", e);
            }
        };

        let position = Position::from_string(&test.position).unwrap();

        assert_eq!(hexchess.is_threatened(position), test.expect, "{}", test.description);
    }
}
