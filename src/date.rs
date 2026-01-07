use jiff::{Error, ToSpan};

pub use jiff::civil::Date;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Frequency {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    SemiAnnual,
    Annual,
}

pub type DateError = Error;

pub fn generate_cashflow_dates(
    start: Date,
    periods: usize,
    frequency: Frequency,
) -> Result<Vec<Date>, Error> {
    let mut dates = Vec::with_capacity(periods);
    for i in 0..periods {
        let offset = i as i64;
        let span = match frequency {
            Frequency::Daily => offset.days(),
            Frequency::Weekly => offset.weeks(),
            Frequency::Monthly => offset.months(),
            Frequency::Quarterly => (offset * 3).months(),
            Frequency::SemiAnnual => (offset * 6).months(),
            Frequency::Annual => (offset * 12).months(),
        };
        dates.push(start.checked_add(span)?);
    }
    Ok(dates)
}

pub fn is_leap_year(year: i16) -> Result<bool, Error> {
    Ok(Date::new(year, 1, 1)?.in_leap_year())
}

pub fn days_in_month(year: i16, month: i8) -> Result<i8, Error> {
    Ok(Date::new(year, month, 1)?.days_in_month())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_rejects_invalid_dates() {
        assert!(Date::new(2024, 0, 1).is_err());
        assert!(Date::new(2024, 2, 30).is_err());
    }

    #[test]
    fn add_months_preserves_end_of_month() -> Result<(), DateError> {
        let jan_31 = Date::new(2023, 1, 31)?;
        assert_eq!(jan_31.checked_add(1.months())?, Date::new(2023, 2, 28)?);

        let jan_31_leap = Date::new(2024, 1, 31)?;
        assert_eq!(
            jan_31_leap.checked_add(1.months())?,
            Date::new(2024, 2, 29)?
        );

        let jan_30 = Date::new(2023, 1, 30)?;
        assert_eq!(jan_30.checked_add(1.months())?, Date::new(2023, 2, 28)?);
        Ok(())
    }

    #[test]
    fn add_days_handles_year_boundary() -> Result<(), DateError> {
        let dec_31 = Date::new(2023, 12, 31)?;
        assert_eq!(dec_31.checked_add(1.days())?, Date::new(2024, 1, 1)?);
        assert_eq!(dec_31.checked_add((-1).days())?, Date::new(2023, 12, 30)?);
        Ok(())
    }

    #[test]
    fn generates_monthly_cashflow_dates() -> Result<(), DateError> {
        let start = Date::new(2023, 1, 31)?;
        let dates = generate_cashflow_dates(start, 4, Frequency::Monthly)?;
        let expected = vec![
            Date::new(2023, 1, 31)?,
            Date::new(2023, 2, 28)?,
            Date::new(2023, 3, 31)?,
            Date::new(2023, 4, 30)?,
        ];
        assert_eq!(dates, expected);
        Ok(())
    }

    #[test]
    fn generates_weekly_cashflow_dates() -> Result<(), DateError> {
        let start = Date::new(2023, 1, 1)?;
        let dates = generate_cashflow_dates(start, 3, Frequency::Weekly)?;
        let expected = vec![
            Date::new(2023, 1, 1)?,
            Date::new(2023, 1, 8)?,
            Date::new(2023, 1, 15)?,
        ];
        assert_eq!(dates, expected);
        Ok(())
    }
}
