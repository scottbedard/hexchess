use crate::yaml;
use hexchess::hexchess::color::Color;
use hexchess::hexchess::game::Game;
use hexchess::hexchess::piece::Piece;
use hexchess::hexchess::position::Position;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Test {
    description: String,
    color: String,
    expected: Option<String>,
    hexchess: String,
}

#[test]
fn test_find_king() {
    let tests = yaml::<Test>("find-king.yaml");

    for test in tests {
        let color = Color::from_string(&test.color);
        let hexchess = Game::parse(&test.hexchess).unwrap();
        let king = hexchess.find_king(color);

        match king {
            Some(p) => assert_eq!(p.to_string(), test.expected.unwrap(), "{}", test.description),
            None => assert_eq!(king, None, "{}", test.description),
        }
    }
}
