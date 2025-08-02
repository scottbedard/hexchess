use crate::constants::INITIAL_POSITION;
use crate::hexchess::bitboard::Bitboard;
use crate::hexchess::bitmaps::knight::get_knight_moves_unsafe;
use crate::hexchess::color::Color;
use crate::hexchess::piece::Piece;
use crate::hexchess::position::Position;
use crate::hexchess::san::San;

#[derive(Debug)]
pub struct Game {
    pub bitboard_black_bishop: Bitboard,
    pub bitboard_black_king: Bitboard,
    pub bitboard_black_knight: Bitboard,
    pub bitboard_black_pawn: Bitboard,
    pub bitboard_black_queen: Bitboard,
    pub bitboard_black_rook: Bitboard,
    pub bitboard_white_bishop: Bitboard,
    pub bitboard_white_king: Bitboard,
    pub bitboard_white_knight: Bitboard,
    pub bitboard_white_pawn: Bitboard,
    pub bitboard_white_queen: Bitboard,
    pub bitboard_white_rook: Bitboard,
    pub ep: Option<Position>,
    pub fullmove: u16,
    pub halfmove: u8,
    pub turn: Color,
}

impl Game {
    /// Create a new game instance with no pieces.
    pub fn new() -> Self {
        Self {
            bitboard_black_bishop: Bitboard::new(),
            bitboard_black_king: Bitboard::new(),
            bitboard_black_knight: Bitboard::new(),
            bitboard_black_pawn: Bitboard::new(),
            bitboard_black_queen: Bitboard::new(),
            bitboard_black_rook: Bitboard::new(),
            bitboard_white_bishop: Bitboard::new(),
            bitboard_white_king: Bitboard::new(),
            bitboard_white_knight: Bitboard::new(),
            bitboard_white_pawn: Bitboard::new(),
            bitboard_white_queen: Bitboard::new(),
            bitboard_white_rook: Bitboard::new(),
            ep: None,
            fullmove: 1,
            halfmove: 0,
            turn: Color::White,
        }
    }

    /// Create a new game instance with the initial position.
    pub fn init() -> Self {
        Self::parse(INITIAL_POSITION).unwrap()
    }

    /// Clear all bitboards at a given position.
    pub fn clear_position(&mut self, position: Position) {
        self.bitboard_black_bishop.clear_position(position);
        self.bitboard_black_king.clear_position(position);
        self.bitboard_black_knight.clear_position(position);
        self.bitboard_black_pawn.clear_position(position);
        self.bitboard_black_queen.clear_position(position);
        self.bitboard_black_rook.clear_position(position);
        self.bitboard_white_bishop.clear_position(position);
        self.bitboard_white_king.clear_position(position);
        self.bitboard_white_knight.clear_position(position);
        self.bitboard_white_pawn.clear_position(position);
        self.bitboard_white_queen.clear_position(position);
        self.bitboard_white_rook.clear_position(position);
    }

    /// Get the color of the piece at the given position.
    pub fn get_color(&self, position: Position) -> Option<Color> {
        let black = self.get_color_bitboard(Color::Black);

        if black.is_position_set(position) {
            return Some(Color::Black);
        }

        let white = self.get_color_bitboard(Color::White);

        if white.is_position_set(position) {
            return Some(Color::White);
        }

        None
    }

    /// Get the bitboard for a given color.
    pub fn get_color_bitboard(&self, color: Color) -> Bitboard {
        match color {
            Color::Black => {
                self.bitboard_black_bishop |
                self.bitboard_black_king |
                self.bitboard_black_knight |
                self.bitboard_black_pawn |
                self.bitboard_black_queen |
                self.bitboard_black_rook
            },
            Color::White => {
                self.bitboard_white_bishop |
                self.bitboard_white_king |
                self.bitboard_white_knight |
                self.bitboard_white_pawn |
                self.bitboard_white_queen |
                self.bitboard_white_rook
            }
        }
    }

