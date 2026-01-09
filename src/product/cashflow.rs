use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

use crate::{Date, DateError, Frequency};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Amount(i64);

impl Amount {
    pub const fn zero() -> Self {
        Self(0)
    }

    pub const fn from_cents(cents: i64) -> Self {
        Self(cents)
    }

    pub const fn cents(self) -> i64 {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CashflowKind {
    Premium,
    Benefit,
    Expense,
    Investment,
    Tax,
    Other,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cashflow {
    pub time: Date,
    pub amount: Amount,
    pub kind: CashflowKind,
}

impl Cashflow {
    pub fn new(time: Date, amount: Amount, kind: CashflowKind) -> Self {
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
        kind: CashflowKind,
    ) -> Result<Self, DateError> {
        let time = crate::cashflow_date_at(start, index, frequency)?;
        Ok(Self { time, amount, kind })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn amount_arithmetic_behaves_as_expected() {
        let mut amount = Amount::from_cents(100);
        amount += Amount::from_cents(50);
        assert_eq!(amount, Amount::from_cents(150));

        amount -= Amount::from_cents(20);
        assert_eq!(amount.cents(), 130);

        let total = amount + Amount::from_cents(70);
        assert_eq!(total, Amount::from_cents(200));

        let diff = total - Amount::from_cents(25);
        assert_eq!(diff, Amount::from_cents(175));

        let negated = -diff;
        assert_eq!(negated, Amount::from_cents(-175));

        assert_eq!(Amount::zero(), Amount::from_cents(0));
    }

    #[test]
    fn cashflow_construction_helpers_match_inputs() -> Result<(), DateError> {
        let date = Date::new(2024, 6, 15)?;
        let amount = Amount::from_cents(12_345);
        let kind = CashflowKind::Premium;

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
}
