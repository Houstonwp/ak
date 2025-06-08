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
