use crate::hexchess::bitboard::Bitboard;
use crate::hexchess::color::Color;
use crate::hexchess::game::Game;
use crate::hexchess::position::Position;
use crate::hexchess::san::San;

pub fn get_pawn_moves_unsafe(game: &Game, from: Position) -> Vec<San> {
    let color = game.get_color(from);
    let _file_bitmask = from.to_file_bitmask();
    
    let mut _result = Bitboard::new();

    // advance forward
    match color {
        Some(Color::White) => {
            // f10 f11
            // e9 e10
            // f9 f10
            // g9 g10
            // d8 d9
            // e8 e9
            // f8 f9
            // g8 g9
            // h8 h9
            // c7 c8
            // d7 d8
            // e7 e8
            // f7 f8
            // g7 g8
            // h7 h8
            // i7 i8
            // b6 b7
            // c6 c7
            // d6 d7
            // e6 e7
            // f6 f7
            // g6 g7
            // h6 h7
            // i6 i7
            // k6 k7
            // a5 a6
            // b5 b6
            // c5 c6
            // d5 d6
            // e5 e6
            // f5 f6
            // g5 g6
            // h5 h6
            // i5 i6
            // k5 k6
            // l5 l6
            // a4 a5
            // b4 b5
            // c4 c5
            // d4 d5
            // e4 e5
            // g4 g5
            // h4 h5
            // i4 i5
            // k4 k5
            // l4 l5
            // a3 a4
            // b3 b4
            // c3 c4
            // d3 d4
            // h3 h4
            // i3 i4
            // k3 k4
            // l3 l4
            // a2 a3
            // b2 b3
            // c2 c3
            // i2 i3
            // k2 k3
            // l2 l3
            // a1 a2
            // b1 b2
            // k1 k2
            // l1 l2
        }
        Some(Color::Black) => {
            // ...
        }
        None => {}
    }

    let output = Vec::new();
    
    // let mut output = Vec::with_capacity(result.count_ones() as usize);

    // for index in result.iter_set_bits() {
    //     let to = Position::from_bitboard_index(index);
    //     let san = San::new(from, to);
    //     output.push(san);
    // }

    output
}