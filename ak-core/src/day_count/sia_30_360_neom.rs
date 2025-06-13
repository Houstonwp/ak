use crate::day_count::DayCount;
use jiff::civil::Date;

pub struct SIA30360NEOM;

impl DayCount for SIA30360NEOM {
    fn day_diff(&self, start: Date, end: Date) -> i64 {
        let min_date = start.min(end);
        let max_date = start.max(end);

        let d1 = min_date.day().clamp(1, 30);
        let d2 = max_date.day().clamp(1, 30);

        let result = (max_date.year() - min_date.year()) as i64 * 360
            + (max_date.month() - min_date.month()) as i64 * 30
            + (d2 - d1) as i64;
        if min_date != start { -result } else { result }
    }

    fn year_diff(&self, start: Date, end: Date) -> f64 {
        self.day_diff(start, end) as f64 / 360.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jiff::civil::date;

    #[test]
    fn test_year_diff() {
        let dc = SIA30360NEOM;
        let start = date(2020, 1, 31);
        let end = date(2021, 1, 31);
        assert_eq!(dc.year_diff(start, end), 1.0);
    }

    #[test]
    fn test_day_diff() {
        let dc = SIA30360NEOM;
        let start = date(2020, 1, 31);
        let end = date(2021, 1, 31);
        assert_eq!(dc.day_diff(start, end), 360);
    }

    #[test]
    fn test_day_diffs() {
        let cases = vec![
            (date(1992, 2, 1), date(1992, 3, 1), 30),
            (date(1993, 1, 1), date(1993, 2, 21), 50),
            (date(1993, 1, 1), date(1994, 1, 1), 360),
            (date(1993, 1, 15), date(1993, 2, 1), 16),
            (date(1993, 2, 1), date(1993, 3, 1), 30),
            (date(1993, 2, 15), date(1993, 4, 1), 46),
            (date(1993, 3, 15), date(1993, 6, 15), 90),
            (date(1993, 3, 31), date(1993, 4, 1), 1),
            (date(1993, 3, 31), date(1993, 4, 30), 30),
            (date(1993, 3, 31), date(1993, 12, 31), 270),
            (date(1993, 7, 15), date(1993, 9, 15), 60),
            (date(1993, 8, 21), date(1994, 4, 11), 230),
            (date(1993, 11, 1), date(1994, 3, 1), 120),
            (date(1993, 12, 15), date(1993, 12, 30), 15),
            (date(1993, 12, 15), date(1993, 12, 31), 15),
            (date(1993, 12, 31), date(1994, 2, 1), 31),
            (date(1996, 1, 15), date(1996, 5, 31), 135),
            (date(1998, 2, 27), date(1998, 3, 27), 30),
            (date(1998, 2, 28), date(1998, 3, 27), 29),
            (date(1999, 1, 1), date(1999, 1, 29), 28),
        ];
        for (start, end, expected) in cases {
            assert_eq!(SIA30360NEOM.day_diff(start, end), expected);
        }
    }
}

