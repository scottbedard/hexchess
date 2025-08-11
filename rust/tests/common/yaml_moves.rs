use crate::yaml;
use hexchess::hexchess::game::Game;
use hexchess::hexchess::position::Position;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Test {
    description: String,
    expected: Vec<String>,
    hexchess: String,
    position: String,
}

fn test_moves_file(file: &str) {
    let tests = yaml::<Test>(file);

    for test in tests {
        let position = Position::from_string(&test.position).unwrap();
        let game = Game::parse(&test.hexchess).unwrap();
        let moves = game.get_moves(position).into_iter().map(|san| san.to_string()).collect::<Vec<String>>();

        let mut expected = test.expected.clone();
        let mut moves = moves;
        expected.sort();
        moves.sort();

        assert_eq!(moves, expected, "{}", test.description);
    }
}

#[test]
fn test_moves() {
    test_moves_file("moves-from.yaml");
    test_moves_file("moves-king.yaml");
    test_moves_file("moves-knight.yaml");
    test_moves_file("moves-pawn.yaml");
    test_moves_file("moves-straight-line.yaml");
}
