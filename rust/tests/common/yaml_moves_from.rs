use crate::yaml;
use hexchess::hexchess::game::Game;
use hexchess::hexchess::position::Position;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Test {
    description: String,
    direction: u8,
    expected: Vec<String>,
    hexchess: String,
    position: String,
}

#[test]
fn test_moves_from() {
    let tests = yaml::<Test>("moves-from.yaml");

    for test in tests {
        let position = Position::from_string(&test.position).unwrap();
        let game = Game::parse(&test.hexchess).unwrap();
        let moves = game.get_moves(position).into_iter().map(|san| san.to.to_string()).collect::<Vec<String>>();

        let mut expected = test.expected.clone();
        let mut moves = moves;
        expected.sort();
        moves.sort();

        assert_eq!(moves, expected, "{}", test.description);
    }
}
