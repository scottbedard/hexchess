use crate::json;
use hexchess::{index, San};
use hexchess::constants::PromotionPiece;
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
fn test_san_to_string() {
    let tests = json::<Test>("san-to-string.json");

    for test in tests {
        let san = San {
            from: index(&test.san.from).unwrap(),
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
            to: index(&test.san.to).unwrap(),
        };

        assert_eq!(san.to_string(), test.expect, "{}", test.description);
    }
}
