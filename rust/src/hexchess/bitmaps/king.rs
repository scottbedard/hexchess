use crate::Bitboard;
use crate::Hexchess;
use crate::Position;
use crate::San;

pub fn get_king_moves_unsafe(hexchess: &Hexchess, from: Position) -> Vec<San> {
    let mut result = Bitboard(from.get_neighbors());

    match hexchess.get_color(from) {
        Some(color) => result &= !hexchess.get_color_bitboard(color),
        None => {}
    };
    
    let mut output = Vec::with_capacity(result.count_ones() as usize);

    result.iter_bits(|index| {
        let to = Position::from_index(index);
        let san = San::new(from, to);
        output.push(san);
    });

    output
}