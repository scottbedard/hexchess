extern crate hexchess_bitmask;

use crate::hexchess::color::Color;
use crate::hexchess::game::Game;
use crate::hexchess::position::Position;
use crate::hexchess::promotion_piece::PromotionPiece;
use crate::hexchess::san::San;
use hexchess_bitmask::{bitmask, bitmask_csv};

const BLACK_PAWN_THREATS: [u128; 91] = [
    bitmask_csv!("g10, e10"), // F11
    bitmask_csv!("f10, d9"), // E10
    bitmask_csv!("g9, e9"), // F10
    bitmask_csv!("h9, f10"), // G10
    bitmask_csv!("e9, c8"), // D9
    bitmask_csv!("f9, d8"), // E9
    bitmask_csv!("g8, e8"), // F9
    bitmask_csv!("h8, f9"), // G9
    bitmask_csv!("i8, g9"), // H9
    bitmask_csv!("d8, b7"), // C8
    bitmask_csv!("e8, c7"), // D8
    bitmask_csv!("f8, d7"), // E8
    bitmask_csv!("g7, e7"), // F8
    bitmask_csv!("h7, f8"), // G8
    bitmask_csv!("i7, g8"), // H8
    bitmask_csv!("k7, h8"), // I8
    bitmask_csv!("c7, a6"), // B7
    bitmask_csv!("d7, b6"), // C7
    bitmask_csv!("e7, c6"), // D7
    bitmask_csv!("f7, d6"), // E7
    bitmask_csv!("g6, e6"), // F7
    bitmask_csv!("h6, f7"), // G7
    bitmask_csv!("i6, g7"), // H7
    bitmask_csv!("k6, h7"), // I7
    bitmask_csv!("l6, i7"), // K7
    bitmask_csv!("b6"), // A6
    bitmask_csv!("c6, a5"), // B6
    bitmask_csv!("d6, b5"), // C6
    bitmask_csv!("e6, c5"), // D6
    bitmask_csv!("f6, d5"), // E6
    bitmask_csv!("g5, e5"), // F6
    bitmask_csv!("h5, f6"), // G6
    bitmask_csv!("i5, g6"), // H6
    bitmask_csv!("k5, h6"), // I6
    bitmask_csv!("l5, i6"), // K6
    bitmask_csv!("k6"), // L6
    bitmask_csv!("b5"), // A5
    bitmask_csv!("c5, a4"), // B5
    bitmask_csv!("d5, b4"), // C5
    bitmask_csv!("e5, c4"), // D5
    bitmask_csv!("f5, d4"), // E5
    bitmask_csv!("g4, e4"), // F5
    bitmask_csv!("h4, f5"), // G5
    bitmask_csv!("i4, g5"), // H5
    bitmask_csv!("k4, h5"), // I5
    bitmask_csv!("l4, i5"), // K5
    bitmask_csv!("k5"), // L5
    bitmask_csv!("b4"), // A4
    bitmask_csv!("c4, a3"), // B4
    bitmask_csv!("d4, b3"), // C4
    bitmask_csv!("e4, c3"), // D4
    bitmask_csv!("f4, d3"), // E4
    bitmask_csv!("g3, e3"), // F4
    bitmask_csv!("h3, f4"), // G4
    bitmask_csv!("i3, g4"), // H4
    bitmask_csv!("k3, h4"), // I4
    bitmask_csv!("l3, i4"), // K4
    bitmask_csv!("k4"), // L4
    bitmask_csv!("b3"), // A3
    bitmask_csv!("c3, a2"), // B3
    bitmask_csv!("d3, b2"), // C3
    bitmask_csv!("e3, c2"), // D3
    bitmask_csv!("f3, d2"), // E3
    bitmask_csv!("g2, e2"), // F3
    bitmask_csv!("h2, f3"), // G3
    bitmask_csv!("i2, g3"), // H3
    bitmask_csv!("k2, h3"), // I3
    bitmask_csv!("l2, i3"), // K3
    bitmask_csv!("k3"), // L3
    bitmask_csv!("b2"), // A2
    bitmask_csv!("c2, a1"), // B2
    bitmask_csv!("d2, b1"), // C2
    bitmask_csv!("e2, c1"), // D2
    bitmask_csv!("f2, d1"), // E2
    bitmask_csv!("g1, e1"), // F2
    bitmask_csv!("h1, f2"), // G2
    bitmask_csv!("i1, g2"), // H2
    bitmask_csv!("k1, h2"), // I2
    bitmask_csv!("l1, i2"), // K2
    bitmask_csv!("k2"), // L2
    bitmask_csv!("b1"), // A1
    bitmask_csv!("c1"), // B1
    bitmask_csv!("d1"), // C1
    bitmask_csv!("e1"), // D1
    bitmask_csv!("f1"), // E1
    0, // F1
    bitmask_csv!("f1"), // G1
    bitmask_csv!("g1"), // H1
    bitmask_csv!("h1"), // I1
    bitmask_csv!("i1"), // K1
    bitmask_csv!("k1"), // L1
];

