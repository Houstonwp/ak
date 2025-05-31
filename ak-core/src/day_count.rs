use jiff::civil::Date;
mod actual_360;
mod isda_actual_actual;

trait DayCount {
    fn year_diff(&self, start: Date, end: Date) -> f64;
    fn day_diff(&self, start: Date, end: Date) -> i64;
}
