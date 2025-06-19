use ak_random::RNG;
use ak_random::mrg32k3a::Mrg32k3a;
use ak_sim::{Model, Product, Scenario, monte_carlo};
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

#[derive(Clone)]
struct DummyModel;

impl Model for DummyModel {
    fn simulation_dimensions(&self) -> usize {
        1
    }
    fn init(&self, _: usize, _: usize) {}
    fn generate_path(&self, g: &[f64], path: &mut Scenario) {
        path.mut_samples()[0].numeraire = g[0];
    }
}

#[derive(Clone)]
struct DummyProduct;

impl Product for DummyProduct {
    fn payoff_labels(&self) -> Vec<String> {
        vec!["dummy".into()]
    }
    fn payoff_count(&self) -> usize {
        1
    }
    fn timeline(&self) -> usize {
        1
    }
    fn defline(&self) -> usize {
        1
    }
    fn payoffs(&self, path: &Scenario, result: &mut Vec<f64>) {
        result[0] = path.samples()[0].numeraire;
    }
}

fn sequential_simulate(prd: &DummyProduct, mdl: &DummyModel, rng: &Mrg32k3a, paths: usize) {
    let payoff_count = prd.payoff_labels().len();
    let mut result = vec![vec![0.0; payoff_count]; paths];
    mdl.init(prd.timeline(), prd.defline());
    let sim_dims = mdl.simulation_dimensions();
    (0..paths).for_each(|i| {
        let mut r_local = rng.clone();
        r_local.init(sim_dims);
        r_local.set_stream(i as u64);
        let mut g = vec![0.0; sim_dims];
        r_local.generate_gaussian(&mut g);
        let mut path = Scenario::new(prd.defline());
        mdl.generate_path(&g, &mut path);
        prd.payoffs(&path, &mut result[i]);
    });
    black_box(result);
}

fn bench_monte_carlo(c: &mut Criterion) {
    let prd = DummyProduct;
    let mdl = DummyModel;
    let paths = 1024;
    let rng = Mrg32k3a::new(12345, paths);

    c.bench_function("sequential", |b| {
        b.iter(|| sequential_simulate(&prd, &mdl, &rng, paths))
    });
    c.bench_function("parallel", |b| {
        b.iter(|| monte_carlo::simulate(&prd, &mdl, &rng, paths))
    });
}

criterion_group!(mc_benches, bench_monte_carlo);
criterion_main!(mc_benches);
