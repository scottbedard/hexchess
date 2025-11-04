use hexchess::{Color, Hexchess, Piece, San};

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

pub fn san_weight(hexchess: &Hexchess, _enemy_color: Color, san: &San) -> i32 {
    let mut weight = 0;

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

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_move_ordering() {
    //     let hexchess = Hexchess::parse("1/3/5/7/9/5Pp4/11/11/11/11/11 w - 0 1").unwrap();
    //     let mut sorted_moves = hexchess.current_moves();

    //     optimize_for_branch_pruning(&hexchess, &mut sorted_moves);

    //     assert_eq!(sorted_moves[0], San::from("f6g6").unwrap());
    //     assert_eq!(sorted_moves[1], San::from("f6f7").unwrap());
    // }
}
