use crate::hexchess::bitboard::Bitboard;
use crate::hexchess::game::Game;
use crate::hexchess::position::Position;
use crate::hexchess::san::San;

pub fn get_king_moves_unsafe(game: &Game, from: Position) -> Vec<San> {
    let mut result = Bitboard(from.get_neighbors());

    match game.get_color(from) {
        Some(color) => result &= !game.get_color_bitboard(color),
        None => {}
    };
    
    let mut output = Vec::with_capacity(result.count_ones() as usize);

    for index in result.iter_set_bits() {
        let to = Position::from_bitboard_index(index);
        let san = San::new(from, to);
        output.push(san);
    }

    output
}