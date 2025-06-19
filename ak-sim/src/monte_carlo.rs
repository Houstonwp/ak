use crate::{Model, Product, RNG, Scenario};

pub fn simulate<P: Product, M: Model, R: RNG>(
    prd: &P,
    mdl: &M,
    rng: &R,
    paths: usize,
) -> Vec<Vec<f64>> {
    let _mdl = mdl.clone();
    let _rng = rng.clone();

    let payoffs = prd.payoff_labels().len();
    let mut results = vec![vec![0.0; payoffs]; paths];
    _mdl.init(prd.timeline(), prd.defline());
    _rng.init(_mdl.simulation_dimensions());

    let mut gaussian_vec = vec![0.0; _mdl.simulation_dimensions()];
    let mut path = Scenario::new(prd.defline());
    results.iter_mut().for_each(|result| {
        _rng.generate_gaussian(&mut gaussian_vec);
        _mdl.generate_path(&gaussian_vec, &mut path);
        prd.payoffs(&path, result);
    });
    results
}
