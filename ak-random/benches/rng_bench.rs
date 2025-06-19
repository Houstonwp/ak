use ak_random::mrg32k3a::{Mrg32k3a, Mrg32k3aCore};
use criterion::{Criterion, criterion_group, criterion_main};
use rand_core::{RngCore, SeedableRng};
use rayon::prelude::*;
use std::hint::black_box;

fn bench_core_step_u64(c: &mut Criterion) {
    let mut rng = Mrg32k3aCore::default();
    c.bench_function("mrg32k3a_core_step_u64", |b| {
        b.iter(|| black_box(rng.step_u64()))
    });
}

fn bench_block_next_u64(c: &mut Criterion) {
    let mut rng = Mrg32k3a::from_seed(Default::default());
    c.bench_function("mrg32k3a_next_u64", |b| {
        b.iter(|| black_box(rng.next_u64()))
    });
}

fn bench_std_rng(c: &mut Criterion) {
    use rand::{RngCore, SeedableRng};
    let mut rng = rand::rngs::StdRng::seed_from_u64(42);
    c.bench_function("std_rng_next_u64", |b| b.iter(|| black_box(rng.next_u64())));
}

fn bench_parallel_u64(c: &mut Criterion) {
    c.bench_function("mrg32k3a_parallel_u64", |b| {
        b.iter(|| {
            let seeds = [1u64, 2, 3, 4];
            let total: u64 = seeds
                .par_iter()
                .map(|&seed| {
                    let mut rng = Mrg32k3aCore::default();
                    rng.set_seed(seed);
                    let mut sum = 0u64;
                    for _ in 0..10_000 {
                        sum = sum.wrapping_add(rng.step_u64());
                    }
                    sum
                })
                .sum();
            black_box(total);
        })
    });
}

fn bench_parallel_f64(c: &mut Criterion) {
    c.bench_function("mrg32k3a_parallel_f64", |b| {
        b.iter(|| {
            let seeds = [1u64, 2, 3, 4];
            let total: f64 = seeds
                .par_iter()
                .map(|&seed| {
                    let mut rng = Mrg32k3aCore::default();
                    rng.set_seed(seed);
                    let mut sum = 0.0f64;
                    for _ in 0..10_000 {
                        sum += rng.step();
                    }
                    sum
                })
                .sum();
            black_box(total);
        })
    });
}

criterion_group!(
    rng_benches,
    bench_core_step_u64,
    bench_block_next_u64,
    bench_std_rng,
    bench_parallel_u64,
    bench_parallel_f64
);
criterion_main!(rng_benches);
