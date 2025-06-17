use crate::ast::DateIndex;

pub struct SimulationData {
    pub spot: f64,
}

pub type Scenario = Vec<SimulationData>;

pub trait Model {
    fn initialize(event_dates: Vec<DateIndex>);
    fn next_scenario(s: Scenario);
}
