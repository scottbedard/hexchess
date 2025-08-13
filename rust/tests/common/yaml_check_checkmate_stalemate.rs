use crate::yaml;
use hexchess::hexchess::hexchess::Hexchess;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Test {
    check: bool,
    checkmate: bool,
    description: String,
    hexchess: String,
    stalemate: bool,
}

#[test]
fn test_check_checkmate_stalemate() {
    let tests = yaml::<Test>("check-checkmate-stalemate.yaml");

    for test in tests {
        let hexchess = Hexchess::parse(&test.hexchess).unwrap();

        assert_eq!(hexchess.is_check(), test.check, "check assertion failed: {}", test.description);
        assert_eq!(hexchess.is_checkmate(), test.checkmate, "checkmate assertion failed: {}", test.description);
        assert_eq!(hexchess.is_stalemate(), test.stalemate, "stalemate assertion failed: {}", test.description);
    }
}
