use serde::{Deserialize, Serialize};
use std::fmt;

/// Piece color
#[derive(Clone, Copy, Debug, Eq, Deserialize, Hash, PartialEq, Serialize)]
pub enum Color {
    Black,

    White,
}

impl Color {
    pub fn from_string(string: &str) -> Self {
        match string {
            "b" => Color::Black,
            "w" => Color::White,
            _ => panic!("invalid color: {}", string),
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::Black => write!(f, "b"),
            Color::White => write!(f, "w"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opposite() {
        assert_eq!(Color::Black.opposite(), Color::White);
        assert_eq!(Color::White.opposite(), Color::Black);
    }
}