use crate::json;
use hexchess::hexchess::game::Game;
use hexchess::hexchess::position::Position;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    description: String,
    from: String,
    position: String,
    result: Option<String>,
}

#[test]
fn test_get() {
    let tests = json::<Test>("get.json");

    for test in tests {
        let game = Game::parse(&test.from).unwrap();

        let position = match Position::from_string(&test.position) {
            Ok(p) => p,
            Err(_) => {
                assert!(test.result.is_none(), "{}", test.description);
                continue;
            },
        };

        let result = game.get_position(position);

        if test.result.is_some() {
            assert_eq!(result.unwrap().to_string(), test.result.unwrap(), "{}", test.description);
        } else {
            assert!(test.result.is_none(), "{}", test.description);
        }
    }
}
