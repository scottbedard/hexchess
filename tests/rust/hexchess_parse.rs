use crate::json;
use hexchess::Hexchess;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    description: String,
    fen: String,
    result: Hexchess,
}

#[test]
fn test_hexchess_parse() {
    let tests = json::<Test>("hexchess-parse.json");

    for test in tests {
        let result = Hexchess::parse(&test.fen).unwrap();

        assert_eq!(result, test.result, "{}", test.description);
    }
}
