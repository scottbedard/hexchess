use crate::yaml;
use hexchess::Position;
use hexchess::PromotionPiece;
use hexchess::San;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct SanStruct {
    from: String,
    promotion: Option<String>,
    to: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Test {
    description: String,
    san: SanStruct,
    expected: String,
}

#[test]
fn test_stringify_san() {
    let tests = yaml::<Test>("stringify-san.yaml");

    for test in tests {
        let san = San {
            from: Position::from_string(&test.san.from).unwrap(),
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
            to: Position::from_string(&test.san.to).unwrap(),
        };

        assert_eq!(san.to_string(), test.expected, "{}", test.description);
    }
}
