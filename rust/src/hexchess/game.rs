use crate::hexchess::bitboard::Bitboard;
use crate::Color;

pub struct Game {
    pub bitboard_black_bishop: Bitboard,
    pub bitboard_black_king: Bitboard,
    pub bitboard_black_knight: Bitboard,
    pub bitboard_black_pawn: Bitboard,
    pub bitboard_black_queen: Bitboard,
    pub bitboard_black_rook: Bitboard,
    pub bitboard_white_bishop: Bitboard,
    pub bitboard_white_king: Bitboard,
    pub bitboard_white_knight: Bitboard,
    pub bitboard_white_pawn: Bitboard,
    pub bitboard_white_queen: Bitboard,
    pub bitboard_white_rook: Bitboard,
    pub ep: Option<u8>,
    pub fullmove: u16,
    pub halfmove: u8,
    pub turn: Color,
}

impl Game {
    /// Create a new game instance with no pieces.
    pub fn new() -> Self {
        Self {
            bitboard_black_bishop: Bitboard::new(),
            bitboard_black_king: Bitboard::new(),
            bitboard_black_knight: Bitboard::new(),
            bitboard_black_pawn: Bitboard::new(),
            bitboard_black_queen: Bitboard::new(),
            bitboard_black_rook: Bitboard::new(),
            bitboard_white_bishop: Bitboard::new(),
            bitboard_white_king: Bitboard::new(),
            bitboard_white_knight: Bitboard::new(),
            bitboard_white_pawn: Bitboard::new(),
            bitboard_white_queen: Bitboard::new(),
            bitboard_white_rook: Bitboard::new(),
            ep: None,
            fullmove: 1,
            halfmove: 0,
            turn: Color::White,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let game = Game::new();
        assert_eq!(game.bitboard_black_bishop.0, 0);
        assert_eq!(game.bitboard_black_king.0, 0);
        assert_eq!(game.bitboard_black_knight.0, 0);
        assert_eq!(game.bitboard_black_pawn.0, 0);
        assert_eq!(game.bitboard_black_queen.0, 0);
        assert_eq!(game.bitboard_black_rook.0, 0);
        assert_eq!(game.bitboard_white_bishop.0, 0);
        assert_eq!(game.bitboard_white_king.0, 0);
        assert_eq!(game.bitboard_white_knight.0, 0);
        assert_eq!(game.bitboard_white_pawn.0, 0);
        assert_eq!(game.bitboard_white_queen.0, 0);
        assert_eq!(game.bitboard_white_rook.0, 0);
        assert_eq!(game.ep, None);
        assert_eq!(game.fullmove, 1);
        assert_eq!(game.halfmove, 0);
        assert_eq!(game.turn, Color::White);
    }
}
