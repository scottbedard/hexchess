use crate::json;
use hexchess::hexchess::color::Color;
use hexchess::hexchess::hexchess::Hexchess;
use hexchess::hexchess::position::Position;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    color: Color,
    description: String,
    from: String,
    result: Option<String>,
}

#[test]
fn test_find_king() {
    let tests = json::<Test>("find-king.json");

    for test in tests {
        let hexchess = Hexchess::parse(&test.from).unwrap();

        let king = hexchess.find_king(test.color);

        if let Some(king) = king {
            assert_eq!(king.to_string(), test.result.unwrap(), "{}", test.description);
        } else {
            assert_eq!(king, None, "{}", test.description);
        }
    }
}
