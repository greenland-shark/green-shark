use std::fs::File;
use std::path::Path;

use crate::error::Error;
use crate::frequency::Frequency;
use crate::transaction::{Amount, Transaction};
use serde::{Deserialize, Serialize};
use serde_json;
use zbus::dbus_interface;

static STATE_FILE_NAME: &str = "state.json";

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

    // NOTE: Potentially we need a routine to write to file.
    // Let's say the user writes a script to convert all their monzo transactions into green-shark transactions
    // if they have a 5k data entries, we don't want to write to file on every addition to app.
    // We need a routine that checks if the state is dirty and needs to be written to file,
    // if it does we need to do it periodically every x milliseconds until the state isn't dirty and flag it as clean
    // potentially this could be done in a seperate thread, which we would have to rework some things in regards to the app
    // so that our data can be shared and mutated accross different threads
    /// Given dir to where configuration directory should be,
    /// offload State as a JSON object to that directory under a file named state.json
    pub fn to_file(&self, config_dir: &str) -> Result<(), Error> {
        let json_string = serde_json::to_string(&self).map_err(Error::SerdeJsonError)?;
        let mut file_path = config_dir.to_string();
        file_path.push_str("/");
        file_path.push_str(STATE_FILE_NAME);


        let file_path = Path::new(&file_path);
        let config_path = Path::new(config_dir);

        if !config_path.exists() {
            std::fs::create_dir_all(config_path).map_err(Error::DirCreateError)?;
        }

        if !file_path.exists() {
            File::create(file_path).map_err(Error::FileCreateError)?;
        }

        std::fs::write(file_path, json_string).map_err(Error::FileWriteError)
    }

    /// Where a state file does not exist, `from_file` creates a file and an empty state.  
    pub fn from_file(dir: &str) -> Result<Self, Error> {
        let mut config_dir = dir.to_string();
        config_dir.push_str("/");
        config_dir.push_str(STATE_FILE_NAME);
        let file_string = std::fs::read_to_string(&config_dir).map_err(Error::FileReadError);
        match file_string {
            Ok(s) => {
                // read state from file
                serde_json::from_str(&*s).map_err(Error::SerdeJsonError)
            }
            Err(_) => {
                let new_state = State::new();
                State::to_file(&new_state, &config_dir)?;
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
        let sterling_amount = Amount::GBP(amount);
        let frequency = Frequency::one_off_now();
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
