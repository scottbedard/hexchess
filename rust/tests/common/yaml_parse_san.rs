use crate::yaml;
use hexchess::hexchess::position::Position;
use hexchess::hexchess::promotion_piece::PromotionPiece;
use hexchess::hexchess::san::San;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Expected {
    from: String,
    promotion: Option<String>,
    to: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Test {
    description: String,
    error: bool,
    expected: Option<Expected>,
    san: String,
}

#[test]
fn test_parse_san() {
    let tests = yaml::<Test>("parse-san.yaml");

    for test in tests {
        let actual: Result<San, String> = San::from_string(&test.san);

        if test.expected.is_none() {
            assert!(actual.is_err(), "{}", test.description);
            continue;
        }

        let actual_san = actual.unwrap();

        let expected_san = test.expected.unwrap();

        assert_eq!(actual_san.from, Position::from_string(&expected_san.from).unwrap());

        assert_eq!(actual_san.promotion, match expected_san.promotion {
            Some(val) => Some(PromotionPiece::from_string(&val).unwrap()),
            None => None,
        });
    }
}
