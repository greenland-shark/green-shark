use green_shark::transaction::{Amount, Frequency, Transaction};

#[test]
fn uids_are_unique() {
    let transaction_1 = Transaction::new(
        Amount::GBP(10.5),
        "out1".to_string(),
        Some("drink".to_string()),
        Frequency::one_off_now(),
        None,
    );
    let transaction_2 = Transaction::new(
        Amount::GBP(10.5),
        "out1".to_string(),
        Some("drink".to_string()),
        Frequency::one_off_now(),
        None,
    );

    assert_ne!(
        transaction_1.id(), transaction_1.start_date,
        "UID shouldn't match start_date because start_date can be the same for multiple transactions."
        );
    assert_ne!(
        transaction_1.id(),
        transaction_2.id(),
        "UIDs shouldn't match where transactions may be created at (or near) the same time."
    );
}
