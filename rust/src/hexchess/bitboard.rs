use crate::constants::{Color, Piece};
use crate::hexchess::hexchess::stringify_board;
use serde::{Deserialize, Serialize};

/*
https://www.redblobgames.com/grids/hexagons/#map-storage

overview:
                     f11                    
                 e10 f10 g10                
              d9  e9  f9  g9  h9            
          c8  d8  e8  f8  g8  h8  i8        
      b7  c7  d7  e7  f7  g7  h7  i7  k7    
  a6  b6  c6  d6  e6  f6  g6  h6  i6  k6  l6
  a5  b5  c5  d5  e5  f5  g5  h5  i5  k5  l5
  a4  b4  c4  d4  e4  f4  g4  h4  i4  k4  l4
  a3  b3  c3  d3  e3  f3  g3  h3  i3  k3  l3
  a2  b2  c2  d2  e2  f2  g2  h2  i2  k2  l2
  a1  b1  c1  d1  e1  f1  g1  h1  i1  k1  l1
*/

/// 11x11 bitboard mask, 0 = empty, 1 = off board
const BOARD_MASK: u128 = 0b1111101111111110001111111000001111100000001110000000001000000000000000000000000000000000000000000000000000000000000000000;

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

    /// get the en passant index, embedded in black king
    pub fn en_passant(&self) -> u8 {
        (self.black_bishop & 0xFF) as u8
    }

    /// get the fullmove number, embedded in black pawn
    pub fn fullmove(&self) -> u16 {
        (self.black_pawn >> 112) as u16
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

    /// get the halfmove clock, embedded in black knight
    pub fn halfmove(&self) -> u16 {
        (self.black_knight >> 112) as u16
    }

    /// increment the fullmove clock, embedded in black pawn
    pub fn inc_fullmove(&mut self) {
        self.set_fullmove(self.fullmove().wrapping_add(1));
    }

    /// increment the halfmove clock, embedded in black knight
    pub fn inc_halfmove(&mut self) {
        self.set_halfmove(self.halfmove().wrapping_add(1));
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
    pub fn set(&mut self, piece: Piece, index: usize) {
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

    /// set the current turn color, embedded in black bishop
    pub fn set_color(&mut self, color: Color) {
        let byte: u8 = match color {
            Color::Black => 0,
            Color::White => 1,
        };
    
        self.black_bishop = (self.black_bishop & !0xFF) | (byte as u128);
    }

    /// set the en passant index, embedded in black king
    pub fn set_en_passant(&mut self, index: u8) {
        self.black_king = (self.black_king & !0xFF) | (index as u128);
    }

    /// set the halfmove clock, embedded in black knight
    pub fn set_halfmove(&mut self, value: u16) {
        let mask: u128 = 0xFFFF << 112;
        self.black_knight = (self.black_knight & !mask) | ((value as u128) << 112);
    }

    /// set the fullmove clock, embedded in black pawn
    pub fn set_fullmove(&mut self, value: u16) {
        let mask: u128 = 0xFFFF << 112;
        self.black_pawn = (self.black_pawn & !mask) | ((value as u128) << 112);
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
    fn test_arbitrary_positions() {
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
    fn test_embedded_halfmove() {
        let mut bitboard = Bitboard::init();
        assert_eq!(bitboard.halfmove(), 0);

        bitboard.set_halfmove(1);
        assert_eq!(bitboard.halfmove(), 1);

        bitboard.inc_halfmove();
        assert_eq!(bitboard.halfmove(), 2);

        bitboard.set_halfmove(1000);
        assert_eq!(bitboard.halfmove(), 1000);

        bitboard.set_halfmove(u16::MAX);
        assert_eq!(bitboard.halfmove(), u16::MAX);
    }

    #[test]
    fn test_embedded_fullmove() {
        let mut bitboard = Bitboard::init();
        assert_eq!(bitboard.fullmove(), 0);

        bitboard.set_fullmove(1);
        assert_eq!(bitboard.fullmove(), 1);

        bitboard.inc_fullmove();
        assert_eq!(bitboard.fullmove(), 2);

        bitboard.set_fullmove(1000);
        assert_eq!(bitboard.fullmove(), 1000);

        bitboard.set_fullmove(u16::MAX);
        assert_eq!(bitboard.fullmove(), u16::MAX);
    }

    #[test]
    fn test_embedded_en_passant() {
        let mut bitboard = Bitboard::init();
        panic!("bitboard: {:?}", bitboard.en_passant());
        assert_eq!(bitboard.en_passant(), 0);
    }

    #[test]
    fn test_to_string() {
        let bitboard = Bitboard::init();
        
        assert_eq!(bitboard.to_string(), "b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1");
    }
}
