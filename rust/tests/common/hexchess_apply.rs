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
#[ignore]
fn test_hexchess_apply() {
    let tests = json::<Test>("hexchess-apply.json");

    for test in tests {
        let mut result = match Game::parse(&test.from) {
            Ok(game) => game,
            Err(_) => {
                assert!(test.error, "{}", test.description);
                continue;
            }
        };

        // let sequence = result.apply(&test.sequence);

        // assert_eq!(sequence.is_err(), test.error, "{}", test.description);

        // match test.to {
        //     Some(to) => assert_eq!(to, result.to_string(), "{}", test.description),
        //     None => (),
        // }
    }
}
