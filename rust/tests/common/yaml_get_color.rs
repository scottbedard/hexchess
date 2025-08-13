use crate::yaml;
use hexchess::hexchess::color::Color;
use hexchess::hexchess::hexchess::Hexchess;
use hexchess::hexchess::position::Position;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Test {
    description: String,
    color: String,
    expected: Vec<String>,
    hexchess: String,
}

#[test]
fn test_get_color() {
    let tests = yaml::<Test>("get-color.yaml");

    for test in tests {
        let color = Color::from_string(&test.color);
        let hexchess = Hexchess::parse(&test.hexchess).unwrap();
        
        let mut actual = Vec::new();

        hexchess.get_color_bitboard(color).iter_bits(|index| {
            actual.push(Position::from_bitboard_index(index as u8).to_string());
        });

        for pos in &actual {
            assert!(
                test.expected.contains(pos),
                "Position {} found in result but not in expected result for test {}",
                pos,
                test.description
            );
        }

        assert_eq!(actual.len(), test.expected.len());
    }
}
