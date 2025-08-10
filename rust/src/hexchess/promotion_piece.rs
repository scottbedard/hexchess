use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PromotionPiece {
    Bishop,
    Knight,
    Queen,
    Rook,
}

impl PromotionPiece {
    pub fn from_string(value: &str) -> Result<Self, String> {
        match value {
            "b" => Ok(PromotionPiece::Bishop),
            "n" => Ok(PromotionPiece::Knight),
            "q" => Ok(PromotionPiece::Queen),
            "r" => Ok(PromotionPiece::Rook),
            _ => Err(format!("invalid promotion piece: {}", value)),
        }
    }
}

impl fmt::Display for PromotionPiece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            PromotionPiece::Bishop => 'b',
            PromotionPiece::Knight => 'n',
            PromotionPiece::Queen => 'q',
            PromotionPiece::Rook => 'r',
        };
        write!(f, "{}", printable)
    }
}
