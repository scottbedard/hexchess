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

        let result: Vec<String> = hexchess.get_color_bitboard(color)
            .iter_set_bits()
            .map(|n| Position::from_bitboard_index(n).to_string())
            .collect();

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
