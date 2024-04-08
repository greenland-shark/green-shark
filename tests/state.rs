use green_shark::{
    state::State,
    transaction::{Amount, Frequency, Transaction},
};

#[test]
fn initial_state_is_empty() {
    let initial_state = State::new();
    let incomes_are_empty = initial_state.incomes.len() == 0;
    let outgoings_are_empty = initial_state.outgoings.len() == 0;

    assert!(
        incomes_are_empty && outgoings_are_empty,
        "Initial state should contain only empty lists."
    )
}

#[test]
fn write_initial_state_to_file() {
    let filepath = "./tests/data/write_initial_state.json";
    let res = State::from_file(filepath);
    match res {
        Ok(s) => {
            let incomes_are_empty = s.incomes.len() == 0;
            let outgoings_are_empty = s.outgoings.len() == 0;

            assert!(
                incomes_are_empty && outgoings_are_empty,
                "Where no args provided, an initial state should be written to file."
            )
        }
        Err(_) => {
            assert!(false, "Fail.");
        }
    }
}

#[test]
fn write_transaction_to_file() {
    let filepath = "./tests/data/write_single_transaction.json";
    let my_salary = Transaction::new(
        Amount::GBP(100.40),
        "company1".to_string(),
        Some("Salary".to_string()),
        Frequency::one_off_now(),
        None,
    );

    let mut state = State::new();
    state.add_income(my_salary.to_owned());
    let written = state.to_file(filepath);
    assert!(written.is_ok(), "Can write a transaction to file");
}

#[test]
fn read_transaction_from_file() {
    let filepath = "./tests/data/read_single_transaction.json";
    let read_state = State::from_file(filepath);
    match read_state {
        Ok(s) => {
            let expected_name = "company1";
            let read_state_incomes = s.incomes.into_iter().find(|t| t.name == expected_name);

            assert!(
                read_state_incomes.is_some(),
                "Can read a transaction from file."
            )
        }
        Err(_) => {
            assert!(false, "Fail");
        }
    }
}

// Test- Where the value is negative, a value should be added to outgoings.
#[test]
fn TODO_write_negative_transaction_to_outgoings() {}

// Test- Where the value is positive, a value should be added to incomes.
#[test]
fn TODO_write_positive_transaction_to_incomes() {}
