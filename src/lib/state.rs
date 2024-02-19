use zbus::dbus_interface;
use crate::transaction::{Transaction, Currency, Frequency};

#[derive(Debug)]
pub struct State {
    pub transactions: Vec<Transaction>,
}

impl State {
    pub fn new() -> State {
        State {
            transactions: vec![],
        }
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
        self.transactions.push(transaction);

        format!("Transactions: {:?}", self)
    }

    #[dbus_interface(property)]
    async fn transactions(&self) -> String {
        format!("{:?}", self.transactions)
    }
}
