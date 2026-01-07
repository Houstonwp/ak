use ak::rng::RngCore;
use ak::rng::mgk32a::Mgk32a;
use ak::rng::sobol::Sobol;
use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn bench_mgk32a_next_u32(c: &mut Criterion) {
    c.bench_function("mgk32a_next_u32", |b| {
        let mut rng = Mgk32a::from_seed64(123);
        b.iter(|| {
            black_box(rng.next_u32());
        })
    });
}

fn bench_sobol_dim1_next_point(c: &mut Criterion) {
    c.bench_function("sobol_dim1_next_point", |b| {
        let mut sobol = Sobol::new(1).expect("sobol dim1");
        let mut out = [0.0f64; 1];
        b.iter(|| {
            sobol.next_point(&mut out).unwrap();
            black_box(out[0]);
        })
    });
}

criterion_group!(
    rng_benches,
    bench_mgk32a_next_u32,
    bench_sobol_dim1_next_point
);
criterion_main!(rng_benches);
