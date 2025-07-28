use crate::constants::{Color, Piece};
use crate::hexchess::hexchess::stringify_board;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
struct Bitboard {
    pub black_bishop: u128, // turn color embedded at first byte, 0 = white, 1 = black
    pub black_king: u128, // en passant index at first u8
    pub black_knight: u128, // halfmove clock embedded at first u16
    pub black_pawn: u128, // fullmove number embedded at first u16
    pub black_queen: u128,
    pub black_rook: u128,
    pub white_bishop: u128,
    pub white_king: u128,
    pub white_knight: u128,
    pub white_pawn: u128,
    pub white_queen: u128,
    pub white_rook: u128,
}


impl Bitboard {

    /// all occupied positions
    pub fn all(&self) -> u128 {
        self.black_bishop |
        self.black_king |
        self.black_knight |
        self.black_pawn |
        self.black_queen |
        self.black_rook |
        self.white_bishop |
        self.white_king |
        self.white_knight |
        self.white_pawn |
        self.white_queen |
        self.white_rook
    }

    /// black occupied positions
    pub fn black(&self) -> u128 {
        self.black_bishop |
        self.black_king |
        self.black_knight |
        self.black_pawn |
        self.black_queen |
        self.black_rook
    }

    /// get the current turn color
    pub fn color(&self) -> Color {
        match (self.black_bishop & 0xFF) as u8 {
            0 => Color::Black,
            _ => Color::White,
        }
    }

    /// get piece at a given index
    pub fn get(&self, index: usize) -> Option<Piece> {
        if self.black_bishop & 1u128 << index != 0 {
            return Some(Piece::BlackBishop);
        }
        if self.black_king & 1u128 << index != 0 {
            return Some(Piece::BlackKing);
        }
        if self.black_knight & 1u128 << index != 0 {
            return Some(Piece::BlackKnight);
        }
        if self.black_pawn & 1u128 << index != 0 {
            return Some(Piece::BlackPawn);
        }
        if self.black_queen & 1u128 << index != 0 {
            return Some(Piece::BlackQueen);
        }
        if self.black_rook & 1u128 << index != 0 {
            return Some(Piece::BlackRook);
        }
        if self.white_bishop & 1u128 << index != 0 {
            return Some(Piece::WhiteBishop);
        }
        if self.white_king & 1u128 << index != 0 {
            return Some(Piece::WhiteKing);
        }
        if self.white_knight & 1u128 << index != 0 {
            return Some(Piece::WhiteKnight);
        }
        if self.white_pawn & 1u128 << index != 0 {
            return Some(Piece::WhitePawn);
        }
        if self.white_queen & 1u128 << index != 0 {
            return Some(Piece::WhiteQueen);
        }
        if self.white_rook & 1u128 << index != 0 {
            return Some(Piece::WhiteRook);
        }
        None
    }

    /// starting position
    pub fn init() -> Self {
        Self {
            black_bishop: 0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000101,
            black_king: 0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000,
            black_knight: 0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100010000,
            black_pawn: 0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001111111110000000000000000,
            black_queen: 0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010,
            black_rook: 0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000001000000000,
            white_bishop: 0b00000000000000000000000000000000000000000010000000000100000000001000000000000000000000000000000000000000000000000000000000000000,
            white_king: 0b00000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
            white_knight: 0b00000000000000000000000000000000000000001000100000000000000000000000000000000000000000000000000000000000000000000000000000000000,
            white_pawn: 0b00000000000000000000000000000000000000100000001000100000100000100010000000101000000000100000000000000000000000000000000000000000,
            white_queen: 0b00000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
            white_rook: 0b00000000000000000000000000000000000000010000010000000000000000000000000000000000000000000000000000000000000000000000000000000000,
        }
    }

    /// empty bitboard
    pub fn new() -> Self {
        Self {
            black_bishop: 0,
            black_king: 0,
            black_knight: 0,
            black_pawn: 0,
            black_queen: 0,
            black_rook: 0,
            white_bishop: 0,
            white_king: 0,
            white_knight: 0,
            white_pawn: 0,
            white_queen: 0,
            white_rook: 0,
        }
    }

    /// set a piece at a given index
    fn set(&mut self, piece: Piece, index: usize) {
        match piece {
            Piece::BlackBishop => self.black_bishop |= 1 << index,
            Piece::BlackKing => self.black_king |= 1 << index,
            Piece::BlackKnight => self.black_knight |= 1 << index,
            Piece::BlackPawn => self.black_pawn |= 1 << index,
            Piece::BlackQueen => self.black_queen |= 1 << index,
            Piece::BlackRook => self.black_rook |= 1 << index,
            Piece::WhiteBishop => self.white_bishop |= 1 << index,
            Piece::WhiteKing => self.white_king |= 1 << index,
            Piece::WhiteKnight => self.white_knight |= 1 << index,
            Piece::WhitePawn => self.white_pawn |= 1 << index,
            Piece::WhiteQueen => self.white_queen |= 1 << index,
            Piece::WhiteRook => self.white_rook |= 1 << index,
        };
    }

