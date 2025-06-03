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
