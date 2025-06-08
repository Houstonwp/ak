use jiff::civil::Date;

pub trait DateExt {
    fn is_last_of_month(&self) -> bool;
    fn is_last_of_february(&self) -> bool;
    fn as_tuple(&self) -> (i16, i8, i8);
}

impl DateExt for Date {
    fn is_last_of_month(&self) -> bool {
        let last_day_of_month = self.last_of_month();
        *self == last_day_of_month
    }

    fn is_last_of_february(&self) -> bool {
        self.month() == 2 && self.is_last_of_month()
    }

    fn as_tuple(&self) -> (i16, i8, i8) {
        (self.year(), self.month(), self.day())
    }
}
