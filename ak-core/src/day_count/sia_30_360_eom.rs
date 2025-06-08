use crate::{day_count::DayCount, jiff_ext::DateExt};
use jiff::civil::Date;

pub struct SIA30360EOM;

impl DayCount for SIA30360EOM {
    fn day_diff(&self, start: Date, end: Date) -> i64 {
        let min_date = start.min(end);
        let max_date = start.max(end);

        let mut d1 = min_date.day();
        let mut d2 = max_date.day();
        if min_date.is_last_of_february() {
            if max_date.is_last_of_february() {
                d2 = 30;
            }
            d1 = 30;
        } else if min_date.is_last_of_month() {
            if max_date.is_last_of_month() {
                d2 = 30;
            }
            d1 = 30;
        };

        let result = (max_date.year() - min_date.year()) as i64 * 360
            + (max_date.month() - min_date.month()) as i64 * 30
            + (d2 - d1) as i64;
        if min_date != start { -result } else { result }
    }

    fn year_diff(&self, start: Date, end: Date) -> f64 {
        self.day_diff(start, end) as f64 / 360.0
    }
}
