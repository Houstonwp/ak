use crate::frequency::Frequency;

pub enum Compounding {
    Simple,
    Compounded(Frequency),
    Continuous,
    SimplteThenCompounded(Frequency),
}
