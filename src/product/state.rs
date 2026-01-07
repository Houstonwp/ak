use super::Amount;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProductState {
    pub in_force: u64,
    pub reserves: Amount,
}

impl ProductState {
    pub const fn new(in_force: u64, reserves: Amount) -> Self {
        Self { in_force, reserves }
    }

    pub fn apply_reserve_change(&mut self, delta: Amount) {
        self.reserves += delta;
    }
}
