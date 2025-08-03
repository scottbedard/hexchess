extern crate hexchess_bitmask;

use crate::hexchess::bitboard::Bitboard;
use crate::hexchess::color::Color;
use crate::hexchess::game::Game;
use crate::hexchess::position::Position;
use crate::hexchess::promotion_piece::PromotionPiece;
use crate::hexchess::san::San;
use hexchess_bitmask::{bitmask, bitmask_csv};

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

fn push_promotion_sans(output: &mut Vec<San>, from: Position, to: Position) {
    output.reserve(4);
    output.push(San { from, to, promotion: Some(PromotionPiece::Queen) });
    output.push(San { from, to, promotion: Some(PromotionPiece::Rook) });
    output.push(San { from, to, promotion: Some(PromotionPiece::Bishop) });
    output.push(San { from, to, promotion: Some(PromotionPiece::Knight) });
}