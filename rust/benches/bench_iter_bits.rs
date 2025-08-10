#![feature(test)]

extern crate hexchess;
extern crate test;

use hexchess::hexchess::bitboard::Bitboard;

use rand::Rng;
use test::Bencher;

#[bench]
fn bench_iter_bits(b: &mut Bencher) {
    // 95.50 ns/iter (+/- 0.92) - change from iterator to callback fn

    let mut rng = rand::rng();

    b.iter(|| {
        let x: u128 = rng.random();

        Bitboard(x).iter_bits(|bit| {
            test::black_box(bit);
        });
    });
}
