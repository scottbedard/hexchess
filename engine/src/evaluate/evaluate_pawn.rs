use hexchess::{Color, Hexchess, Position};
use engine_macros::calc_advancement_bonus;

pub fn evaluate_pawn(_hexchess: &Hexchess, index: u8, color: Color) -> f32 {
    let base_value = 10.0;
    let position = Position::from_index(index);

    // pawns closer to promotion are more valuable
    let advancement_bonus = calc_advancement_bonus!(position, color, 10.0);

    base_value + advancement_bonus
}
