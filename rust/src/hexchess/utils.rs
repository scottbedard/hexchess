use crate::constants::HEXBOARD_GRAPH;
use crate::{color, h};
use crate::hexchess::color::Color;
use crate::hexchess::hexchess::Hexchess;
use crate::hexchess::piece::Piece;
use smallvec::SmallVec;

/// test if position is black en passant target
pub fn is_legal_black_en_passant(position: &u8) -> bool {
    match position {
        h!("b6") |
        h!("c6") |
        h!("d6") |
        h!("e6") |
        h!("f6") |
        h!("g6") |
        h!("h6") |
        h!("i6") |
        h!("k6") => true,
        _ => false,
    }
}

/// test if position is white en passant target
pub fn is_legal_white_en_passant_position(position: &u8) -> bool {
    match position {
        h!("b2") |
        h!("c3") |
        h!("d4") |
        h!("e5") |
        h!("f6") |
        h!("g5") |
        h!("h4") |
        h!("i3") |
        h!("k2") => true,
        _ => false,
    }
}

/// test if position is en passant target
#[inline(always)]
pub fn is_legal_en_passant(position: &u8) -> bool {
    is_legal_black_en_passant(position) || is_legal_white_en_passant_position(position)
}

/// test if position is black promotion position
pub fn is_black_promotion_position(position: &u8) -> bool {
    match position {
        h!("a1") |
        h!("b1") |
        h!("c1") |
        h!("d1") |
        h!("e1") |
        h!("f1") |
        h!("g1") |
        h!("h1") |
        h!("i1") |
        h!("k1") |
        h!("l1") => true,
        _ => false,
    }
}

/// test if position is on first or last rank
pub fn is_white_promotion_position(position: &u8) -> bool {
    match position {
        h!("f11") |
        h!("e10") |
        h!("g10") |
        h!("d9") |
        h!("h9") |
        h!("c8") |
        h!("i8") |
        h!("b7") |
        h!("k7") |
        h!("a6") |
        h!("l6") => true,
        _ => false,
    }
}

/// test if position is a promotion position
pub fn is_promotion_position(position: &u8) -> bool {
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

/// convert index to position name
pub fn position(index: &u8) -> &'static str {
    match index {
        0 => "f11",
        1 => "e10",
        2 => "f10",
        3 => "g10",
        4 => "d9",
        5 => "e9",
        6 => "f9",
        7 => "g9",
        8 => "h9",
        9 => "c8",
        10 => "d8",
        11 => "e8",
        12 => "f8",
        13 => "g8",
        14 => "h8",
        15 => "i8",
        16 => "b7",
        17 => "c7",
        18 => "d7",
        19 => "e7",
        20 => "f7",
        21 => "g7",
        22 => "h7",
        23 => "i7",
        24 => "k7",
        25 => "a6",
        26 => "b6",
        27 => "c6",
        28 => "d6",
        29 => "e6",
        30 => "f6",
        31 => "g6",
        32 => "h6",
        33 => "i6",
        34 => "k6",
        35 => "l6",
        36 => "a5",
        37 => "b5",
        38 => "c5",
        39 => "d5",
        40 => "e5",
        41 => "f5",
        42 => "g5",
        43 => "h5",
        44 => "i5",
        45 => "k5",
        46 => "l5",
        47 => "a4",
        48 => "b4",
        49 => "c4",
        50 => "d4",
        51 => "e4",
        52 => "f4",
        53 => "g4",
        54 => "h4",
        55 => "i4",
        56 => "k4",
        57 => "l4",
        58 => "a3",
        59 => "b3",
        60 => "c3",
        61 => "d3",
        62 => "e3",
        63 => "f3",
        64 => "g3",
        65 => "h3",
        66 => "i3",
        67 => "k3",
        68 => "l3",
        69 => "a2",
        70 => "b2",
        71 => "c2",
        72 => "d2",
        73 => "e2",
        74 => "f2",
        75 => "g2",
        76 => "h2",
        77 => "i2",
        78 => "k2",
        79 => "l2",
        80 => "a1",
        81 => "b1",
        82 => "c1",
        83 => "d1",
        84 => "e1",
        85 => "f1",
        86 => "g1",
        87 => "h1",
        88 => "i1",
        89 => "k1",
        90 => "l1",
        _ => panic!("invalid position index: {}", index),
    }
}

// step along the hexboard graph
pub fn step(from: u8, direction: u8) -> Option<u8> {
    HEXBOARD_GRAPH[from as usize][direction as usize]
}

/// walk along the board in a given direction
pub fn walk(hexchess: &Hexchess, from: u8, direction: u8, color: &Color) -> SmallVec<[u8; 11]> {
    let mut path: SmallVec<[u8; 11]> = SmallVec::new();
    let mut position: u8 = from;

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
pub fn walk_until_piece(hexchess: &Hexchess, from: u8, direction: u8) -> Option<Piece> {
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

    #[test]
    #[should_panic]
    fn test_position_panics_on_out_of_bounds() {
        position(&91);
    }

    #[test]
    fn test_walk_until_piece() {
        let hexchess = Hexchess::parse("1/3/2r2/7/9/11/11/11/5R5/11/11 w - 0 1").unwrap();
        assert_eq!(walk_until_piece(&hexchess, h!("f3"), 0), Some(Piece::BlackRook));
        assert_eq!(walk_until_piece(&hexchess, h!("f3"), 2), None);
    }
}
