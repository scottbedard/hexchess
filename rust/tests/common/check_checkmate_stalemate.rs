use crate::json;
use hexchess::Hexchess;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    check: bool,
    checkmate: bool,
    description: String,
    from: String,
    stalemate: bool,
}

#[test]
#[ignore]
fn test_check_checkmate_stalemate() {
    let tests = json::<Test>("check-checkmate-stalemate.json");

    for test in tests {
        let hexchess = Hexchess::parse(&test.from).unwrap();

        assert_eq!(hexchess.is_check(), test.check, "check assertion failed: {}", test.description);
        assert_eq!(hexchess.is_checkmate(), test.checkmate, "checkmate assertion failed: {}", test.description);
        assert_eq!(hexchess.is_stalemate(), test.stalemate, "stalemate assertion failed: {}", test.description);
    }
}
