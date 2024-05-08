use chrono::Weekday;
use green_shark::frequency::{from_date, Date, Frequency};

#[test]
/// A frequency is defined given a start_date and an end in n weeks.
fn for_n_weeks() {
    let start = Date {
        year: 2024,
        month: 1,
        date: 15,
    };

    let n_weeks = 8;
    let calcd = Frequency::weekly(start, Some(n_weeks)).unwrap();
    let expected = {
        let expected_start = from_date(start).unwrap();
        let expected_end = from_date(Date {
            year: 2024,
            month: 3,
            date: 11,
        })
        .unwrap();
        Frequency::Weekly(expected_start, Some(expected_end), Weekday::Mon)
    };

    assert_eq!(calcd, expected)
}

#[test]
/// A frequency is defined given a start_date and an end in n months.
fn for_n_months() {
    let start = Date {
        year: 2024,
        month: 1,
        date: 15,
    };
    let n_months = 6;
    let calcd = Frequency::monthly_by_date(start, Some(n_months)).unwrap();

    let expected = {
        let expected_start = from_date(start).unwrap();
        let expected_end = from_date(Date {
            year: 2024,
            month: 7,
            date: 15,
        })
        .unwrap();

        Frequency::MonthlyByDate(expected_start, Some(expected_end))
    };

    assert_eq!(calcd, expected)
}
