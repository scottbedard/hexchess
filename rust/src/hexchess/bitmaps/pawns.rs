extern crate hexchess_bitmask;

use crate::hexchess::color::Color;
use crate::hexchess::game::Game;
use crate::hexchess::position::Position;
use crate::hexchess::promotion_piece::PromotionPiece;
use crate::hexchess::san::San;
use hexchess_bitmask::{bitmask, bitmask_csv};

/// get bitmask for pawn threats
pub fn get_pawn_threats_bitmask(position: Position, color: Color) -> u128 {
    match color {
        Color::Black => match position {
            Position::F11 => bitmask_csv!("g10, e10"),
            Position::E10 => bitmask_csv!("f10, d9"),
            Position::F10 => bitmask_csv!("g9, e9"),
            Position::G10 => bitmask_csv!("h9, f10"),
            Position::D9 => bitmask_csv!("e9, c8"),
            Position::E9 => bitmask_csv!("f9, d8"),
            Position::F9 => bitmask_csv!("g8, e8"),
            Position::G9 => bitmask_csv!("h8, f9"),
            Position::H9 => bitmask_csv!("i8, g9"),
            Position::C8 => bitmask_csv!("d8, b7"),
            Position::D8 => bitmask_csv!("e8, c7"),
            Position::E8 => bitmask_csv!("f8, d7"),
            Position::F8 => bitmask_csv!("g7, e7"),
            Position::G8 => bitmask_csv!("h7, f8"),
            Position::H8 => bitmask_csv!("i7, g8"),
            Position::I8 => bitmask_csv!("k7, h8"),
            Position::B7 => bitmask_csv!("c7, a6"),
            Position::C7 => bitmask_csv!("d7, b6"),
            Position::D7 => bitmask_csv!("e7, c6"),
            Position::E7 => bitmask_csv!("f7, d6"),
            Position::F7 => bitmask_csv!("g6, e6"),
            Position::G7 => bitmask_csv!("h6, f7"),
            Position::H7 => bitmask_csv!("i6, g7"),
            Position::I7 => bitmask_csv!("k6, h7"),
            Position::K7 => bitmask_csv!("l6, i7"),
            Position::A6 => bitmask_csv!("b6"),
            Position::B6 => bitmask_csv!("c6, a5"),
            Position::C6 => bitmask_csv!("d6, b5"),
            Position::D6 => bitmask_csv!("e6, c5"),
            Position::E6 => bitmask_csv!("f6, d5"),
            Position::F6 => bitmask_csv!("g5, e5"),
            Position::G6 => bitmask_csv!("h5, f6"),
            Position::H6 => bitmask_csv!("i5, g6"),
            Position::I6 => bitmask_csv!("k5, h6"),
            Position::K6 => bitmask_csv!("l5, i6"),
            Position::L6 => bitmask_csv!("k6"),
            Position::A5 => bitmask_csv!("b5"),
            Position::B5 => bitmask_csv!("c5, a4"),
            Position::C5 => bitmask_csv!("d5, b4"),
            Position::D5 => bitmask_csv!("e5, c4"),
            Position::E5 => bitmask_csv!("f5, d4"),
            Position::F5 => bitmask_csv!("g4, e4"),
            Position::G5 => bitmask_csv!("h4, f5"),
            Position::H5 => bitmask_csv!("i4, g5"),
            Position::I5 => bitmask_csv!("k4, h5"),
            Position::K5 => bitmask_csv!("l4, i5"),
            Position::L5 => bitmask_csv!("k5"),
            Position::A4 => bitmask_csv!("b4"),
            Position::B4 => bitmask_csv!("c4, a3"),
            Position::C4 => bitmask_csv!("d4, b3"),
            Position::D4 => bitmask_csv!("e4, c3"),
            Position::E4 => bitmask_csv!("f4, d3"),
            Position::F4 => bitmask_csv!("g3, e3"),
            Position::G4 => bitmask_csv!("h3, f4"),
            Position::H4 => bitmask_csv!("i3, g4"),
            Position::I4 => bitmask_csv!("k3, h4"),
            Position::K4 => bitmask_csv!("l3, i4"),
            Position::L4 => bitmask_csv!("k4"),
            Position::A3 => bitmask_csv!("b3"),
            Position::B3 => bitmask_csv!("c3, a2"),
            Position::C3 => bitmask_csv!("d3, b2"),
            Position::D3 => bitmask_csv!("e3, c2"),
            Position::E3 => bitmask_csv!("f3, d2"),
            Position::F3 => bitmask_csv!("g2, e2"),
            Position::G3 => bitmask_csv!("h2, f3"),
            Position::H3 => bitmask_csv!("i2, g3"),
            Position::I3 => bitmask_csv!("k2, h3"),
            Position::K3 => bitmask_csv!("l2, i3"),
            Position::L3 => bitmask_csv!("k3"),
            Position::A2 => bitmask_csv!("b2"),
            Position::B2 => bitmask_csv!("c2, a1"),
            Position::C2 => bitmask_csv!("d2, b1"),
            Position::D2 => bitmask_csv!("e2, c1"),
            Position::E2 => bitmask_csv!("f2, d1"),
            Position::F2 => bitmask_csv!("g1, e1"),
            Position::G2 => bitmask_csv!("h1, f2"),
            Position::H2 => bitmask_csv!("i1, g2"),
            Position::I2 => bitmask_csv!("k1, h2"),
            Position::K2 => bitmask_csv!("l1, i2"),
            Position::L2 => bitmask_csv!("k2"),
            Position::A1 => bitmask_csv!("b1"),
            Position::B1 => bitmask_csv!("c1"),
            Position::C1 => bitmask_csv!("d1"),
            Position::D1 => bitmask_csv!("e1"),
            Position::E1 => bitmask_csv!("f1"),
            Position::F1 => 0,
            Position::G1 => bitmask_csv!("f1"),
            Position::H1 => bitmask_csv!("g1"),
            Position::I1 => bitmask_csv!("h1"),
            Position::K1 => bitmask_csv!("i1"),
            Position::L1 => bitmask_csv!("k1"),
        },
        Color::White => match position {
            Position::F11 => 0,
            Position::E10 => bitmask_csv!("f11"),
            Position::F10 => bitmask_csv!("e10, g10"),
            Position::G10 => bitmask_csv!("f11"),
            Position::D9 => bitmask_csv!("e10"),
            Position::E9 => bitmask_csv!("d9, f10"),
            Position::F9 => bitmask_csv!("e9, g9"),
            Position::G9 => bitmask_csv!("f10, h9"),
            Position::H9 => bitmask_csv!("g10"),
            Position::C8 => bitmask_csv!("d9"),
            Position::D8 => bitmask_csv!("c8, e9"),
            Position::E8 => bitmask_csv!("d8, f9"),
            Position::F8 => bitmask_csv!("e8, g8"),
            Position::G8 => bitmask_csv!("f9, h8"),
            Position::H8 => bitmask_csv!("g9, i8"),
            Position::I8 => bitmask_csv!("h9"),
            Position::B7 => bitmask_csv!("c8"),
            Position::C7 => bitmask_csv!("b7, d8"),
            Position::D7 => bitmask_csv!("c7, e8"),
            Position::E7 => bitmask_csv!("d7, f8"),
            Position::F7 => bitmask_csv!("e7, g7"),
            Position::G7 => bitmask_csv!("f8, h7"),
            Position::H7 => bitmask_csv!("g8, i7"),
            Position::I7 => bitmask_csv!("h8, k7"),
            Position::K7 => bitmask_csv!("i8"),
            Position::A6 => bitmask_csv!("b7"),
            Position::B6 => bitmask_csv!("a6, c7"),
            Position::C6 => bitmask_csv!("b6, d7"),
            Position::D6 => bitmask_csv!("c6, e7"),
            Position::E6 => bitmask_csv!("d6, f7"),
            Position::F6 => bitmask_csv!("e6, g6"),
            Position::G6 => bitmask_csv!("f7, h6"),
            Position::H6 => bitmask_csv!("g7, i6"),
            Position::I6 => bitmask_csv!("h7, k6"),
            Position::K6 => bitmask_csv!("i7, l6"),
            Position::L6 => bitmask_csv!("k7"),
            Position::A5 => bitmask_csv!("b6"),
            Position::B5 => bitmask_csv!("a5, c6"),
            Position::C5 => bitmask_csv!("b5, d6"),
            Position::D5 => bitmask_csv!("c5, e6"),
            Position::E5 => bitmask_csv!("d5, f6"),
            Position::F5 => bitmask_csv!("e5, g5"),
            Position::G5 => bitmask_csv!("f6, h5"),
            Position::H5 => bitmask_csv!("g6, i5"),
            Position::I5 => bitmask_csv!("h6, k5"),
            Position::K5 => bitmask_csv!("i6, l5"),
            Position::L5 => bitmask_csv!("k6"),
            Position::A4 => bitmask_csv!("b5"),
            Position::B4 => bitmask_csv!("a4, c5"),
            Position::C4 => bitmask_csv!("b4, d5"),
            Position::D4 => bitmask_csv!("c4, e5"),
            Position::E4 => bitmask_csv!("d4, f5"),
            Position::F4 => bitmask_csv!("e4, g4"),
            Position::G4 => bitmask_csv!("f5, h4"),
            Position::H4 => bitmask_csv!("g5, i4"),
            Position::I4 => bitmask_csv!("h5, k4"),
            Position::K4 => bitmask_csv!("i5, l4"),
            Position::L4 => bitmask_csv!("k5"),
            Position::A3 => bitmask_csv!("b4"),
            Position::B3 => bitmask_csv!("a3, c4"),
            Position::C3 => bitmask_csv!("b3, d4"),
            Position::D3 => bitmask_csv!("c3, e4"),
            Position::E3 => bitmask_csv!("d3, f4"),
            Position::F3 => bitmask_csv!("e3, g3"),
            Position::G3 => bitmask_csv!("f4, h3"),
            Position::H3 => bitmask_csv!("g4, i3"),
            Position::I3 => bitmask_csv!("h4, k3"),
            Position::K3 => bitmask_csv!("i4, l3"),
            Position::L3 => bitmask_csv!("k4"),
            Position::A2 => bitmask_csv!("b3"),
            Position::B2 => bitmask_csv!("a2, c3"),
            Position::C2 => bitmask_csv!("b2, d3"),
            Position::D2 => bitmask_csv!("c2, e3"),
            Position::E2 => bitmask_csv!("d2, f3"),
            Position::F2 => bitmask_csv!("e2, g2"),
            Position::G2 => bitmask_csv!("f3, h2"),
            Position::H2 => bitmask_csv!("g3, i2"),
            Position::I2 => bitmask_csv!("h3, k2"),
            Position::K2 => bitmask_csv!("i3, l2"),
            Position::L2 => bitmask_csv!("k3"),
            Position::A1 => bitmask_csv!("b2"),
            Position::B1 => bitmask_csv!("a1, c2"),
            Position::C1 => bitmask_csv!("b1, d2"),
            Position::D1 => bitmask_csv!("c1, e2"),
            Position::E1 => bitmask_csv!("d1, f2"),
            Position::F1 => bitmask_csv!("e1, g1"),
            Position::G1 => bitmask_csv!("f2, h1"),
            Position::H1 => bitmask_csv!("g2, i1"),
            Position::I1 => bitmask_csv!("h2, k1"),
            Position::K1 => bitmask_csv!("i2, l1"),
            Position::L1 => bitmask_csv!("k2"),
        },
    }
}

