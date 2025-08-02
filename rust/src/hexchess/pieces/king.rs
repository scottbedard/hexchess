use crate::constants::hexboard_graph;
use crate::hexchess::color::Color;
use crate::hexchess::hexchess::Hexchess;
use crate::hexchess::san::San;
use crate::hexchess::position::Position;
use smallvec::SmallVec;


pub fn king_moves_unsafe(
    hexchess: &Hexchess,
    from: Position,
    color: &Color,
) -> SmallVec<[San; 12]> {
    let mut result: SmallVec<[San; 12]> = SmallVec::new();

    for position in hexboard_graph(from) {
        let to = match position {
            Some(t) => t,
            None => continue,
        };

        match hexchess.board[to as usize] {
            Some(piece) => {
                if piece.is_enemy(*color) {
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
