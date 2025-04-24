use crate::constants::PromotionPiece;
use serde::{Deserialize, Serialize};
use std::fmt;
use tsify_next::Tsify;

use crate::hexchess::utils::{
    is_promotion_position,
    index,
};

use super::utils::position;

/// Struct representing a single move.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi, type_suffix = "Struct")]
pub struct San {
    /// From position index, 0..91
    pub from: u8,

    /// Promotion piece
    #[tsify(type = "PromotionPiece | null")]
    pub promotion: Option<PromotionPiece>,

    /// Target position index, 0..91
    pub to: u8,
}

impl San {
    pub fn from(source: &str) -> Result<Self, String> {
        let mut chars = source.chars();

        // first file
        let from_file = match chars.next() {
            Some(val) => match is_file(val) {
                true => val,
                false => return Err(format!("invalid from file: {}", val)),
            },
            None => return Err("missing from file".to_string()),
        };
    
        // get next two chars to determine if from rank is 11
        let second_char = match chars.next() {
            Some(val) => match is_rank(val) {
                true => val,
                false => return Err(format!("invalid second character: {}", val)),
            },
            None => return Err("missing second character".to_string()),
        };
    
        let third_char = match chars.next() {
            Some(c) => c,
            None => return Err("missing third character".to_string()),
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
                None => return Err("missing from file".to_string()),
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
            None => return Err("missing second to character".to_string()),
        };
    
        let to_third_char = chars.next();
    
        // to rank
        let to_rank = match (to_second_char, to_third_char) {
            ('1', Some('0')) => String::from("10"),
            ('1', Some('1')) => String::from("11"),
            _ => match (is_rank(second_char), to_third_char) {
                (true, Some('b' | 'n' | 'r' | 'q') | None) => to_second_char.to_string(),
                _ => return Err("invalid to rank".to_string()),
            }
        };

        // assemble and validate from and to positions
        let from_source = from_file.to_string() + &from_rank;

        let from = match index(&from_source) {
            Ok(value) => value,
            Err(_) => return Err(format!("invalid from position: {}", from_source)),
        };

        let to_source = to_file.to_string() + &to_rank;

        let to = match index(&to_source) {
            Ok(value) => value,
            Err(_) => return Err(format!("invalid to position: {}", to_source)),
        };
        
        if from == to {
            return Err("to and from positions are the same".to_string());
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
        if promotion.is_some() && !is_promotion_position(&to) {
            return Err(format!("invalid promotion position: {}", to_source));
        }
    
        // prohibit post-promotion characters
        if chars.next().is_some() {
            return Err("post promotion character".to_string());
        }
    
        Ok(Self { from, promotion, to })
    }
}

impl fmt::Display for San {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut value = position(&self.from).to_string() + &position(&self.to).to_string();

        match self.promotion {
            Some(promotion) => {
                value.push(match promotion {
                    PromotionPiece::Bishop => 'b',
                    PromotionPiece::Knight => 'n',
                    PromotionPiece::Queen => 'q',
                    PromotionPiece::Rook => 'r',
                });
            }
            None => {}
        };

        write!(f, "{}", value)
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
