use crate::constants::Color;
use crate::hexchess::hexchess::Hexchess;
use crate::hexchess::san::San;

use crate::hexchess::utils::{
    get_color,
    step,
};


pub fn king_moves_unsafe(
    hexchess: &Hexchess,
    from: u8,
    color: &Color,
) -> Vec<San> {
    let mut result: Vec<San> = vec![];

    let directions: [u8; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

    for n in directions {
        let to = match step(from, n) {
            Some(to) => to,
            None => continue,
        };

        match hexchess.board[to as usize] {
            Some(piece) => {
                if get_color(&piece) != *color {
                    result.push(San { from, promotion: None, to });
                }
            },
            None => {
                result.push(San { from, promotion: None, to });
            }
        };
    }

    result
}
