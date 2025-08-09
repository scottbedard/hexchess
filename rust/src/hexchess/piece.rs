use crate::hexchess::color::Color;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Piece symbols
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Piece {
    #[serde(rename(deserialize = "P", serialize = "P"))]
    WhitePawn,

    #[serde(rename(deserialize = "N", serialize = "N"))]
    WhiteKnight,

    #[serde(rename(deserialize = "B", serialize = "B"))]
    WhiteBishop,

    #[serde(rename(deserialize = "R", serialize = "R"))]
    WhiteRook,

    #[serde(rename(deserialize = "Q", serialize = "Q"))]
    WhiteQueen,

    #[serde(rename(deserialize = "K", serialize = "K"))]
    WhiteKing,

    #[serde(rename(deserialize = "p", serialize = "p"))]
    BlackPawn,

    #[serde(rename(deserialize = "n", serialize = "n"))]
    BlackKnight,

    #[serde(rename(deserialize = "b", serialize = "b"))]
    BlackBishop,

    #[serde(rename(deserialize = "r", serialize = "r"))]
    BlackRook,

    #[serde(rename(deserialize = "q", serialize = "q"))]
    BlackQueen,

    #[serde(rename(deserialize = "k", serialize = "k"))]
    BlackKing,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Piece::BlackBishop => 'b',
            Piece::BlackKing => 'k',
            Piece::BlackKnight => 'n',
            Piece::BlackPawn => 'p',
            Piece::BlackQueen => 'q',
            Piece::BlackRook => 'r',
            Piece::WhiteBishop => 'B',
            Piece::WhiteKing => 'K',
            Piece::WhiteKnight => 'N',
            Piece::WhitePawn => 'P',
            Piece::WhiteQueen => 'Q',
            Piece::WhiteRook => 'R',
        };
        write!(f, "{}", printable)
    }
}

impl From<char> for Piece {
    fn from(c: char) -> Self {
        match c {
            'b' => Piece::BlackBishop,
            'k' => Piece::BlackKing,
            'n' => Piece::BlackKnight,
            'p' => Piece::BlackPawn,
            'q' => Piece::BlackQueen,
            'r' => Piece::BlackRook,
            'B' => Piece::WhiteBishop,
            'K' => Piece::WhiteKing,
            'N' => Piece::WhiteKnight,
            'P' => Piece::WhitePawn,
            'Q' => Piece::WhiteQueen,
            'R' => Piece::WhiteRook,
            _ => panic!("Invalid piece character: {}", c),
        }
    }
}

impl Piece {
    /// Check if a piece is of a given color.
    pub fn is_friendly(&self, color: Color) -> bool {
        self.color() == color
    }

    /// Check if a piece is an enemy of a given color.
    pub fn is_enemy(&self, color: Color) -> bool {
        self.color() != color
    }

    /// Get the color of a piece.
    pub fn color(&self) -> Color {
        match self {
            Piece::BlackBishop => Color::Black,
            Piece::BlackKing => Color::Black,
            Piece::BlackKnight => Color::Black,
            Piece::BlackPawn => Color::Black,
            Piece::BlackQueen => Color::Black,
            Piece::BlackRook => Color::Black,
            Piece::WhiteBishop => Color::White,
            Piece::WhiteKing => Color::White,
            Piece::WhiteKnight => Color::White,
            Piece::WhitePawn => Color::White,
            Piece::WhiteQueen => Color::White,
            Piece::WhiteRook => Color::White,
        }
    }

    /// Create a piece from a string.
    pub fn from_string(s: &str) -> Result<Self, String> {
        match s {
            "b" => Ok(Piece::BlackBishop),
            "k" => Ok(Piece::BlackKing),
            "n" => Ok(Piece::BlackKnight),
            "p" => Ok(Piece::BlackPawn),
            "q" => Ok(Piece::BlackQueen),
            "r" => Ok(Piece::BlackRook),
            "B" => Ok(Piece::WhiteBishop),
            "K" => Ok(Piece::WhiteKing),
            "N" => Ok(Piece::WhiteKnight),
            "P" => Ok(Piece::WhitePawn),
            "Q" => Ok(Piece::WhiteQueen),
            "R" => Ok(Piece::WhiteRook),
            _ => Err(format!("invalid piece: {}", s)),
        }
    }

    /// Convert a piece to a character.
    pub fn to_char(&self) -> char {
        match self {
            Piece::BlackBishop => 'b',
            Piece::BlackKing => 'k',
            Piece::BlackKnight => 'n',
            Piece::BlackPawn => 'p',
            Piece::BlackQueen => 'q',
            Piece::BlackRook => 'r',
            Piece::WhiteBishop => 'B',
            Piece::WhiteKing => 'K',
            Piece::WhiteKnight => 'N',
            Piece::WhitePawn => 'P',
            Piece::WhiteQueen => 'Q',
            Piece::WhiteRook => 'R',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece_display() {
        assert_eq!(format!("{}", Piece::BlackBishop), "b");
        assert_eq!(format!("{}", Piece::BlackKing), "k");
        assert_eq!(format!("{}", Piece::BlackKnight), "n");
        assert_eq!(format!("{}", Piece::BlackPawn), "p");
        assert_eq!(format!("{}", Piece::BlackQueen), "q");
        assert_eq!(format!("{}", Piece::BlackRook), "r");
        assert_eq!(format!("{}", Piece::WhiteBishop), "B");
        assert_eq!(format!("{}", Piece::WhiteKing), "K");
        assert_eq!(format!("{}", Piece::WhiteKnight), "N");
        assert_eq!(format!("{}", Piece::WhitePawn), "P");
        assert_eq!(format!("{}", Piece::WhiteQueen), "Q");
        assert_eq!(format!("{}", Piece::WhiteRook), "R");
    }

    #[test]
    fn test_piece_is_color() {
        assert!(Piece::BlackBishop.is_friendly(Color::Black));
        assert!(Piece::BlackKing.is_friendly(Color::Black));
        assert!(Piece::BlackKnight.is_friendly(Color::Black));
        assert!(Piece::BlackPawn.is_friendly(Color::Black));
        assert!(Piece::BlackQueen.is_friendly(Color::Black));
        assert!(Piece::BlackRook.is_friendly(Color::Black));
        assert!(Piece::WhiteBishop.is_friendly(Color::White));
        assert!(Piece::WhiteKing.is_friendly(Color::White));
        assert!(Piece::WhiteKnight.is_friendly(Color::White));
        assert!(Piece::WhitePawn.is_friendly(Color::White));
        assert!(Piece::WhiteQueen.is_friendly(Color::White));
        assert!(Piece::WhiteRook.is_friendly(Color::White));
    }

    #[test]
    fn test_piece_color() {
        assert_eq!(Piece::BlackBishop.color(), Color::Black);
        assert_eq!(Piece::BlackKing.color(), Color::Black);
        assert_eq!(Piece::BlackKnight.color(), Color::Black);
        assert_eq!(Piece::BlackPawn.color(), Color::Black);
        assert_eq!(Piece::BlackQueen.color(), Color::Black);
        assert_eq!(Piece::BlackRook.color(), Color::Black);
        assert_eq!(Piece::WhiteBishop.color(), Color::White);
        assert_eq!(Piece::WhiteKing.color(), Color::White);
        assert_eq!(Piece::WhiteKnight.color(), Color::White);
        assert_eq!(Piece::WhitePawn.color(), Color::White);
        assert_eq!(Piece::WhiteQueen.color(), Color::White);
        assert_eq!(Piece::WhiteRook.color(), Color::White);
    }
}