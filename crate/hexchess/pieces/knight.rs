use crate::constants::Color;
use crate::hexchess::hexchess::Hexchess;
use crate::hexchess::san::San;

use crate::hexchess::utils::{
    get_color,
    step,
};

pub fn knight_moves_unsafe(
    hexchess: &Hexchess,
    from: u8,
    color: &Color,
) -> Vec<San> {
    let mut result: Vec<San> = vec![];

    // diagonal direction, first orthogonal direction, second orthogonal direction
    let targets: [(u8, u8, u8); 6] = [
        (1, 0, 2),
        (3, 2, 4),
        (5, 4, 6),
        (7, 6, 8),
        (9, 8, 10),
        (11, 10, 0),
    ];

    for (diagonal, orthogonal1, orthagonal2) in targets {
        let intermediate = match step(from, diagonal) {
            Some(index) => index,
            None => continue,
        };

        match knight_steps(hexchess, from, intermediate, orthogonal1, color) {
            Some(to) => result.push(to),
            None => (),
        };

        match knight_steps(hexchess, from, intermediate, orthagonal2, color) {
            Some(to) => result.push(to),
            None => (),
        };
    }

    result
}

fn knight_steps(hexchess: &Hexchess, from: u8, intermediate: u8, orthogonal: u8, color: &Color) -> Option<San> {
    match step(intermediate, orthogonal) {
        Some(to) => match hexchess.board[to as usize] {
            Some(piece) => match get_color(&piece) != *color {
                true => Some(San { from, promotion: None, to }),
                false => None,
            },
            None => Some(San { from, promotion: None, to }),
        },
        None => None,
    }
}
