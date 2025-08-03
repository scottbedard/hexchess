use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PromotionPiece {
    Bishop,
    Knight,
    Queen,
    Rook,
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
