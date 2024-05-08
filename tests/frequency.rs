use green_shark::frequency::{num_occurences, Frequency, NumberMonths, NumberWeeks, StartDate};

#[test]
/// A frequency is defined given a start_date and an end in n months.
fn for_n_months() {
    let start = StartDate {
        year: 2024,
        month: 1,
        date: 15,
    };
    let end = num_occurences::<NumberMonths>(6);
    let test_frequency = Frequency::monthly_by_date(start, Some(end));

    let expected_start_unix: i64 = 1705276800;
    let expected_end_unix: i64 = 1721001600;
    let expected_monthly = Frequency::MonthlyByDate(expected_start_unix, Some(expected_end_unix));

    assert_eq!(expected_monthly, test_frequency);
}

#[test]
/// A frequency is defined given a start_date and an end in n weeks.
fn for_n_weeks() {
    let start = StartDate {
        year: 2024,
        month: 1,
        date: 15,
    };
    let end = num_occurences::<NumberWeeks>(8);
    let test_frequency = Frequency::weekly(start, Some(end));

    let expected_start_unix: i64 = 1705276800;
    let expected_end_unix: i64 = 1710115200;
    let expected_weekly = Frequency::Weekly(expected_start_unix, Some(expected_end_unix));

    assert_eq!(expected_weekly, test_frequency);
}
