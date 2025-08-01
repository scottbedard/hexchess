use crate::color;
use crate::constants::HEXBOARD_GRAPH;
use crate::hexchess::color::Color;
use crate::hexchess::hexchess::Hexchess;
use crate::hexchess::san::San;
use smallvec::SmallVec;


pub fn king_moves_unsafe(
    hexchess: &Hexchess,
    from: u8,
    color: &Color,
) -> SmallVec<[San; 12]> {
    let mut result: SmallVec<[San; 12]> = SmallVec::new();

    for position in HEXBOARD_GRAPH[from as usize] {
        let to = match position {
            Some(t) => t,
            None => continue,
        };

        match hexchess.board[to as usize] {
            Some(piece) => {
                if color!(&piece) != *color {
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
