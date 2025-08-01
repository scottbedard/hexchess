use serde::{Deserialize, Serialize};
use std::fmt;

/// Piece color
#[derive(Clone, Copy, Debug, Eq, Deserialize, Hash, PartialEq, Serialize)]
pub enum Color {
    #[serde(rename(deserialize = "b", serialize = "b"))]
    Black,
    #[serde(rename(deserialize = "w", serialize = "w"))]
    White,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::Black => write!(f, "b"),
            Color::White => write!(f, "w"),
        }
    }
}