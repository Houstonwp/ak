use crate::day_count::DayCount;
use jiff::civil::Date;

pub struct ISDA11;

impl DayCount for ISDA11 {
    fn year_diff(&self, start: Date, end: Date) -> f64 {
        (end - start).signum() as f64
    }

    fn day_diff(&self, start: Date, end: Date) -> i64 {
        (end - start).signum() as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jiff::civil::date;

    #[test]
    fn test_year_diff() {
        let day_count = ISDA11;
        let start = date(2004, 2, 1);
        let end = date(2004, 5, 1);
        assert_eq!(day_count.year_diff(start, end), 1.0);
    }

    #[test]
    fn test_day_diff() {
        let day_count = ISDA11;
        let start = date(2004, 2, 1);
        let end = date(2004, 3, 1);
        assert_eq!(day_count.day_diff(start, end), 1);
    }
}
