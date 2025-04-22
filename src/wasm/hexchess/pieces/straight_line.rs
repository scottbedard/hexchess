use crate::constants::Color;
use crate::hexchess::hexchess::Hexchess;
use crate::hexchess::san::San;
use crate::hexchess::utils::walk;

pub fn straight_line_moves_unsafe(
    hexchess: &Hexchess,
    from: &u8,
    color: &Color,
    directions: &[u8],
) -> Vec<San> {
    let mut result: Vec<San> = vec![];

    for n in directions {
        let path = walk(hexchess, *from, *n, color);

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
