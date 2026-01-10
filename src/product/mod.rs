pub mod cashflow;
pub mod definition;
pub mod required;
pub mod state;

pub use cashflow::{Amount, Cashflow, CashflowBuffer, CashflowKindId};
pub use definition::{Product, ProductDefinition};
pub use required::{RequiredDataBuffer, RequiredDataLayout, RequiredDataLayoutError};
pub use state::ProductState;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Date, DateError, Frequency};

    #[test]
    fn cashflow_new_sets_fields() -> Result<(), DateError> {
        let amount = Amount::from_f64(12.5);
        let date = Date::new(2024, 5, 15)?;
        let cf = Cashflow::new(date, amount, CashflowKindId(0));

        assert_eq!(cf.time, date);
        assert_eq!(cf.amount, amount);
        assert_eq!(cf.kind, CashflowKindId(0));
        Ok(())
    }

    #[test]
    fn cashflow_from_period_uses_date_utilities() -> Result<(), DateError> {
        let amount = Amount::from_f64(25.0);
        let start = Date::new(2023, 1, 31)?;

        let cf = Cashflow::from_period(start, 1, Frequency::Monthly, amount, CashflowKindId(2))?;

        assert_eq!(cf.time, Date::new(2023, 2, 28)?);
        Ok(())
    }

    #[test]
    fn cashflows_can_be_ordered_by_date() -> Result<(), DateError> {
        let start = Date::new(2023, 1, 1)?;
        let amount = Amount::from_f64(10.0);
        let mut flows = [
            Cashflow::from_period(start, 2, Frequency::Weekly, amount, CashflowKindId(1))?,
            Cashflow::from_period(start, 0, Frequency::Weekly, amount, CashflowKindId(1))?,
            Cashflow::from_period(start, 1, Frequency::Weekly, amount, CashflowKindId(1))?,
        ];

        flows.sort_by_key(|flow| flow.time);

        assert_eq!(flows[0].time, Date::new(2023, 1, 1)?);
        assert_eq!(flows[1].time, Date::new(2023, 1, 8)?);
        assert_eq!(flows[2].time, Date::new(2023, 1, 15)?);
        Ok(())
    }

    #[test]
    fn reserve_change_updates_reserves_only() {
        let mut state = ProductState::new(1, 75, Amount::from_f64(80.0));

        state.apply_reserve_change(Amount::from_f64(-5.0));

        assert_eq!(state.reserves, Amount::from_f64(75.0));
    }
}
