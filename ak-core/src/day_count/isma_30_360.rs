use crate::day_count::DayCount;
use jiff::civil::Date;

pub struct ISMA30360;

impl DayCount for ISMA30360 {
    fn year_diff(&self, start: Date, end: Date) -> f64 {
        self.day_diff(start, end) as f64 / 360.0
    }

    fn day_diff(&self, start: Date, end: Date) -> i64 {
        (end.year() - start.year()) as i64 * 360
            + (end.month() - start.month()) as i64 * 30
            + (end.day().clamp(1, 30) - start.day().clamp(1, 30)) as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jiff::civil::date;

    #[test]
    fn test_year_diff() {
        let day_count = ISMA30360;
        let start = date(2020, 1, 31);
        let end = date(2021, 1, 31);
        assert_eq!(day_count.year_diff(start, end), 1.0);
    }

    #[test]
    fn test_day_diff() {
        let day_count = ISMA30360;
        let start = date(2020, 1, 31);
        let end = date(2021, 1, 31);
        assert_eq!(day_count.day_diff(start, end), 360);
    }
}
