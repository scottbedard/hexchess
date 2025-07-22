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
