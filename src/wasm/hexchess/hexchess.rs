use crate::h;
use crate::hexchess::pieces::king::king_moves_unsafe;
use crate::hexchess::pieces::knight::knight_moves_unsafe;
use crate::hexchess::pieces::pawn::pawn_moves_unsafe;
use crate::hexchess::pieces::straight_line::straight_line_moves_unsafe;
use crate::hexchess::san::San;
use serde_with::serde_as;
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

use crate::constants::{
    Color,
    INITIAL_POSITION,
    Piece,
    PromotionPiece,
};

use crate::hexchess::utils::{
    get_color,
    is_legal_en_passant,
    step,
    index,
    position,
};

/// Hexchess game state
#[serde_as]
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi, type_suffix = "Struct")]
pub struct Hexchess {
    #[tsify(type = "Board")]
    #[serde_as(as = "[_; 91]")]
    pub board: [Option<Piece>; 91],

    pub ep: Option<u8>,

    pub fullmove: u16,

    pub halfmove: u8,

    #[tsify(type = "Color")]
    pub turn: Color,
}

impl Hexchess {
    /// apply a whitespace separated sequence of moves
    pub fn apply(&mut self, sequence: &str) -> Result<Self, String> {
        let mut clone = self.clone();
        let mut i: u32 = 0;

        for part in sequence.split_whitespace() {
            let san = match San::from(&part.to_string()) {
                Ok(san) => san,
                Err(_) => {
                    return Err(format!("invalid san at index {}: {}", i, part));
                },
            };

            if clone.apply_move(&san).is_err() {
                return Err(format!("illegal move at index {}: {}", i, part));
            }

            i += 1;
        }

        self.board = clone.board;
        self.turn = clone.turn;
        self.ep = clone.ep;
        self.fullmove = clone.fullmove;
        self.halfmove = clone.halfmove;

        Ok(*self)
    }

    /// apply legal move
    pub fn apply_move(&mut self, san: &San) -> Result<(), String> {
        if !self.is_legal(san) {
            return Err(format!("illegal move: {:?}", san));
        }

        self.apply_move_unsafe(san);

        Ok(())
    }

    /// apply move, regardless of turn or legality
    pub fn apply_move_unsafe(&mut self, san: &San) -> &Self {
        let piece = match self.board[san.from as usize] {
            Some(piece) => piece,
            None => panic!("cannot apply move from empty position: {}", san.from),
        };

        // update halfmove
        if self.board[san.to as usize].is_some() || (
            piece == Piece::BlackPawn ||
            piece == Piece::WhitePawn
        ) {
            self.halfmove = 0;
        } else {
            self.halfmove += 1;
        }

        let color = get_color(&piece);

        // update fullmove and turn color
        if color == Color::Black {
            self.fullmove += 1;
            self.turn = Color::White;
        } else {
            self.turn = Color::Black;
        }

        // set from positions
        self.board[san.from as usize] = None;

        // set to position
        self.board[san.to as usize] = Some(
            match san.promotion {
                None => piece,
                Some(piece) => match color {
                    Color::Black => match piece {
                        PromotionPiece::Bishop => Piece::BlackBishop,
                        PromotionPiece::Knight => Piece::BlackKnight,
                        PromotionPiece::Queen => Piece::BlackQueen,
                        PromotionPiece::Rook => Piece::BlackRook,
                    },
                    Color::White => match piece {
                        PromotionPiece::Bishop => Piece::WhiteBishop,
                        PromotionPiece::Knight => Piece::WhiteKnight,
                        PromotionPiece::Queen => Piece::WhiteQueen,
                        PromotionPiece::Rook => Piece::WhiteRook,
                    },
                },
            }
        );

        // clear captured en passant
        if Some(san.to) == self.ep {
            let captured = match piece {
                Piece::BlackPawn => step(san.to, 0),
                Piece::WhitePawn => step(san.to, 6),
                _ => None,
            };

            match captured {
                Some(position) => self.board[position as usize] = None,
                None => {},
            };
        }

        // set en passsant
        self.ep = match piece {
            Piece::BlackPawn => match (san.from, san.to) {
                (h!("c7"), h!("c5")) => Some(h!("c6")),
                (h!("d7"), h!("d5")) => Some(h!("d6")),
                (h!("e7"), h!("e5")) => Some(h!("e6")),
                (h!("f7"), h!("f5")) => Some(h!("f6")),
                (h!("g7"), h!("g5")) => Some(h!("g6")),
                (h!("h7"), h!("h5")) => Some(h!("h6")),
                (h!("i7"), h!("i5")) => Some(h!("i6")),
                (h!("k7"), h!("k5")) => Some(h!("k6")),
                _ => None,
            },
            Piece::WhitePawn => match (san.from, san.to) {
                (h!("c2"), h!("c4")) => Some(h!("c3")),
                (h!("d3"), h!("d5")) => Some(h!("d4")),
                (h!("e4"), h!("e6")) => Some(h!("e5")),
                (h!("f5"), h!("f7")) => Some(h!("f6")),
                (h!("g4"), h!("g6")) => Some(h!("g5")),
                (h!("h3"), h!("h5")) => Some(h!("h4")),
                (h!("i2"), h!("i4")) => Some(h!("i3")),
                (h!("k1"), h!("k3")) => Some(h!("k2")),
                _ => None,
            },
            _ => None,
        };

        self
    }

