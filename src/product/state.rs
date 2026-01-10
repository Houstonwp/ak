use super::Amount;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProductState {
    pub state_id: usize,
    pub in_force: u64,
    pub reserves: Amount,
}

impl ProductState {
    pub const fn new(state_id: usize, in_force: u64, reserves: Amount) -> Self {
        Self {
            state_id,
            in_force,
            reserves,
        }
    }

    pub fn apply_reserve_change(&mut self, delta: Amount) {
        self.reserves += delta;
    }
}
