use crate::hexchess::color::Color;
use crate::hexchess::game::Game;
use crate::hexchess::position::Position;
use crate::hexchess::san::San;
use hexchess_bitmask::{bitmask_csv};

pub fn get_diagonal_bitmask(position: Position) -> u128 {
    match position {
        Position::F11 => bitmask_csv!("g9, h7, i5, k3, l1, e9, d7, c5, b3, a1"),
        Position::E10 => bitmask_csv!("g10, f9, g7, h5, i3, k1, d8, c6, b4, a2"),
        Position::F10 => bitmask_csv!("h9, g8, h6, i4, k2, e8, d6, c4, b2, d9"),
        Position::G10 => bitmask_csv!("h8, i6, k4, l2, f9, e7, d5, c3, b1, e10"),
        Position::D9 => bitmask_csv!("f10, h9, e8, f7, g5, h3, i1, c7, b5, a3"),
        Position::E9 => bitmask_csv!("f11, g9, i8, f8, g6, h4, i2, d7, c5, b3, a1, c8"),
        Position::F9 => bitmask_csv!("g10, h8, k7, g7, h5, i3, k1, e7, d5, c3, b1, d8, b7, e10"),
        Position::G9 => bitmask_csv!("i8, h7, i5, k3, l1, f8, e6, d4, c2, e9, c8, f11"),
        Position::H9 => bitmask_csv!("i7, k5, l3, g8, f7, e5, d3, c1, f10, d9"),
        Position::C8 => bitmask_csv!("e9, g9, i8, d7, e6, f5, g3, h1, b6, a4"),
        Position::D8 => bitmask_csv!("e10, f9, h8, k7, e7, f6, g4, h2, c6, b4, a2, b7"),
        Position::E8 => bitmask_csv!("f10, g8, i7, l6, f7, g5, h3, i1, d6, c4, b2, c7, a6, d9"),
        Position::F8 => bitmask_csv!("g9, h7, k6, g6, h4, i2, e6, d4, c2, d7, b6, e9"),
        Position::G8 => bitmask_csv!("h9, i7, l6, h6, i4, k2, f7, e5, d3, c1, e8, c7, a6, f10"),
        Position::H8 => bitmask_csv!("k7, i6, k4, l2, g7, f6, e4, d2, f9, d8, b7, g10"),
        Position::I8 => bitmask_csv!("k6, l4, h7, g6, f5, e3, d1, g9, e9, c8"),
        Position::B7 => bitmask_csv!("d8, f9, h8, k7, c6, d5, e4, f3, g1, a5"),
        Position::C7 => bitmask_csv!("d9, e8, g8, i7, l6, d6, e5, f4, g2, b5, a3, a6"),
        Position::D7 => bitmask_csv!("e9, f11, f8, h7, k6, e6, f5, g3, h1, c5, b3, a1, b6, c8"),
        Position::E7 => bitmask_csv!("f9, g10, g7, i6, l5, f6, g4, h2, d5, c3, b1, c6, a5, d8"),
        Position::F7 => bitmask_csv!("g8, h9, h6, k5, g5, h3, i1, e5, d3, c1, d6, b5, e8, d9"),
        Position::G7 => bitmask_csv!("h8, i6, l5, h5, i3, k1, f6, e4, d2, e7, c6, a5, f9, e10"),
        Position::H7 => bitmask_csv!("i8, k6, i5, k3, l1, g6, f5, e3, d1, f8, d7, b6, g9, f11"),
        Position::I7 => bitmask_csv!("l6, k5, l3, h6, g5, f4, e2, g8, e8, c7, a6, h9"),
        Position::K7 => bitmask_csv!("l5, i6, h5, g4, f3, e1, h8, f9, d8, b7"),
        Position::A6 => bitmask_csv!("c7, e8, g8, i7, l6, b5, c4, d3, e2, f1"),
        Position::B6 => bitmask_csv!("c8, d7, f8, h7, k6, c5, d4, e3, f2, a4"),
        Position::C6 => bitmask_csv!("d8, e10, e7, g7, i6, l5, d5, e4, f3, g1, b4, a2, a5, b7"),
        Position::D6 => bitmask_csv!("e8, f10, f7, h6, k5, e5, f4, g2, c4, b2, b5, c7"),
        Position::E6 => bitmask_csv!("f8, g9, g6, i5, l4, f5, g3, h1, d4, c2, c5, a4, d7, c8"),
        Position::F6 => bitmask_csv!("g7, h8, h5, k4, g4, h2, e4, d2, d5, b4, e7, d8"),
        Position::G6 => bitmask_csv!("h7, i8, i5, l4, h4, i2, f5, e3, d1, e6, c5, a4, f8, e9"),
        Position::H6 => bitmask_csv!("i7, k5, i4, k2, g5, f4, e2, f7, d6, b5, g8, f10"),
        Position::I6 => bitmask_csv!("k7, l5, k4, l2, h5, g4, f3, e1, g7, e7, c6, a5, h8, g10"),
        Position::K6 => bitmask_csv!("l4, i5, h4, g3, f2, h7, f8, d7, b6, i8"),
        Position::L6 => bitmask_csv!("k5, i4, h3, g2, f1, i7, g8, e8, c7, a6"),
        Position::A5 => bitmask_csv!("b7, c6, e7, g7, i6, l5, b4, c3, d2, e1"),
        Position::B5 => bitmask_csv!("c7, d9, d6, f7, h6, k5, c4, d3, e2, f1, a3, a6"),
        Position::C5 => bitmask_csv!("d7, e9, f11, e6, g6, i5, l4, d4, e3, f2, b3, a1, a4, b6"),
        Position::D5 => bitmask_csv!("e7, f9, g10, f6, h5, k4, e4, f3, g1, c3, b1, b4, c6, b7"),
        Position::E5 => bitmask_csv!("f7, g8, h9, g5, i4, l3, f4, g2, d3, c1, c4, a3, d6, c7"),
        Position::F5 => bitmask_csv!("g6, h7, i8, h4, k3, g3, h1, e3, d1, d4, b3, e6, d7, c8"),
        Position::G5 => bitmask_csv!("h6, i7, i4, l3, h3, i1, f4, e2, e5, c4, a3, f7, e8, d9"),
        Position::H5 => bitmask_csv!("i6, k7, k4, i3, k1, g4, f3, e1, f6, d5, b4, g7, f9, e10"),
        Position::I5 => bitmask_csv!("k6, l4, k3, l1, h4, g3, f2, g6, e6, c5, a4, h7, g9, f11"),
        Position::K5 => bitmask_csv!("l6, l3, i4, h3, g2, f1, h6, f7, d6, b5, i7, h9"),
        Position::L5 => bitmask_csv!("k4, i3, h2, g1, i6, g7, e7, c6, a5, k7"),
        Position::A4 => bitmask_csv!("b6, c8, c5, e6, g6, i5, l4, b3, c2, d1"),
        Position::B4 => bitmask_csv!("c6, d8, e10, d5, f6, h5, k4, c3, d2, e1, a2, a5"),
        Position::C4 => bitmask_csv!("d6, e8, f10, e5, g5, i4, l3, d3, e2, f1, b2, a3, b5, a6"),
        Position::D4 => bitmask_csv!("e6, f8, g9, f5, h4, k3, e3, f2, c2, b3, c5, b6"),
        Position::E4 => bitmask_csv!("f6, g7, h8, g4, i3, l2, f3, g1, d2, c3, a2, d5, c6, b7"),
        Position::F4 => bitmask_csv!("g5, h6, i7, h3, k2, g2, e2, d3, b2, e5, d6, c7"),
        Position::G4 => bitmask_csv!("h5, i6, k7, i3, l2, h2, f3, e1, e4, c3, a2, f6, e7, d8"),
        Position::H4 => bitmask_csv!("i5, k6, k3, i2, g3, f2, f5, d4, b3, g6, f8, e9"),
        Position::I4 => bitmask_csv!("k5, l6, l3, k2, h3, g2, f1, g5, e5, c4, a3, h6, g8, f10"),
        Position::K4 => bitmask_csv!("l5, l2, i3, h2, g1, h5, f6, d5, b4, i6, h8, g10"),
        Position::L4 => bitmask_csv!("k3, i2, h1, i5, g6, e6, c5, a4, k6, i8"),
        Position::A3 => bitmask_csv!("b5, c7, d9, c4, e5, g5, i4, l3, b2, c1"),
        Position::B3 => bitmask_csv!("c5, d7, e9, f11, d4, f5, h4, k3, c2, d1, a1, a4"),
        Position::C3 => bitmask_csv!("d5, e7, f9, g10, e4, g4, i3, l2, d2, e1, b1, a2, b4, a5"),
        Position::D3 => bitmask_csv!("e5, f7, g8, h9, f4, h3, k2, e2, f1, c1, b2, c4, b5, a6"),
        Position::E3 => bitmask_csv!("f5, g6, h7, i8, g3, i2, l1, f2, d1, c2, a1, d4, c5, b6"),
        Position::F3 => bitmask_csv!("g4, h5, i6, k7, h2, k1, g1, e1, d2, b1, e4, d5, c6, b7"),
        Position::G3 => bitmask_csv!("h4, i5, k6, i2, l1, h1, f2, e3, c2, a1, f5, e6, d7, c8"),
        Position::H3 => bitmask_csv!("i4, k5, l6, k2, i1, g2, f1, f4, d3, b2, g5, f7, e8, d9"),
        Position::I3 => bitmask_csv!("k4, l5, l2, k1, h2, g1, g4, e4, c3, a2, h5, g7, f9, e10"),
        Position::K3 => bitmask_csv!("l4, l1, i2, h1, h4, f5, d4, b3, i5, h7, g9, f11"),
        Position::L3 => bitmask_csv!("k2, i1, i4, g5, e5, c4, a3, k5, i7, h9"),
        Position::A2 => bitmask_csv!("b4, c6, d8, e10, c3, e4, g4, i3, l2, b1"),
        Position::B2 => bitmask_csv!("c4, d6, e8, f10, d3, f4, h3, k2, c1, a3"),
        Position::C2 => bitmask_csv!("d4, e6, f8, g9, e3, g3, i2, l1, d1, a1, b3, a4"),
        Position::D2 => bitmask_csv!("e4, f6, g7, h8, f3, h2, k1, e1, b1, c3, b4, a5"),
        Position::E2 => bitmask_csv!("f4, g5, h6, i7, g2, i1, f1, c1, d3, c4, b5, a6"),
        Position::F2 => bitmask_csv!("g3, h4, i5, k6, h1, d1, e3, d4, c5, b6"),
        Position::G2 => bitmask_csv!("h3, i4, k5, l6, i1, f1, e2, c1, f4, e5, d6, c7"),
        Position::H2 => bitmask_csv!("i3, k4, l5, k1, g1, f3, d2, b1, g4, f6, e7, d8"),
        Position::I2 => bitmask_csv!("k3, l4, l1, h1, g3, e3, c2, a1, h4, g6, f8, e9"),
        Position::K2 => bitmask_csv!("l3, i1, h3, f4, d3, b2, i4, h6, g8, f10"),
        Position::L2 => bitmask_csv!("k1, i3, g4, e4, c3, a2, k4, i6, h8, g10"),
        Position::A1 => bitmask_csv!("b3, c5, d7, e9, f11, c2, e3, g3, i2, l1"),
        Position::B1 => bitmask_csv!("c3, d5, e7, f9, g10, d2, f3, h2, k1, a2"),
        Position::C1 => bitmask_csv!("d3, e5, f7, g8, h9, e2, g2, i1, b2, a3"),
        Position::D1 => bitmask_csv!("e3, f5, g6, h7, i8, f2, h1, c2, b3, a4"),
        Position::E1 => bitmask_csv!("f3, g4, h5, i6, k7, g1, d2, c3, b4, a5"),
        Position::F1 => bitmask_csv!("g2, h3, i4, k5, l6, e2, d3, c4, b5, a6"),
        Position::G1 => bitmask_csv!("h2, i3, k4, l5, e1, f3, e4, d5, c6, b7"),
        Position::H1 => bitmask_csv!("i2, k3, l4, f2, d1, g3, f5, e6, d7, c8"),
        Position::I1 => bitmask_csv!("k2, l3, g2, e2, c1, h3, g5, f7, e8, d9"),
        Position::K1 => bitmask_csv!("l2, h2, f3, d2, b1, i3, h5, g7, f9, e10"),
        Position::L1 => bitmask_csv!("i2, g3, e3, c2, a1, k3, i5, h7, g9, f11"),
    }
}