    /// get legal moves for current turn
    pub fn current_moves(&self) -> Vec<San> {
        let mut result: Vec<San> = vec![];

        for n in self.get_color(self.turn) {
            result.extend(self.moves_from(n));
        }

        result
    }

    /// get piece at position
    pub fn get(&self, position: &str) -> Option<Piece> {
        match index(position) {
            Ok(index) => self.board[index as usize],
            Err(_) => None,
        }
    }

    /// get positions occupied by a color
    pub fn get_color(&self, color: Color) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];

        for (index, piece) in self.board.iter().enumerate() {
            match piece {
                Some(piece) => match get_color(piece) == color {
                    true => result.push(index as u8),
                    false => continue,
                },
                None => continue,
            };
        }

        result
    }

    /// get legal moves a position
    pub fn moves_from(&self, from: u8) -> Vec<San> {
        let piece = match self.board[from as usize] {
            Some(piece) => piece,
            None => return vec![],
        };

        let color = get_color(&piece);

        self.moves_from_unsafe(from)
            .into_iter()
            .filter(|san| {
                // prevent self check
                let mut clone = self.clone();

                clone.apply_move_unsafe(san);
                
                match clone.find_king(color) {
                    Some(king) => !clone.is_threatened(king),
                    None => true,
                }
            })
            .collect()
    }

    /// get moves from a position, regardless of turn or legality
    pub fn moves_from_unsafe(&self, from: u8) -> Vec<San> {
        let mut result: Vec<San> = vec![];

        let piece = match self.board[from as usize] {
            Some(piece) => piece,
            None => return result,
        };
        
        let color = get_color(&piece);

        result.extend(match piece {
            Piece::BlackKing | Piece::WhiteKing => {
                king_moves_unsafe(&self, from, &color)
            },
            Piece::BlackKnight | Piece::WhiteKnight => {
                knight_moves_unsafe(&self, from, &color)
            },
            Piece::BlackPawn | Piece::WhitePawn => {
                pawn_moves_unsafe(&self, from, &color)
            },
            Piece::BlackBishop | Piece::WhiteBishop => {
                straight_line_moves_unsafe(&self, &from, &color, &[1, 3, 5, 7, 9, 11])
            },
            Piece::BlackRook | Piece::WhiteRook => {
                straight_line_moves_unsafe(&self, &from, &color, &[0, 2, 4, 6, 8, 10])
            },
            Piece::BlackQueen | Piece::WhiteQueen => {
                straight_line_moves_unsafe(&self, &from, &color, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11])
            }
        });
        
        result
    }

    /// create a new hexchess instance
    pub fn new() -> Self {
        Self {
            board: [None; 91],
            ep: None,
            fullmove: 1,
            halfmove: 0,
            turn: Color::White,
        }
    }

    /// find king by color
    pub fn find_king(&self, color: Color) -> Option<u8> {
        let king = match color {
            Color::Black => Piece::BlackKing,
            Color::White => Piece::WhiteKing,
        };

        for (index, piece) in self.board.iter().enumerate() {
            if piece == &Some(king) {
                return Some(index as u8);
            }
        }

        None
    }

    /// initialize a hexchess instance to the starting position
    pub fn init() -> Self {
        Self::parse(INITIAL_POSITION).unwrap()
    }

    /// test if the board is in check
    pub fn is_check(&self) -> bool {
        let king = match self.find_king(self.turn) {
            Some(king) => king,
            None => return false
        };

        let opposite_turn = match self.turn {
            Color::Black => Color::White,
            Color::White => Color::Black,
        };

        for n in self.get_color(opposite_turn) {
            for san in self.moves_from_unsafe(n) {
                if san.to == king {
                    return true
                }
            }
        }
        
        false
    }

    /// test if the board is in checkmate
    pub fn is_checkmate(&self) -> bool {
        self.is_check() && self.current_moves().len() == 0
    }

    /// test if move is legal
    pub fn is_legal(&self, san: &San) -> bool {
        let piece = match self.board[san.from as usize] {
            Some(piece) => piece,
            None => return false,
        };
        
        if get_color(&piece) != self.turn {
            return false;
        }

        self.moves_from(san.from)
            .iter()
            .any(|move_san| move_san == san)
    }

    /// test if the board is in stalemate
    pub fn is_stalemate(&self) -> bool {
        !self.is_check() && self.current_moves().len() == 0
    }

    /// test if position is threatened
    pub fn is_threatened(&self, position: u8) -> bool {
        let threatened_piece = match self.board[position as usize] {
            Some(piece) => piece,
            None => return false,
        };

        let color = get_color(&threatened_piece);

        for n in 0u8..91u8 {
            match self.board[n as usize] {
                Some(piece) => match color == get_color(&piece) {
                    true => continue,
                    false => {
                        for san in self.moves_from_unsafe(n) {
                            if san.to == position {
                                return true
                            }
                        }
                    }
                },
                None => continue,
            };
        }

        false
    }

    /// create hexchess instance from fen
    pub fn parse(source: &str) -> Result<Self, String> {
        let mut parts = source.split_whitespace();

        let board = match parts.next() {
            Some(part) => match parse_board(&part.to_string()) {
                Ok(result) => result,
                Err(failure) => return Err(failure),
            }
            _ => return Err("board not found".to_string()),
        };

        let turn = match parts.next() {
            Some(part) => match part {
                "b" => Color::Black,
                "w" => Color::White,
                _ => return Err(format!("invalid turn color: {}", part)),
            },
            None => Color::White,
        };

        let ep = match parts.next() {
            Some(part) => match part {
                "-" => None,
                _ => match index(&part) {
                    Ok(result) => match is_legal_en_passant(&result) {
                        true => Some(result),
                        false => return Err(format!("illegal en passant position: {}", part)),
                    },
                    Err(_) => return Err(format!("invalid en passant position: {}", part)),
                },
            },
            None => None,
        };

        let halfmove = match parts.next() {
            Some(part) => match part.parse::<u8>() {
              Ok(result) => result,
              Err(_) => return Err(format!("invalid halfmove: {}", part)),
            },
            None => 0,
        };

        let fullmove = match parts.next() {
            Some(part) => match part.parse::<u16>() {
                Ok(result) => match result >= 1 {
                    true => result,
                    false => return Err(format!("invalid fullmove: {}", part)),
                },
                Err(_) => return Err(format!("invalid fullmove: {}", part)),
            },
            None => 1,
        };

        Ok(Self {
            board,
            ep,
            fullmove,
            halfmove,
            turn,
        })
    }

    /// format as fen string
    pub fn to_string(&self) -> String {
        format!(
            "{} {} {} {} {}",
            stringify_board(&self.board),
            match self.turn {
                Color::Black => 'b',
                Color::White => 'w',
            },
            match self.ep {
                Some(ep) => position(&ep),
                None => "-",
            },
            self.halfmove,
            self.fullmove,
        )
    }
}

