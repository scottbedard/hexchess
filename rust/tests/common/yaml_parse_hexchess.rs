use crate::yaml;
use hexchess::hexchess::game::Game;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Test {
    description: String,
    error: bool,
    expected: Option<Vec<Option<String>>>,
    hexchess: String,
}

#[test]
fn test_parse_hexchess() {
    let tests = yaml::<Test>("parse-hexchess.yaml");

    for test in tests {
        let hexchess = match Game::parse(&test.hexchess) {
            Ok(hexchess) => hexchess,
            Err(e) => {
                assert!(test.error, "{}", e);
                continue;
            }
        };

        let board = hexchess
            .to_board_array()
            .into_iter()
            .map(|x| match x {
                Some(p) => Some(p.to_string()),
                None => None,
            })
            .collect::<Vec<Option<String>>>();

        assert_eq!(board, test.expected.unwrap(), "{}", test.description);
    }
}