const WHITE_PAWN_THREATS: [u128; 91] = [
    0, // F11
    bitmask_csv!("f11"), // E10
    bitmask_csv!("e10, g10"), // F10
    bitmask_csv!("f11"), // G10
    bitmask_csv!("e10"), // D9
    bitmask_csv!("d9, f10"), // E9
    bitmask_csv!("e9, g9"), // F9
    bitmask_csv!("f10, h9"), // G9
    bitmask_csv!("g10"), // H9
    bitmask_csv!("d9"), // C8
    bitmask_csv!("c8, e9"), // D8
    bitmask_csv!("d8, f9"), // E8
    bitmask_csv!("e8, g8"), // F8
    bitmask_csv!("f9, h8"), // G8
    bitmask_csv!("g9, i8"), // H8
    bitmask_csv!("h9"), // I8
    bitmask_csv!("c8"), // B7
    bitmask_csv!("b7, d8"), // C7
    bitmask_csv!("c7, e8"), // D7
    bitmask_csv!("d7, f8"), // E7
    bitmask_csv!("e7, g7"), // F7
    bitmask_csv!("f8, h7"), // G7
    bitmask_csv!("g8, i7"), // H7
    bitmask_csv!("h8, k7"), // I7
    bitmask_csv!("i8"), // K7
    bitmask_csv!("b7"), // A6
    bitmask_csv!("a6, c7"), // B6
    bitmask_csv!("b6, d7"), // C6
    bitmask_csv!("c6, e7"), // D6
    bitmask_csv!("d6, f7"), // E6
    bitmask_csv!("e6, g6"), // F6
    bitmask_csv!("f7, h6"), // G6
    bitmask_csv!("g7, i6"), // H6
    bitmask_csv!("h7, k6"), // I6
    bitmask_csv!("i7, l6"), // K6
    bitmask_csv!("k7"), // L6
    bitmask_csv!("b6"), // A5
    bitmask_csv!("a5, c6"), // B5
    bitmask_csv!("b5, d6"), // C5
    bitmask_csv!("c5, e6"), // D5
    bitmask_csv!("d5, f6"), // E5
    bitmask_csv!("e5, g5"), // F5
    bitmask_csv!("f6, h5"), // G5
    bitmask_csv!("g6, i5"), // H5
    bitmask_csv!("h6, k5"), // I5
    bitmask_csv!("i6, l5"), // K5
    bitmask_csv!("k6"), // L5
    bitmask_csv!("b5"), // A4
    bitmask_csv!("a4, c5"), // B4
    bitmask_csv!("b4, d5"), // C4
    bitmask_csv!("c4, e5"), // D4
    bitmask_csv!("d4, f5"), // E4
    bitmask_csv!("e4, g4"), // F4
    bitmask_csv!("f5, h4"), // G4
    bitmask_csv!("g5, i4"), // H4
    bitmask_csv!("h5, k4"), // I4
    bitmask_csv!("i5, l4"), // K4
    bitmask_csv!("k5"), // L4
    bitmask_csv!("b4"), // A3
    bitmask_csv!("a3, c4"), // B3
    bitmask_csv!("b3, d4"), // C3
    bitmask_csv!("c3, e4"), // D3
    bitmask_csv!("d3, f4"), // E3
    bitmask_csv!("e3, g3"), // F3
    bitmask_csv!("f4, h3"), // G3
    bitmask_csv!("g4, i3"), // H3
    bitmask_csv!("h4, k3"), // I3
    bitmask_csv!("i4, l3"), // K3
    bitmask_csv!("k4"), // L3
    bitmask_csv!("b3"), // A2
    bitmask_csv!("a2, c3"), // B2
    bitmask_csv!("b2, d3"), // C2
    bitmask_csv!("c2, e3"), // D2
    bitmask_csv!("d2, f3"), // E2
    bitmask_csv!("e2, g2"), // F2
    bitmask_csv!("f3, h2"), // G2
    bitmask_csv!("g3, i2"), // H2
    bitmask_csv!("h3, k2"), // I2
    bitmask_csv!("i3, l2"), // K2
    bitmask_csv!("k3"), // L2
    bitmask_csv!("b2"), // A1
    bitmask_csv!("a1, c2"), // B1
    bitmask_csv!("b1, d2"), // C1
    bitmask_csv!("c1, e2"), // D1
    bitmask_csv!("d1, f2"), // E1
    bitmask_csv!("e1, g1"), // F1
    bitmask_csv!("f2, h1"), // G1
    bitmask_csv!("g2, i1"), // H1
    bitmask_csv!("h2, k1"), // I1
    bitmask_csv!("i2, l1"), // K1
    bitmask_csv!("k2"), // L1
];

/// get bitmask for pawn threats
pub fn get_pawn_threats_bitmask(position: Position, color: Color) -> u128 {
    match color {
        Color::Black => BLACK_PAWN_THREATS[position.to_fen_index() as usize],
        Color::White => WHITE_PAWN_THREATS[position.to_fen_index() as usize],
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moves_unsafe_from_empty() {
        let game = Game::new();

        let sans = get_pawn_moves_unsafe(&game, Position::F1);

        assert_eq!(sans.len(), 0);
    }

    #[test]
    fn test_all_bitmasks_return_number() {
        for n in 0..91 {
            let position = Position::from_fen_index(n as u8);

            if position == Position::F1 {
                assert_eq!(get_pawn_threats_bitmask(position, Color::Black), 0, "black {:?}", position);
                assert!(get_pawn_threats_bitmask(position, Color::White) > 0, "white {:?}", position);
            } else if position == Position::F11 {
                assert!(get_pawn_threats_bitmask(position, Color::Black) > 0, "black {:?}", position);
                assert_eq!(get_pawn_threats_bitmask(position, Color::White), 0, "white {:?}", position);
            } else {
                assert!(get_pawn_threats_bitmask(position, Color::Black) > 0, "black {:?}", position);
                assert!(get_pawn_threats_bitmask(position, Color::White) > 0, "white {:?}", position);
            }
        }
    }
}