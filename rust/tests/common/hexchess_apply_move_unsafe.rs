use crate::json;
use hexchess::hexchess::game::Game;
use hexchess::hexchess::san::San;
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
#[ignore]
fn test_hexchess_apply_move_unsafe() {
    let tests = json::<Test>("hexchess-apply-move-unsafe.json");

    for test in tests {
        let mut result = match Game::parse(&test.from) {
            Ok(game) => game,
            Err(_) => {
                assert!(test.error, "{}", test.description);
                continue;
            }
        };

        let san = San::from_string(&test.sequence);

        let output = result.apply_move_unsafe(&san);

        if output.is_err() {
            assert!(test.error, "{}", test.description);
            continue;
        }

        if test.to.is_some() {
            assert_eq!(test.to.unwrap(), result.to_string(), "{}", test.description);
        }
    }
}
