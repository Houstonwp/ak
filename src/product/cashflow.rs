use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

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
    pub time: u32,
    pub amount: Amount,
    pub kind: CashflowKind,
}

impl Cashflow {
    pub const fn new(time: u32, amount: Amount, kind: CashflowKind) -> Self {
        Self { time, amount, kind }
    }
}
