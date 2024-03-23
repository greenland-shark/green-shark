// move cli stuff to lib
use {
    clap::Parser,
    green_shark::cli::{self, CliArgs},
};

fn main() {
    let args = CliArgs::parse();

    if let Some(transaction) = cli::extact_transaction_from_args(args) {
        println!("{:?}", transaction);
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
