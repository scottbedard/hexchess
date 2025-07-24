use crate::constants::{Color, KNIGHT_GRAPH};
use crate::hexchess::hexchess::Hexchess;
use crate::hexchess::san::San;
use crate::hexchess::utils::get_color;

const REACHABLE_POSITIONS: [u8; 91] = [
    4,  5,  6,  5,  6,  7,  8,  7,  6,  6,  8, 10,
    12, 10,  8,  6,  5,  7, 10, 12, 12, 12, 10,  7,
    5,  4,  6,  8, 12, 12, 12, 12, 12,  8,  6,  4,
    5,  7, 10, 12, 12, 12, 12, 12, 10,  7,  5,  6,
    8, 10, 12, 12, 12, 12, 12, 10,  8,  6,  6,  7,
    8, 10, 10,  8, 10, 10,  8,  7,  6,  5,  6,  7,
    8,  7,  6,  7,  8,  7,  6,  5,  4,  5,  6,  6,
    5,  4,  5,  6,  6,  5,  4
];

pub fn knight_moves_unsafe(
    hexchess: &Hexchess,
    from: u8,
    color: &Color,
) -> Vec<San> {
    let mut moves: Vec<San> = KNIGHT_GRAPH[from as usize]
        .iter()
        .filter(|&to| match hexchess.board[*to as usize] {
            Some(piece) => get_color(&piece) != *color,
            None => true,
        })
        .map(|&to| San { from, promotion: None, to })
        .collect();
    
    moves.sort_by_key(|san| REACHABLE_POSITIONS[san.to as usize]);

    moves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knight_moves() {
        let hexchess = Hexchess::init();
        let moves = knight_moves_unsafe(&hexchess, 0, &Color::White);
        assert!(!moves.is_empty());
    }
}
