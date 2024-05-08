use chrono::{Datelike, Duration, NaiveDate, Utc, Weekday};
use serde::{Deserialize, Serialize};

// TODO Performance testing on the bus

// TODO Add this logic to view
// If Freq start is not after window end and no end to Transaction Freq, then find and show transaction
// Or if Freq start is not after window end and end of Frequency end IS after view end, then find and show transaction

type UnixT = i64;
pub type Year = i32;
pub type Month = u32;

pub type Weeks = i64;
pub type Months = u32;

/// Corresponds with args to NaiveDate from_ymd
/// When not in a struct(or in variables), the values could easily be mixed up
#[derive(Debug, PartialEq, Clone, Copy, Deserialize, Serialize)]
pub struct Date {
    pub year: Year,
    pub month: Month,
    pub date: u32,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
/// TODO Yearly(u8, u8),
pub enum Frequency {
    OneOff(UnixT),

    /// Weekly transaction start and possible end date
    Weekly(NaiveDate, Option<NaiveDate>, Weekday),

    /// Monthly transaction start and possible end date
    MonthlyByDate(NaiveDate, Option<NaiveDate>),
    // Yearly(u8, u8),
}

impl Frequency {
    pub fn one_off_now() -> Self {
        let now = Utc::now();
        let now = now.timestamp();
        Self::OneOff(now)
    }

    /// Define a transaction to be on a fixed day each week.
    pub fn weekly(start: Date, num_weeks: Option<Weeks>) -> Option<Self> {
        let naive_end = num_weeks.and_then(|n| increase_date_n_weeks(start.to_owned(), n));

        from_date(start).map(|naive_start| {
            let weekday = naive_start.weekday();
            Self::Weekly(naive_start, naive_end, weekday)
        })
    }

    /// Define a transaction to be on a fixed date each month.
    /// Where date doesn't exist in month, the closest suitable date should be used.
    pub fn monthly_by_date(start: Date, num_months: Option<Months>) -> Option<Self> {
        let naive_end = num_months.and_then(|n| increase_date_n_months(start.to_owned(), n));
        from_date(start).map(|naive_start| Self::MonthlyByDate(naive_start, naive_end))
    }
}

pub fn from_date(d: Date) -> Option<NaiveDate> {
    NaiveDate::from_ymd_opt(d.year, d.month, d.date)
}

pub fn increase_date_n_weeks(start: Date, num_weeks: Weeks) -> Option<NaiveDate> {
    let n = Duration::weeks(num_weeks);
    from_date(start).map(|date| date + n)
}

pub fn increase_date_n_months(start: Date, num_months: Months) -> Option<NaiveDate> {
    from_date(start).and_then(|date| add_months(date, num_months))
}

fn add_months(date: NaiveDate, num_months: u32) -> Option<NaiveDate> {
    let mut month = date.month() + num_months;
    let year = date.year() + (month / 12) as i32;
    month = month % 12;

    let mut day = date.day();
    get_days_from_month(year, month).and_then(|max_days| {
        day = if day > max_days { max_days } else { day };
        NaiveDate::from_ymd_opt(year, month, day)
    })
}

fn get_days_from_month(year: Year, month: Month) -> Option<u32> {
    let start_next_month = NaiveDate::from_ymd_opt(
        match month {
            12 => year + 1,
            _ => year,
        },
        match month {
            12 => 1,
            _ => month + 1,
        },
        1,
    );
    let start_curr = NaiveDate::from_ymd_opt(year, month, 1);

    match (start_next_month, start_curr) {
        (Some(nxt), Some(cur)) => Some(nxt.signed_duration_since(cur).num_days() as u32),
        _ => None,
    }
}
