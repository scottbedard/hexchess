use crate::json;
use hexchess::hexchess::game::Game;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    description: String,
    error: bool,
    from: String,
    sequence: String,
    to: Option<String>,
}

#[test]
fn test_hexchess_apply() {
    let tests = json::<Test>("hexchess-apply.json");

    for test in tests {
        let mut game = match Game::parse(&test.from) {
            Ok(g) => g,
            Err(_) => {
                assert!(test.error, "{}", test.description);
                continue;
            }
        };

        let result = game.apply_sequence(&test.sequence);

        if test.error {
            assert!(result.is_err(), "{}", test.description);
            continue;
        }

        let expected = Game::parse(&test.to.unwrap()).unwrap();

        assert_eq!(expected.to_string(), game.to_string(), "{}", test.description);
    }
}
