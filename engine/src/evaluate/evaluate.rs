use hexchess::{Color, Hexchess, Piece, Position};
use crate::evaluate::evaluate_pawn::evaluate_pawn;

pub fn evaluate(hexchess: &Hexchess) -> f32 {
    let score = score_material(hexchess);

    // @todo: apply heuristic functions to modify score...

    score
}

fn score_material(hexchess: &Hexchess) -> f32 {
    let mut score: f32 = 0.0;

    for (index, piece) in hexchess.to_board_array().iter().enumerate() {
        score += match piece {
            Some(Piece::BlackPawn) => -pawn_weight(hexchess, index, Color::Black),
            Some(Piece::BlackKnight) => -knight_weight(hexchess, index),
            Some(Piece::BlackBishop) => -bishop_weight(hexchess, index),
            Some(Piece::BlackRook) => -rook_weight(hexchess, index),
            Some(Piece::BlackQueen) => -queen_weight(hexchess, index),
            Some(Piece::BlackKing) => -king_weight(hexchess, index),
            Some(Piece::WhitePawn) => pawn_weight(hexchess, index, Color::White),
            Some(Piece::WhiteKnight) => knight_weight(hexchess, index),
            Some(Piece::WhiteBishop) => bishop_weight(hexchess, index),
            Some(Piece::WhiteRook) => rook_weight(hexchess, index),
            Some(Piece::WhiteQueen) => queen_weight(hexchess, index),
            Some(Piece::WhiteKing) => king_weight(hexchess, index),
            None => 0.0,
        };
    }

    score
}

fn pawn_weight(hexchess: &Hexchess, index: usize, color: Color) -> f32 {
    // evaluate_pawn(hexchess, index as u8, color)
    10.0
}

fn bishop_weight(_hexchess: &Hexchess, _index: usize) -> f32 {
    let weight: f32 = 30.0;

    weight * 1.0
}

fn knight_weight(_hexchess: &Hexchess, _index: usize) -> f32 {
    let weight: f32 = 30.0;

    weight * 1.0
}

fn rook_weight(_hexchess: &Hexchess, _index: usize) -> f32 {
    let weight: f32 = 50.0;

    weight * 1.0
}

fn queen_weight(_hexchess: &Hexchess, _index: usize) -> f32 {
    let weight: f32 = 90.0;

    weight * 1.0
}

fn king_weight(_hexchess: &Hexchess, _index: usize) -> f32 {
    let weight: f32 = 900.0;

    weight * 1.0
}
