use crate::json;
use hexchess::hexchess::position::Position;
use hexchess::hexchess::promotion_piece::PromotionPiece;
use hexchess::hexchess::san::San;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    description: String,
    san: TestSan,
    expect: String
}

#[derive(Debug, Serialize, Deserialize)]
struct TestSan {
    from: String,
    promotion: Option<String>,
    to: String,
}

#[test]
#[ignore]
fn test_san_to_string() {
    let tests = json::<Test>("san-to-string.json");

    for test in tests {
        let san = San {
            from: Position::from_string(&test.san.from),
            promotion: match test.san.promotion {
                Some(val) => match val.as_str() {
                    "b" => Some(PromotionPiece::Bishop),
                    "n" => Some(PromotionPiece::Knight),
                    "q" => Some(PromotionPiece::Queen),
                    "r" => Some(PromotionPiece::Rook),
                    _ => panic!("invalid promotion character: {}", val),
                },
                None => None,
            },
            to: Position::from_string(&test.san.to),
        };

        assert_eq!(san.to_string(), test.expect, "{}", test.description);
    }
}
