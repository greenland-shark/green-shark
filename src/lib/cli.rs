use {
    crate::transaction::Transaction,
    crate::transaction::{Currency, Frequency},
    chrono::{NaiveDateTime, Utc},
    clap::{Args, Parser, Subcommand},
};

fn extract_transaction_from_action(action: Action) -> Option<Transaction> {
    if let ActionType::Create(transaction_values) = action.action {
        let amount = (Currency::GBP, transaction_values.amount);
        let name = transaction_values.name;
        let label = transaction_values.label;
        let frequency = transaction_values
            .start_date
            .map(|mut sd| {
                sd.push_str(" 00:00:00");
                NaiveDateTime::parse_from_str(&sd, "%yyyy/mm/dd")
                    .map(|op| Frequency::OneOff(op.timestamp()))
                    .ok()
            })
            .unwrap_or(None)
            .unwrap_or(Frequency::OneOff(Utc::now().timestamp()));
        Some(Transaction::new(amount, name, label, frequency, None))
    } else {
        None
    }
}

pub fn extact_transaction_from_args(args: CliArgs) -> Option<Transaction> {
    match args.subject {
        Subject::Income(action) => extract_transaction_from_action(action),
        Subject::Outcome(action) => extract_transaction_from_action(action),
    }
}

#[derive(Parser, Debug)]
#[command(version, author, about)]
pub struct CliArgs {
    #[command(subcommand)]
    /// what subject you want to perform an aciton on
    pub subject: Subject,
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
    pub amount: f32,
    /// Name of the transaction, e.g "Coffee"
    #[arg[long, value_name = "NAME"]]
    pub name: String,
    /// The label to categorise the transaction, e.g "Leisure"
    #[arg[long, value_name = "LABEL"]]
    pub label: Option<String>,
    /// The start date of the transaction in the format of yyyy/mm/dd, this will default to current date time
    #[arg[long, value_name = "START DATE"]]
    pub start_date: Option<String>,
    /// The end date of transaction, this will default to null for none recurring transactions
    #[arg[long, value_name = "END DATE"]]
    pub end_date: Option<String>,
}
