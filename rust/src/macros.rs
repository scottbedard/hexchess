/// get the color of a piece (macro version for compile-time evaluation)
#[macro_export]
macro_rules! color {
    ($piece:expr) => {
        match $piece {
            $crate::hexchess::piece::Piece::WhitePawn => $crate::hexchess::color::Color::White,
            $crate::hexchess::piece::Piece::WhiteKnight => $crate::hexchess::color::Color::White,
            $crate::hexchess::piece::Piece::WhiteBishop => $crate::hexchess::color::Color::White,
            $crate::hexchess::piece::Piece::WhiteRook => $crate::hexchess::color::Color::White,
            $crate::hexchess::piece::Piece::WhiteQueen => $crate::hexchess::color::Color::White,
            $crate::hexchess::piece::Piece::WhiteKing => $crate::hexchess::color::Color::White,
            $crate::hexchess::piece::Piece::BlackPawn => $crate::hexchess::color::Color::Black,
            $crate::hexchess::piece::Piece::BlackKnight => $crate::hexchess::color::Color::Black,
            $crate::hexchess::piece::Piece::BlackBishop => $crate::hexchess::color::Color::Black,
            $crate::hexchess::piece::Piece::BlackRook => $crate::hexchess::color::Color::Black,
            $crate::hexchess::piece::Piece::BlackQueen => $crate::hexchess::color::Color::Black,
            $crate::hexchess::piece::Piece::BlackKing => $crate::hexchess::color::Color::Black,
        }
    };
}

