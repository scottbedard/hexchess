use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};



fn setup() -> (u128, u128, u64, u64, u32, u32) {
    let u128_alpha: u128 = rand::random::<u128>();
    let u128_beta: u128 = rand::random::<u128>();
    
    let u64_alpha: u64 = (u128_alpha >> 64) as u64;
    let u64_beta: u64 = (u128_beta >> 64) as u64;
    
    let u32_alpha: u32 = (u128_alpha >> 96) as u32;
    let u32_beta: u32 = (u128_beta >> 96) as u32;

    (u128_alpha, u128_beta, u64_alpha, u64_beta, u32_alpha, u32_beta)
}

fn criterion_benchmark(c: &mut Criterion) {
    let (u128_alpha, u128_beta, u64_alpha, u64_beta, u32_alpha, u32_beta) = setup();

    c.bench_function("u128", |b| {
        b.iter(|| {
            let result = black_box(u128_alpha) & black_box(u128_beta);
            black_box(result)
        })
    });

    c.bench_function("u64 & u32", |b| {
        b.iter(|| {
            let result_1 = black_box(u64_alpha) & black_box(u64_beta);
            let result_2 = black_box(u32_alpha) & black_box(u32_beta);
            (black_box(result_2), black_box(result_1))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);