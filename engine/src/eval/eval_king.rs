use crate::structs::EvalOptions;
use hexchess::{Color, Hexchess};

pub fn eval_king(
    _hexchess: &Hexchess,
    _index: u8,
    _color: Color,
    opts: &EvalOptions,
) -> f32 {
    opts.king_value
}
