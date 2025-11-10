use crate::structs::EvalOptions;
use hexchess::{Color, Hexchess};

pub fn eval_bishop(
    _hexchess: &Hexchess,
    _index: u8,
    _color: Color,
    opts: &EvalOptions,
) -> f32 {
    opts.bishop_value
}
