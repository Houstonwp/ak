use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

use crate::{Date, DateError, Frequency};

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Amount(f64);

impl Amount {
    pub const fn zero() -> Self {
        Self(0.0)
    }

    pub fn from_cents(cents: i64) -> Self {
        Self((cents as f64) / 100.0)
    }

    pub const fn from_f64(value: f64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> f64 {
        self.0
    }
}

impl Add for Amount {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Amount {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl AddAssign for Amount {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl SubAssign for Amount {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl Neg for Amount {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CashflowKindId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cashflow {
    pub time: Date,
    pub amount: Amount,
    pub kind: CashflowKindId,
}

impl Cashflow {
    pub fn new(time: Date, amount: Amount, kind: CashflowKindId) -> Self {
        Self { time, amount, kind }
    }

    /// Creates a cashflow at a projection period derived from the start date.
    ///
    /// Period index 0 is the start date. Periods advance using the shared date utility's
    /// calendar rules for the given frequency (e.g. monthly uses month arithmetic and
    /// preserves end-of-month where applicable).
    pub fn from_period(
        start: Date,
        index: usize,
        frequency: Frequency,
        amount: Amount,
        kind: CashflowKindId,
    ) -> Result<Self, DateError> {
        let time = crate::cashflow_date_at(start, index, frequency)?;
        Ok(Self { time, amount, kind })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CashflowBufferError;

/// SoA cashflow storage with fixed dimensions per state/kind/step.
#[derive(Debug, Clone)]
pub struct CashflowBuffer {
    times: Vec<Date>,
    amounts: Vec<Amount>,
    n_states: usize,
    n_kinds: usize,
}

impl CashflowBuffer {
    pub fn new(
        n_states: usize,
        n_kinds: usize,
        times: Vec<Date>,
    ) -> Result<Self, CashflowBufferError> {
        if n_states == 0 || n_kinds == 0 || times.is_empty() {
            return Err(CashflowBufferError);
        }
        let len = n_states
            .checked_mul(n_kinds)
            .and_then(|v| v.checked_mul(times.len()))
            .ok_or(CashflowBufferError)?;
        Ok(Self {
            times,
            amounts: vec![Amount::zero(); len],
            n_states,
            n_kinds,
        })
    }

    pub fn from_parts(
        n_states: usize,
        n_kinds: usize,
        times: Vec<Date>,
        amounts: Vec<Amount>,
    ) -> Result<Self, CashflowBufferError> {
        if n_states == 0 || n_kinds == 0 || times.is_empty() {
            return Err(CashflowBufferError);
        }
        let expected = n_states
            .checked_mul(n_kinds)
            .and_then(|v| v.checked_mul(times.len()))
            .ok_or(CashflowBufferError)?;
        if expected != amounts.len() {
            return Err(CashflowBufferError);
        }
        Ok(Self {
            times,
            amounts,
            n_states,
            n_kinds,
        })
    }

    pub fn times(&self) -> &[Date] {
        &self.times
    }

    pub fn n_states(&self) -> usize {
        self.n_states
    }

    pub fn n_kinds(&self) -> usize {
        self.n_kinds
    }

    pub fn len_steps(&self) -> usize {
        self.times.len()
    }

    pub fn amount(&self, state: usize, kind: usize, step: usize) -> Amount {
        let idx = self.offset(state, kind, step);
        self.amounts[idx]
    }

    pub fn amount_mut(&mut self, state: usize, kind: usize, step: usize) -> &mut Amount {
        let idx = self.offset(state, kind, step);
        &mut self.amounts[idx]
    }

    fn offset(&self, state: usize, kind: usize, step: usize) -> usize {
        debug_assert!(state < self.n_states);
        debug_assert!(kind < self.n_kinds);
        debug_assert!(step < self.times.len());
        (state * self.n_kinds + kind) * self.times.len() + step
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn amount_arithmetic_behaves_as_expected() {
        let mut amount = Amount::from_f64(100.0);
        amount += Amount::from_f64(50.0);
        assert_eq!(amount, Amount::from_f64(150.0));

        amount -= Amount::from_f64(20.0);
        assert_eq!(amount.value(), 130.0);

        let total = amount + Amount::from_f64(70.0);
        assert_eq!(total, Amount::from_f64(200.0));

        let diff = total - Amount::from_f64(25.0);
        assert_eq!(diff, Amount::from_f64(175.0));

        let negated = -diff;
        assert_eq!(negated, Amount::from_f64(-175.0));

        assert_eq!(Amount::zero(), Amount::from_f64(0.0));
    }

    #[test]
    fn cashflow_construction_helpers_match_inputs() -> Result<(), DateError> {
        let date = Date::new(2024, 6, 15)?;
        let amount = Amount::from_f64(12_345.0);
        let kind = CashflowKindId(3);

        let flow = Cashflow::new(date, amount, kind);
        assert_eq!(flow.time, date);
        assert_eq!(flow.amount, amount);
        assert_eq!(flow.kind, kind);

        let from_period = Cashflow::from_period(date, 2, Frequency::Monthly, amount, kind)?;
        assert_eq!(from_period.amount, amount);
        assert_eq!(from_period.kind, kind);
        assert_eq!(from_period.time, Date::new(2024, 8, 15)?);
        Ok(())
    }

    #[test]
    fn cashflow_buffer_indexes_are_stable() -> Result<(), DateError> {
        let times = vec![
            Date::new(2024, 1, 1)?,
            Date::new(2024, 2, 1)?,
            Date::new(2024, 3, 1)?,
        ];
        let mut buffer = CashflowBuffer::new(2, 3, times).unwrap();
        *buffer.amount_mut(1, 2, 0) = Amount::from_f64(42.0);
        *buffer.amount_mut(0, 1, 2) = Amount::from_f64(-7.5);
        assert_eq!(buffer.amount(1, 2, 0), Amount::from_f64(42.0));
        assert_eq!(buffer.amount(0, 1, 2), Amount::from_f64(-7.5));
        Ok(())
    }
}
