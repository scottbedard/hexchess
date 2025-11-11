use hexchess::{Color, Hexchess};
use crate::structs::EvalOptions;

pub fn eval_rook(
    _hexchess: &Hexchess,
    _index: u8,
    _color: Color,
    opts: &EvalOptions,
) -> f32 {
    opts.rook_value
}
