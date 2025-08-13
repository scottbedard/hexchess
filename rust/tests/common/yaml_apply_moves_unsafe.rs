use crate::yaml;
use hexchess::hexchess::hexchess::Hexchess;
use hexchess::hexchess::san::San;
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
fn test_apply_moves_unsafe() {
    let tests = yaml::<Test>("apply-moves-unsafe.yaml");

    for test in tests {
        let mut game = match Hexchess::parse(&test.hexchess) {
            Ok(g) => g,
            Err(_) => {
                assert!(test.error, "{}", test.description);
                continue;
            }
        };

        let sans: Vec<San> = test.moves
            .split_whitespace()
            .map(|s| San::from_string(s).unwrap())
            .collect();

        if test.error {
            let mut failed = false;

            for san in &sans {
                if game.apply_move_unsafe(&san).is_err() {
                    failed = true;
                    break;
                }
            }

            assert!(failed, "{}", test.description);
            continue;
        }

        for san in &sans {
            game.apply_move_unsafe(san).unwrap();
        }

        let expected = Hexchess::parse(&test.expected.unwrap()).unwrap();

        assert_eq!(expected.to_string(), game.to_string(), "{}", test.description);
    }
}
