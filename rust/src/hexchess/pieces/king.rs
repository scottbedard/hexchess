use crate::constants::{Color, HEXBOARD_GRAPH};
use crate::hexchess::hexchess::Hexchess;
use crate::hexchess::san::San;
use crate::hexchess::utils:: get_color;


pub fn king_moves_unsafe(
    hexchess: &Hexchess,
    from: u8,
    color: &Color,
) -> Vec<San> {
    let mut result: Vec<San> = vec![];

    for position in HEXBOARD_GRAPH[from as usize] {
        let to = match position {
            Some(t) => t,
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
