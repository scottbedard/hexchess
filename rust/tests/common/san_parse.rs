use crate::json;
use hexchess::hexchess::position::Position;
use hexchess::hexchess::promotion_piece::PromotionPiece;
use hexchess::hexchess::san::San;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    description: String,
    error: bool,
    san: String,
    expect: Option<SanStruct>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SanStruct {
    from: String,
    promotion: Option<String>,
    to: String,
}

#[test]
fn test_san_parse() {
    let tests = json::<Test>("san-parse.json");

    for test in tests {
        let actual: Result<San, String> = San::from_string(&test.san);

        if test.expect.is_none() {
            assert!(actual.is_err(), "{}", test.description);
            continue;
        }

        let actual_san = actual.unwrap();

        let expected_san = test.expect.unwrap();

        assert_eq!(actual_san.from, Position::from_string(&expected_san.from).unwrap());

        assert_eq!(actual_san.promotion, match expected_san.promotion {
            Some(val) => Some(PromotionPiece::from_string(&val).unwrap()),
            None => None,
        });
    }
}
