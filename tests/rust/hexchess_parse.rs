use crate::json;
use hexchess::Hexchess;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    description: String,
    error: bool,
    fen: String,
    result: Option<Hexchess>,
}

#[test]
fn test_hexchess_parse() {
    let tests = json::<Test>("hexchess-parse.json");

    for test in tests {
        let result = Hexchess::parse(&test.fen);

        if test.error {
            assert!(result.is_err(), "{}", test.description);
        } else {
            assert_eq!(result.unwrap(), test.result.unwrap(), "{}", test.description);
        }
    }
}
