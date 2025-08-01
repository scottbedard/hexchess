use crate::hexchess::color::Color;
use crate::hexchess::hexchess::Hexchess;
use crate::hexchess::san::San;
use crate::hexchess::utils::walk;

pub fn queen_moves_unsafe(
    hexchess: &Hexchess,
    from: &u8,
    color: &Color,
) -> Vec<San> {
    let mut result: Vec<San> = Vec::with_capacity(42); // Queen can have up to 42 moves

    for n in 0u8..12 {
        let path = walk(hexchess, *from, n, color);

        for to in path {
            result.push(San {
                from: *from,
                promotion: None,
                to
            });
        }
    }

    result
}