    /// Get moves from a position, regardless of turn or legality.
    pub fn get_moves_unsafe(&self, position: Position) -> Vec<San> {
        // Get the piece at this position
        let piece = match self.get_position(position) {
            Some(piece) => piece,
            None => return Vec::new(), // no piece at this position
        };

        let result = match piece {
            Piece::BlackKnight | Piece::WhiteKnight => get_knight_moves_unsafe(&self, position),
            _ => Vec::new(),
        };

        // result.extend(match piece {
        //     Piece::BlackKing | Piece::WhiteKing => {
        //         king_moves_unsafe(&self, from, &color).into_vec().into_iter()
        //     },
        //     Piece::BlackKnight | Piece::WhiteKnight => {
        //         knight_moves_unsafe(&self, from, &color).into_vec().into_iter()
        //     },
        //     Piece::BlackPawn | Piece::WhitePawn => {
        //         pawn_moves_unsafe(&self, from, &color).into_vec().into_iter()
        //     },
        //     Piece::BlackBishop | Piece::WhiteBishop => {
        //         bishop_moves_unsafe(&self, &from, &color).into_vec().into_iter()
        //     },
        //     Piece::BlackRook | Piece::WhiteRook => {
        //         rook_moves_unsafe(&self, &from, &color).into_vec().into_iter()
        //     },
        //     Piece::BlackQueen | Piece::WhiteQueen => {
        //         queen_moves_unsafe(&self, &from, &color).into_iter()
        //     }
        // });
        
        result
    }

    /// Get the piece at the given position.
    pub fn get_position(&self, position: Position) -> Option<Piece> {
        if self.bitboard_black_bishop.is_position_set(position) {
            Some(Piece::BlackBishop)
        } else if self.bitboard_black_king.is_position_set(position) {
            Some(Piece::BlackKing)
        } else if self.bitboard_black_knight.is_position_set(position) {
            Some(Piece::BlackKnight)
        } else if self.bitboard_black_pawn.is_position_set(position) {
            Some(Piece::BlackPawn)
        } else if self.bitboard_black_queen.is_position_set(position) {
            Some(Piece::BlackQueen)
        } else if self.bitboard_black_rook.is_position_set(position) {
            Some(Piece::BlackRook)
        } else if self.bitboard_white_bishop.is_position_set(position) {
            Some(Piece::WhiteBishop)
        } else if self.bitboard_white_king.is_position_set(position) {
            Some(Piece::WhiteKing)
        } else if self.bitboard_white_knight.is_position_set(position) {
            Some(Piece::WhiteKnight)
        } else if self.bitboard_white_pawn.is_position_set(position) {
            Some(Piece::WhitePawn)
        } else if self.bitboard_white_queen.is_position_set(position) {
            Some(Piece::WhiteQueen)
        } else if self.bitboard_white_rook.is_position_set(position) {
            Some(Piece::WhiteRook)
        } else {
            None
        }
    }

