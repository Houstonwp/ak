use crate::frequency::Frequency;

#[derive(Clone, Copy, Debug)]
pub enum Compounding {
    Simple,
    Compounded(Frequency),
    Continuous,
    SimplteThenCompounded(Frequency),
}
