use crate::yaml;
use hexchess::Hexchess;
use hexchess::San;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Test {
    description: String,
    expected: bool,
    hexchess: String,
    san: String,
}

#[test]
fn test_move_legality() {
    let tests = yaml::<Test>("move-legality.yaml");

    for test in tests {
        let game = Hexchess::parse(&test.hexchess).unwrap();
        let san = San::from_string(&test.san);

        if san.is_err() {
            assert_eq!(test.expected, false, "{}", test.description);
            continue;
        }

        let actual = game.is_legal(&san.unwrap());

        assert_eq!(actual, test.expected, "{}", test.description);
    }
}
