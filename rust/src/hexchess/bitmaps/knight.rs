extern crate hexchess_bitmask;

use crate::Bitboard;
use crate::Hexchess;
use crate::Position;
use crate::San;
use hexchess_bitmask::bitmask_csv;

const KNIGHT_MOVES_BITMASKS: [u128; 91] = [
    bitmask_csv!("h8, g8, e8, d8"), // F11
    bitmask_csv!("h9, g8, f8, d7, c7"), // E10
    bitmask_csv!("i8, h7, g7, e7, d7, c8"), // F10
    bitmask_csv!("i7, h7, f8, e8, d9"), // G10
    bitmask_csv!("g10, g9, f8, e7, c6, b6"), // D9
    bitmask_csv!("h9, h8, g7, f7, d6, c6, b7"), // E9
    bitmask_csv!("i8, i7, h6, g6, e6, d6, c7, c8"), // F9
    bitmask_csv!("k7, i6, h6, f7, e7, d8, d9"), // G9
    bitmask_csv!("k6, i6, g7, f8, e9, e10"), // H9
    bitmask_csv!("f10, f9, e7, d6, b5, a5"), // C8
    bitmask_csv!("f11, g9, g8, f7, e6, c5, b5, a6"), // D8
    bitmask_csv!("f11, g10, h8, h7, g6, f6, d5, c5, b6, b7"), // E8
    bitmask_csv!("g10, h9, i7, i6, h5, g5, e5, d5, c6, c7, d9, e10"), // F8
    bitmask_csv!("k7, k6, i5, h5, f6, e6, d7, d8, e10, f11"), // G8
    bitmask_csv!("l6, k5, i5, g6, f7, e8, e9, f11"), // H8
    bitmask_csv!("l5, k5, h6, g7, f9, f10"), // I8
    bitmask_csv!("e9, e8, d6, c5, a4"), // B7
    bitmask_csv!("e10, f9, f8, e6, d5, b4, a4"), // C7
    bitmask_csv!("e10, f10, g8, g7, f6, e5, c4, b4, a5, a6"), // D7
    bitmask_csv!("f10, g9, h7, h6, g5, f5, d4, c4, b5, b6, c8, d9"), // E7
    bitmask_csv!("g9, h8, i6, i5, h4, g4, e4, d4, c5, c6, d8, e9"), // F7
    bitmask_csv!("h9, i8, k6, k5, i4, h4, f5, e5, d6, d7, e9, f10"), // G7
    bitmask_csv!("l6, l5, k4, i4, g5, f6, e7, e8, f10, g10"), // H7
    bitmask_csv!("l4, k4, h5, g6, f8, f9, g10"), // I7
    bitmask_csv!("l4, i5, h6, g8, g9"), // K7
    bitmask_csv!("d8, d7, c5, b4"), // A6
    bitmask_csv!("d9, e8, e7, d5, c4, a3"), // B6
    bitmask_csv!("d9, e9, f8, f7, e5, d4, b3, a3"), // C6
    bitmask_csv!("e9, f9, g7, g6, f5, e4, c3, b3, a4, a5, b7, c8"), // D6
    bitmask_csv!("f9, g8, h6, h5, g4, f4, d3, c3, b4, b5, c7, d8"), // E6
    bitmask_csv!("g8, h7, i5, i4, h3, g3, e3, d3, c4, c5, d7, e8"), // F6
    bitmask_csv!("h8, i7, k5, k4, i3, h3, f4, e4, d5, d6, e8, f9"), // G6
    bitmask_csv!("i8, k7, l5, l4, k3, i3, g4, f5, e6, e7, f9, g9"), // H6
    bitmask_csv!("l3, k3, h4, g5, f7, f8, g9, h9"), // I6
    bitmask_csv!("l3, i4, h5, g7, g8, h9"), // K6
    bitmask_csv!("k4, i5, h7, h8"), // L6
    bitmask_csv!("c8, d7, d6, c4, b3"), // A5
    bitmask_csv!("c8, d8, e7, e6, d4, c3, a2"), // B5
    bitmask_csv!("d8, e8, f7, f6, e4, d3, b2, a2, a6, b7"), // C5
    bitmask_csv!("e8, f8, g6, g5, f4, e3, c2, b2, a3, a4, b6, c7"), // D5
    bitmask_csv!("f8, g7, h5, h4, g3, f3, d2, c2, b3, b4, c6, d7"), // E5
    bitmask_csv!("g7, h6, i4, i3, h2, g2, e2, d2, c3, c4, d6, e7"), // F5
    bitmask_csv!("h7, i6, k4, k3, i2, h2, f3, e3, d4, d5, e7, f8"), // G5
    bitmask_csv!("i7, k6, l4, l3, k2, i2, g3, f4, e5, e6, f8, g8"), // H5
    bitmask_csv!("k7, l6, l2, k2, h3, g4, f6, f7, g8, h8"), // I5
    bitmask_csv!("l2, i3, h4, g6, g7, h8, i8"), // K5
    bitmask_csv!("k3, i4, h6, h7, i8"), // L5
    bitmask_csv!("b7, c7, d6, d5, c3, b2"), // A4
    bitmask_csv!("c7, d7, e6, e5, d3, c2, a1, a6"), // B4
    bitmask_csv!("d7, e7, f6, f5, e3, d2, b1, a1, a5, b6"), // C4
    bitmask_csv!("e7, f7, g5, g4, f3, e2, c1, b1, a2, a3, b5, c6"), // D4
    bitmask_csv!("f7, g6, h4, h3, g2, f2, d1, c1, b2, b3, c5, d6"), // E4
    bitmask_csv!("g6, h5, i3, i2, h1, g1, e1, d1, c2, c3, d5, e6"), // F4
    bitmask_csv!("h6, i5, k3, k2, i1, h1, f2, e2, d3, d4, e6, f7"), // G4
    bitmask_csv!("i6, k5, l3, l2, k1, i1, g2, f3, e4, e5, f7, g7"), // H4
    bitmask_csv!("k6, l5, l1, k1, h2, g3, f5, f6, g7, h7"), // I4
    bitmask_csv!("l6, l1, i2, h3, g5, g6, h7, i7"), // K4
    bitmask_csv!("k2, i3, h5, h6, i7, k7"), // L4
    bitmask_csv!("b6, c6, d5, d4, c2, b1"), // A3
    bitmask_csv!("c6, d6, e5, e4, d2, c1, a5"), // B3
    bitmask_csv!("d6, e6, f5, f4, e2, d1, a4, b5"), // C3
    bitmask_csv!("e6, f6, g4, g3, f2, e1, a1, a2, b4, c5"), // D3
    bitmask_csv!("f6, g5, h3, h2, g1, f1, b1, b2, c4, d5"), // E3
    bitmask_csv!("g5, h4, i2, i1, c1, c2, d4, e5"), // F3
    bitmask_csv!("h5, i4, k2, k1, f1, e1, d2, d3, e5, f6"), // G3
    bitmask_csv!("i5, k4, l2, l1, g1, f2, e3, e4, f6, g6"), // H3
    bitmask_csv!("k5, l4, h1, g2, f4, f5, g6, h6"), // I3
    bitmask_csv!("l5, i1, h2, g4, g5, h6, i6"), // K3
    bitmask_csv!("k1, i2, h4, h5, i6, k6"), // L3
    bitmask_csv!("b5, c5, d4, d3, c1"), // A2
    bitmask_csv!("c5, d5, e4, e3, d1, a4"), // B2
    bitmask_csv!("d5, e5, f4, f3, e1, a3, b4"), // C2
    bitmask_csv!("e5, f5, g3, g2, f1, a1, b3, c4"), // D2
    bitmask_csv!("f5, g4, h2, h1, b1, c3, d4"), // E2
    bitmask_csv!("g4, h3, i1, c1, d3, e4"), // F2
    bitmask_csv!("h4, i3, k1, d1, d2, e4, f5"), // G2
    bitmask_csv!("i4, k3, l1, f1, e2, e3, f5, g5"), // H2
    bitmask_csv!("k4, l3, g1, f3, f4, g5, h5"), // I2
    bitmask_csv!("l4, h1, g3, g4, h5, i5"), // K2
    bitmask_csv!("i1, h3, h4, i5, k5"), // L2
    bitmask_csv!("b4, c4, d3, d2"), // A1
    bitmask_csv!("c4, d4, e3, e2, a3"), // B1
    bitmask_csv!("d4, e4, f3, f2, a2, b3"), // C1
    bitmask_csv!("e4, f4, g2, g1, b2, c3"), // D1
    bitmask_csv!("f4, g3, h1, c2, d3"), // E1
    bitmask_csv!("g3, h2, d2, e3"), // F1
    bitmask_csv!("h3, i2, d1, e3, f4"), // G1
    bitmask_csv!("i3, k2, e1, e2, f4, g4"), // H1
    bitmask_csv!("k3, l2, f2, f3, g4, h4"), // I1
    bitmask_csv!("l3, g2, g3, h4, i4"), // K1
    bitmask_csv!("h2, h3, i4, k4"), // L1
];

