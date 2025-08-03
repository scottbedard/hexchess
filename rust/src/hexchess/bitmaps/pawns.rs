extern crate hexchess_bitmask;

use crate::hexchess::color::Color;
use crate::hexchess::game::Game;
use crate::hexchess::position::Position;
use crate::hexchess::promotion_piece::PromotionPiece;
use crate::hexchess::san::San;
use hexchess_bitmask::bitmask;

pub fn get_pawn_moves_unsafe(game: &Game, from: Position) -> Vec<San> {
    let color = match game.get_color(from) {
        Some(color) => color,
        None => return Vec::new(),
    };

    let mut output = Vec::new();

    let promotion_mask = match color {
        Color::White => bitmask!("x/x1x/x3x/x5x/x7x/x9x/11/11/11/11/11"),
        Color::Black => bitmask!("1/3/5/7/9/11/11/11/11/11/xxxxxxxxxxx"),
    };

    // advance forward
    let forward = match color {
        Color::White => 0,
        Color::Black => 6,
    };

    match from.step(forward) {
        Some(forward_position) => if game.is_position_empty(forward_position) {
            let forward_mask = forward_position.to_bitmask();

            // forward promotion
            if forward_mask & promotion_mask != 0 {
                push_promotion_sans(&mut output, from, forward_position);
            } else {
                // advance forward
                output.push(San::new(from, forward_position));

                let double_forward_mask = match color {
                    Color::Black => bitmask!("1/3/5/7/9/1xxxxxxxxx1/11/11/11/11/11"),
                    Color::White => bitmask!("1/3/5/7/9/5x5/4x1x4/3x3x3/2x5x2/1x7x1/11"),
                };

                // advance forward again
                if forward_mask & double_forward_mask != 0 {
                    let double_forward_position = forward_position.step(forward).unwrap();

                    if game.is_position_empty(double_forward_position) {
                        output.push(San::new(from, double_forward_position));
                    }
                }
            }
        },
        None => {},
    };

    // captures

    output
}

fn push_promotion_sans(output: &mut Vec<San>, from: Position, to: Position) {
    output.reserve(4);
    output.push(San { from, to, promotion: Some(PromotionPiece::Queen) });
    output.push(San { from, to, promotion: Some(PromotionPiece::Rook) });
    output.push(San { from, to, promotion: Some(PromotionPiece::Bishop) });
    output.push(San { from, to, promotion: Some(PromotionPiece::Knight) });
}