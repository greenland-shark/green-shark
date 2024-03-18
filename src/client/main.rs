use clap::{Args, Parser, Subcommand, ValueEnum};

fn main() {
    let args = Cli::parse();
    println!("Hello, world!");
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    subject: Subject,
}

#[derive(Subcommand, Clone, Debug)]
enum Subject {
    // Doc comment
    Income(Action),
    // Doc comment
    Outcome(Action),
}

#[derive(Clone, Debug, Args)]
struct Action {
    #[command(subcommand)]
    action: Action_,
}

#[derive(Clone, Debug, Subcommand)]
enum Action_ {
    Add(Flags),
    Get(Flags),
}

#[derive(Clone, Debug, Args)]
struct Flags {
    #[arg[long, value_name = "AMOUNT"]]
    amount: i32,
    #[arg[long, value_name = "NAME"]]
    name: String,
    #[arg[long, value_name = "LABEL"]]
    label: String,
    #[arg[long, value_name = "START DATE"]]
    start_date: Option<String>,
    #[arg[long, value_name = "END DATE"]]
    end_date: Option<String>,
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