    /// Parse a FEN string into a game instance.
    pub fn parse(source: &str) -> Result<Self, String> {
        let mut game = Self::new();
        let mut parts = source.split_whitespace();

        // board
        let board = match parts.next() {
            Some(part) => match parse_board(&part.to_string()) {
                Ok(result) => result,
                Err(failure) => return Err(failure),
            }
            _ => return Err("fen segment not found: board".to_string()),
        };

        for (i, piece) in board.iter().enumerate() {
            let position = Position::from_fen_index(i as u8);

            match piece {
                None => continue,
                Some(Piece::BlackBishop) => game.bitboard_black_bishop.set_position(position),
                Some(Piece::BlackKing) => game.bitboard_black_king.set_position(position),
                Some(Piece::BlackKnight) => game.bitboard_black_knight.set_position(position),
                Some(Piece::BlackPawn) => game.bitboard_black_pawn.set_position(position),
                Some(Piece::BlackQueen) => game.bitboard_black_queen.set_position(position),
                Some(Piece::BlackRook) => game.bitboard_black_rook.set_position(position),
                Some(Piece::WhiteBishop) => game.bitboard_white_bishop.set_position(position),
                Some(Piece::WhiteKing) => game.bitboard_white_king.set_position(position),
                Some(Piece::WhiteKnight) => game.bitboard_white_knight.set_position(position),
                Some(Piece::WhitePawn) => game.bitboard_white_pawn.set_position(position),
                Some(Piece::WhiteQueen) => game.bitboard_white_queen.set_position(position),
                Some(Piece::WhiteRook) => game.bitboard_white_rook.set_position(position),
            }
        }

        // turn color
        game.turn = match parts.next() {
            Some(part) => match part {
                "b" => Color::Black,
                "w" => Color::White,
                _ => return Err(format!("invalid turn color: {}", part)),
            },
            None => Color::White,
        };

        game.ep = match parts.next() {
            Some(part) => match part {
                "-" => None,
                part => Some(Position::from_string(part))
            },
            None => None,
        };

        game.halfmove = match parts.next() {
            Some(part) => match part.parse::<u8>() {
              Ok(result) => result,
              Err(_) => return Err(format!("invalid halfmove: {}", part)),
            },
            None => 0,
        };

        game.fullmove = match parts.next() {
            Some(part) => match part.parse::<u16>() {
                Ok(result) => match result >= 1 {
                    true => result,
                    false => return Err(format!("invalid fullmove: {}", part)),
                },
                Err(_) => return Err(format!("invalid fullmove: {}", part)),
            },
            None => 1,
        };

        Ok(game)
    }

    /// Convert bitboard states to a piece array.
    pub fn to_array(&self) -> [Option<Piece>; 91] {
        let mut arr: [Option<Piece>; 91] = [None; 91];
        
        for i in 0..91 {
            let position = Position::from_fen_index(i as u8);
            arr[i] = self.get_position(position);
        }

        arr
    }

    /// Set the piece at the given position.
    pub fn set_position(&mut self, position: Position, piece: Piece) {
        self.clear_position(position);

        match piece {
            Piece::BlackBishop => self.bitboard_black_bishop.set_position(position),
            Piece::BlackKing => self.bitboard_black_king.set_position(position),
            Piece::BlackKnight => self.bitboard_black_knight.set_position(position),
            Piece::BlackPawn => self.bitboard_black_pawn.set_position(position),
            Piece::BlackQueen => self.bitboard_black_queen.set_position(position),
            Piece::BlackRook => self.bitboard_black_rook.set_position(position),
            Piece::WhiteBishop => self.bitboard_white_bishop.set_position(position),
            Piece::WhiteKing => self.bitboard_white_king.set_position(position),
            Piece::WhiteKnight => self.bitboard_white_knight.set_position(position),
            Piece::WhitePawn => self.bitboard_white_pawn.set_position(position),
            Piece::WhiteQueen => self.bitboard_white_queen.set_position(position),
            Piece::WhiteRook => self.bitboard_white_rook.set_position(position),
        }
    }
}

/// Display the game state as a FEN string.
impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let board = self.to_array();
        let mut blank: u8 = 0;
        let mut index: u8 = 0;
        let mut fen = String::new();

        for val in board.iter() {
            match val {
                None => {
                    blank += 1;
                },
                Some(piece) => {
                    if blank > 0 {
                        fen.push_str(&blank.to_string());
                        blank = 0;
                    }

                    fen.push_str(&piece.to_string());
                },
            };

            match index {
                0 | 3 | 8 | 15 | 24 | 35 | 46 | 57 | 68 | 79 => {
                    if blank > 0 {
                        fen.push_str(&blank.to_string());
                    }

                    fen.push('/');
                    blank = 0;
                },
                _ => {}
            };

            index += 1;
        }

        if blank > 0 {
            fen.push_str(&blank.to_string());
        }

        write!(
            f, 
            "{} {} {} {} {}",
            fen,
            self.turn,
            match self.ep {
                Some(ep) => ep.to_string(),
                None => "-".to_string(),
            },
            self.halfmove,
            self.fullmove,
        )
    }
}