/// parse the board segment of fen
fn parse_board(source: &String) -> Result<[Option<Piece>; 91], String> {
    let mut arr: [Option<Piece>; 91] = [None; 91];
    let mut black = false;
    let mut white = false;
    let mut fen_index: u8 = 0;

    for (index, current) in source.chars().enumerate() {
        match current {
            '/' => continue,
            '0' => continue,
            '1' => match source.chars().nth(index as usize + 1) {
                Some('0') | Some('1') => fen_index += 10,
                _ => fen_index += 1,
            },
            '2' => fen_index += 2,
            '3' => fen_index += 3,
            '4' => fen_index += 4,
            '5' => fen_index += 5,
            '6' => fen_index += 6,
            '7' => fen_index += 7,
            '8' => fen_index += 8,
            '9' => fen_index += 9,
            'b' | 'B' | 'n' | 'N' | 'p' | 'P' | 'Q' | 'q' | 'r' | 'R' => {
                // // it's safe to unwrap current because our match already checks for it
                arr[fen_index as usize] = Some(to_piece(current).unwrap());

                fen_index += 1;
            }
            'k' => {
                if black {
                    return Err("multiple black kings".to_string());
                }

                arr[fen_index as usize] = Some(Piece::BlackKing);
                black = true;
                fen_index += 1;
            }
            'K' => {
                if white {
                    return Err("multiple white kings".to_string());
                }

                arr[fen_index as usize] = Some(Piece::WhiteKing);
                white = true;
                fen_index += 1;
            },
            _ => return Err(format!("invalid character at index {}: {}", index, current)),
        }
    }

    if fen_index != 91 {
        return Err("board overflow".to_string());
    }

    Ok(arr)
}

