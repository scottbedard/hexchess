#![feature(test)]

extern crate test;
extern crate hexchess;

use test::Bencher;

#[bench]
fn bench_knight_moves_unsafe(b: &mut Bencher) {
    // let hexchess = Hexchess::init();
    // b.iter(|| {
    //     for n in 0..90 {
    //         knight_moves_unsafe(&hexchess, n, &Color::White);
    //     }
    // });
}

#[bench]
fn bench_knight_moves_unsafe_black(b: &mut Bencher) {
    // let hexchess = Hexchess::init();
    // b.iter(|| {
    //     // Test moves from different positions to get a good average
    //     for n in 0..90 {
    //         knight_moves_unsafe(&hexchess, n, &Color::White);
    //     }
    // });
} 