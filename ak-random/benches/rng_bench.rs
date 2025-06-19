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
    let mut rng = Mrg32k3a::from_seed([0u8; 8]);
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
    const TOTAL: usize = 10_000;
    const BATCH: usize = 512;
    c.bench_function("mrg32k3a_parallel_u64", |b| {
        b.iter(|| {
            let mut buf = vec![0u64; TOTAL];
            buf.par_chunks_mut(BATCH)
                .enumerate()
                .for_each(|(stream, chunk)| {
                    let mut rng = Mrg32k3aCore::default();
                    rng.set_stream(stream as u64);
                    for v in chunk {
                        *v = rng.step_u64();
                    }
                });
            let sum: u64 = buf.iter().copied().sum();
            black_box(sum);
        })
    });
}

fn bench_parallel_f64(c: &mut Criterion) {
    const TOTAL: usize = 10_000;
    const BATCH: usize = 512;
    c.bench_function("mrg32k3a_parallel_f64", |b| {
        b.iter(|| {
            let mut buf = vec![0f64; TOTAL];
            buf.par_chunks_mut(BATCH)
                .enumerate()
                .for_each(|(stream, chunk)| {
                    let mut rng = Mrg32k3aCore::default();
                    rng.set_stream(stream as u64);
                    for v in chunk {
                        *v = rng.next_f64();
                    }
                });
            let sum: f64 = buf.iter().copied().sum();
            black_box(sum);
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
