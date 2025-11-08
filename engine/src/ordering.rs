use hexchess::{Color, Hexchess, San};

/// determine which moves to evaluate first, optimizing for nodes that
/// are likely to cause branch pruning
pub fn optimize_for_branch_pruning(hexchess: &Hexchess, sans: &mut Vec<San>) {
    let enemy_color = match hexchess.turn {
        Color::White => Color::Black,
        Color::Black => Color::White,
    };

    sans.sort_by(|a, b| {
        let a_weight = san_weight(hexchess, enemy_color, a);
        let b_weight = san_weight(hexchess, enemy_color, b);
        b_weight.cmp(&a_weight)
    });
}

pub fn san_weight(_hexchess: &Hexchess, _enemy_color: Color, _san: &San) -> i32 {
    let weight = 0;

    // // investigate checks, then captures, then non-captures
    // weight += match hexchess.board[san.to as usize] {
    //     Some(piece) => match piece {
    //         Piece::WhiteKing | Piece::BlackKing => 200,
    //         _ => 10,
    //     },
    //     None => 0,
    // };

    // @todo: check for takeback by a less valuable piece

    weight
}
