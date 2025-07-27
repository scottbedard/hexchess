use crate::h;
use crate::hexchess::pieces::bishop::bishop_moves_unsafe;
use crate::hexchess::pieces::king::king_moves_unsafe;
use crate::hexchess::pieces::knight::knight_moves_unsafe;
use crate::hexchess::pieces::pawn::pawn_moves_unsafe;
use crate::hexchess::pieces::queen::queen_moves_unsafe;
use crate::hexchess::pieces::rook::rook_moves_unsafe;
use crate::hexchess::san::San;
use serde_with::serde_as;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::hash::Hash;

use crate::constants::{
    Color,
    HEXBOARD_GRAPH,
    INITIAL_POSITION,
    KNIGHT_GRAPH,
    Piece,
    PromotionPiece,
};

use crate::hexchess::utils::{
    get_color,
    is_legal_en_passant,
    step,
    index,
    position,
    walk_until_piece,
};

/// Hexchess game state
#[serde_as]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Hexchess {
    #[serde_as(as = "[_; 91]")]
    pub board: [Option<Piece>; 91],

    pub ep: Option<u8>,

    pub fullmove: u16,

    pub halfmove: u8,

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
    pub fn apply_move(&mut self, san: &San) -> Result<&Self, String> {
        if !self.is_legal(san) {
            return Err(format!("illegal move: {:?}", san));
        }

        self.apply_move_unsafe(san)
    }

    /// apply move, regardless of turn or legality
    pub fn apply_move_unsafe(&mut self, san: &San) -> Result<&Self, String> {
        let piece = match self.board[san.from as usize] {
            Some(piece) => piece,
            None => return Err(format!("cannot apply move from empty position: {}", san.from)),
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

        Ok(self)
    }

    /// get legal moves for current turn
    #[inline(always)]
    pub fn current_moves(&self) -> Vec<San> {
        let mut result: Vec<San> = Vec::with_capacity(40); // Most positions have <40 legal moves

        for n in self.get_color(self.turn) {
            result.extend(self.moves_from(n));
        }

        result
    }

    /// get piece at position
    #[inline(always)]
    pub fn get(&self, position: &str) -> Option<Piece> {
        match index(position) {
            Ok(index) => unsafe { *self.board.get_unchecked(index as usize) },
            Err(_) => None,
        }
    }

    /// get positions occupied by a color
    pub fn get_color(&self, color: Color) -> SmallVec<[u8; 16]> {
        let mut result: SmallVec<[u8; 16]> = SmallVec::new();

        // Use unchecked access for better performance
        for index in 0..91 {
            unsafe {
                match *self.board.get_unchecked(index) {
                    Some(piece) => if get_color(&piece) == color {
                        result.push(index as u8);
                    },
                    None => continue,
                }
            }
        }

        result
    }

    /// get legal moves a position
    #[inline(always)]
    pub fn moves_from(&self, from: u8) -> Vec<San> {
        let piece = match self.board[from as usize] {
            Some(piece) => piece,
            None => return Vec::new(),
        };

        let color = get_color(&piece);

        self.moves_from_unsafe(from)
            .into_iter()
            .filter(|san| {
                // clone the board and apply an unsafe move
                let mut clone = self.clone();
                
                let _ = clone.apply_move_unsafe(san);

                let king_position = match clone.find_king(color) {
                    Some(king) => king,
                    None => return true,
                };

                // filter self-check moves
                if
                    is_king_threat(&clone, &color, king_position) ||
                    is_knight_threat(&clone, &color, king_position) ||
                    is_pawn_threat(&clone, &color, king_position) ||
                    is_straight_line_threat(&clone, &color, king_position)
                {
                    return false;
                }

                true
            })
            .collect()
    }

    /// get moves from a position, regardless of turn or legality
    #[inline(always)]
    pub fn moves_from_unsafe(&self, from: u8) -> Vec<San> {
        let mut result: Vec<San> = Vec::with_capacity(12); // Most pieces have <12 moves

        let piece = match self.board[from as usize] {
            Some(piece) => piece,
            None => return result,
        };
        
        let color = get_color(&piece);

        result.extend(match piece {
            Piece::BlackKing | Piece::WhiteKing => {
                king_moves_unsafe(&self, from, &color).into_vec().into_iter()
            },
            Piece::BlackKnight | Piece::WhiteKnight => {
                knight_moves_unsafe(&self, from, &color).into_vec().into_iter()
            },
            Piece::BlackPawn | Piece::WhitePawn => {
                pawn_moves_unsafe(&self, from, &color).into_vec().into_iter()
            },
            Piece::BlackBishop | Piece::WhiteBishop => {
                bishop_moves_unsafe(&self, &from, &color).into_vec().into_iter()
            },
            Piece::BlackRook | Piece::WhiteRook => {
                rook_moves_unsafe(&self, &from, &color).into_vec().into_iter()
            },
            Piece::BlackQueen | Piece::WhiteQueen => {
                queen_moves_unsafe(&self, &from, &color).into_iter()
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
    #[inline(always)]
    pub fn find_king(&self, color: Color) -> Option<u8> {
        let king = match color {
            Color::Black => Piece::BlackKing,
            Color::White => Piece::WhiteKing,
        };

        // Use unchecked access for better performance
        for index in 0..91 {
            unsafe {
                if *self.board.get_unchecked(index) == Some(king) {
                    return Some(index as u8);
                }
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

        // Use unchecked access for better performance
        for n in 0u8..91u8 {
            unsafe {
                match *self.board.get_unchecked(n as usize) {
                    Some(piece) => if get_color(&piece) != color {
                        for san in self.moves_from_unsafe(n) {
                            if san.to == position {
                                return true
                            }
                        }
                    },
                    None => continue,
                }
            }
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

/// test if knight threatens a position
#[inline(always)]
fn is_knight_threat(hexchess: &Hexchess, color: &Color, position: u8) -> bool {
    let hostile_knight = match color {
        Color::Black => Piece::WhiteKnight,
        Color::White => Piece::BlackKnight,
    };
    
    for n in KNIGHT_GRAPH[position as usize].iter() {
        unsafe {
            if *hexchess.board.get_unchecked(*n as usize) == Some(hostile_knight) {
                return true
            }
        }
    }

    false
}

/// test if position touches the hostile king
#[inline(always)]
fn is_king_threat(hexchess: &Hexchess, color: &Color, position: u8) -> bool {
    let hostile_king = match color {
        Color::Black => Piece::WhiteKing,
        Color::White => Piece::BlackKing,
    };

    for n in HEXBOARD_GRAPH[position as usize] {
        if let Some(index) = n {
            unsafe {
                if *hexchess.board.get_unchecked(index as usize) == Some(hostile_king) {
                    return true
                }
            }
        }
    }

    false
}

/// test if pawn threatens a position
#[inline(always)]
fn is_pawn_threat(hexchess: &Hexchess, color: &Color, position: u8) -> bool {
    // portside and starboard are reversed from the pawn's perspective because
    // we only know the king's position. so we must move the king like friendly
    // pawn to see if it encounters a hostile pawn
    let (
        hostile_pawn,
        reverse_portside,
        reverse_starboard,
    ) = match color {
        Color::Black => (Piece::WhitePawn, 4, 8),
        Color::White => (Piece::BlackPawn, 10, 2),
    };

    match step(position, reverse_portside) {
        Some(index) => unsafe {
            if *hexchess.board.get_unchecked(index as usize) == Some(hostile_pawn) {
                return true
            }
        },
        None => {},
    }

    match step(position, reverse_starboard) {
        Some(index) => unsafe {
            if *hexchess.board.get_unchecked(index as usize) == Some(hostile_pawn) {
                return true
            }
        },
        None => {},
    }

    false
}

/// test if straight line piece threatens a position
#[inline(always)]
fn is_straight_line_threat(hexchess: &Hexchess, color: &Color, position: u8) -> bool {
    let hostile_bishop = match color {
        Color::Black => Piece::WhiteBishop,
        Color::White => Piece::BlackBishop,
    };

    let hostile_queen = match color {
        Color::Black => Piece::WhiteQueen,
        Color::White => Piece::BlackQueen,
    };

    let hostile_rook = match color {
        Color::Black => Piece::WhiteRook,
        Color::White => Piece::BlackRook,
    };

    // check for diagonal threats
    for diagonal_direction in &[1u8, 3, 5, 7, 9, 11] {
        let piece = walk_until_piece(hexchess, position, *diagonal_direction);

        if piece == Some(hostile_bishop) || piece == Some(hostile_queen) {
            return true;
        }
    }

    // check for orthogonal threats
    for orthogonal_direction in &[0u8, 2, 4, 6, 8, 10] {
        let piece = walk_until_piece(hexchess, position, *orthogonal_direction);

        if piece == Some(hostile_rook) || piece == Some(hostile_queen) {
            return true;
        }
    }

    false
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
    use super::*;
    use std::hash::{DefaultHasher, Hasher};

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
    fn test_hash_equality() {
        let hexchess1 = Hexchess::init();
        let hexchess2 = Hexchess::init();

        assert_eq!(hexchess1, hexchess2);

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();
        hexchess1.hash(&mut hasher1);
        hexchess2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
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
}
