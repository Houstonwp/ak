pub mod cashflow;
pub mod state;

pub use cashflow::{Amount, Cashflow, CashflowKind};
pub use state::ProductState;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cashflow_new_sets_fields() {
        let amount = Amount::from_cents(12_50);
        let cf = Cashflow::new(4, amount, CashflowKind::Premium);

        assert_eq!(cf.time, 4);
        assert_eq!(cf.amount, amount);
        assert_eq!(cf.kind, CashflowKind::Premium);
    }

    #[test]
    fn reserve_change_updates_reserves_only() {
        let mut state = ProductState::new(75, Amount::from_cents(80_00));

        state.apply_reserve_change(Amount::from_cents(-5_00));

        assert_eq!(state.reserves, Amount::from_cents(75_00));
    }
}
