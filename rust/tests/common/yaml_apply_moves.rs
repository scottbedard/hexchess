use crate::yaml;
use hexchess::Hexchess;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Test {
    description: String,
    error: bool,
    expected: Option<String>,
    hexchess: String,
    moves: String,
}

#[test]
fn test_apply_moves() {
    let tests = yaml::<Test>("apply-moves.yaml");

    for test in tests {
        let mut game = match Hexchess::parse(&test.hexchess) {
            Ok(g) => g,
            Err(_) => {
                assert!(test.error, "{}", test.description);
                continue;
            }
        };

        let result = game.apply_sequence(&test.moves);

        if test.error {
            assert!(result.is_err(), "{}", test.description);
            continue;
        }

        let expected = Hexchess::parse(&test.expected.unwrap()).unwrap();

        assert_eq!(expected.to_string(), game.to_string(), "{}", test.description);
    }
}
