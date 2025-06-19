pub trait RNG: Clone + Send {
    fn init(&mut self, dimensions: usize);
    fn generate_uniform(&mut self, output: &mut [f64]);
    fn generate_gaussian(&mut self, output: &mut [f64]);
    fn set_stream(&mut self, _stream: u64) {}
}
