use crate::h;
use crate::hexchess::hexchess::Hexchess;
use crate::hexchess::san::San;

use crate::constants::{
    Color,
    PromotionPiece,
};

use crate::hexchess::utils::{
    get_color,
    step,
};


pub fn pawn_moves_unsafe(
    hexchess: &Hexchess,
    from: u8,
    color: &Color,
) -> Vec<San> {
    let mut result: Vec<San> = vec![];

    let (
      forward_direction, 
      portside_direction, 
      starboard_direction
    ) = match color {
      Color::White => (0u8, 10u8, 2u8),
      Color::Black => (6u8, 4u8, 8u8),
    };

    // advance forward one position
    match advance(hexchess, from, from, forward_direction) {
        None => {},
        Some(san) => {
            push_moves(&mut result, san, *color);

            // advance forward another position if possible
            if is_starting_position(from, *color) {
                match advance(hexchess, from, san.to, forward_direction) {
                    None => {}
                    Some(san) => result.push(san),
                };
            }
        },
    };

    // capture portside
    match capture(hexchess, from, portside_direction, *color) {
        None => {},
        Some(san) => push_moves(&mut result, san, *color),
    };

    // capture starboard
    match capture(hexchess, from, starboard_direction, *color) {
        None => {},
        Some(san) => push_moves(&mut result, san, *color),
    };
        
    result
}

fn advance(hexchess: &Hexchess, start: u8, from: u8, forward_direction: u8) -> Option<San> {
    // we don't need to verify the step exists, because pawns cannot exist
    // on the final rank without promoting. there will always be one more step.
    let to = step(from, forward_direction).unwrap();

    match hexchess.board[to as usize] {
        None => Some(San { from: start, promotion: None, to }),
        Some(_) => None,
    }
}

fn capture(hexchess: &Hexchess, from: u8, capture_direction: u8, friendly_color: Color) -> Option<San> {
    match step(from, capture_direction) {
        None => None,
        Some(to) => match hexchess.board[to as usize] {
            None => match hexchess.ep {
                None => None,
                Some(ep) => match to == ep && hexchess.turn == friendly_color {
                    true => Some(San { from, promotion: None, to }),
                    false => None,
                },
            },
            Some(piece) => match get_color(&piece) != friendly_color {
                true => Some(San { from, promotion: None, to }),
                false => None,
            },
        }
    }
}

fn is_promotion_position(position: u8, color: Color) -> bool {
    match color {
        Color::Black => match position {
            h!("a1") |
            h!("b1") |
            h!("c1") |
            h!("d1") |
            h!("e1") |
            h!("f1") |
            h!("g1") |
            h!("h1") |
            h!("i1") |
            h!("k1") |
            h!("l1") => true,
            _ => false,
        },
        Color::White => match position {
            h!("a6") |
            h!("b7") |
            h!("c8") |
            h!("d9") |
            h!("e10") |
            h!("f11") |
            h!("g10") |
            h!("h9") |
            h!("i8") |
            h!("k7") |
            h!("l6") => true,
            _ => false,
        }
    }
}

fn is_starting_position(position: u8, color: Color) -> bool {
    match color {
        Color::Black => match position {
            h!("b7") |
            h!("c7") |
            h!("d7") |
            h!("e7") |
            h!("f7") |
            h!("g7") |
            h!("h7") |
            h!("i7") |
            h!("k7") => true,
            _ => false,
        },
        Color::White => match position {
            h!("b1") |
            h!("c2") |
            h!("d3") |
            h!("e4") |
            h!("f5") |
            h!("g4") |
            h!("h3") |
            h!("i2") |
            h!("k1") => true,
            _ => false,
        }
    }
}

fn push_moves(
    result: &mut Vec<San>,
    san: San,
    color: Color,
) {
    if is_promotion_position(san.to, color) {
        result.push(San { from: san.from, promotion: Some(PromotionPiece::Bishop), to: san.to });
        result.push(San { from: san.from, promotion: Some(PromotionPiece::Knight), to: san.to });
        result.push(San { from: san.from, promotion: Some(PromotionPiece::Queen), to: san.to });
        result.push(San { from: san.from, promotion: Some(PromotionPiece::Rook), to: san.to });
    } else {
        result.push(san);
    }
}