/// format the board section of a fen
fn stringify_board(board: &[Option<Piece>; 91]) -> String {
    let mut blank: u8 = 0;
    let mut index: u8 = 0;
    let mut result = String::new();

    for val in board.iter() {
        match val {
            None => {
                blank += 1;
            },
            Some(piece) => {
                if blank > 0 {
                    result.push_str(&blank.to_string());
                    blank = 0;
                }

                result.push(match piece {
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
                });
            },
        };

        match index {
            0 | 3 | 8 | 15 | 24 | 35 | 46 | 57 | 68 | 79 => {
                if blank > 0 {
                    result.push_str(&blank.to_string());
                }

                result.push('/');
                blank = 0;
            },
            _ => {}
        };

        index += 1;
    }

    if blank > 0 {
        result.push_str(&blank.to_string());
    }

    result
}

/// convert character to piece
fn to_piece(source: char) -> Result<Piece, &'static str> {
    match source {
        'p' => Ok(Piece::BlackPawn),
        'n' => Ok(Piece::BlackKnight),
        'b' => Ok(Piece::BlackBishop),
        'r' => Ok(Piece::BlackRook),
        'q' => Ok(Piece::BlackQueen),
        'k' => Ok(Piece::BlackKing),
        'P' => Ok(Piece::WhitePawn),
        'N' => Ok(Piece::WhiteKnight),
        'B' => Ok(Piece::WhiteBishop),
        'R' => Ok(Piece::WhiteRook),
        'Q' => Ok(Piece::WhiteQueen),
        'K' => Ok(Piece::WhiteKing),
        _ => Err("invalid_piece_character")
    }
}

#[cfg(test)]
mod tests {
    use crate::{h, s};
    use super::*;

    #[test]
    fn test_clone() {
        let hexchess = Hexchess::init();
        let clone = hexchess.clone();

        assert_eq!(clone.board, hexchess.board);
        assert_eq!(clone.ep, hexchess.ep);
        assert_eq!(clone.turn, hexchess.turn);
        assert_eq!(clone.halfmove, hexchess.halfmove);  
        assert_eq!(clone.fullmove, hexchess.fullmove);
    }

    #[test]
    fn test_apply_move_unsafe() {
        let mut hexchess = Hexchess::init();

        hexchess.apply_move_unsafe(&s!("b1b6")); // <- illegal pawn move

        assert_eq!(hexchess.to_string(), "b/qbk/n1b1n/r5r/ppppppppp/1P9/5P5/4P1P4/3P1B1P3/2P2B2P2/2RNQBKNRP1 b - 0 1");
    }

