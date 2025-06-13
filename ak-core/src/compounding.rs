use crate::frequency::Frequency;

#[derive(Clone, Copy, Debug)]
pub enum Compounding {
    Simple,
    Compounded(Frequency),
    Continuous,
    /// Apply simple interest until a switch date and compound afterwards.
    SimpleThenCompounded(Frequency),
}
