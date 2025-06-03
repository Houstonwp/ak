use crate::day_count::DayCount;
use jiff::civil::Date;

pub struct Actual36525;

impl DayCount for Actual36525 {
    fn year_diff(&self, start: Date, end: Date) -> f64 {
        let days = (end - start).get_days() as f64;
        days / 365.25
    }

    fn day_diff(&self, start: Date, end: Date) -> i64 {
        (end - start).get_days() as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jiff::civil::date;

    #[test]
    fn test_year_diff() {
        let day_count = Actual36525;
        let start = date(2004, 2, 1);
        let end = date(2004, 5, 1);
        let result = day_count.year_diff(start, end);
        let expected = 0.2464;
        assert!(
            (result - expected).abs() < 1e-4,
            "Expected {}, got {}",
            expected,
            result
        );
    }

    #[test]
    fn test_day_diff() {
        let day_count = Actual36525;
        let start = date(2004, 2, 1);
        let end = date(2004, 3, 1);
        assert_eq!(day_count.day_diff(start, end), 29);
    }

    #[test]
    fn test_year_diffs() {
        let cases = vec![
            (date(1992, 2, 1), date(1992, 3, 1), 0.0794),
            (date(1992, 2, 1), date(1993, 3, 1), 1.0787),
            (date(1993, 1, 1), date(1993, 2, 21), 0.1396),
            (date(1993, 1, 1), date(1994, 1, 1), 0.9993),
            (date(1993, 1, 15), date(1993, 2, 1), 0.0465),
            (date(1993, 2, 1), date(1993, 3, 1), 0.0767),
            (date(1993, 2, 1), date(1996, 2, 1), 2.9979),
            (date(1993, 2, 1), date(1993, 3, 1), 0.0767),
            (date(1993, 2, 15), date(1993, 4, 1), 0.1232),
            (date(1993, 3, 15), date(1993, 6, 15), 0.2519),
            (date(1993, 3, 31), date(1993, 4, 1), 0.0027),
            (date(1993, 3, 31), date(1993, 4, 30), 0.0821),
            (date(1993, 3, 31), date(1993, 12, 31), 0.7529),
            (date(1993, 7, 15), date(1993, 9, 15), 0.1697),
            (date(1993, 8, 21), date(1994, 4, 11), 0.6379),
            (date(1993, 11, 1), date(1994, 3, 1), 0.3285),
            (date(1993, 12, 15), date(1993, 12, 30), 0.0411),
            (date(1993, 12, 15), date(1993, 12, 31), 0.0438),
            (date(1993, 12, 31), date(1994, 2, 1), 0.0876),
            (date(1996, 1, 15), date(1996, 5, 31), 0.3751),
            (date(1998, 2, 27), date(1998, 3, 27), 0.0767),
            (date(1998, 2, 28), date(1998, 3, 27), 0.0739),
        ];
        for (start, end, expected) in cases {
            let result = Actual36525.year_diff(start, end);
            assert!(
                (result - expected).abs() < 1e-4,
                "Expected {}, got {}",
                expected,
                result
            );
        }
    }

    #[test]
    fn test_day_diffs() {
        let cases = vec![
            (date(1992, 2, 1), date(1992, 3, 1), 29),
            (date(1992, 2, 1), date(1993, 3, 1), 394),
            (date(1993, 1, 1), date(1993, 2, 21), 51),
            (date(1993, 1, 1), date(1994, 1, 1), 365),
            (date(1993, 1, 15), date(1993, 2, 1), 17),
            (date(1993, 2, 1), date(1993, 3, 1), 28),
            (date(1993, 2, 1), date(1996, 2, 1), 1095),
            (date(1993, 2, 1), date(1993, 3, 1), 28),
            (date(1993, 2, 15), date(1993, 4, 1), 45),
            (date(1993, 3, 15), date(1993, 6, 15), 92),
            (date(1993, 3, 31), date(1993, 4, 1), 1),
            (date(1993, 3, 31), date(1993, 4, 30), 30),
            (date(1993, 3, 31), date(1993, 12, 31), 275),
            (date(1993, 7, 15), date(1993, 9, 15), 62),
            (date(1993, 8, 21), date(1994, 4, 11), 233),
            (date(1993, 11, 1), date(1994, 3, 1), 120),
            (date(1993, 12, 15), date(1993, 12, 30), 15),
            (date(1993, 12, 15), date(1993, 12, 31), 16),
            (date(1993, 12, 31), date(1994, 2, 1), 32),
            (date(1996, 1, 15), date(1996, 5, 31), 137),
            (date(1998, 2, 27), date(1998, 3, 27), 28),
            (date(1998, 2, 28), date(1998, 3, 27), 27),
            (date(1999, 1, 1), date(1999, 1, 29), 28),
            (date(1999, 1, 29), date(1999, 1, 30), 1),
            (date(1999, 1, 29), date(1999, 1, 31), 2),
            (date(1999, 1, 29), date(1999, 3, 29), 59),
            (date(1999, 1, 29), date(1999, 3, 30), 60),
            (date(1999, 1, 29), date(1999, 3, 31), 61),
            (date(1999, 1, 30), date(1999, 1, 31), 1),
            (date(1999, 1, 30), date(1999, 2, 27), 28),
            (date(1999, 1, 30), date(1999, 2, 28), 29),
            (date(1999, 1, 30), date(1999, 3, 29), 58),
            (date(1999, 1, 30), date(1999, 3, 30), 59),
            (date(1999, 1, 30), date(1999, 3, 31), 60),
            (date(1999, 1, 31), date(1999, 3, 29), 57),
            (date(1999, 1, 31), date(1999, 3, 30), 58),
            (date(1999, 1, 31), date(1999, 3, 31), 59),
            (date(1999, 2, 27), date(1999, 2, 27), 0),
            (date(1999, 2, 27), date(1999, 2, 28), 1),
            (date(1999, 2, 28), date(1999, 2, 27), -1),
            (date(1999, 2, 28), date(1999, 2, 28), 0),
            (date(2000, 1, 29), date(2000, 1, 30), 1),
            (date(2000, 1, 29), date(2000, 1, 31), 2),
            (date(2000, 1, 29), date(2000, 3, 29), 60),
            (date(2000, 1, 29), date(2000, 3, 30), 61),
            (date(2000, 1, 29), date(2000, 3, 31), 62),
            (date(2000, 1, 30), date(2000, 1, 31), 1),
            (date(2000, 1, 30), date(2000, 2, 27), 28),
            (date(2000, 1, 30), date(2000, 2, 28), 29),
            (date(2000, 1, 30), date(2000, 2, 29), 30),
            (date(2000, 1, 30), date(2000, 3, 29), 59),
            (date(2000, 1, 30), date(2000, 3, 30), 60),
            (date(2000, 1, 30), date(2000, 3, 31), 61),
            (date(2000, 1, 31), date(2000, 3, 29), 58),
            (date(2000, 1, 31), date(2000, 3, 30), 59),
            (date(2000, 1, 31), date(2000, 3, 31), 60),
            (date(2000, 2, 27), date(2000, 2, 27), 0),
            (date(2000, 2, 27), date(2000, 2, 28), 1),
            (date(2000, 2, 27), date(2000, 2, 29), 2),
            (date(2000, 2, 28), date(2000, 2, 27), -1),
            (date(2000, 2, 28), date(2000, 2, 28), 0),
            (date(2000, 2, 28), date(2000, 2, 29), 1),
            (date(2000, 2, 29), date(2000, 2, 27), -2),
            (date(2000, 2, 29), date(2000, 2, 28), -1),
            (date(2000, 2, 29), date(2000, 2, 29), 0),
            (date(2000, 7, 29), date(2000, 8, 31), 33),
            (date(2000, 7, 29), date(2000, 9, 1), 34),
            (date(2000, 7, 30), date(2000, 8, 31), 32),
            (date(2000, 7, 30), date(2000, 9, 1), 33),
            (date(2000, 7, 31), date(2000, 8, 31), 31),
            (date(2000, 7, 31), date(2000, 9, 1), 32),
            (date(2000, 8, 1), date(2000, 8, 31), 30),
            (date(2000, 8, 1), date(2000, 9, 1), 31),
            (date(2003, 2, 28), date(2004, 2, 29), 366),
        ];
        for (start, end, expected) in cases {
            let result = Actual36525.day_diff(start, end);
            assert_eq!(result, expected, "Expected {}, got {}", expected, result);
        }
    }
}