    #[test]
    fn test_current_moves() {
        let hexchess = Hexchess::init();
        let result = hexchess.current_moves().iter().map(|s| s.to_string()).collect::<Vec<String>>();

        assert_eq!(result.len(), 51);
        assert_eq!(result[0], "f5f6");
        assert_eq!(result[1], "e4e5");
        assert_eq!(result[2], "e4e6");
        assert_eq!(result[3], "g4g5");
        assert_eq!(result[4], "g4g6");
        assert_eq!(result[5], "d3d4");
        assert_eq!(result[6], "d3d5");
        assert_eq!(result[7], "f3h2");
        assert_eq!(result[8], "f3d2");
        assert_eq!(result[9], "h3h4");
        assert_eq!(result[10], "h3h5");
        assert_eq!(result[11], "c2c3");
        assert_eq!(result[12], "c2c4");
        assert_eq!(result[13], "f2g3");
        assert_eq!(result[14], "f2h4");
        assert_eq!(result[15], "f2i5");
        assert_eq!(result[16], "f2k6");
        assert_eq!(result[17], "f2e3");
        assert_eq!(result[18], "f2d4");
        assert_eq!(result[19], "f2c5");
        assert_eq!(result[20], "f2b6");
        assert_eq!(result[21], "i2i3");
        assert_eq!(result[22], "i2i4");
        assert_eq!(result[23], "b1b2");
        assert_eq!(result[24], "b1b3");
        assert_eq!(result[25], "c1d2");
        assert_eq!(result[26], "c1e3");
        assert_eq!(result[27], "c1f4");
        assert_eq!(result[28], "d1f4");
        assert_eq!(result[29], "d1g2");
        assert_eq!(result[30], "d1b2");
        assert_eq!(result[31], "d1c3");
        assert_eq!(result[32], "e1e2");
        assert_eq!(result[33], "e1e3");
        assert_eq!(result[34], "e1d2");
        assert_eq!(result[35], "e1c3");
        assert_eq!(result[36], "e1b4");
        assert_eq!(result[37], "e1a5");
        assert_eq!(result[38], "f1g2");
        assert_eq!(result[39], "f1e2");
        assert_eq!(result[40], "g1g2");
        assert_eq!(result[41], "g1h2");
        assert_eq!(result[42], "h1i3");
        assert_eq!(result[43], "h1k2");
        assert_eq!(result[44], "h1e2");
        assert_eq!(result[45], "h1f4");
        assert_eq!(result[46], "i1h2");
        assert_eq!(result[47], "i1g3");
        assert_eq!(result[48], "i1f4");
        assert_eq!(result[49], "k1k2");
        assert_eq!(result[50], "k1k3");
    }

    #[test]
    fn find_kings_by_color() {
        let hexchess = Hexchess::init();

        assert_eq!(hexchess.find_king(Color::Black), Some(h!("g10")));
        assert_eq!(hexchess.find_king(Color::White), Some(h!("g1")));
    }

    #[test]
    fn test_get() {
        let hexchess = Hexchess::init();

        assert_eq!(hexchess.get("g10"), Some(Piece::BlackKing));
        assert_eq!(hexchess.get("g1"), Some(Piece::WhiteKing));
        assert_eq!(hexchess.get("a4"), None);
        assert_eq!(hexchess.get("whoops"), None);
    }

    #[test]
    fn get_color() {
        let hexchess = Hexchess::init();
        let results = hexchess.get_color(Color::Black);

        assert_eq!(results.len(), 18);
        assert_eq!(results[0], h!("f11"));
        assert_eq!(results[1], h!("e10"));
        assert_eq!(results[2], h!("f10"));
        assert_eq!(results[3], h!("g10"));
        assert_eq!(results[4], h!("d9"));
        assert_eq!(results[5], h!("f9"));
        assert_eq!(results[6], h!("h9"));
        assert_eq!(results[7], h!("c8"));
        assert_eq!(results[8], h!("i8"));
        assert_eq!(results[9], h!("b7"));
        assert_eq!(results[10], h!("c7"));
        assert_eq!(results[11], h!("d7"));
        assert_eq!(results[12], h!("e7"));
        assert_eq!(results[13], h!("f7"));
        assert_eq!(results[14], h!("g7"));
        assert_eq!(results[15], h!("h7"));
        assert_eq!(results[16], h!("i7"));
        assert_eq!(results[17], h!("k7"));
    }

    mod is_check {
        use super::*;

        #[test]
        fn no_king() {
            let hexchess = Hexchess::new();

            assert_eq!(hexchess.is_check(), false);
        }

        #[test]
        fn not_in_check() {
            let hexchess = Hexchess::parse("1/3/5/7/9/11/11/11/11/11/11 w - 0 1").unwrap();

            assert_eq!(hexchess.is_check(), false);
        }

        #[test]
        fn in_check() {
            let hexchess = Hexchess::parse("K/3/5/7/9/5r5/11/11/11/11/11 w - 0 1").unwrap();

            assert_eq!(hexchess.is_check(), true);
        }
    }

    #[test]
    fn is_checkmate() {
        let mut hexchess = Hexchess::parse("K/3/5/3q3/2q6/11/11/11/11/11/11 b - 0 1").unwrap();

        assert_eq!(hexchess.is_checkmate(), false);
  
        let _ = hexchess.apply_move(&s!("d7f9"));
  
        assert_eq!(hexchess.is_checkmate(), true);
    }

