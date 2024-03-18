use crate::{
    error::Error,
    transaction::{Currency, Frequency, Transaction},
};
use serde::{Deserialize, Serialize};
use serde_json;
use zbus::dbus_interface;

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub incomes: Vec<Transaction>,
    pub outgoings: Vec<Transaction>,
}

impl State {
    pub fn new() -> State {
        State {
            incomes: vec![],
            outgoings: vec![],
        }
    }

    pub fn to_file(&self, filepath: &str) -> Result<(), Error> {
        let json_string = serde_json::to_string(&self).map_err(Error::SerdeJsonError)?;
        std::fs::write(filepath, json_string).map_err(Error::FileWriteError)
    }

    /// Where a state file does not exist, `from_file` creates a file and an empty state.  
    pub fn from_file(f: &str) -> Result<Self, Error> {
        let filepath = f;
        let file_string = std::fs::read_to_string(filepath).map_err(Error::FileReadError);
        match file_string {
            Ok(s) => {
                // read state from file
                serde_json::from_str(&*s).map_err(Error::SerdeJsonError)
            }
            Err(_) => {
                let new_state = State::new();
                State::to_file(&new_state, filepath)?;
                Ok(new_state)
            }
        }
    }

    pub fn add_income(&mut self, t: Transaction) -> &mut Self {
        self.incomes.push(t);
        self
    }

    pub fn add_outgoing(&mut self, t: Transaction) -> &mut Self {
        self.outgoings.push(t);
        self
    }
}

#[dbus_interface(name = "org.green_sharkd.Commands")]
impl State {
    async fn add_transaction(&mut self, amount: f32, name: &str, label: &str) -> String {
        println!("adding transaction");

        let label = if label.is_empty() {
            None
        } else {
            Some(label.to_string())
        };

        let sterling_amount = (Currency::GBP, amount);
        let frequency = Frequency::one_off();
        let end_date = None;
        let transaction = Transaction::new(
            sterling_amount,
            name.to_string(),
            label,
            frequency,
            end_date,
        );

        if transaction.amount_value().is_sign_negative() {
            self.outgoings.push(transaction);
        } else {
            self.incomes.push(transaction);
        }

        format!("Transactions: {:?}", self)
    }

    #[dbus_interface(property)]
    /// Returns complete state as a JSON string.
    async fn transactions(&self) -> String {
        serde_json::to_string(self)
            .map_err(Error::SerdeJsonError)
            .unwrap()
    }
}
