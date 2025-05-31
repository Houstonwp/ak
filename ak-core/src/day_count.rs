use jiff::civil::Date;
mod actual_360;
mod actual_36525;
mod actual_365_fixed;
mod bus_252;
mod isda_1_1;
mod isda_30_360;
mod isda_actual_actual;
mod nl_365;
mod psa_30_360_eom;
mod sia_30_360_eom;
mod sia_30_360_neom;

trait DayCount {
    fn year_diff(&self, start: Date, end: Date) -> f64;
    fn day_diff(&self, start: Date, end: Date) -> i64;
}
