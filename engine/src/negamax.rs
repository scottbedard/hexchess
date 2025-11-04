use crate::evaluation::evaluate;
use crate::ordering::optimize_for_branch_pruning;
use crate::structs::{ScoredSan, SearchResult};
use hexchess::{Color, Hexchess};
use std::cmp::max;
use std::collections::HashMap;

#[derive(Clone, Copy)]
enum Flag {
    Exact,
    LowerBound,
    UpperBound,
}

struct SearchedNode {
    depth: u8,
    flag: Flag,
    value: i32,
}

/// search position to a given depth
pub fn search(root: &Hexchess, depth: u8) -> SearchResult {
    let mut transposition_table = HashMap::<Hexchess, SearchedNode>::new();
    let mut evaluations: u32 = 0;

    // gather all possible moves and score them
    let mut sans: Vec<ScoredSan> = root
        .current_moves()
        .into_iter()
        .map(|san| {
            let mut child: Hexchess = root.clone();
            let _ = child.apply_move(&san);

            let score = negamax(
                &mut transposition_table,
                &child,
                depth - 1,
                i32::MIN,
                i32::MAX,
                &mut evaluations,
            );

            ScoredSan { san, score }
        })
        .collect();

    // sort best moves first, from perspective of the current player
    sans.sort_by_key(|s| s.score);

    SearchResult {
        depth,
        evaluations,
        sans,
    }
}

fn negamax(
    table: &mut HashMap<Hexchess, SearchedNode>,
    hexchess: &Hexchess,
    depth: u8,
    mut alpha: i32,
    beta: i32,
    evals: &mut u32,
) -> i32 {
    let alpha_orig = alpha;

    // check if our result has already been computed
    match table.get(hexchess) {
        Some(entry) => {
            if entry.depth >= depth {
                match entry.flag {
                    Flag::Exact => return entry.value,
                    Flag::LowerBound if entry.value >= beta => return entry.value,
                    Flag::UpperBound if entry.value <= alpha => return entry.value,
                    _ => {}
                }
            }
        }
        None => {}
    };

    // if not, begin evaluating the position
    let mut current_moves = hexchess.current_moves();

    if depth == 0 || current_moves.is_empty() {
        *evals += 1;

        return match hexchess.turn {
            Color::White => evaluate(hexchess),
            Color::Black => -evaluate(hexchess),
        };
    }

    // sort moves with the hope of pruning branches quicker
    // for example, we should probably investigate captures before non-captures
    optimize_for_branch_pruning(hexchess, &mut current_moves);

    let mut value = i32::MIN;

    for san in current_moves {
        let mut child = hexchess.clone();
        let _ = child.apply_move(&san);

        value = max(
            value,
            -negamax(table, &child, depth - 1, -beta, -alpha, evals),
        );

        alpha = max(alpha, value);

        if alpha >= beta {
            break;
        }
    }

    // store our result in the transposition table for future lookups
    let searched_node = SearchedNode {
        value,
        depth,
        flag: if value <= alpha_orig {
            Flag::UpperBound
        } else if value >= beta {
            Flag::LowerBound
        } else {
            Flag::Exact
        },
    };

    table.insert(hexchess.clone(), searched_node);

    // results from initial position, depth 4
    // {score: 0, evaluations: 295873, turn: 'w'} // alpha-beta pruning
    // {score: 0, evaluations: 283445, turn: 'w'} // transposition table
    // {score: 10, evaluations: 143173, turn: 'w'} // ordering
    // ^ why is this score 10, should probably be 0

    value
}
