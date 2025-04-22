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

#[cfg(test)]
mod tests {
    use crate::{h, s};
    use super::*;

    #[test]
    fn black_en_passant_portside() {
        let result: Vec<San> = Hexchess::parse("1/3/5/7/4P4/4p6/11/11/11/11/11 b f6 0 1")
            .unwrap()
            .moves_from(h!("e6"));

        assert_eq!(result.len(), 2);
        assert_eq!(result[0], San { from: h!("e6"), promotion: None, to: h!("e5") });
        assert_eq!(result[1], San { from: h!("e6"), promotion: None, to: h!("f6") });
    }
    
    #[test]
    fn black_en_passant_portside_out_of_turn() {
        let result: Vec<San> = Hexchess::parse("1/3/5/7/4P4/4p6/11/11/11/11/11 w f6 0 1")
            .unwrap()
            .moves_from(h!("e6"));

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], San { from: h!("e6"), promotion: None, to: h!("e5") });
        // f6 is out of turn
    }

    #[test]
    fn black_en_passant_starboard() {
        let result: Vec<San> = Hexchess::parse("1/3/5/7/4P4/6p4/11/11/11/11/11 b f6 0 1")
            .unwrap()
            .moves_from(h!("g6"));

        assert_eq!(result.len(), 2);
        assert_eq!(result[0], San { from: h!("g6"), promotion: None, to: h!("g5") });
        assert_eq!(result[1], San { from: h!("g6"), promotion: None, to: h!("f6") });
    }

    #[test]
    fn black_en_passant_starboard_out_of_turn() {
        let result: Vec<San> = Hexchess::parse("1/3/5/7/4P4/6p4/11/11/11/11/11 w f6 0 1")
            .unwrap()
            .moves_from(h!("g6"));

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], San { from: h!("g6"), promotion: None, to: h!("g5") });
        // f6 is out of turn
    }

    #[test]
    fn promote_black_forward() {
        let result = Hexchess::parse("1/3/5/7/9/11/11/11/11/5p5/11 b - 0 1")
            .unwrap()
            .moves_from(h!("f2"));

        assert_eq!(result.len(), 4);
        assert_eq!(result[0], San { from: h!("f2"), promotion: Some(PromotionPiece::Bishop), to: h!("f1") });
        assert_eq!(result[1], San { from: h!("f2"), promotion: Some(PromotionPiece::Knight), to: h!("f1") });
        assert_eq!(result[2], San { from: h!("f2"), promotion: Some(PromotionPiece::Queen), to: h!("f1") });
        assert_eq!(result[3], San { from: h!("f2"), promotion: Some(PromotionPiece::Rook), to: h!("f1") });
    }

    #[test]
    fn promote_black_capture_portside() {
        let result = Hexchess::parse("1/3/5/7/9/11/11/11/11/5p5/4rrK4 w - 0 1")
            .unwrap()
            .moves_from(h!("f2"));

        assert_eq!(result.len(), 4);
        assert_eq!(result[0], San { from: h!("f2"), promotion: Some(PromotionPiece::Bishop), to: h!("g1") });
        assert_eq!(result[1], San { from: h!("f2"), promotion: Some(PromotionPiece::Knight), to: h!("g1") });
        assert_eq!(result[2], San { from: h!("f2"), promotion: Some(PromotionPiece::Queen), to: h!("g1") });
        assert_eq!(result[3], San { from: h!("f2"), promotion: Some(PromotionPiece::Rook), to: h!("g1") });
    }

    #[test]
    fn promote_black_capture_starboard() {
        let result = Hexchess::parse("1/3/5/7/9/11/11/11/11/5p5/4Krr4 w - 0 1")
            .unwrap()
            .moves_from(h!("f2"));

        assert_eq!(result.len(), 4);
        assert_eq!(result[0], San { from: h!("f2"), promotion: Some(PromotionPiece::Bishop), to: h!("e1") });
        assert_eq!(result[1], San { from: h!("f2"), promotion: Some(PromotionPiece::Knight), to: h!("e1") });
        assert_eq!(result[2], San { from: h!("f2"), promotion: Some(PromotionPiece::Queen), to: h!("e1") });
        assert_eq!(result[3], San { from: h!("f2"), promotion: Some(PromotionPiece::Rook), to: h!("e1") });
    }
    
    #[test]
    fn promote_white_forward() {
        let result = Hexchess::parse("1/1P1/5/7/9/11/11/11/11/11/11 w - 0 1")
            .unwrap()
            .moves_from(h!("f10"));

        assert_eq!(result.len(), 4);
        assert_eq!(result[0], San { from: h!("f10"), promotion: Some(PromotionPiece::Bishop), to: h!("f11") });
        assert_eq!(result[1], San { from: h!("f10"), promotion: Some(PromotionPiece::Knight), to: h!("f11") });
        assert_eq!(result[2], San { from: h!("f10"), promotion: Some(PromotionPiece::Queen), to: h!("f11") });
        assert_eq!(result[3], San { from: h!("f10"), promotion: Some(PromotionPiece::Rook), to: h!("f11") });
    }
    
    #[test]
    fn promote_white_capture_portside() {
        let result = Hexchess::parse("R/kPR/5/7/9/11/11/11/11/11/11 w - 0 1")
            .unwrap()
            .moves_from(h!("f10"));

        assert_eq!(result.len(), 4);
        assert_eq!(result[0], San { from: h!("f10"), promotion: Some(PromotionPiece::Bishop), to: h!("e10") });
        assert_eq!(result[1], San { from: h!("f10"), promotion: Some(PromotionPiece::Knight), to: h!("e10") });
        assert_eq!(result[2], San { from: h!("f10"), promotion: Some(PromotionPiece::Queen), to: h!("e10") });
        assert_eq!(result[3], San { from: h!("f10"), promotion: Some(PromotionPiece::Rook), to: h!("e10") });
    }

    #[test]
    fn promote_white_capture_starboard() {
        let result = Hexchess::parse("R/RPk/5/7/9/11/11/11/11/11/11 w - 0 1")
            .unwrap()
            .moves_from(h!("f10"));

        assert_eq!(result.len(), 4);
        assert_eq!(result[0], San { from: h!("f10"), promotion: Some(PromotionPiece::Bishop), to: h!("g10") });
        assert_eq!(result[1], San { from: h!("f10"), promotion: Some(PromotionPiece::Knight), to: h!("g10") });
        assert_eq!(result[2], San { from: h!("f10"), promotion: Some(PromotionPiece::Queen), to: h!("g10") });
        assert_eq!(result[3], San { from: h!("f10"), promotion: Some(PromotionPiece::Rook), to: h!("g10") });
    }
}
