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
    result: Option<String>,
}

#[test]
fn test_find_king() {
    let tests = json::<Test>("find-king.json");

    for test in tests {
        let color = Color::from_string(&test.color);
        let hexchess = Game::parse(&test.from).unwrap();

        let king = hexchess.find_king(color);

        if test.result.is_none() {
            assert_eq!(king, None, "{}", test.description);
        } else {
            let position = Position::from_string(&test.result.unwrap()).unwrap();

            assert_eq!(king, Some(position), "{}", test.description);
        }
    }
}