#[macro_export]
macro_rules! position {
    ("f11") => { $crate::hexchess::position::Position::F11 };
    ("e10") => { $crate::hexchess::position::Position::E10 };
    ("f10") => { $crate::hexchess::position::Position::F10 };
    ("g10") => { $crate::hexchess::position::Position::G10 };
    ("d9") => { $crate::hexchess::position::Position::D9 };
    ("e9") => { $crate::hexchess::position::Position::E9 };
    ("f9") => { $crate::hexchess::position::Position::F9 };
    ("g9") => { $crate::hexchess::position::Position::G9 };
    ("h9") => { $crate::hexchess::position::Position::H9 };
    ("c8") => { $crate::hexchess::position::Position::C8 };
    ("d8") => { $crate::hexchess::position::Position::D8 };
    ("e8") => { $crate::hexchess::position::Position::E8 };
    ("f8") => { $crate::hexchess::position::Position::F8 };
    ("g8") => { $crate::hexchess::position::Position::G8 };
    ("h8") => { $crate::hexchess::position::Position::H8 };
    ("i8") => { $crate::hexchess::position::Position::I8 };
    ("b7") => { $crate::hexchess::position::Position::B7 };
    ("c7") => { $crate::hexchess::position::Position::C7 };
    ("d7") => { $crate::hexchess::position::Position::D7 };
    ("e7") => { $crate::hexchess::position::Position::E7 };
    ("f7") => { $crate::hexchess::position::Position::F7 };
    ("g7") => { $crate::hexchess::position::Position::G7 };
    ("h7") => { $crate::hexchess::position::Position::H7 };
    ("i7") => { $crate::hexchess::position::Position::I7 };
    ("k7") => { $crate::hexchess::position::Position::K7 };
    ("a6") => { $crate::hexchess::position::Position::A6 };
    ("b6") => { $crate::hexchess::position::Position::B6 };
    ("c6") => { $crate::hexchess::position::Position::C6 };
    ("d6") => { $crate::hexchess::position::Position::D6 };
    ("e6") => { $crate::hexchess::position::Position::E6 };
    ("f6") => { $crate::hexchess::position::Position::F6 };
    ("g6") => { $crate::hexchess::position::Position::G6 };
    ("h6") => { $crate::hexchess::position::Position::H6 };
    ("i6") => { $crate::hexchess::position::Position::I6 };
    ("k6") => { $crate::hexchess::position::Position::K6 };
    ("l6") => { $crate::hexchess::position::Position::L6 };
    ("a5") => { $crate::hexchess::position::Position::A5 };
    ("b5") => { $crate::hexchess::position::Position::B5 };
    ("c5") => { $crate::hexchess::position::Position::C5 };
    ("d5") => { $crate::hexchess::position::Position::D5 };
    ("e5") => { $crate::hexchess::position::Position::E5 };
    ("f5") => { $crate::hexchess::position::Position::F5 };
    ("g5") => { $crate::hexchess::position::Position::G5 };
    ("h5") => { $crate::hexchess::position::Position::H5 };
    ("i5") => { $crate::hexchess::position::Position::I5 };
    ("k5") => { $crate::hexchess::position::Position::K5 };
    ("l5") => { $crate::hexchess::position::Position::L5 };
    ("a4") => { $crate::hexchess::position::Position::A4 };
    ("b4") => { $crate::hexchess::position::Position::B4 };
    ("c4") => { $crate::hexchess::position::Position::C4 };
    ("d4") => { $crate::hexchess::position::Position::D4 };
    ("e4") => { $crate::hexchess::position::Position::E4 };
    ("f4") => { $crate::hexchess::position::Position::F4 };
    ("g4") => { $crate::hexchess::position::Position::G4 };
    ("h4") => { $crate::hexchess::position::Position::H4 };
    ("i4") => { $crate::hexchess::position::Position::I4 };
    ("k4") => { $crate::hexchess::position::Position::K4 };
    ("l4") => { $crate::hexchess::position::Position::L4 };
    ("a3") => { $crate::hexchess::position::Position::A3 };
    ("b3") => { $crate::hexchess::position::Position::B3 };
    ("c3") => { $crate::hexchess::position::Position::C3 };
    ("d3") => { $crate::hexchess::position::Position::D3 };
    ("e3") => { $crate::hexchess::position::Position::E3 };
    ("f3") => { $crate::hexchess::position::Position::F3 };
    ("g3") => { $crate::hexchess::position::Position::G3 };
    ("h3") => { $crate::hexchess::position::Position::H3 };
    ("i3") => { $crate::hexchess::position::Position::I3 };
    ("k3") => { $crate::hexchess::position::Position::K3 };
    ("l3") => { $crate::hexchess::position::Position::L3 };
    ("a2") => { $crate::hexchess::position::Position::A2 };
    ("b2") => { $crate::hexchess::position::Position::B2 };
    ("c2") => { $crate::hexchess::position::Position::C2 };
    ("d2") => { $crate::hexchess::position::Position::D2 };
    ("e2") => { $crate::hexchess::position::Position::E2 };
    ("f2") => { $crate::hexchess::position::Position::F2 };
    ("g2") => { $crate::hexchess::position::Position::G2 };
    ("h2") => { $crate::hexchess::position::Position::H2 };
    ("i2") => { $crate::hexchess::position::Position::I2 };
    ("k2") => { $crate::hexchess::position::Position::K2 };
    ("l2") => { $crate::hexchess::position::Position::L2 };
    ("a1") => { $crate::hexchess::position::Position::A1 };
    ("b1") => { $crate::hexchess::position::Position::B1 };
    ("c1") => { $crate::hexchess::position::Position::C1 };
    ("d1") => { $crate::hexchess::position::Position::D1 };
    ("e1") => { $crate::hexchess::position::Position::E1 };
    ("f1") => { $crate::hexchess::position::Position::F1 };
    ("g1") => { $crate::hexchess::position::Position::G1 };
    ("h1") => { $crate::hexchess::position::Position::H1 };
    ("i1") => { $crate::hexchess::position::Position::I1 };
    ("k1") => { $crate::hexchess::position::Position::K1 };
    ("l1") => { $crate::hexchess::position::Position::L1 };
    ($other:expr) => {
        compile_error!("Unknown position string in position! macro")
    };
}

#[macro_export]
macro_rules! san {
    ($expression:expr) => {
        crate::hexchess::san::San::from_string(&$expression.to_string())
    };
}
