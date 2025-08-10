use crate::json;
use hexchess::hexchess::color::Color;
use hexchess::hexchess::game::Game;
use hexchess::hexchess::position::Position;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct TestResult {
    board: Vec<Option<String>>,
    ep: Option<u8>,
    fullmove: u16,
    halfmove: u8,
    turn: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    description: String,
    error: bool,
    fen: String,
    result: Option<TestResult>,
}

#[test]
fn test_hexchess_parse() {
    let tests = json::<Test>("hexchess-parse.json");

    for test in tests {
        let result = Game::parse(&test.fen);

        if test.error {
            assert!(result.is_err(), "{}", test.description);
            continue;
        }
        
        if test.result.is_some() {
            let actual = result.unwrap();
            let expected = test.result.unwrap();

            match expected.ep {
                Some(ep) => {
                    let actual_ep = actual.ep.unwrap();
                    let expected_ep = Position::from_fen_index(ep);
                    assert_eq!(actual_ep, expected_ep);
                },
                None => assert!(actual.ep.is_none()),
            }

            actual
                .to_board_array()
                .iter()
                .enumerate()
                .for_each(|(i, actual_piece)| {
                let expected_piece = &expected.board[i];
                    match (actual_piece, expected_piece) {
                        (None, None) => {},
                        (Some(a), Some(e)) => assert_eq!(a.to_string(), *e, "Mismatch at index {}: expected {:?}, got {:?}", i, e, a),
                        (None, Some(e)) => panic!("Expected piece {:?} at index {}, but got None", e, i),
                        (Some(a), None) => panic!("Expected None at index {}, but got {:?}", i, a),
                    }
                });

            assert_eq!(expected.fullmove, actual.fullmove);
            assert_eq!(expected.halfmove, actual.halfmove);
            assert_eq!(Color::from_string(&expected.turn), actual.turn);
        }
    }
}