/// parse the board segment of fen
fn parse_board(source: &String) -> Result<[Option<Piece>; 91], String> {
    let mut arr: [Option<Piece>; 91] = [None; 91];
    let mut existing_black_king = false;
    let mut existing_white_king = false;
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
                arr[fen_index as usize] = Some(Piece::from(current));
                fen_index += 1;
            }
            'k' => {
                if existing_black_king {
                    return Err("multiple black kings".to_string());
                }

                arr[fen_index as usize] = Some(Piece::BlackKing);
                existing_black_king = true;
                fen_index += 1;
            }
            'K' => {
                if existing_white_king {
                    return Err("multiple white kings".to_string());
                }

                arr[fen_index as usize] = Some(Piece::WhiteKing);
                existing_white_king = true;
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

#[cfg(test)]
mod tests {
    use super::*;
    use hexchess_bitmask::bitmask;

    #[test]
    fn test_new() {
        let game = Game::new();
        assert_eq!(game.bitboard_black_bishop.0, 0);
        assert_eq!(game.bitboard_black_king.0, 0);
        assert_eq!(game.bitboard_black_knight.0, 0);
        assert_eq!(game.bitboard_black_pawn.0, 0);
        assert_eq!(game.bitboard_black_queen.0, 0);
        assert_eq!(game.bitboard_black_rook.0, 0);
        assert_eq!(game.bitboard_white_bishop.0, 0);
        assert_eq!(game.bitboard_white_king.0, 0);
        assert_eq!(game.bitboard_white_knight.0, 0);
        assert_eq!(game.bitboard_white_pawn.0, 0);
        assert_eq!(game.bitboard_white_queen.0, 0);
        assert_eq!(game.bitboard_white_rook.0, 0);
        assert_eq!(game.ep, None);
        assert_eq!(game.fullmove, 1);
        assert_eq!(game.halfmove, 0);
        assert_eq!(game.turn, Color::White);
    }

    #[test]
    fn test_get_piece() {
        let game = Game::init();
        assert_eq!(game.get_position(Position::F11), Some(Piece::BlackBishop));
    }

    #[test]
    fn test_get_and_set_piece() {
        let mut game = Game::new();

        game.set_position(Position::F1, Piece::BlackBishop);
        game.set_position(Position::F2, Piece::BlackPawn);
        game.set_position(Position::F3, Piece::BlackKnight);
        game.set_position(Position::F4, Piece::BlackQueen);
        game.set_position(Position::F5, Piece::BlackKing);
        game.set_position(Position::F6, Piece::BlackRook);
        game.set_position(Position::F7, Piece::BlackBishop);
        game.set_position(Position::F8, Piece::BlackKnight);
        game.set_position(Position::G1, Piece::WhiteBishop);
        game.set_position(Position::G2, Piece::WhitePawn);
        game.set_position(Position::G3, Piece::WhiteKnight);
        game.set_position(Position::G4, Piece::WhiteQueen);
        game.set_position(Position::G5, Piece::WhiteKing);
        game.set_position(Position::G6, Piece::WhiteRook);
        game.set_position(Position::G7, Piece::WhiteBishop);

        assert_eq!(game.get_position(Position::F1), Some(Piece::BlackBishop));
        assert_eq!(game.get_position(Position::F2), Some(Piece::BlackPawn));
        assert_eq!(game.get_position(Position::F3), Some(Piece::BlackKnight));
        assert_eq!(game.get_position(Position::F4), Some(Piece::BlackQueen));
        assert_eq!(game.get_position(Position::F5), Some(Piece::BlackKing));
        assert_eq!(game.get_position(Position::F6), Some(Piece::BlackRook));
        assert_eq!(game.get_position(Position::F7), Some(Piece::BlackBishop));
        assert_eq!(game.get_position(Position::F8), Some(Piece::BlackKnight));
        assert_eq!(game.get_position(Position::G1), Some(Piece::WhiteBishop));
        assert_eq!(game.get_position(Position::G2), Some(Piece::WhitePawn));
        assert_eq!(game.get_position(Position::G3), Some(Piece::WhiteKnight));
        assert_eq!(game.get_position(Position::G4), Some(Piece::WhiteQueen));
        assert_eq!(game.get_position(Position::G5), Some(Piece::WhiteKing));
        assert_eq!(game.get_position(Position::G6), Some(Piece::WhiteRook));
        assert_eq!(game.get_position(Position::G7), Some(Piece::WhiteBishop));
    }

    #[test]
    fn test_setting_a_position_clears_other_bitboards() {
        let mut game = Game::new();
        game.set_position(Position::F11, Piece::BlackBishop);
        assert_eq!(game.bitboard_black_bishop.is_position_set(Position::F11), true);
        game.set_position(Position::F11, Piece::BlackKing);
        assert_eq!(game.bitboard_black_bishop.is_position_set(Position::F11), false);
        assert_eq!(game.bitboard_black_king.is_position_set(Position::F11), true);
    }

    #[test]
    fn test_clear_position() {
        let mut game = Game::init();
        game.set_position(Position::F11, Piece::BlackBishop);
        game.clear_position(Position::F11);
        assert_eq!(game.get_position(Position::F11), None);
    }

    #[test]
    fn test_bitmask() {
        let mask = bitmask!("x/3/5/7/9/11/11/11/11/11/11");
        let game = Game::parse("p/3/5/7/9/11/11/11/11/11/11 w - 0 1").unwrap();
        assert_eq!(game.bitboard_black_pawn.0, mask);
    }

    #[test]
    fn test_to_array() {
        assert_eq!(Game::init().to_array(), [
            Some(Piece::BlackBishop),
            Some(Piece::BlackQueen),
            Some(Piece::BlackBishop),
            Some(Piece::BlackKing),
            Some(Piece::BlackKnight),
            None,
            Some(Piece::BlackBishop),
            None,
            Some(Piece::BlackKnight),
            Some(Piece::BlackRook),
            None,
            None,
            None,
            None,
            None,
            Some(Piece::BlackRook),
            Some(Piece::BlackPawn),
            Some(Piece::BlackPawn),
            Some(Piece::BlackPawn),
            Some(Piece::BlackPawn),
            Some(Piece::BlackPawn),
            Some(Piece::BlackPawn),
            Some(Piece::BlackPawn),
            Some(Piece::BlackPawn),
            Some(Piece::BlackPawn),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(Piece::WhitePawn),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(Piece::WhitePawn),
            None,
            Some(Piece::WhitePawn),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(Piece::WhitePawn),
            None,
            Some(Piece::WhiteBishop),
            None,
            Some(Piece::WhitePawn),
            None,
            None,
            None,
            None,
            None,
            Some(Piece::WhitePawn),
            None,
            None,
            Some(Piece::WhiteBishop),
            None,
            None,
            Some(Piece::WhitePawn),
            None,
            None,
            None,
            Some(Piece::WhitePawn),
            Some(Piece::WhiteRook),
            Some(Piece::WhiteKnight),
            Some(Piece::WhiteQueen),
            Some(Piece::WhiteBishop),
            Some(Piece::WhiteKing),
            Some(Piece::WhiteKnight),
            Some(Piece::WhiteRook),
            Some(Piece::WhitePawn),
            None
        ]);
    }

    #[test]
    fn test_to_string() {
        let game = Game::init();

        assert_eq!(game.to_string(), INITIAL_POSITION);
    }

    #[test]
    fn test_get_color_bitboard() {
        let game = Game::init();
        let black = bitmask!("x/xxx/x1x1x/x5x/xxxxxxxxx/11/11/11/11/11/11");
        let white = bitmask!("1/3/5/7/9/11/5x5/4x1x4/3x1x1x3/2x2x2x2/1xxxxxxxxx1");

        assert_eq!(game.get_color_bitboard(Color::Black).0, black);
        assert_eq!(game.get_color_bitboard(Color::White).0, white);
    }
}
