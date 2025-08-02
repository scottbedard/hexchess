use crate::constants::hexboard_graph;
use crate::hexchess::color::Color;
use crate::hexchess::hexchess::Hexchess;
use crate::hexchess::piece::Piece;
use crate::hexchess::position::Position;
use crate::position;
use smallvec::SmallVec;

/// test if position is black en passant target
pub fn is_legal_black_en_passant(position: &Position) -> bool {
    match position {
        position!("b6") |
        position!("c6") |
        position!("d6") |
        position!("e6") |
        position!("f6") |
        position!("g6") |
        position!("h6") |
        position!("i6") |
        position!("k6") => true,
        _ => false,
    }
}

/// test if position is white en passant target
pub fn is_legal_white_en_passant_position(position: &Position) -> bool {
    match position {
        position!("b2") |
        position!("c3") |
        position!("d4") |
        position!("e5") |
        position!("f6") |
        position!("g5") |
        position!("h4") |
        position!("i3") |
        position!("k2") => true,
        _ => false,
    }
}

/// test if position is en passant target
#[inline(always)]
pub fn is_legal_en_passant(position: &Position) -> bool {
    is_legal_black_en_passant(position) || is_legal_white_en_passant_position(position)
}

/// test if position is black promotion position
pub fn is_black_promotion_position(position: &Position) -> bool {
    match position {
        position!("a1") |
        position!("b1") |
        position!("c1") |
        position!("d1") |
        position!("e1") |
        position!("f1") |
        position!("g1") |
        position!("h1") |
        position!("i1") |
        position!("k1") |
        position!("l1") => true,
        _ => false,
    }
}

/// test if position is on first or last rank
pub fn is_white_promotion_position(position: &Position) -> bool {
    match position {
        position!("f11") |
        position!("e10") |
        position!("g10") |
        position!("d9") |
        position!("h9") |
        position!("c8") |
        position!("i8") |
        position!("b7") |
        position!("k7") |
        position!("a6") |
        position!("l6") => true,
        _ => false,
    }
}

/// test if position is a promotion position
pub fn is_promotion_position(position: &Position) -> bool {
    is_black_promotion_position(position) || is_white_promotion_position(position)
}

/// convert fen position string  to it's index
pub fn fen_index(source: &str) -> Result<u8, &str> {
    match source {
        "f11" => Ok(0),
        "e10" => Ok(1),
        "f10" => Ok(2),
        "g10" => Ok(3),
        "d9" => Ok(4),
        "e9" => Ok(5),
        "f9" => Ok(6),
        "g9" => Ok(7),
        "h9" => Ok(8),
        "c8" => Ok(9),
        "d8" => Ok(10),
        "e8" => Ok(11),
        "f8" => Ok(12),
        "g8" => Ok(13),
        "h8" => Ok(14),
        "i8" => Ok(15),
        "b7" => Ok(16),
        "c7" => Ok(17),
        "d7" => Ok(18),
        "e7" => Ok(19),
        "f7" => Ok(20),
        "g7" => Ok(21),
        "h7" => Ok(22),
        "i7" => Ok(23),
        "k7" => Ok(24),
        "a6" => Ok(25),
        "b6" => Ok(26),
        "c6" => Ok(27),
        "d6" => Ok(28),
        "e6" => Ok(29),
        "f6" => Ok(30),
        "g6" => Ok(31),
        "h6" => Ok(32),
        "i6" => Ok(33),
        "k6" => Ok(34),
        "l6" => Ok(35),
        "a5" => Ok(36),
        "b5" => Ok(37),
        "c5" => Ok(38),
        "d5" => Ok(39),
        "e5" => Ok(40),
        "f5" => Ok(41),
        "g5" => Ok(42),
        "h5" => Ok(43),
        "i5" => Ok(44),
        "k5" => Ok(45),
        "l5" => Ok(46),
        "a4" => Ok(47),
        "b4" => Ok(48),
        "c4" => Ok(49),
        "d4" => Ok(50),
        "e4" => Ok(51),
        "f4" => Ok(52),
        "g4" => Ok(53),
        "h4" => Ok(54),
        "i4" => Ok(55),
        "k4" => Ok(56),
        "l4" => Ok(57),
        "a3" => Ok(58),
        "b3" => Ok(59),
        "c3" => Ok(60),
        "d3" => Ok(61),
        "e3" => Ok(62),
        "f3" => Ok(63),
        "g3" => Ok(64),
        "h3" => Ok(65),
        "i3" => Ok(66),
        "k3" => Ok(67),
        "l3" => Ok(68),
        "a2" => Ok(69),
        "b2" => Ok(70),
        "c2" => Ok(71),
        "d2" => Ok(72),
        "e2" => Ok(73),
        "f2" => Ok(74),
        "g2" => Ok(75),
        "h2" => Ok(76),
        "i2" => Ok(77),
        "k2" => Ok(78),
        "l2" => Ok(79),
        "a1" => Ok(80),
        "b1" => Ok(81),
        "c1" => Ok(82),
        "d1" => Ok(83),
        "e1" => Ok(84),
        "f1" => Ok(85),
        "g1" => Ok(86),
        "h1" => Ok(87),
        "i1" => Ok(88),
        "k1" => Ok(89),
        "l1" => Ok(90),
        _ => Err(source),
    }
}

// step along the hexboard graph
pub fn step(from: Position, direction: u8) -> Option<Position> {
    hexboard_graph(from)[direction as usize]
}

/// walk along the board in a given direction
pub fn walk(hexchess: &Hexchess, from: Position, direction: u8, color: &Color) -> SmallVec<[Position; 11]> {
    let mut path: SmallVec<[Position; 11]> = SmallVec::new();
    let mut position = from;

    loop {
        position = match step(position, direction) {
            Some(index) => index,
            None => return path // <- end of board
        };

        let piece = match hexchess.board[position as usize] {
            Some(value) => value,
            None => {
                path.push(position); // <- unoccupied position
                continue;
            }
        };

        if piece.is_friendly(*color) {
            return path // <- shop short of friendly piece
        }
        
        path.push(position); // <- and captury enemy piece
        return path;
    }
}

/// walk along the board until a piece is found
pub fn walk_until_piece(hexchess: &Hexchess, from: Position, direction: u8) -> Option<Piece> {
    match step(from, direction) {
        Some(next) => match hexchess.board[next as usize] {
            Some(piece) => Some(piece),
            None => walk_until_piece(hexchess, next, direction), // unoccupied, continue walking
        },
        None => None, // end of board
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hexchess::hexchess::Hexchess;
    use crate::hexchess::position::Position;

    #[test]
    #[should_panic]
    fn test_position_panics_on_out_of_bounds() {
        Position::from_fen_index(91);
    }

    #[test]
    #[ignore]
    fn test_walk_until_piece() {
        let _hexchess = Hexchess::parse("1/3/2r2/7/9/11/11/11/5R5/11/11 w - 0 1").unwrap();

        let _result = position!("f3");

        // panic!("result: {:?}", walk_until_piece(&hexchess, position!("f3"), 0));


        // assert_eq!(walk_until_piece(&hexchess, position!("f3"), 0), Some(Piece::BlackRook));
        // assert_eq!(walk_until_piece(&hexchess, position!("f3"), 2), None);
    }
}