    mod is_legal {
        use super::*;

        #[test]
        fn legal_move() {
            let hexchess = Hexchess::init();

            assert_eq!(hexchess.is_legal(&s!("g4g5")), true);
        }

        #[test]
        fn illegal_move() {
            let hexchess = Hexchess::init();

            assert_eq!(hexchess.is_legal(&s!("b1b4")), false);
        }

        #[test]
        fn illegal_move_out_of_turn() {
            let mut hexchess = Hexchess::init();

            assert_eq!(hexchess.is_legal(&s!("g7g6")), false);

            hexchess.turn = Color::Black;

            assert_eq!(hexchess.is_legal(&s!("g7g6")), true);
        }

        #[test]
        fn white_cannot_promote_on_blacks_positions() {
            let hexchess = Hexchess::parse("1/3/5/7/p7p/R9R/11/11/11/11/rP7Pr w - 0 1").unwrap();

            let b1b2 = San { from: h!("b1"), to: h!("b2"), promotion: None };
            let b1b2q = San { from: h!("b1"), to: h!("b2"), promotion: Some(PromotionPiece::Queen) };
            assert_eq!(hexchess.is_legal(&b1b2), true);
            assert_eq!(hexchess.is_legal(&b1b2q), false);

            let k1l1 = San { from: h!("k1"), to: h!("l1"), promotion: None };
            let k1l1q = San { from: h!("k1"), to: h!("l1"), promotion: Some(PromotionPiece::Queen) };
            assert_eq!(hexchess.is_legal(&k1l1), true);
            assert_eq!(hexchess.is_legal(&k1l1q), false);
        }

        #[test]
        fn black_cannot_promote_on_whites_positions() {
            let hexchess = Hexchess::parse("1/3/5/7/p7p/R9R/11/11/11/11/rP7Pr b - 0 1").unwrap();

            let b7a6 = San { from: h!("b7"), to: h!("a6"), promotion: None };
            let b7a6q = San { from: h!("b7"), to: h!("a6"), promotion: Some(PromotionPiece::Queen) };
            assert_eq!(hexchess.is_legal(&b7a6), true);
            assert_eq!(hexchess.is_legal(&b7a6q), false);

            let k7l6 = San { from: h!("k7"), to: h!("l6"), promotion: None };
            let k7l6q = San { from: h!("k7"), to: h!("l6"), promotion: Some(PromotionPiece::Queen) };
            assert_eq!(hexchess.is_legal(&k7l6), true);
            assert_eq!(hexchess.is_legal(&k7l6q), false);
        }

        #[test]
        fn pawn_must_promote_on_final_rank() {
            let mut hexchess = Hexchess::parse("1/1P1/5/7/9/11/11/11/11/5p5/11 w - 0 1").unwrap();

            assert_eq!(hexchess.is_legal(&s!("f10f11")), false);
            assert_eq!(hexchess.is_legal(&s!("f10f11q")), true);

            hexchess.turn = Color::Black;

            assert_eq!(hexchess.is_legal(&s!("f2f1")), false);
            assert_eq!(hexchess.is_legal(&s!("f2f1q")), true);
        }
    }

    #[test]
    fn is_stalemate() {
        let mut hexchess = Hexchess::parse("k/1P1/5/3K3/9/11/11/11/11/11/11 w - 0 1").unwrap();

        assert_eq!(hexchess.is_stalemate(), false);
  
        let _ = hexchess.apply_move(&s!("f8f9"));
  
        assert_eq!(hexchess.is_stalemate(), true);
    }

    mod is_threatened {
        use super::*;

        #[test]
        fn unattacked_position_is_not_threatened() {
            let hexchess = Hexchess::parse("1/2K/5/7/9/11/11/11/11/11/11 w - 0 1").unwrap();

            assert_eq!(hexchess.is_threatened(h!("g10")), false);
        }

        #[test]
        fn threatened_by_enemy_piece() {
            let hexchess = Hexchess::parse("1/2K/5/7/9/11/11/11/11/11/6r4 w - 0 1").unwrap();

            assert_eq!(hexchess.is_threatened(h!("g10")), true);
        }
        
        #[test]
        fn not_threatened_by_friendly_piece() {
            let hexchess = Hexchess::parse("1/2K/5/7/9/11/11/11/11/11/6R4 w - 0 1").unwrap();

            assert_eq!(hexchess.is_threatened(h!("g10")), false);
        }

