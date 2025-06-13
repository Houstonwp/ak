use crate::day_count::DayCount;
use jiff::civil::Date;

pub struct NL365;

impl DayCount for NL365 {
    fn day_diff(&self, start: Date, end: Date) -> i64 {
        let month_correction = [0, 0, 0, 3, 3, 4, 4, 5, 5, 5, 6, 6, 7];

        let y1 = start.year();
        let y2 = end.year();
        let m1 = start.month();
        let m2 = end.month();
        let d1 = start.day();
        let d2 = end.day();

        ((y2 - y1) * 365) as i64 + ((m2 - m1) * 31) as i64
            - (month_correction[m1 as usize] - month_correction[m2 as usize]) as i64
            + (d2 - d1) as i64
    }

    fn year_diff(&self, start: Date, end: Date) -> f64 {
        self.day_diff(start, end) as f64 / 365.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jiff::civil::date;

    #[test]
    fn test_year_diff() {
        let dc = NL365;
        let start = date(2020, 1, 1);
        let end = date(2021, 1, 1);
        assert_eq!(dc.year_diff(start, end), 1.0);
    }

    #[test]
    fn test_day_diff() {
        let dc = NL365;
        let start = date(2020, 1, 1);
        let end = date(2020, 1, 31);
        assert_eq!(dc.day_diff(start, end), 30);
    }

    #[test]
    fn test_day_diffs() {
        let cases = vec![
            (date(1992, 2, 1), date(1992, 3, 1), 34),
            (date(1993, 1, 1), date(1993, 2, 21), 51),
            (date(1993, 1, 15), date(1993, 2, 1), 17),
            (date(1993, 2, 1), date(1993, 3, 1), 34),
            (date(1993, 2, 15), date(1993, 4, 1), 51),
            (date(1993, 3, 15), date(1993, 6, 15), 94),
            (date(1993, 3, 31), date(1993, 4, 1), 1),
            (date(1993, 3, 31), date(1993, 4, 30), 30),
            (date(1993, 7, 15), date(1993, 9, 15), 62),
            (date(1993, 12, 15), date(1993, 12, 30), 15),
        ];
        for (start, end, expected) in cases {
            assert_eq!(NL365.day_diff(start, end), expected);
        }
    }

    #[test]
    fn test_year_diffs() {
        let cases = vec![
            (date(1992, 2, 1), date(1992, 3, 1), 34),
            (date(1993, 1, 1), date(1993, 2, 21), 51),
            (date(1993, 1, 15), date(1993, 2, 1), 17),
            (date(1993, 2, 1), date(1993, 3, 1), 34),
            (date(1993, 2, 15), date(1993, 4, 1), 51),
            (date(1993, 3, 15), date(1993, 6, 15), 94),
            (date(1993, 3, 31), date(1993, 4, 1), 1),
            (date(1993, 3, 31), date(1993, 4, 30), 30),
            (date(1993, 7, 15), date(1993, 9, 15), 62),
            (date(1993, 12, 15), date(1993, 12, 30), 15),
        ];
        for (start, end, expected_days) in cases {
            let expected = expected_days as f64 / 365.0;
            let result = NL365.year_diff(start, end);
            assert!((result - expected).abs() < 1e-10, "{} {}", result, expected);
        }
    }
}

