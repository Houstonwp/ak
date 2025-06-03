use jiff::ToSpan;
use jiff::civil::Date;
use jiff::civil::Weekday;
use std::collections::HashMap;
use std::collections::HashSet;
// Holidays
//
// Weekends
//
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Holiday {
    pub name: String,
    pub id: String,
    pub date: Date, // ISO 8601 format
}

impl Holiday {
    pub fn new(name: &str, id: &str, date: Date) -> Self {
        Holiday {
            name: name.to_string(),
            id: id.to_string(),
            date,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PackedCalendar {
    pub holidays: Vec<Holiday>,
    pub weekends: HashSet<Weekday>, // Set of days of the week that are considered weekends
}

impl PackedCalendar {
    pub fn new() -> Self {
        PackedCalendar {
            holidays: Vec::new(),
            weekends: HashSet::new(),
        }
    }

    pub fn add_holiday(&mut self, name: &str, id: &str, date: Date) {
        self.holidays.push(Holiday::new(name, id, date));
    }

    pub fn add_weekend(&mut self, day_of_week: Weekday) {
        self.weekends.insert(day_of_week);
    }

    pub fn add_weekends(&mut self, days: &[Weekday]) {
        for &day in days {
            self.weekends.insert(day);
        }
    }

    pub fn is_holiday(&self, date: Date) -> bool {
        self.holidays.iter().any(|h| h.date == date)
    }

    pub fn is_weekend(&self, &day_of_week: &Weekday) -> bool {
        self.weekends.contains(&day_of_week)
    }

    pub fn holidays(&self) -> Vec<Date> {
        self.holidays.iter().map(|h| h.date).collect()
    }

    pub fn weekends(&self) -> Vec<Weekday> {
        self.weekends.iter().cloned().collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Calendar {
    pub name: String,
    pub packed_calendar: PackedCalendar,
    pub business_days: HashMap<Date, bool>,
}

impl Calendar {
    pub fn new(name: &str, packed_calendar: PackedCalendar) -> Self {
        Calendar {
            name: name.to_string(),
            packed_calendar,
            business_days: HashMap::new(),
        }
    }

    pub fn with_range(self, from: Date, to: Date) -> Calendar {
        let d1 = from;
        let d2 = to;
        let holidays = self.holidays();
        let weekends = self.weekends();
        let ds = d1.series(1.days()).take_while(|&d| d <= d2);

        let bds: HashMap<Date, bool> = ds
            .map(|d| {
                let is_holiday = holidays.contains(&d);
                let is_weekend = weekends.contains(&d.weekday());
                (d, !(is_holiday || is_weekend))
            })
            .collect();
        Calendar {
            name: self.name,
            packed_calendar: self.packed_calendar,
            business_days: bds,
        }
    }

    pub fn is_holiday(&self, day: Date) -> bool {
        self.packed_calendar.is_holiday(day)
    }

    pub fn is_weekend(&self, day: Date) -> bool {
        self.packed_calendar.is_weekend(&day.weekday())
    }

    pub fn holidays(&self) -> Vec<Date> {
        self.packed_calendar.holidays()
    }

    pub fn weekends(&self) -> Vec<Weekday> {
        self.packed_calendar.weekends()
    }

    pub fn is_business_day(&self, day: Date) -> bool {
        *self.business_days.get(&day).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jiff::civil::date;

    #[test]
    fn test_calendar() {
        let mut packed_calendar = PackedCalendar::new();
        packed_calendar.add_holiday("New Year's Day", "NYD", date(2023, 1, 1));
        packed_calendar.add_weekend(Weekday::Saturday);
        packed_calendar.add_weekend(Weekday::Sunday);

        let calendar = Calendar::new("Test Calendar", packed_calendar)
            .with_range(date(2023, 1, 1), date(2023, 1, 10));

        assert!(calendar.is_holiday(date(2023, 1, 1)));
        assert!(calendar.is_weekend(date(2023, 1, 7)));
        assert!(calendar.is_business_day(date(2023, 1, 2)));
    }
}
