use crate::structs::EvalOptions;
use engine_macros::calc_advancement_bonus;
use hexchess::{Color, Hexchess, Position};

pub fn eval_pawn(
    _hexchess: &Hexchess,
    index: u8,
    color: Color,
    options: &EvalOptions,
) -> f32 {
    let position = Position::from_index(index);

    // pawns closer to promotion are more valuable
    let advancement_bonus = calc_advancement_bonus!(position, color, 2.0);

    options.pawn_value + advancement_bonus
}
