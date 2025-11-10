use hexchess::{Color, Hexchess, Piece};
use crate::eval::eval_bishop::eval_bishop;
use crate::eval::eval_king::eval_king;
use crate::eval::eval_knight::eval_knight;
use crate::eval::eval_pawn::eval_pawn;
use crate::eval::eval_queen::eval_queen;
use crate::eval::eval_rook::eval_rook;
use crate::structs::EvalOptions;

pub fn evaluate(hexchess: &Hexchess, options: &EvalOptions) -> f32 {
    let score = score_material(hexchess, options);

    score
}

fn score_material(hexchess: &Hexchess, opts: &EvalOptions) -> f32 {
    let mut score: f32 = 0.0;

    for (index, piece) in hexchess.to_board_array().iter().enumerate() {
        let i = index as u8;
        
        score += match piece {
            Some(Piece::BlackPawn) => -eval_pawn(hexchess, i, Color::Black, opts),
            Some(Piece::BlackKnight) => -eval_knight(hexchess, i, Color::Black, opts),
            Some(Piece::BlackBishop) => -eval_bishop(hexchess, i, Color::Black, opts),
            Some(Piece::BlackRook) => -eval_rook(hexchess, i, Color::Black, opts),
            Some(Piece::BlackQueen) => -eval_queen(hexchess, i, Color::Black, opts),
            Some(Piece::BlackKing) => -eval_king(hexchess, i, Color::Black, opts),
            Some(Piece::WhitePawn) => eval_pawn(hexchess, i, Color::White, opts),
            Some(Piece::WhiteKnight) => eval_knight(hexchess, i, Color::White, opts),
            Some(Piece::WhiteBishop) => eval_bishop(hexchess, i, Color::White, opts),
            Some(Piece::WhiteRook) => eval_rook(hexchess, i, Color::White, opts),
            Some(Piece::WhiteQueen) => eval_queen(hexchess, i, Color::White, opts),
            Some(Piece::WhiteKing) => eval_king(hexchess, i, Color::White, opts),
            _ => 0.0,
        };
    }

    score
}
