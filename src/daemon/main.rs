use green_shark::{
    state::State,
    transaction::{Amount, Transaction}, frequency::Frequency,
};
use serde_json::error::Category;
use std::{env, error::Error, ffi::OsString, fs::File, process};
use zbus::ConnectionBuilder;
use chrono::DateTime;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = check_if_for_env_config_exists_if_not_create_it();

    let mut state = State::new();

    let file_path = "/home/milton/code/green-shark/transaction_data.csv";
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.records() {
        let record = result?;
        // position 1 is date of transaction
        // position 2 is time of transactionnd
        // position 4 name of transaction
        // position 6 category of transaction
        // position 9 is local amount
        // position 14 is Description
        let date = record.get(1).unwrap().to_string();
        let time = record.get(2).unwrap().to_string();
        let name = record.get(4); //TODO: check if it's none and adjust
        let label = record.get(6).map(|v| v.to_string());
        let amount = record.get(9).unwrap();
        let description = record.get(14);

        let amount_f: f32 = amount.parse::<f32>().unwrap();

        let amount = Amount::GBP(amount_f);

        let name = match name {
            None => "default",
            Some(n) => n,
        };
        let mut date_time = date;
        date_time.push_str(" ");
        date_time.push_str(&time);
        date_time.push_str(" +0000");


        let date_time_of_transaction = DateTime::parse_from_str(&date_time, "%d/%m/%Y %H:%M:%S %z")?;
        let frequency = Frequency::OneOff(date_time_of_transaction.timestamp());

        let transaction = Transaction::new(amount, name.to_string(), label, frequency, None);

        if amount_f.is_sign_negative() {
            state.outgoings.push(transaction);
        } else {
            state.incomes.push(transaction);
        }
    }

    println!("{:#?}", state);

    let _connection = ConnectionBuilder::session()?
        .name("org.green_sharkd.GreenSharkd")?
        .serve_at("/org/green_sharkd/State", state)?
        .build()
        .await?;

    loop {
        futures::future::pending::<()>().await;
    }
}

fn check_if_for_env_config_exists_if_not_create_it() -> String {
    if let Ok(path) = env::var("GREEN_SHARK_CONFIG") {
        path
    } else {
        let home = env::var("HOME").unwrap();
        home + "/.config/green_shark/"
    }
}
