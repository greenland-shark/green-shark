use {
    clap::Parser,
    green_shark::{cli::CliArgs, transaction::Transaction},
};

fn main() {
    let args = CliArgs::parse();

    match Transaction::try_from(args) {
        Ok(transaction) => println!("{:?}", transaction),
        Err(_) => (),
    };
}

// shark [SUBJECT] [Optional [ACTION]] (flags)
//
// SUBJECT
// income
// outcome
//
// ACTION
// create (flags)
//
// flags (short form to be figured out later)
// --amount (required)
// --name (required)
// --label (required)
// --start_date (optional)
// --end_date (optional)
// --frequency (optional)
//
//
// ACTION
// get (optional val {id})
// flags (short form to be figured out later)
// --name (optional) (val str)
// --label (optional) (val str)
// --number_of_transaction (optional) (val int)
// TO BE DECIDED SORT BY
// --id (val int)
// --date yyyy/mm/dd
// --year yyyy
// --month mm (only when year is specified)
//
// TUI
// ------------------------
// |   add                |
// |                      |
// |   edit               |
// |                      |
// |   view               |
// |                      |
// |   projection         |
// ------------------------
