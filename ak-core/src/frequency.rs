use jiff::{Span, ToSpan};

#[derive(Clone, Copy, Debug)]
pub enum Frequency {
    Yearly,
    Monthly,
    Weekly,
    Daily,
}

impl Frequency {
    pub fn to_span(self, interval: i64) -> Span {
        let base = match self {
            Frequency::Yearly => 1.year(),
            Frequency::Monthly => 1.month(),
            Frequency::Weekly => 1.week(),
            Frequency::Daily => 1.day(),
        };
        base.checked_mul(interval)
            .expect("Interval multiplication overflow")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frequency_to_span() {
        assert_eq!(Frequency::Yearly.to_span(2), 2.year().fieldwise());
        assert_eq!(Frequency::Monthly.to_span(3), 3.month().fieldwise());
        assert_eq!(Frequency::Weekly.to_span(4), 4.week().fieldwise());
        assert_eq!(Frequency::Daily.to_span(5), 5.day().fieldwise());
    }
}