pub fn get_orthogonal_bitmask(position: Position) -> u128 {
    match position {
        Position::F11 => bitmask_csv!("g10, h9, i8, k7, l6, f10, f9, f8, f7, f6, f5, f4, f3, f2, f1, e10, d9, c8, b7, a6"),
        Position::E10 => bitmask_csv!("f11, f10, g9, h8, i7, k6, l5, e9, e8, e7, e6, e5, e4, e3, e2, e1, d9, c8, b7, a6"),
        Position::F10 => bitmask_csv!("f11, g10, g9, h8, i7, k6, l5, f9, f8, f7, f6, f5, f4, f3, f2, f1, e9, d8, c7, b6, a5, e10"),
        Position::G10 => bitmask_csv!("h9, i8, k7, l6, g9, g8, g7, g6, g5, g4, g3, g2, g1, f10, e9, d8, c7, b6, a5, f11"),
        Position::D9 => bitmask_csv!("e10, f11, e9, f9, g8, h7, i6, k5, l4, d8, d7, d6, d5, d4, d3, d2, d1, c8, b7, a6"),
        Position::E9 => bitmask_csv!("e10, f10, g10, f9, g8, h7, i6, k5, l4, e8, e7, e6, e5, e4, e3, e2, e1, d8, c7, b6, a5, d9"),
        Position::F9 => bitmask_csv!("f10, f11, g9, h9, g8, h7, i6, k5, l4, f8, f7, f6, f5, f4, f3, f2, f1, e8, d7, c6, b5, a4, e9, d9"),
        Position::G9 => bitmask_csv!("g10, h9, h8, i7, k6, l5, g8, g7, g6, g5, g4, g3, g2, g1, f9, e8, d7, c6, b5, a4, f10, e10"),
        Position::H9 => bitmask_csv!("i8, k7, l6, h8, h7, h6, h5, h4, h3, h2, h1, g9, f9, e8, d7, c6, b5, a4, g10, f11"),
        Position::C8 => bitmask_csv!("d9, e10, f11, d8, e8, f8, g7, h6, i5, k4, l3, c7, c6, c5, c4, c3, c2, c1, b7, a6"),
        Position::D8 => bitmask_csv!("d9, e9, f10, g10, e8, f8, g7, h6, i5, k4, l3, d7, d6, d5, d4, d3, d2, d1, c7, b6, a5, c8"),
        Position::E8 => bitmask_csv!("e9, e10, f9, g9, h9, f8, g7, h6, i5, k4, l3, e7, e6, e5, e4, e3, e2, e1, d7, c6, b5, a4, d8, c8"),
        Position::F8 => bitmask_csv!("f9, f10, f11, g8, h8, i8, g7, h6, i5, k4, l3, f7, f6, f5, f4, f3, f2, f1, e7, d6, c5, b4, a3, e8, d8, c8"),
        Position::G8 => bitmask_csv!("g9, g10, h8, i8, h7, i6, k5, l4, g7, g6, g5, g4, g3, g2, g1, f8, e7, d6, c5, b4, a3, f9, e9, d9"),
        Position::H8 => bitmask_csv!("h9, i8, i7, k6, l5, h7, h6, h5, h4, h3, h2, h1, g8, f8, e7, d6, c5, b4, a3, g9, f10, e10"),
        Position::I8 => bitmask_csv!("k7, l6, i7, i6, i5, i4, i3, i2, i1, h8, g8, f8, e7, d6, c5, b4, a3, h9, g10, f11"),
        Position::B7 => bitmask_csv!("c8, d9, e10, f11, c7, d7, e7, f7, g6, h5, i4, k3, l2, b6, b5, b4, b3, b2, b1, a6"),
        Position::C7 => bitmask_csv!("c8, d8, e9, f10, g10, d7, e7, f7, g6, h5, i4, k3, l2, c6, c5, c4, c3, c2, c1, b6, a5, b7"),
        Position::D7 => bitmask_csv!("d8, d9, e8, f9, g9, h9, e7, f7, g6, h5, i4, k3, l2, d6, d5, d4, d3, d2, d1, c6, b5, a4, c7, b7"),
        Position::E7 => bitmask_csv!("e8, e9, e10, f8, g8, h8, i8, f7, g6, h5, i4, k3, l2, e6, e5, e4, e3, e2, e1, d6, c5, b4, a3, d7, c7, b7"),
        Position::F7 => bitmask_csv!("f8, f9, f10, f11, g7, h7, i7, k7, g6, h5, i4, k3, l2, f6, f5, f4, f3, f2, f1, e6, d5, c4, b3, a2, e7, d7, c7, b7"),
        Position::G7 => bitmask_csv!("g8, g9, g10, h7, i7, k7, h6, i5, k4, l3, g6, g5, g4, g3, g2, g1, f7, e6, d5, c4, b3, a2, f8, e8, d8, c8"),
        Position::H7 => bitmask_csv!("h8, h9, i7, k7, i6, k5, l4, h6, h5, h4, h3, h2, h1, g7, f7, e6, d5, c4, b3, a2, g8, f9, e9, d9"),
        Position::I7 => bitmask_csv!("i8, k7, k6, l5, i6, i5, i4, i3, i2, i1, h7, g7, f7, e6, d5, c4, b3, a2, h8, g9, f10, e10"),
        Position::K7 => bitmask_csv!("l6, k6, k5, k4, k3, k2, k1, i7, h7, g7, f7, e6, d5, c4, b3, a2, i8, h9, g10, f11"),
        Position::A6 => bitmask_csv!("b7, c8, d9, e10, f11, b6, c6, d6, e6, f6, g5, h4, i3, k2, l1, a5, a4, a3, a2, a1"),
        Position::B6 => bitmask_csv!("b7, c7, d8, e9, f10, g10, c6, d6, e6, f6, g5, h4, i3, k2, l1, b5, b4, b3, b2, b1, a5, a6"),
        Position::C6 => bitmask_csv!("c7, c8, d7, e8, f9, g9, h9, d6, e6, f6, g5, h4, i3, k2, l1, c5, c4, c3, c2, c1, b5, a4, b6, a6"),
        Position::D6 => bitmask_csv!("d7, d8, d9, e7, f8, g8, h8, i8, e6, f6, g5, h4, i3, k2, l1, d5, d4, d3, d2, d1, c5, b4, a3, c6, b6, a6"),
        Position::E6 => bitmask_csv!("e7, e8, e9, e10, f7, g7, h7, i7, k7, f6, g5, h4, i3, k2, l1, e5, e4, e3, e2, e1, d5, c4, b3, a2, d6, c6, b6, a6"),
        Position::F6 => bitmask_csv!("f7, f8, f9, f10, f11, g6, h6, i6, k6, l6, g5, h4, i3, k2, l1, f5, f4, f3, f2, f1, e5, d4, c3, b2, a1, e6, d6, c6, b6, a6"),
        Position::G6 => bitmask_csv!("g7, g8, g9, g10, h6, i6, k6, l6, h5, i4, k3, l2, g5, g4, g3, g2, g1, f6, e5, d4, c3, b2, a1, f7, e7, d7, c7, b7"),
        Position::H6 => bitmask_csv!("h7, h8, h9, i6, k6, l6, i5, k4, l3, h5, h4, h3, h2, h1, g6, f6, e5, d4, c3, b2, a1, g7, f8, e8, d8, c8"),
        Position::I6 => bitmask_csv!("i7, i8, k6, l6, k5, l4, i5, i4, i3, i2, i1, h6, g6, f6, e5, d4, c3, b2, a1, h7, g8, f9, e9, d9"),
        Position::K6 => bitmask_csv!("k7, l6, l5, k5, k4, k3, k2, k1, i6, h6, g6, f6, e5, d4, c3, b2, a1, i7, h8, g9, f10, e10"),
        Position::L6 => bitmask_csv!("l5, l4, l3, l2, l1, k6, i6, h6, g6, f6, e5, d4, c3, b2, a1, k7, i8, h9, g10, f11"),
        Position::A5 => bitmask_csv!("a6, b6, c7, d8, e9, f10, g10, b5, c5, d5, e5, f5, g4, h3, i2, k1, a4, a3, a2, a1"),
        Position::B5 => bitmask_csv!("b6, b7, c6, d7, e8, f9, g9, h9, c5, d5, e5, f5, g4, h3, i2, k1, b4, b3, b2, b1, a4, a5"),
        Position::C5 => bitmask_csv!("c6, c7, c8, d6, e7, f8, g8, h8, i8, d5, e5, f5, g4, h3, i2, k1, c4, c3, c2, c1, b4, a3, b5, a5"),
        Position::D5 => bitmask_csv!("d6, d7, d8, d9, e6, f7, g7, h7, i7, k7, e5, f5, g4, h3, i2, k1, d4, d3, d2, d1, c4, b3, a2, c5, b5, a5"),
        Position::E5 => bitmask_csv!("e6, e7, e8, e9, e10, f6, g6, h6, i6, k6, l6, f5, g4, h3, i2, k1, e4, e3, e2, e1, d4, c3, b2, a1, d5, c5, b5, a5"),
        Position::F5 => bitmask_csv!("f6, f7, f8, f9, f10, f11, g5, h5, i5, k5, l5, g4, h3, i2, k1, f4, f3, f2, f1, e4, d3, c2, b1, e5, d5, c5, b5, a5"),
        Position::G5 => bitmask_csv!("g6, g7, g8, g9, g10, h5, i5, k5, l5, h4, i3, k2, l1, g4, g3, g2, g1, f5, e4, d3, c2, b1, f6, e6, d6, c6, b6, a6"),
        Position::H5 => bitmask_csv!("h6, h7, h8, h9, i5, k5, l5, i4, k3, l2, h4, h3, h2, h1, g5, f5, e4, d3, c2, b1, g6, f7, e7, d7, c7, b7"),
        Position::I5 => bitmask_csv!("i6, i7, i8, k5, l5, k4, l3, i4, i3, i2, i1, h5, g5, f5, e4, d3, c2, b1, h6, g7, f8, e8, d8, c8"),
        Position::K5 => bitmask_csv!("k6, k7, l5, l4, k4, k3, k2, k1, i5, h5, g5, f5, e4, d3, c2, b1, i6, h7, g8, f9, e9, d9"),
        Position::L5 => bitmask_csv!("l6, l4, l3, l2, l1, k5, i5, h5, g5, f5, e4, d3, c2, b1, k6, i7, h8, g9, f10, e10"),
        Position::A4 => bitmask_csv!("a5, a6, b5, c6, d7, e8, f9, g9, h9, b4, c4, d4, e4, f4, g3, h2, i1, a3, a2, a1"),
        Position::B4 => bitmask_csv!("b5, b6, b7, c5, d6, e7, f8, g8, h8, i8, c4, d4, e4, f4, g3, h2, i1, b3, b2, b1, a3, a4"),
        Position::C4 => bitmask_csv!("c5, c6, c7, c8, d5, e6, f7, g7, h7, i7, k7, d4, e4, f4, g3, h2, i1, c3, c2, c1, b3, a2, b4, a4"),
        Position::D4 => bitmask_csv!("d5, d6, d7, d8, d9, e5, f6, g6, h6, i6, k6, l6, e4, f4, g3, h2, i1, d3, d2, d1, c3, b2, a1, c4, b4, a4"),
        Position::E4 => bitmask_csv!("e5, e6, e7, e8, e9, e10, f5, g5, h5, i5, k5, l5, f4, g3, h2, i1, e3, e2, e1, d3, c2, b1, d4, c4, b4, a4"),
        Position::F4 => bitmask_csv!("f5, f6, f7, f8, f9, f10, f11, g4, h4, i4, k4, l4, g3, h2, i1, f3, f2, f1, e3, d2, c1, e4, d4, c4, b4, a4"),
        Position::G4 => bitmask_csv!("g5, g6, g7, g8, g9, g10, h4, i4, k4, l4, h3, i2, k1, g3, g2, g1, f4, e3, d2, c1, f5, e5, d5, c5, b5, a5"),
        Position::H4 => bitmask_csv!("h5, h6, h7, h8, h9, i4, k4, l4, i3, k2, l1, h3, h2, h1, g4, f4, e3, d2, c1, g5, f6, e6, d6, c6, b6, a6"),
        Position::I4 => bitmask_csv!("i5, i6, i7, i8, k4, l4, k3, l2, i3, i2, i1, h4, g4, f4, e3, d2, c1, h5, g6, f7, e7, d7, c7, b7"),
        Position::K4 => bitmask_csv!("k5, k6, k7, l4, l3, k3, k2, k1, i4, h4, g4, f4, e3, d2, c1, i5, h6, g7, f8, e8, d8, c8"),
        Position::L4 => bitmask_csv!("l5, l6, l3, l2, l1, k4, i4, h4, g4, f4, e3, d2, c1, k5, i6, h7, g8, f9, e9, d9"),
        Position::A3 => bitmask_csv!("a4, a5, a6, b4, c5, d6, e7, f8, g8, h8, i8, b3, c3, d3, e3, f3, g2, h1, a2, a1"),
        Position::B3 => bitmask_csv!("b4, b5, b6, b7, c4, d5, e6, f7, g7, h7, i7, k7, c3, d3, e3, f3, g2, h1, b2, b1, a2, a3"),
        Position::C3 => bitmask_csv!("c4, c5, c6, c7, c8, d4, e5, f6, g6, h6, i6, k6, l6, d3, e3, f3, g2, h1, c2, c1, b2, a1, b3, a3"),
        Position::D3 => bitmask_csv!("d4, d5, d6, d7, d8, d9, e4, f5, g5, h5, i5, k5, l5, e3, f3, g2, h1, d2, d1, c2, b1, c3, b3, a3"),
        Position::E3 => bitmask_csv!("e4, e5, e6, e7, e8, e9, e10, f4, g4, h4, i4, k4, l4, f3, g2, h1, e2, e1, d2, c1, d3, c3, b3, a3"),
        Position::F3 => bitmask_csv!("f4, f5, f6, f7, f8, f9, f10, f11, g3, h3, i3, k3, l3, g2, h1, f2, f1, e2, d1, e3, d3, c3, b3, a3"),
        Position::G3 => bitmask_csv!("g4, g5, g6, g7, g8, g9, g10, h3, i3, k3, l3, h2, i1, g2, g1, f3, e2, d1, f4, e4, d4, c4, b4, a4"),
        Position::H3 => bitmask_csv!("h4, h5, h6, h7, h8, h9, i3, k3, l3, i2, k1, h2, h1, g3, f3, e2, d1, g4, f5, e5, d5, c5, b5, a5"),
        Position::I3 => bitmask_csv!("i4, i5, i6, i7, i8, k3, l3, k2, l1, i2, i1, h3, g3, f3, e2, d1, h4, g5, f6, e6, d6, c6, b6, a6"),
        Position::K3 => bitmask_csv!("k4, k5, k6, k7, l3, l2, k2, k1, i3, h3, g3, f3, e2, d1, i4, h5, g6, f7, e7, d7, c7, b7"),
        Position::L3 => bitmask_csv!("l4, l5, l6, l2, l1, k3, i3, h3, g3, f3, e2, d1, k4, i5, h6, g7, f8, e8, d8, c8"),
        Position::A2 => bitmask_csv!("a3, a4, a5, a6, b3, c4, d5, e6, f7, g7, h7, i7, k7, b2, c2, d2, e2, f2, g1, a1"),
        Position::B2 => bitmask_csv!("b3, b4, b5, b6, b7, c3, d4, e5, f6, g6, h6, i6, k6, l6, c2, d2, e2, f2, g1, b1, a1, a2"),
        Position::C2 => bitmask_csv!("c3, c4, c5, c6, c7, c8, d3, e4, f5, g5, h5, i5, k5, l5, d2, e2, f2, g1, c1, b1, b2, a2"),
        Position::D2 => bitmask_csv!("d3, d4, d5, d6, d7, d8, d9, e3, f4, g4, h4, i4, k4, l4, e2, f2, g1, d1, c1, c2, b2, a2"),
        Position::E2 => bitmask_csv!("e3, e4, e5, e6, e7, e8, e9, e10, f3, g3, h3, i3, k3, l3, f2, g1, e1, d1, d2, c2, b2, a2"),
        Position::F2 => bitmask_csv!("f3, f4, f5, f6, f7, f8, f9, f10, f11, g2, h2, i2, k2, l2, g1, f1, e1, e2, d2, c2, b2, a2"),
        Position::G2 => bitmask_csv!("g3, g4, g5, g6, g7, g8, g9, g10, h2, i2, k2, l2, h1, g1, f2, e1, f3, e3, d3, c3, b3, a3"),
        Position::H2 => bitmask_csv!("h3, h4, h5, h6, h7, h8, h9, i2, k2, l2, i1, h1, g2, f2, e1, g3, f4, e4, d4, c4, b4, a4"),
        Position::I2 => bitmask_csv!("i3, i4, i5, i6, i7, i8, k2, l2, k1, i1, h2, g2, f2, e1, h3, g4, f5, e5, d5, c5, b5, a5"),
        Position::K2 => bitmask_csv!("k3, k4, k5, k6, k7, l2, l1, k1, i2, h2, g2, f2, e1, i3, h4, g5, f6, e6, d6, c6, b6, a6"),
        Position::L2 => bitmask_csv!("l3, l4, l5, l6, l1, k2, i2, h2, g2, f2, e1, k3, i4, h5, g6, f7, e7, d7, c7, b7"),
        Position::A1 => bitmask_csv!("a2, a3, a4, a5, a6, b2, c3, d4, e5, f6, g6, h6, i6, k6, l6, b1, c1, d1, e1, f1"),
        Position::B1 => bitmask_csv!("b2, b3, b4, b5, b6, b7, c2, d3, e4, f5, g5, h5, i5, k5, l5, c1, d1, e1, f1, a1"),
        Position::C1 => bitmask_csv!("c2, c3, c4, c5, c6, c7, c8, d2, e3, f4, g4, h4, i4, k4, l4, d1, e1, f1, b1, a1"),
        Position::D1 => bitmask_csv!("d2, d3, d4, d5, d6, d7, d8, d9, e2, f3, g3, h3, i3, k3, l3, e1, f1, c1, b1, a1"),
        Position::E1 => bitmask_csv!("e2, e3, e4, e5, e6, e7, e8, e9, e10, f2, g2, h2, i2, k2, l2, f1, d1, c1, b1, a1"),
        Position::F1 => bitmask_csv!("f2, f3, f4, f5, f6, f7, f8, f9, f10, f11, g1, h1, i1, k1, l1, e1, d1, c1, b1, a1"),
        Position::G1 => bitmask_csv!("g2, g3, g4, g5, g6, g7, g8, g9, g10, h1, i1, k1, l1, f1, f2, e2, d2, c2, b2, a2"),
        Position::H1 => bitmask_csv!("h2, h3, h4, h5, h6, h7, h8, h9, i1, k1, l1, g1, f1, g2, f3, e3, d3, c3, b3, a3"),
        Position::I1 => bitmask_csv!("i2, i3, i4, i5, i6, i7, i8, k1, l1, h1, g1, f1, h2, g3, f4, e4, d4, c4, b4, a4"),
        Position::K1 => bitmask_csv!("k2, k3, k4, k5, k6, k7, l1, i1, h1, g1, f1, i2, h3, g4, f5, e5, d5, c5, b5, a5"),
        Position::L1 => bitmask_csv!("l2, l3, l4, l5, l6, k1, i1, h1, g1, f1, k2, i3, h4, g5, f6, e6, d6, c6, b6, a6"),
    }
}

