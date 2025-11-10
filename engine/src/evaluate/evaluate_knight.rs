use hexchess::{Color, Hexchess, Position};
use engine_macros::calc_advancement_bonus;

pub fn evaluate_knight(_hexchess: &Hexchess, index: u8, color: Color) -> f32 {
    let base_value = 30.0;

    base_value * 1.0
}
