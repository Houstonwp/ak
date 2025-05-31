use jiff::civil::Date;

trait DayCount {
    fn year_diff(&self, start: Date, end: Date) -> f64;
    fn day_diff(&self, start: Date, end: Date) -> i64;
}

struct ISDAActualActual;

impl DayCount for ISDAActualActual {
    fn year_diff(&self, start: Date, end: Date) -> f64 {
        let start_year = start.year() as f64;
        let end_year = end.year() as f64;
        let years = end_year - start_year - 1.0;
        let start_year_days = start.days_in_year() as f64;
        let start_days = start.day_of_year() as f64;
        let end_year_days = end.days_in_year() as f64;
        let end_days = end.day_of_year() as f64;
        let start_days_remaining_in_year = start_year_days - start_days;
        let numerator = years * start_year_days * end_year_days
            + start_days_remaining_in_year * end_year_days
            + end_days * start_year_days;
        let denominator = start_year_days * end_year_days;
        numerator / denominator
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
        let day_count = ISDAActualActual;
        let start = date(2003, 10, 19);
        let end = date(2003, 12, 31);
        assert_eq!(day_count.year_diff(start, end), 0.2);
    }

    #[test]
    fn test_day_diff() {
        let day_count = ISDAActualActual;
        let start = date(2003, 10, 19);
        let end = date(2003, 12, 31);
        assert_eq!(day_count.day_diff(start, end), 73);
    }
}