pub fn get_bishop_moves_unsafe(game: &Game, from_position: Position) -> Vec<San> {
    match game.get_color(from_position) {
        Some(color) => {
            // check all 6 diagonal directions
            let mut result = Vec::new();
            let hostile_color = color.opposite();
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 1));
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 3));
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 5));
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 7));
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 9));
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 11));
            result
        },
        None => vec![],
    }
}

pub fn get_rook_moves_unsafe(game: &Game, from_position: Position) -> Vec<San> {
    match game.get_color(from_position) {
        Some(color) => {
            // check all 6 orthogonal directions
            let mut result = Vec::new();
            let hostile_color = color.opposite();
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 0));
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 2));
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 4));
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 6));
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 8));
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 10));
            result
        },
        None => vec![],
    }
}

pub fn get_queen_moves_unsafe(game: &Game, from_position: Position) -> Vec<San> {
    match game.get_color(from_position) {
        Some(color) => {
            // check all 12 directions (orthogonal + diagonal)
            let mut result = Vec::new();
            let hostile_color = color.opposite();
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 0));
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 1));
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 2));
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 3));
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 4));
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 5));
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 6));
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 7));
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 8));
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 9));
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 10));
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 11));
            result
        },
        None => vec![],
    }
}

fn get_sliding_moves_unsafe(
    game: &Game,
    from_position: Position,
    hostile_color: Color,
    direction: u8,
) -> Vec<San> {
    let mut result: Vec<San> = Vec::new();
    
    let mut next_position = from_position.step(direction);

    while next_position.is_some() {
        let position = next_position.unwrap();

        match game.get_color(position) {
            Some(color) => {
                if color == hostile_color {
                    result.push(San { from: from_position, to: position, promotion: None });
                }

                break;
            }
            None => {
                result.push(San { from: from_position, to: position, promotion: None });
            }
        };
        
        next_position = position.step(direction);
    }

    result
}
