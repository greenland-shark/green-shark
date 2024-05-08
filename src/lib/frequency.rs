use chrono::{Days, Months, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use either::Either;
use serde::{Deserialize, Serialize};

type UnixT = i64;
pub type NumberMonths = u32;
pub type NumberWeeks = u64;

/// Useful in CLI where providing and end after n...
pub fn num_occurences<T>(num: T) -> EndOf<T> {
    Either::Right(num)
}

/// Corresponds with args to NaiveDate from_ymd
/// When not in a struct(or in variables), the values could easily be mixed up
pub struct StartDate {
    pub year: i32,
    pub month: NumberMonths,
    pub date: u32,
}

pub struct EndDate {
    year: i32,
    month: NumberMonths,
    date: u32,
}

type EndOf<T> = Either<EndDate, T>;

impl EndDate {
    pub fn new<T>(year: i32, month: NumberMonths, date: u32) -> EndOf<T> {
        Either::Left(Self { year, month, date })
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
/// TODO Yearly(u8, u8),
pub enum Frequency {
    OneOff(i64),

    /// Monthly transaction start and possible finish
    MonthlyByDate(UnixT, Option<UnixT>),

    /// Weekly transaction start and possible finish
    Weekly(UnixT, Option<UnixT>),
    // Yearly(u8, u8),
}

impl Frequency {
    pub fn one_off() -> Self {
        let now = Utc::now();
        let now = now.timestamp();
        Self::OneOff(now)
    }

    /// Define a transaction to be on a fixed date each month (Default start is today).
    /// **TODO** Where the date doesn't exist in the month, CLI should retreive the closest earlier date.
    /// **TODO** Handle/remove unwraps
    /// **TODO** Think the CLI should have some logic along the lines - if value from previous
    /// view == Monthly then check if a repeat occurence is needed in current view
    pub fn monthly_by_date(start: StartDate, end: Option<EndOf<NumberMonths>>) -> Self {
        let dt = NaiveDate::from_ymd_opt(start.year, start.month, start.date).unwrap();
        let t = NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap();
        let start_date = NaiveDateTime::new(dt, t).and_utc().timestamp();

        let end_date = end.map(|end| match end {
            Either::Left(e) => {
                let the_end = NaiveDate::from_ymd_opt(e.year, e.month, e.date).unwrap();
                NaiveDateTime::new(the_end, t).and_utc().timestamp()
            }
            Either::Right(occurences) => {
                let months = Months::new(occurences);
                let from_start = dt.checked_add_months(months).unwrap();
                NaiveDateTime::new(from_start, t).and_utc().timestamp()
            }
        });

        Self::MonthlyByDate(start_date, end_date)
    }

    /// Define a transaction to be on a fixed day each week (Default start is today).
    /// TODO Find out which day is current
    pub fn weekly(start: StartDate, end: Option<EndOf<NumberWeeks>>) -> Self {
        let dt = NaiveDate::from_ymd_opt(start.year, start.month, start.date).unwrap();
        let t = NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap();
        let start_date = NaiveDateTime::new(dt, t).and_utc().timestamp();

        let end_date = end.map(|end| match end {
            Either::Left(e) => {
                let the_end = NaiveDate::from_ymd_opt(e.year, e.month, e.date).unwrap();
                NaiveDateTime::new(the_end, t).and_utc().timestamp()
            }
            Either::Right(number_weeks) => {
                let weeks = Days::new(number_weeks * 7);
                let from_start = dt.checked_add_days(weeks).unwrap();
                NaiveDateTime::new(from_start, t).and_utc().timestamp()
            }
        });

        Self::Weekly(start_date, end_date)
    }
}
