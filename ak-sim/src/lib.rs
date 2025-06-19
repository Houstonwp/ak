pub mod monte_carlo;

pub struct Scenario(Vec<Sample>);

impl Scenario {
    pub fn new(defline: usize) -> Self {
        Self(vec![Sample {
            numeraire: 1.0,
            forwards: vec![0.0; defline],
            discounts: vec![0.0; defline],
            libors: vec![0.0; defline],
        }])
    }

    pub fn add_sample(&mut self, sample: Sample) {
        self.0.push(sample);
    }

    pub fn samples(&self) -> &[Sample] {
        &self.0
    }

    pub fn mut_samples(&mut self) -> &mut [Sample] {
        &mut self.0
    }
}

pub struct Sample {
    pub numeraire: f64,
    pub forwards: Vec<f64>,
    pub discounts: Vec<f64>,
    pub libors: Vec<f64>,
}

pub trait Product {
    fn payoff_labels(&self) -> Vec<String>;
    fn payoff_count(&self) -> usize;
    fn timeline(&self) -> usize;
    fn defline(&self) -> usize;
    fn payoffs(&self, path: &Scenario, result: &mut Vec<f64>);
}

pub trait Model: Clone {
    fn simulation_dimensions(&self) -> usize;
    fn init(&self, timeline: usize, defline: usize);
    fn generate_path(&self, gaussian_vec: &[f64], path: &mut Scenario);
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
