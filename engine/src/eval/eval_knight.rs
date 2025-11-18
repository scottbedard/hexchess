use crate::structs::EvalOptions;
use hexchess::{Color, Hexchess};

pub fn eval_knight(
    _hexchess: &Hexchess,
    _index: u8,
    _color: Color,
    options: &EvalOptions,
) -> f32 {
    options.knight_value
}
