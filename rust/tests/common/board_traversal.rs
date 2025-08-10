use crate::json;
use hexchess::hexchess::color::Color;
use hexchess::hexchess::game::Game;
use hexchess::hexchess::position::Position;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    color: String,
    description: String,
    direction: u8,
    from: String,
    hexchess: String,
    result: Vec<String>,
}


#[test]
#[ignore]
fn test_board_traversal() {
    // board traversal tests no longer apply when using bitmasks.
    // a new test suite will be needed for traversal assertions.
    let tests = json::<Test>("board-traversal.json");

    for test in tests {
        let _color = Color::from_string(&test.color);
        let _game = Game::parse(&test.hexchess).unwrap();
        let _position = Position::from_string(&test.from).unwrap();

        // ...    
    }
}
