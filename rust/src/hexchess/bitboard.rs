use crate::constants::Piece;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
struct Bitboard {
    pub black_bishop: u128,
    pub black_king: u128,
    pub black_knight: u128,
    pub black_pawn: u128,
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
        assert_eq!(bitboard.black_bishop, 0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001);

        bitboard.set(Piece::BlackKing, 1);
        assert_eq!(bitboard.black_king, 0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010);

        bitboard.set(Piece::BlackKnight, 2);
        assert_eq!(bitboard.black_knight, 0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100);

        bitboard.set(Piece::BlackPawn, 3);
        assert_eq!(bitboard.black_pawn, 0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000);

        bitboard.set(Piece::BlackQueen, 4);
        assert_eq!(bitboard.black_queen, 0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010000);

        bitboard.set(Piece::BlackRook, 5);
        assert_eq!(bitboard.black_rook, 0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000);

        bitboard.set(Piece::WhiteBishop, 6);
        assert_eq!(bitboard.white_bishop, 0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000000);

        bitboard.set(Piece::WhiteKing, 7);
        assert_eq!(bitboard.white_king, 0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010000000);

        bitboard.set(Piece::WhiteKnight, 8);
        assert_eq!(bitboard.white_knight, 0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000000);

        bitboard.set(Piece::WhitePawn, 9);
        assert_eq!(bitboard.white_pawn, 0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000000000);

        bitboard.set(Piece::WhiteQueen, 10);
        assert_eq!(bitboard.white_queen, 0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010000000000);

        bitboard.set(Piece::WhiteRook, 11);
        assert_eq!(bitboard.white_rook, 0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000000000);
    
        // bitwise methods
        assert_eq!(bitboard.all(), 0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000111111111111);
        assert_eq!(bitboard.white(), 0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000111111000000);
        assert_eq!(bitboard.black(), 0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000111111);
    }
}
