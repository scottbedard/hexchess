use crate::hexchess::color::Color;
use crate::hexchess::game::Game;
use crate::hexchess::position::Position;
use crate::hexchess::san::San;

pub fn get_bishop_moves_unsafe(game: &Game, from_position: Position) -> Vec<San> {
    match game.get_color(from_position) {
        Some(color) => {
            // check all 6 diagonal directions
            let mut result = Vec::new();
            let hostile_color = color.opposite();
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 1));
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 3));
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 5));
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 7));
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 9));
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 11));
            result
        },
        None => vec![],
    }
}

pub fn get_rook_moves_unsafe(game: &Game, from_position: Position) -> Vec<San> {
    match game.get_color(from_position) {
        Some(color) => {
            // check all 6 orthogonal directions
            let mut result = Vec::new();
            let hostile_color = color.opposite();
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 0));
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 2));
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 4));
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 6));
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 8));
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 10));
            result
        },
        None => vec![],
    }
}

pub fn get_queen_moves_unsafe(game: &Game, from_position: Position) -> Vec<San> {
    match game.get_color(from_position) {
        Some(color) => {
            // check all 12 directions (orthogonal + diagonal)
            let mut result = Vec::new();
            let hostile_color = color.opposite();
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 0));
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 1));
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 2));
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 3));
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 4));
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 5));
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 6));
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 7));
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 8));
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 9));
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 10));
            result.extend(get_sliding_moves_unsafe(game, from_position, hostile_color, 11));
            result
        },
        None => vec![],
    }
}

fn get_sliding_moves_unsafe(
    game: &Game,
    from_position: Position,
    hostile_color: Color,
    direction: u8,
) -> Vec<San> {
    let mut result: Vec<San> = Vec::new();
    
    let mut next_position = from_position.step(direction);

    while next_position.is_some() {
        let position = next_position.unwrap();

        match game.get_color(position) {
            Some(color) => {
                if color == hostile_color {
                    result.push(San { from: from_position, to: position, promotion: None });
                    break;
                }
            }
            None => {
                result.push(San { from: from_position, to: position, promotion: None });
            }
        };
        
        next_position = position.step(direction);
    }

    result
}
