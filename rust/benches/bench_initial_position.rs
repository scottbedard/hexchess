#![feature(test)]

extern crate test;
extern crate hexchess;

use hexchess::hexchess::hexchess::Hexchess;
use test::Bencher;

#[bench]
fn bench_initial_position(b: &mut Bencher) {
    // 18,182,566.70 ns/iter (+/- 311,939.48)
    // 17,940,987.40 ns/iter (+/- 331,128.34) - pre-calc pawns
    // 815,047.90 ns/iter (+/- 24,078.53) - test unsafe from king's perspective
    // 720,664.60 ns/iter (+/- 5,080.18) - smallvec
    // 880,891.60 ns/iter (+/- 17,080.93) - bitmaps
    // 779,362.55 ns/iter (+/- 5,740.49) - bitmaps + graph step
    // 683,964.55 ns/iter (+/- 6,605.64) - better bitmap iteration
    // 677,835.45 ns/iter (+/- 11,782.53) - avoid redundant find_king calls on get_moves
    // 663,612.50 ns/iter (+/- 3,863.87) - pre-calc knight, pawn, and neighbor bitmasks
    // 654,495.80 ns/iter (+/- 47,311.67) - pre-calc sliding bitmasks
    // 646,839.55 ns/iter (+/- 6,157.47) - prevent extra hostile piece matches in is_threatened
    // 614,970.83 ns/iter (+/- 8,537.56) - remove bitmask-index, only use fen-index

    b.iter(|| {
        let hexchess_0 = Hexchess::init();
        let white_moves = hexchess_0.current_moves();

        for white_move in white_moves {
            let mut hexchess_1 = hexchess_0.clone();
            hexchess_1.apply_move(&white_move).unwrap();

            let black_moves = hexchess_1.current_moves();

            for black_move in black_moves {
                let mut hexchess_2 = hexchess_1.clone();
                hexchess_2.apply_move(&black_move).unwrap();
            }
        }
    });
}
