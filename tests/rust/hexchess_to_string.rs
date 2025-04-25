use crate::json;
use hexchess::Hexchess;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    description: String,
    from: Hexchess,
    result: String,
}

#[test]
fn test_hexchess_to_string() {
    let tests = json::<Test>("hexchess-to-string.json");

    for test in tests {
        let mut hexchess = Hexchess::new();
        hexchess.board = test.from.board;
        hexchess.ep = test.from.ep;
        hexchess.fullmove = test.from.fullmove;
        hexchess.halfmove = test.from.halfmove;
        hexchess.turn = test.from.turn;

        assert_eq!(hexchess.to_string(), test.result, "{}", test.description);
    }
}
