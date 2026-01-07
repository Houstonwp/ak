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