/// get pawn moves unsafe
pub fn get_pawn_moves_unsafe(game: &Game, from_position: Position) -> Vec<San> {
    let color = match game.get_color(from_position) {
        Some(color) => color,
        None => return Vec::new(),
    };

    let (
        forward_direction, 
        portside_direction, 
        starboard_direction,
        promotion_mask
    ) = match color {
        Color::Black => (6u8, 4u8, 8u8, bitmask!("1/3/5/7/9/11/11/11/11/11/xxxxxxxxxxx")),
        Color::White => (0u8, 10u8, 2u8, bitmask!("x/x1x/x3x/x5x/x7x/x9x/11/11/11/11/11")),
    };

    let mut output = Vec::new();

    // advance forward
    match from_position.step(forward_direction) {
        Some(forward_position) => if game.is_position_empty(forward_position) {
            let forward_mask = forward_position.to_bitmask();

            if forward_mask & promotion_mask != 0 {
                push_promotion_sans(&mut output, from_position, forward_position);
            } else {
                output.push(San::new(from_position, forward_position));

                let double_forward_mask = match color {
                    Color::Black => bitmask!("1/3/5/7/9/1xxxxxxxxx1/11/11/11/11/11"),
                    Color::White => bitmask!("1/3/5/7/9/5x5/4x1x4/3x3x3/2x5x2/1x7x1/11"),
                };

                // double forward advancement
                if forward_mask & double_forward_mask != 0 {
                    let double_forward_position = forward_position.step(forward_direction).unwrap();

                    if game.is_position_empty(double_forward_position) {
                        output.push(San::new(from_position, double_forward_position));
                    }
                }
            }
        },
        None => {},
    };
    
    // captures
    let ep_mask = match game.ep {   
        Some(ep) => match game.turn == color {
            true => ep.to_bitmask(),
            false => 0,
        },
        None => 0,
    };

    let capturable_mask = ep_mask | *game.get_color_bitboard(color.opposite());
    
    match from_position.step(portside_direction) {
        Some(portside_position) => {
            let portside_mask = portside_position.to_bitmask();

            if portside_mask & capturable_mask != 0 {
                if portside_mask & promotion_mask != 0 {
                    push_promotion_sans(&mut output, from_position, portside_position);
                } else {
                    output.push(San::new(from_position, portside_position));
                }
            }
        },
        None => {},
    };

    match from_position.step(starboard_direction) {
        Some(starboard_position) => {
            let starboard_mask = starboard_position.to_bitmask();

            if starboard_mask & capturable_mask != 0 {
                if starboard_mask & promotion_mask != 0 {
                    push_promotion_sans(&mut output, from_position, starboard_position);
                } else {
                    output.push(San::new(from_position, starboard_position));
                }
            }
        },
        None => {},
    };

    output
}

/// test if a position is a legal en passant target
pub fn is_legal_en_passant(position: Position) -> bool {
    position.to_bitmask() & bitmask!("1/3/5/7/9/1xxxxxxxxx1/4x1x4/3x3x3/2x5x2/1x7x1/11") != 0
}

/// push promotion sans
fn push_promotion_sans(output: &mut Vec<San>, from: Position, to: Position) {
    output.reserve(4);
    output.push(San { from, to, promotion: Some(PromotionPiece::Queen) });
    output.push(San { from, to, promotion: Some(PromotionPiece::Rook) });
    output.push(San { from, to, promotion: Some(PromotionPiece::Bishop) });
    output.push(San { from, to, promotion: Some(PromotionPiece::Knight) });
}