pub fn get_knight_moves_bitmask(position: Position) -> u128 {
    let index = position.to_fen_index() as usize;
    KNIGHT_MOVES_BITMASKS[index]
}

pub fn get_knight_moves_unsafe(hexchess: &Hexchess, from: Position) -> Vec<San> {
    let mut targets = Bitboard(get_knight_moves_bitmask(from));

    match hexchess.get_color(from) {
        Some(color) => targets &= !hexchess.get_color_bitboard(color),
        None => {}
    };

    let mut output = Vec::with_capacity(targets.count_ones() as usize);

    targets.iter_bits(|index| {
        let to = Position::from_bitboard_index(index as u8);
        let san = San::new(from, to);
        output.push(san);
    });

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_knight_moves_unsafe_f11() {
        let expected = ["f11h8", "f11g8", "f11d8", "f11e8"];

        let game = Hexchess::new();

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

        let game = Hexchess::new();

        let sans = get_knight_moves_unsafe(&game, Position::F6)
            .iter()
            .map(|san| san.to_string())
            .collect::<Vec<String>>();

        assert_eq!(sans.len(), 12);

        for san in sans {
            assert!(expected.contains(&san.as_str()));
        }
    }

    #[test]
    fn test_all_positions_have_moves() {
        for n in 0..91 {
            let position = Position::from_fen_index(n as u8);
            let moves = get_knight_moves_unsafe(&Hexchess::new(), position);

            assert!(moves.len() > 0);
        }
    }
}
