use std::cmp::max;

use crate::day_count::DayCount;
use jiff::civil::Date;

struct PSA30360EOM;

impl DayCount for PSA30360EOM {
    fn year_diff(&self, start: Date, end: Date) -> f64 {
        self.day_diff(start, end) as f64 / 360.0
    }

    fn day_diff(&self, start: Date, end: Date) -> i64 {
        let negative = start > end;
        let negative_factor = if negative { -1 } else { 1 };
        let min_date = start.min(end);
        let max_date = start.max(end);

        let y1 = min_date.year();
        let y2 = max_date.year();
        let m1 = min_date.month();
        let m2 = max_date.month();

        let d1 = if (min_date.month() == 2 && min_date == min_date.last_of_month())
            || min_date.day() == 31
        {
            30
        } else {
            min_date.day()
        };

        let d2 = if d1 == 30 && max_date.day() == 31 {
            30
        } else {
            max_date.day()
        };

        negative_factor
            * max(
                (y2 - y1) as i64 * 360 + (m2 - m1) as i64 * 30 + (d2 - d1) as i64,
                0,
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jiff::civil::date;

    #[test]
    fn test_year_diff() {
        let day_count = PSA30360EOM;
        let start = date(2020, 1, 31);
        let end = date(2021, 1, 31);
        assert_eq!(day_count.year_diff(start, end), 1.0);
    }

    #[test]
    fn test_day_diff() {
        let day_count = PSA30360EOM;
        let start = date(2020, 1, 31);
        let end = date(2021, 1, 31);
        assert_eq!(day_count.day_diff(start, end), 360);
    }

    #[test]
    fn test_day_diffs() {
        let day_count = PSA30360EOM;
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
            (date(1993, 12, 15), date(1993, 12, 31), 16),
            (date(1993, 12, 31), date(1994, 2, 1), 31),
            (date(1996, 1, 15), date(1996, 5, 31), 136),
            (date(1998, 2, 27), date(1998, 3, 27), 30),
            (date(1998, 2, 28), date(1998, 3, 27), 27),
            (date(1999, 1, 1), date(1999, 1, 29), 28),
            (date(1999, 1, 29), date(1999, 1, 30), 1),
            (date(1999, 1, 29), date(1999, 1, 31), 2),
            (date(1999, 1, 29), date(1999, 3, 29), 60),
            (date(1999, 1, 29), date(1999, 3, 30), 61),
            (date(1999, 1, 29), date(1999, 3, 31), 62),
            (date(1999, 1, 30), date(1999, 1, 31), 0),
            (date(1999, 1, 30), date(1999, 2, 27), 27),
            (date(1999, 1, 30), date(1999, 2, 28), 28),
            (date(1999, 1, 30), date(1999, 3, 29), 59),
            (date(1999, 1, 30), date(1999, 3, 30), 60),
            (date(1999, 1, 30), date(1999, 3, 31), 60),
            (date(1999, 1, 31), date(1999, 3, 29), 59),
            (date(1999, 1, 31), date(1999, 3, 30), 60),
            (date(1999, 1, 31), date(1999, 3, 31), 60),
            (date(1999, 2, 27), date(1999, 2, 27), 0),
            (date(1999, 2, 27), date(1999, 2, 28), 1),
            (date(1999, 2, 28), date(1999, 2, 27), -1),
            (date(1999, 2, 28), date(1999, 2, 28), 0),
            (date(2000, 1, 29), date(2000, 1, 30), 1),
            (date(2000, 1, 29), date(2000, 1, 31), 2),
            (date(2000, 1, 29), date(2000, 3, 29), 60),
            (date(2000, 1, 29), date(2000, 3, 30), 61),
            (date(2000, 1, 29), date(2000, 3, 31), 62),
            (date(2000, 1, 30), date(2000, 1, 31), 0),
            (date(2000, 1, 30), date(2000, 2, 27), 27),
            (date(2000, 1, 30), date(2000, 2, 28), 28),
            (date(2000, 1, 30), date(2000, 2, 29), 29),
            (date(2000, 1, 30), date(2000, 3, 29), 59),
            (date(2000, 1, 30), date(2000, 3, 30), 60),
            (date(2000, 1, 30), date(2000, 3, 31), 60),
            (date(2000, 1, 31), date(2000, 3, 29), 59),
            (date(2000, 1, 31), date(2000, 3, 30), 60),
            (date(2000, 1, 31), date(2000, 3, 31), 60),
            (date(2000, 1, 29), date(2004, 1, 30), 1441),
            (date(2000, 1, 29), date(2004, 1, 31), 1442),
            (date(2000, 1, 29), date(2004, 3, 29), 1500),
            (date(2000, 1, 29), date(2004, 3, 30), 1501),
            (date(2000, 1, 29), date(2004, 3, 31), 1502),
            (date(2000, 1, 30), date(2004, 1, 31), 1440),
            (date(2000, 1, 30), date(2004, 2, 27), 1467),
            (date(2000, 1, 30), date(2004, 2, 28), 1468),
            (date(2000, 1, 30), date(2004, 2, 29), 1469),
            (date(2000, 1, 30), date(2004, 3, 29), 1499),
            (date(2000, 1, 30), date(2004, 3, 30), 1500),
            (date(2000, 1, 30), date(2004, 3, 31), 1500),
            (date(2000, 1, 31), date(2004, 3, 29), 1499),
            (date(2000, 1, 31), date(2004, 3, 30), 1500),
            (date(2000, 1, 31), date(2004, 3, 31), 1500),
            (date(2004, 1, 29), date(2000, 1, 30), -1439),
            (date(2004, 1, 29), date(2000, 1, 31), -1439),
            (date(2004, 1, 29), date(2000, 3, 29), -1380),
            (date(2004, 1, 29), date(2000, 3, 30), -1379),
            (date(2004, 1, 29), date(2000, 3, 31), -1379),
            (date(2004, 1, 30), date(2000, 1, 31), -1440),
            (date(2004, 1, 30), date(2000, 2, 27), -1413),
            (date(2004, 1, 30), date(2000, 2, 28), -1412),
            (date(2004, 1, 30), date(2000, 2, 29), -1410),
            (date(2004, 1, 30), date(2000, 3, 29), -1381),
            (date(2004, 1, 30), date(2000, 3, 30), -1380),
            (date(2004, 1, 30), date(2000, 3, 31), -1380),
            (date(2004, 1, 31), date(2000, 3, 29), -1382),
            (date(2004, 1, 31), date(2000, 3, 30), -1380),
            (date(2004, 1, 31), date(2000, 3, 31), -1380),
            (date(2000, 2, 27), date(2000, 2, 27), 0),
            (date(2000, 2, 27), date(2000, 2, 28), 1),
            (date(2000, 2, 27), date(2000, 2, 29), 2),
            (date(2000, 2, 28), date(2000, 2, 27), -1),
            (date(2000, 2, 28), date(2000, 2, 28), 0),
            (date(2000, 2, 28), date(2000, 2, 29), 1),
            (date(2000, 2, 29), date(2000, 2, 27), -2),
            (date(2000, 2, 29), date(2000, 2, 28), -1),
            (date(2000, 2, 29), date(2000, 2, 29), 0),
            (date(2000, 7, 29), date(2000, 8, 31), 32),
            (date(2000, 7, 29), date(2000, 9, 1), 32),
            (date(2000, 7, 30), date(2000, 8, 31), 30),
            (date(2000, 7, 30), date(2000, 9, 1), 31),
            (date(2000, 7, 31), date(2000, 8, 31), 30),
            (date(2000, 7, 31), date(2000, 9, 1), 31),
            (date(2000, 8, 1), date(2000, 8, 31), 30),
            (date(2000, 8, 1), date(2000, 9, 1), 30),
            (date(2003, 2, 28), date(2004, 2, 29), 359),
        ];
        for (start, end, expected) in cases {
            assert_eq!(day_count.day_diff(start, end), expected);
        }
    }
}
