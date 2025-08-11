use crate::yaml;
use hexchess::hexchess::color::Color;
use hexchess::hexchess::game::Game;
use hexchess::hexchess::piece::Piece;
use hexchess::hexchess::position::Position;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct HexchessStruct {
    board: Vec<Option<String>>,
    ep: Option<u8>,
    fullmove: u16,
    halfmove: u8,
    turn: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Test {
    description: String,
    expected: String,
    hexchess: HexchessStruct,
}

#[test]
fn test_stringify_hexchess() {
    let tests = yaml::<Test>("stringify-hexchess.yaml");

    for test in tests {
        let mut game = Game::new();

        let expected_array: [Option<Piece>; 91] = test
            .hexchess
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
        game.ep = match test.hexchess.ep {
            Some(ep) => Some(Position::from_fen_index(ep)),
            None => None,
        };
        game.fullmove = test.hexchess.fullmove;
        game.halfmove = test.hexchess.halfmove;
        game.turn = Color::from_string(&test.hexchess.turn);

        assert_eq!(game.to_string(), test.expected, "{}", test.description);
    }
}
