extern crate hexchess_bitmask;

use crate::hexchess::bitboard::Bitboard;
use crate::hexchess::game::Game;
use crate::hexchess::position::Position;
use crate::hexchess::san::San;
use hexchess_bitmask::bitmask_csv;

pub fn get_knight_moves_unsafe(_game: &Game,  from: Position) -> Vec<San> {
    let move_mask = Bitboard(
        match from {
            Position::F11 => bitmask_csv!("h8, g8, e8, d8"), 
            Position::E10 => bitmask_csv!("h9, g8, f8, d7, c7"), 
            Position::F10 => bitmask_csv!("i8, h7, g7, e7, d7, c8"), 
            Position::G10 => bitmask_csv!("i7, h7, f8, e8, d9"), 
            Position::D9 => bitmask_csv!("g10, g9, f8, e7, c6, b6"), 
            Position::E9 => bitmask_csv!("h9, h8, g7, f7, d6, c6, b7"), 
            Position::F9 => bitmask_csv!("i8, i7, h6, g6, e6, d6, c7, c8"), 
            Position::G9 => bitmask_csv!("k7, i6, h6, f7, e7, d8, d9"), 
            Position::H9 => bitmask_csv!("k6, i6, g7, f8, e9, e10"), 
            Position::C8 => bitmask_csv!("f10, f9, e7, d6, b5, a5"), 
            Position::D8 => bitmask_csv!("f11, g9, g8, f7, e6, c5, b5, a6"), 
            Position::E8 => bitmask_csv!("f11, g10, h8, h7, g6, f6, d5, c5, b6, b7"), 
            Position::F8 => bitmask_csv!("g10, h9, i7, i6, h5, g5, e5, d5, c6, c7, d9, e10"), 
            Position::G8 => bitmask_csv!("k7, k6, i5, h5, f6, e6, d7, d8, e10, f11"), 
            Position::H8 => bitmask_csv!("l6, k5, i5, g6, f7, e8, e9, f11"), 
            Position::I8 => bitmask_csv!("l5, k5, h6, g7, f9, f10"), 
            Position::B7 => bitmask_csv!("e9, e8, d6, c5, a4"), 
            Position::C7 => bitmask_csv!("e10, f9, f8, e6, d5, b4, a4"), 
            Position::D7 => bitmask_csv!("e10, f10, g8, g7, f6, e5, c4, b4, a5, a6"), 
            Position::E7 => bitmask_csv!("f10, g9, h7, h6, g5, f5, d4, c4, b5, b6, c8, d9"), 
            Position::F7 => bitmask_csv!("g9, h8, i6, i5, h4, g4, e4, d4, c5, c6, d8, e9"), 
            Position::G7 => bitmask_csv!("h9, i8, k6, k5, i4, h4, f5, e5, d6, d7, e9, f10"), 
            Position::H7 => bitmask_csv!("l6, l5, k4, i4, g5, f6, e7, e8, f10, g10"), 
            Position::I7 => bitmask_csv!("l4, k4, h5, g6, f8, f9, g10"), 
            Position::K7 => bitmask_csv!("l4, i5, h6, g8, g9"), 
            Position::A6 => bitmask_csv!("d8, d7, c5, b4"), 
            Position::B6 => bitmask_csv!("d9, e8, e7, d5, c4, a3"), 
            Position::C6 => bitmask_csv!("d9, e9, f8, f7, e5, d4, b3, a3"), 
            Position::D6 => bitmask_csv!("e9, f9, g7, g6, f5, e4, c3, b3, a4, a5, b7, c8"), 
            Position::E6 => bitmask_csv!("f9, g8, h6, h5, g4, f4, d3, c3, b4, b5, c7, d8"), 
            Position::F6 => bitmask_csv!("g8, h7, i5, i4, h3, g3, e3, d3, c4, c5, d7, e8"), 
            Position::G6 => bitmask_csv!("h8, i7, k5, k4, i3, h3, f4, e4, d5, d6, e8, f9"), 
            Position::H6 => bitmask_csv!("i8, k7, l5, l4, k3, i3, g4, f5, e6, e7, f9, g9"), 
            Position::I6 => bitmask_csv!("l3, k3, h4, g5, f7, f8, g9, h9"), 
            Position::K6 => bitmask_csv!("l3, i4, h5, g7, g8, h9"), 
            Position::L6 => bitmask_csv!("k4, i5, h7, h8"), 
            Position::A5 => bitmask_csv!("c8, d7, d6, c4, b3"), 
            Position::B5 => bitmask_csv!("c8, d8, e7, e6, d4, c3, a2"), 
            Position::C5 => bitmask_csv!("d8, e8, f7, f6, e4, d3, b2, a2, a6, b7"), 
            Position::D5 => bitmask_csv!("e8, f8, g6, g5, f4, e3, c2, b2, a3, a4, b6, c7"), 
            Position::E5 => bitmask_csv!("f8, g7, h5, h4, g3, f3, d2, c2, b3, b4, c6, d7"), 
            Position::F5 => bitmask_csv!("g7, h6, i4, i3, h2, g2, e2, d2, c3, c4, d6, e7"), 
            Position::G5 => bitmask_csv!("h7, i6, k4, k3, i2, h2, f3, e3, d4, d5, e7, f8"), 
            Position::H5 => bitmask_csv!("i7, k6, l4, l3, k2, i2, g3, f4, e5, e6, f8, g8"), 
            Position::I5 => bitmask_csv!("k7, l6, l2, k2, h3, g4, f6, f7, g8, h8"), 
            Position::K5 => bitmask_csv!("l2, i3, h4, g6, g7, h8, i8"), 
            Position::L5 => bitmask_csv!("k3, i4, h6, h7, i8"), 
            Position::A4 => bitmask_csv!("b7, c7, d6, d5, c3, b2"), 
            Position::B4 => bitmask_csv!("c7, d7, e6, e5, d3, c2, a1, a6"), 
            Position::C4 => bitmask_csv!("d7, e7, f6, f5, e3, d2, b1, a1, a5, b6"), 
            Position::D4 => bitmask_csv!("e7, f7, g5, g4, f3, e2, c1, b1, a2, a3, b5, c6"), 
            Position::E4 => bitmask_csv!("f7, g6, h4, h3, g2, f2, d1, c1, b2, b3, c5, d6"), 
            Position::F4 => bitmask_csv!("g6, h5, i3, i2, h1, g1, e1, d1, c2, c3, d5, e6"), 
            Position::G4 => bitmask_csv!("h6, i5, k3, k2, i1, h1, f2, e2, d3, d4, e6, f7"), 
            Position::H4 => bitmask_csv!("i6, k5, l3, l2, k1, i1, g2, f3, e4, e5, f7, g7"), 
            Position::I4 => bitmask_csv!("k6, l5, l1, k1, h2, g3, f5, f6, g7, h7"), 
            Position::K4 => bitmask_csv!("l6, l1, i2, h3, g5, g6, h7, i7"), 
            Position::L4 => bitmask_csv!("k2, i3, h5, h6, i7, k7"), 
            Position::A3 => bitmask_csv!("b6, c6, d5, d4, c2, b1"), 
            Position::B3 => bitmask_csv!("c6, d6, e5, e4, d2, c1, a5"), 
            Position::C3 => bitmask_csv!("d6, e6, f5, f4, e2, d1, a4, b5"), 
            Position::D3 => bitmask_csv!("e6, f6, g4, g3, f2, e1, a1, a2, b4, c5"), 
            Position::E3 => bitmask_csv!("f6, g5, h3, h2, g1, f1, b1, b2, c4, d5"), 
            Position::F3 => bitmask_csv!("g5, h4, i2, i1, c1, c2, d4, e5"), 
            Position::G3 => bitmask_csv!("h5, i4, k2, k1, f1, e1, d2, d3, e5, f6"), 
            Position::H3 => bitmask_csv!("i5, k4, l2, l1, g1, f2, e3, e4, f6, g6"), 
            Position::I3 => bitmask_csv!("k5, l4, h1, g2, f4, f5, g6, h6"), 
            Position::K3 => bitmask_csv!("l5, i1, h2, g4, g5, h6, i6"), 
            Position::L3 => bitmask_csv!("k1, i2, h4, h5, i6, k6"), 
            Position::A2 => bitmask_csv!("b5, c5, d4, d3, c1"), 
            Position::B2 => bitmask_csv!("c5, d5, e4, e3, d1, a4"), 
            Position::C2 => bitmask_csv!("d5, e5, f4, f3, e1, a3, b4"), 
            Position::D2 => bitmask_csv!("e5, f5, g3, g2, f1, a1, b3, c4"), 
            Position::E2 => bitmask_csv!("f5, g4, h2, h1, b1, c3, d4"), 
            Position::F2 => bitmask_csv!("g4, h3, i1, c1, d3, e4"), 
            Position::G2 => bitmask_csv!("h4, i3, k1, d1, d2, e4, f5"), 
            Position::H2 => bitmask_csv!("i4, k3, l1, f1, e2, e3, f5, g5"), 
            Position::I2 => bitmask_csv!("k4, l3, g1, f3, f4, g5, h5"), 
            Position::K2 => bitmask_csv!("l4, h1, g3, g4, h5, i5"), 
            Position::L2 => bitmask_csv!("i1, h3, h4, i5, k5"), 
            Position::A1 => bitmask_csv!("b4, c4, d3, d2"), 
            Position::B1 => bitmask_csv!("c4, d4, e3, e2, a3"), 
            Position::C1 => bitmask_csv!("d4, e4, f3, f2, a2, b3"), 
            Position::D1 => bitmask_csv!("e4, f4, g2, g1, b2, c3"), 
            Position::E1 => bitmask_csv!("f4, g3, h1, c2, d3"), 
            Position::F1 => bitmask_csv!("g3, h2, d2, e3"), 
            Position::G1 => bitmask_csv!("h3, i2, d1, e3, f4"), 
            Position::H1 => bitmask_csv!("i3, k2, e1, e2, f4, g4"), 
            Position::I1 => bitmask_csv!("k3, l2, f2, f3, g4, h4"), 
            Position::K1 => bitmask_csv!("l3, g2, g3, h4, i4"), 
            Position::L1 => bitmask_csv!("h2, h3, i4, k4"), 
        }
    );

    let mut output = Vec::with_capacity(move_mask.count_ones() as usize);

    for index in move_mask.iter_set_bits() {
        let to = Position::from_bitboard_index(index);
        let san = San::new(from, to);
        output.push(san);
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_knight_moves_unsafe_f11() {
        let expected = ["f11h8", "f11g8", "f11d8", "f11e8"];

        let game = Game::new();

        let sans = get_knight_moves_unsafe(&game, Position::F11)
            .iter()
            .map(|san| san.to_string())
            .collect::<Vec<String>>();

        assert_eq!(sans.len(), 4);

        for san in sans {
            assert!(expected.contains(&san.as_str()));
        }
    }

    #[test]
    fn test_get_knight_moves_unsafe_f6() {
        let expected = [
            "f6g8",
            "f6h7",
            "f6e8",
            "f6i5",
            "f6d7",
            "f6i4",
            "f6c5",
            "f6h3",
            "f6c4",
            "f6g3",
            "f6d3",
            "f6e3",
        ];

        let game = Game::new();

        let sans = get_knight_moves_unsafe(&game, Position::F6)
            .iter()
            .map(|san| san.to_string())
            .collect::<Vec<String>>();

        assert_eq!(sans.len(), 12);

        for san in sans {
            assert!(expected.contains(&san.as_str()));
        }
    }
}
