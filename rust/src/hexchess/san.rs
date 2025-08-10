use crate::hexchess::position::Position;
use crate::hexchess::promotion_piece::PromotionPiece;
use std::fmt;

/// Struct representing a single move.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct San {
    /// From position
    pub from: Position,

    /// Promotion piece
    pub promotion: Option<PromotionPiece>,

    /// Target position
    pub to: Position,
}

impl San {
    /// Create a new SAN from a string.
    pub fn from_string(source: &str) -> Result<Self, String> {
        let mut chars = source.chars();

        // first file
        let from_file = match chars.next() {
            Some(val) => match is_file(val) {
                true => val,
                false => return Err(format!("invalid from file: {}", val)),
            },
            None => return Err(format!("missing from file: {}", source)),
        };
    
        // get next two chars to determine if from rank is 11
        let second_char = match chars.next() {
            Some(val) => match is_rank(val) {
                true => val,
                false => return Err(format!("invalid second character: {}", val)),
            },
            None => return Err(format!("missing second character: {}", source)),
        };
    
        let third_char = match chars.next() {
            Some(c) => c,
            None => return Err(format!("missing third character: {}", source)),
        };

        // first rank
        let from_rank = match (second_char, third_char) {
            ('1', '0') => String::from("10"),
            ('1', '1') => String::from("11"),
            _ => second_char.to_string(),
        };
    
        let to_file = match from_rank.as_str() {
            "10" | "11" => match chars.next() {
                Some(val) => match is_file(val) {
                  true => val,
                  false => return Err(format!("invalid to file: {}", val)),
                },
                None => return Err(format!("missing from file: {}", source)),
            },
            _ => match is_file(third_char) {
                true => third_char,
                false => return Err(format!("invalid to file: {}", third_char)),
            },
        };
    
        // gather next two chars to determine if to rank is 11
        let to_second_char = match chars.next() {
            Some(val) => match is_rank(val) {
                true => val,
                false => return Err(format!("invalid second to character: {}", val)),
            },
            None => return Err(format!("missing second to character: {}", source)),
        };
    
        let to_third_char = chars.next();
    
        // to rank
        let to_rank = match (to_second_char, to_third_char) {
            ('1', Some('0')) => String::from("10"),
            ('1', Some('1')) => String::from("11"),
            _ => match (is_rank(second_char), to_third_char) {
                (true, Some('b' | 'n' | 'r' | 'q') | None) => to_second_char.to_string(),
                _ => return Err(format!("invalid to rank: {}", source)),
            }
        };

        // assemble and validate from and to positions
        let from_source = from_file.to_string() + &from_rank;
        let from = Position::from_string(&from_source)?;
        
        let to_source = to_file.to_string() + &to_rank;
        let to = Position::from_string(&to_source)?;
        
        if from == to {
            return Err(format!("to and from positions are the same: {}", source));
        }
    
        // parse and validate promotion
        let promotion = match to_third_char {
            Some(val) => match val {
                'b' => Some(PromotionPiece::Bishop),
                'n' => Some(PromotionPiece::Knight),
                'q' => Some(PromotionPiece::Queen),
                'r' => Some(PromotionPiece::Rook),
                _ => match chars.next() {
                    Some(val_2) => match val_2 {
                        'b' => Some(PromotionPiece::Bishop),
                        'n' => Some(PromotionPiece::Knight),
                        'q' => Some(PromotionPiece::Queen),
                        'r' => Some(PromotionPiece::Rook),
                        _ => return Err(format!("invalid promotion character: {}", val_2)),
                    },
                    _ => None,
                }
            },
            None => None
        };
    
        // validate promotion to is valid
        if promotion.is_some() && !to.is_promotion_position() {
            return Err(format!("invalid promotion position: {}", to_source));
        }
    
        // prohibit post-promotion characters
        if chars.next().is_some() {
            return Err(format!("post promotion character: {}", source));
        }
    
        Ok(Self { from, promotion, to })
    }

    /// Create a new SAN from a position.
    pub fn new(from: Position, to: Position) -> Self {
        Self { from, promotion: None, to }
    }
}

impl fmt::Display for San {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.from.to_string(), self.to.to_string())?;
        
        if let Some(promotion) = self.promotion {
            write!(f, "{}", promotion.to_string())?;
        }

        Ok(())
    }
}

/// test if character is a file
fn is_file(c: char) -> bool {
    match c {
        'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'k' | 'l' => true,
        _ => false,
    }
}

/// test if character is a digit
fn is_rank(c: char) -> bool {
    match c {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_san_from_and_to_string() {
        let san1 = San::from_string("a1a2").unwrap();
        assert_eq!(san1.from, Position::A1);
        assert_eq!(san1.to, Position::A2);
        assert_eq!(san1.promotion, None);
        assert_eq!(san1.to_string(), "a1a2");
    }
}