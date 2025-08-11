use crate::yaml;
use hexchess::hexchess::game::Game;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Test {
    description: String,
    expected: Vec<String>,
    hexchess: String,
}

#[test]
fn test_current_moves() {
    let tests = yaml::<Test>("current-moves.yaml");

    for test in tests {
        let game = Game::parse(&test.hexchess).unwrap();
        let moves = game.current_moves().into_iter().map(|san| san.to_string()).collect::<Vec<String>>();

        let mut expected = test.expected.clone();
        let mut moves = moves;
        expected.sort();
        moves.sort();

        assert_eq!(moves, expected, "{}", test.description);
    }
}
