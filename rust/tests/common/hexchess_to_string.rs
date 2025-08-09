use crate::json;
use hexchess::hexchess::color::Color;
use hexchess::hexchess::game::Game;
use hexchess::hexchess::piece::Piece;
use hexchess::hexchess::position::Position;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct TestResult {
    board: Vec<Option<String>>,
    turn: String,
    ep: Option<u8>,
    halfmove: u8,
    fullmove: u16,
}

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    description: String,
    from: TestResult,
    result: String,
}

#[test]
fn test_hexchess_to_string() {
    let tests = json::<Test>("hexchess-to-string.json");

    for test in tests {
        // let mut hexchess = Hexchess::new();
        let mut game = Game::new();

        let expected_array: [Option<Piece>; 91] = test
            .from
            .board
            .iter()
            .map(|p1| match p1 {
                Some(p2) => Some(Piece::from_string(p2.as_str()).unwrap()),
                None => None,
            })
            .collect::<Vec<Option<Piece>>>()
            .try_into()
            .unwrap();

        game.set_board_array(expected_array);
        game.ep = match test.from.ep {
            Some(ep) => Some(Position::from_fen_index(ep)),
            None => None,
        };
        game.fullmove = test.from.fullmove;
        game.halfmove = test.from.halfmove;
        game.turn = Color::from_string(&test.from.turn);

        assert_eq!(game.to_string(), test.result, "{}", test.description);
    }
}
