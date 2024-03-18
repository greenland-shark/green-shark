use clap::{Args, Parser, Subcommand};
use termimad::ansi;

fn main() {
    let args = CliArgs::parse();
    println!("Hello, world!");
}

#[derive(Parser, Debug)]
#[command(version, author, about, disable_help_flag = true)]
pub struct CliArgs {
    /// Print help information
    #[arg(long, short)]
    pub help: bool,
    #[command(subcommand)]
    /// what subject you want to perform an aciton on
    pub subject: Subject,
}

impl CliArgs {
    pub fn print_help(&self) {
        let mut printer = clap_help::Printer::new(Args::command())
            .with("introduction", "hahah")
            .with("options", clap_help::TEMPLATE_OPTIONS_MERGED_VALUE)
            .without("author");
        let skin = printer.skin_mut();
        skin.headers[0].compound_style.set_fg(ansi(204));
        skin.bold.set_fg(ansi(204));
        skin.italic = termimad::CompoundStyle::with_fg(ansi(204));

        printer.template_keys_mut().push("examples");
        printer.set_template("examples", EXAMPLES_TEMPLATE);
    }
}

#[derive(Subcommand, Clone, Debug)]
pub enum Subject {
    /// Perform an action on incomes
    Income(Action),
    /// Perform an action on outgoings
    Outcome(Action),
}

#[derive(Clone, Debug, Args)]
pub struct Action {
    #[command(subcommand)]
    pub action: ActionType,
}

#[derive(Clone, Debug, Subcommand)]
pub enum ActionType {
    /// Create a transaction
    Create(TransactionValues),
    /// Retrive one or more transaction
    Get(Get),
}

#[derive(Clone, Debug, Args)]
pub struct Get {
    /// Optional id to retrive from
    pub id: Option<i64>,
    #[command(flatten)]
    /// Flags to filter retrivable transactions
    pub filter: FilterFlags,
}

#[derive(Clone, Debug, Args)]
pub struct FilterFlags {
    /// Optional number of transactions to retrive, if not set all will be returned
    #[arg[long, value_name = "NUMBER OF TRANSACTIONS"]]
    pub number_of_transactions: Option<i32>,
    /// Filter by name
    #[arg[long, value_name = "NAME"]]
    pub name: Option<String>,
    /// Filter by label
    #[arg[long, value_name = "LABEL"]]
    pub label: Option<String>,
    /// Filter by start date
    #[arg[long, value_name = "START DATE"]]
    pub start_date: Option<String>,
    /// Filter by end date
    #[arg[long, value_name = "END DATE"]]
    pub end_date: Option<String>,
}

#[derive(Clone, Debug, Args)]
pub struct TransactionValues {
    /// The amount that the transaction is
    #[arg[long, value_name = "AMOUNT"]]
    pub amount: i32,
    /// Name of the transaction, e.g "Coffee"
    #[arg[long, value_name = "NAME"]]
    pub name: String,
    /// The label to categorise the transaction, e.g "Leisure"
    #[arg[long, value_name = "LABEL"]]
    pub label: String,
    /// The start date of the transaction, this will default to current date time
    #[arg[long, value_name = "START DATE"]]
    pub start_date: Option<String>,
    /// The end date of transaction, this will default to null for none recurring transactions
    #[arg[long, value_name = "END DATE"]]
    pub end_date: Option<String>,
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
/// A bacon launch example to display in the --help message
pub struct Example {
    pub title: &'static str,
    pub cmd: &'static str,
}

pub static EXAMPLES_TEMPLATE: &str = "
**Examples:**

${examples
*${example-number})* ${example-title}: `${example-cmd}`
}
";

pub static EXAMPLES: &[Example] = &[
    Example {
        title: "Start with the default job",
        cmd: "bacon",
    },
    Example {
        title: "Start with a specific job",
        cmd: "bacon clippy",
    },
    Example {
        title: "Start with features",
        cmd: "bacon --features clipboard",
    },
    Example {
        title: "Start a specific job on another path",
        cmd: "bacon ../broot test",
    },
    Example {
        title: "Start in summary mode",
        cmd: "bacon -s",
    },
];
