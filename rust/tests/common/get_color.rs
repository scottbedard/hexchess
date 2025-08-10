use crate::json;
use hexchess::hexchess::color::Color;
use hexchess::hexchess::game::Game;
use hexchess::hexchess::position::Position;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    color: String,
    description: String,
    from: String,
    result: Vec<String>,
}

#[test]
fn test_get_color() {
    let tests = json::<Test>("get-color.json");

    for test in tests {
        let color = Color::from_string(&test.color);
        let hexchess = Game::parse(&test.from).unwrap();

        let mut result = Vec::new();
        hexchess.get_color_bitboard(color).iter_bits(|index| {
            result.push(Position::from_bitboard_index(index as u8).to_string());
        });

        for pos in &result {
            assert!(
                test.result.contains(pos),
                "Position {} found in result but not in expected result for test {}",
                pos,
                test.description
            );
        }

        assert_eq!(result.len(), test.result.len());
    }
}
