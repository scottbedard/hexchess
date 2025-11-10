use crate::structs::EvalOptions;
use hexchess::{Color, Hexchess};

pub fn eval_queen(
    _hexchess: &Hexchess,
    _index: u8,
    _color: Color,
    opts: &EvalOptions,
) -> f32 {
    opts.queen_value
}
