use hexchess::{Hexchess, Piece};

pub fn evaluate(hexchess: &Hexchess) -> i32 {
    let score = score_material(hexchess);

    // @todo: apply heuristic functions to modify score...

    score
}

fn score_material(hexchess: &Hexchess) -> i32 {
    let mut score = 0;

    for (index, piece) in hexchess.to_board_array().iter().enumerate() {
        score += match piece {
            Some(Piece::BlackPawn) => -pawn_weight(hexchess, index),
            Some(Piece::BlackKnight) => -knight_weight(hexchess, index),
            Some(Piece::BlackBishop) => -bishop_weight(hexchess, index),
            Some(Piece::BlackRook) => -rook_weight(hexchess, index),
            Some(Piece::BlackQueen) => -queen_weight(hexchess, index),
            Some(Piece::BlackKing) => -king_weight(hexchess, index),
            Some(Piece::WhitePawn) => pawn_weight(hexchess, index),
            Some(Piece::WhiteKnight) => knight_weight(hexchess, index),
            Some(Piece::WhiteBishop) => bishop_weight(hexchess, index),
            Some(Piece::WhiteRook) => rook_weight(hexchess, index),
            Some(Piece::WhiteQueen) => queen_weight(hexchess, index),
            Some(Piece::WhiteKing) => king_weight(hexchess, index),
            None => 0,
        };
    }

    score
}

fn pawn_weight(_hexchess: &Hexchess, _index: usize) -> i32 {
    let weight = 10;

    weight * 1
}

fn bishop_weight(_hexchess: &Hexchess, _index: usize) -> i32 {
    let weight = 30;

    weight
}

fn knight_weight(_hexchess: &Hexchess, _index: usize) -> i32 {
    let weight = 30;

    weight
}

fn rook_weight(_hexchess: &Hexchess, _index: usize) -> i32 {
    let weight = 50;

    weight
}

fn queen_weight(_hexchess: &Hexchess, _index: usize) -> i32 {
    let weight = 90;

    weight
}

fn king_weight(_hexchess: &Hexchess, _index: usize) -> i32 {
    let weight = 900;

    weight * 1
}