        #[test]
        fn  position_is_threatened_in_and_out_of_turn() {
            let mut hexchess = Hexchess::parse("1/3/5/7/4q4/5K5/11/11/11/11/11 w - 0 1").unwrap();

            hexchess.turn = Color::Black;
            assert_eq!(hexchess.is_threatened(h!("f6")), true);

            hexchess.turn = Color::White;
            assert_eq!(hexchess.is_threatened(h!("f6")), true);
        }

        #[test]
        fn unoccupied_position_is_not_threatened() {
            let hexchess = Hexchess::new();

            assert_eq!(hexchess.is_threatened(h!("f5")), false);
        }
    }

    mod moves_from {
        use super::*;

        #[test]
        fn returns_empty_vector_for_empty_position() {
            let hexchess = Hexchess::init();

            assert_eq!(hexchess.moves_from(h!("a4")).len(), 0);
            assert_eq!(hexchess.moves_from_unsafe(h!("a4")).len(), 0);
        }
    }

    mod self_check {
        use super::*;

        #[test]
        fn cannot_step_out_of_a_pin() {
            let hexchess = Hexchess::parse("1/3/5/7/4K4/5R5/5q5/11/11/11/11 w - 0 1").unwrap();

            let moves = hexchess.moves_from(h!("f6"));
            assert_eq!(moves.len(), 1);
            assert_eq!(moves[0], s!("f6f5"));
        }

        // cannot self check on opponent's turn
        #[test]
        fn cannot_self_check_on_opponents_turn() {
            let hexchess    = Hexchess::parse("1/3/5/7/4K4/5R5/5q5/11/11/11/11 b - 0 1").unwrap();
            let moves = hexchess.moves_from(h!("f6"));

            assert_eq!(moves.len(), 1);
            assert_eq!(moves[0], s!("f6f5"));
        }

        // king cannot step into check
        #[test]
        fn king_cannot_step_into_check() {
            let hexchess = Hexchess::parse("K/3/2q2/7/9/11/11/11/11/11/11 w - 0 1").unwrap();
            let moves = hexchess.moves_from(h!("f11"));

            assert_eq!(moves.len(), 0);
        }
    }

    #[test]
    fn test_to_piece() {
        assert_eq!(to_piece('b'), Ok(Piece::BlackBishop));
        assert_eq!(to_piece('B'), Ok(Piece::WhiteBishop));
        assert_eq!(to_piece('k'), Ok(Piece::BlackKing)); // <- not called during normal board parsing
        assert_eq!(to_piece('K'), Ok(Piece::WhiteKing)); // <- not called during normal board parsing
        assert_eq!(to_piece('n'), Ok(Piece::BlackKnight));
        assert_eq!(to_piece('N'), Ok(Piece::WhiteKnight));
        assert_eq!(to_piece('p'), Ok(Piece::BlackPawn));
        assert_eq!(to_piece('P'), Ok(Piece::WhitePawn));
        assert_eq!(to_piece('q'), Ok(Piece::BlackQueen));
        assert_eq!(to_piece('Q'), Ok(Piece::WhiteQueen));
        assert_eq!(to_piece('r'), Ok(Piece::BlackRook));
        assert_eq!(to_piece('R'), Ok(Piece::WhiteRook));
    }

    #[test]
    fn test_to_piece_invalid() {
        assert_eq!(to_piece('x'), Err("invalid_piece_character"));
        assert_eq!(to_piece('1'), Err("invalid_piece_character"));
        assert_eq!(to_piece('/'), Err("invalid_piece_character"));
        assert_eq!(to_piece(' '), Err("invalid_piece_character"));
    }

    mod to_string {
        use super::*;

        #[test]
        fn empty_position() {
            let hexchess = Hexchess::new();

            assert_eq!(hexchess.to_string(), "1/3/5/7/9/11/11/11/11/11/11 w - 0 1");
        }

        #[test]
        fn initial_position() {
            let hexchess = Hexchess::init();

            assert_eq!(hexchess.to_string(), "b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 1");
        }

        #[test]
        fn with_en_passant() {
            let mut hexchess = Hexchess::init();

            let _ = hexchess.apply_move(&s!("g4g6"));

            assert_eq!(hexchess.to_string(), "b/qbk/n1b1n/r5r/ppppppppp/6P4/5P5/4P6/3P1B1P3/2P2B2P2/1PRNQBKNRP1 b g5 0 1");
        }
    }
}
