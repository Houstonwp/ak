use crate::{Model, Product, Scenario};
use ak_random::RNG;
use rayon::prelude::*;

pub fn simulate<P, M, R>(
    prd: &P,
    mdl: &M,
    rng: &R,
    paths: usize,
) -> Vec<Vec<f64>>
where
    P: Product + Sync,
    M: Model + Sync,
    R: RNG + Sync,
{
    let payoffs = prd.payoff_labels().len();
    let mut results = vec![vec![0.0; payoffs]; paths];

    mdl.init(prd.timeline(), prd.defline());

    let sim_dims = mdl.simulation_dimensions();
    let defline = prd.defline();

    results
        .par_iter_mut()
        .enumerate()
        .for_each_init(
            || {
                let mut local_rng = rng.clone();
                local_rng.init(sim_dims);
                (
                    local_rng,
                    vec![0.0f64; sim_dims],
                    Scenario::new(defline),
                )
            },
            |state, (i, result)| {
                let (rng, gaussian_vec, path) = state;
                rng.set_stream(i as u64);
                rng.generate_gaussian(gaussian_vec);
                mdl.generate_path(gaussian_vec, path);
                prd.payoffs(path, result);
            },
        );
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use ak_random::{mrg32k3a::Mrg32k3a, RNG};

    #[derive(Clone)]
    struct DummyModel;

    impl Model for DummyModel {
        fn simulation_dimensions(&self) -> usize { 1 }
        fn init(&self, _: usize, _: usize) {}
        fn generate_path(&self, gaussian_vec: &[f64], path: &mut Scenario) {
            path.0[0].numeraire = gaussian_vec[0];
        }
    }

    #[derive(Clone)]
    struct DummyProduct;

    impl Product for DummyProduct {
        fn payoff_labels(&self) -> Vec<String> { vec!["dummy".into()] }
        fn timeline(&self) -> usize { 1 }
        fn defline(&self) -> usize { 1 }
        fn payoffs(&self, path: &Scenario, result: &mut Vec<f64>) {
            result[0] = path.samples()[0].numeraire;
        }
    }

    #[test]
    fn parallel_matches_sequential() {
        let prd = DummyProduct;
        let mdl = DummyModel;
        let rng = Mrg32k3a::default();
        let paths = 16;

        // Sequential reference using per-path streams
        let mut expected = Vec::new();
        let mut g = vec![0.0; mdl.simulation_dimensions()];
        for i in 0..paths {
            let mut r_local = rng.clone();
            r_local.init(mdl.simulation_dimensions());
            r_local.set_stream(i as u64);
            let mut s = Scenario::new(prd.defline());
            r_local.generate_gaussian(&mut g);
            mdl.generate_path(&g, &mut s);
            let mut res = vec![0.0];
            prd.payoffs(&s, &mut res);
            expected.push(res);
        }

        // Parallel simulation
        let results = super::simulate(&prd, &mdl, &rng, paths);

        for (r, e) in results.iter().zip(expected.iter()) {
            for (a, b) in r.iter().zip(e.iter()) {
                assert!(!a.is_nan() && !b.is_nan());
                assert!((a - b).abs() < 1e-12);
            }
        }
    }
}