    /// set the current turn color
    pub fn set_color(&mut self, color: Color) {
        let byte: u8 = match color {
            Color::Black => 0,
            Color::White => 1,
        };
    
        self.black_bishop = (self.black_bishop & !0xFF) | (byte as u128);
    }

    /// format as fen string
    pub fn to_string(&self) -> String {
        let mut board: [Option<Piece>; 91] = [
            None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None,
            None, None, None,
        ];

        for i in 0..91 {
            match self.get(i) {
                Some(Piece::BlackBishop) => board[i] = Some(Piece::BlackBishop),
                Some(Piece::BlackKing) => board[i] = Some(Piece::BlackKing),
                Some(Piece::BlackKnight) => board[i] = Some(Piece::BlackKnight),
                Some(Piece::BlackPawn) => board[i] = Some(Piece::BlackPawn),
                Some(Piece::BlackQueen) => board[i] = Some(Piece::BlackQueen),
                Some(Piece::BlackRook) => board[i] = Some(Piece::BlackRook),
                Some(Piece::WhiteBishop) => board[i] = Some(Piece::WhiteBishop),
                Some(Piece::WhiteKing) => board[i] = Some(Piece::WhiteKing),
                Some(Piece::WhiteKnight) => board[i] = Some(Piece::WhiteKnight),
                Some(Piece::WhitePawn) => board[i] = Some(Piece::WhitePawn),
                Some(Piece::WhiteQueen) => board[i] = Some(Piece::WhiteQueen),
                Some(Piece::WhiteRook) => board[i] = Some(Piece::WhiteRook),
                _ => {}
            }
        }

        stringify_board(&board)
    }

    /// white occupied positions
    pub fn white(&self) -> u128 {
        self.white_bishop |
        self.white_king |
        self.white_knight |
        self.white_pawn |
        self.white_queen |
        self.white_rook
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_arbitrary_values() {
        let mut bitboard = Bitboard::new();

        bitboard.set(Piece::BlackBishop, 0);
        assert_eq!(bitboard.get(0), Some(Piece::BlackBishop));
        assert_eq!(bitboard.black_bishop, 0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001);

        bitboard.set(Piece::BlackKing, 1);
        assert_eq!(bitboard.get(1), Some(Piece::BlackKing));
        assert_eq!(bitboard.black_king, 0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010);

        bitboard.set(Piece::BlackKnight, 2);
        assert_eq!(bitboard.get(2), Some(Piece::BlackKnight));
        assert_eq!(bitboard.black_knight, 0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100);

        bitboard.set(Piece::BlackPawn, 3);
        assert_eq!(bitboard.get(3), Some(Piece::BlackPawn));
        assert_eq!(bitboard.black_pawn, 0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000);

        bitboard.set(Piece::BlackQueen, 4);
        assert_eq!(bitboard.get(4), Some(Piece::BlackQueen));
        assert_eq!(bitboard.black_queen, 0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010000);

        bitboard.set(Piece::BlackRook, 5);
        assert_eq!(bitboard.get(5), Some(Piece::BlackRook));
        assert_eq!(bitboard.black_rook, 0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000);

        bitboard.set(Piece::WhiteBishop, 6);
        assert_eq!(bitboard.get(6), Some(Piece::WhiteBishop));
        assert_eq!(bitboard.white_bishop, 0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000000);

        bitboard.set(Piece::WhiteKing, 7);
        assert_eq!(bitboard.get(7), Some(Piece::WhiteKing));
        assert_eq!(bitboard.white_king, 0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010000000);

        bitboard.set(Piece::WhiteKnight, 8);
        assert_eq!(bitboard.get(8), Some(Piece::WhiteKnight));
        assert_eq!(bitboard.white_knight, 0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000000);

        bitboard.set(Piece::WhitePawn, 9);
        assert_eq!(bitboard.get(9), Some(Piece::WhitePawn));
        assert_eq!(bitboard.white_pawn, 0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000000000);

        bitboard.set(Piece::WhiteQueen, 10);
        assert_eq!(bitboard.get(10), Some(Piece::WhiteQueen));
        assert_eq!(bitboard.white_queen, 0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010000000000);

        bitboard.set(Piece::WhiteRook, 11);
        assert_eq!(bitboard.get(11), Some(Piece::WhiteRook));
        assert_eq!(bitboard.white_rook, 0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000000000);
    
        // bitwise methods
        assert_eq!(bitboard.all(), 0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000111111111111);
        assert_eq!(bitboard.white(), 0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000111111000000);
        assert_eq!(bitboard.black(), 0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000111111);
    }

    #[test]
    fn test_embedded_color() {
        let mut bitboard = Bitboard::init();
        assert_eq!(bitboard.color(), Color::White);

        bitboard.set_color(Color::Black);
        assert_eq!(bitboard.color(), Color::Black);

        bitboard.set_color(Color::White);
        assert_eq!(bitboard.color(), Color::White);

        bitboard.set_color(Color::Black);
        assert_eq!(bitboard.color(), Color::Black);
    }

    #[test]
    fn test_to_string() {
        let bitboard = Bitboard::init();
        
        assert_eq!(bitboard.to_string(), "b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1");
    }
}
