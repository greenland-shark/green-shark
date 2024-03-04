use chrono::Utc;
use rand::Rng;

#[derive(Debug)]
pub enum Currency {
    NZD,
    GBP,
    BRL,
}

#[derive(Debug)]
pub enum Frequency {
    OneOff(i64),
    // Monthly(u8),
    // Weekly(u8),
    // Yearly((u8, u8)),
}

impl Frequency {
    pub fn one_off() -> Self {
        let now = Utc::now();
        let now = now.timestamp();
        Self::OneOff(now)
    }
}

type Amount = (Currency, f32);

#[derive(Debug)]
pub struct Transaction {
    pub amount: Amount,
    pub name: String,
    pub label: Option<String>,
    id: i64,
    pub frequency: Frequency,
    pub start_date: i64,
    pub end_date: Option<i64>,
}

impl Transaction {
    pub fn new(
        amount: Amount,
        name: String,
        label: Option<String>,
        frequency: Frequency,
        end_date: Option<i64>,
    ) -> Self {
        let now = Utc::now();
        let mut rng = rand::thread_rng();
        let n: i64 = rng.gen_range(0..100);
        let uid = now.timestamp() + n;
        let now = now.timestamp();
        Self {
            amount,
            name,
            label,
            id: uid,
            frequency,
            start_date: now,
            end_date,
        }
    }

    pub fn id(&self) -> i64 {
        self.id
    }
    pub fn start_date(&self) -> i64 {
        self.start_date
    }
}

#[cfg(test)]
mod test {
    use crate::transaction::{Currency, Frequency, Transaction};

    #[test]
    fn uids_are_unique() {
        let transaction_1 = Transaction::new(
            (Currency::GBP, 10.5),
            "out1".to_string(),
            Some("drink".to_string()),
            Frequency::one_off(),
            None,
        );
        let transaction_2 = Transaction::new(
            (Currency::GBP, 10.5),
            "out1".to_string(),
            Some("drink".to_string()),
            Frequency::one_off(),
            None,
        );

        assert_ne!(
            transaction_1.id(), transaction_1.start_date(),
            "UID shouldn't match start_date because start_date can be the same for multiple transactions."
        );
        assert_ne!(
            transaction_1.id(),
            transaction_2.id(),
            "UIDs shouldn't match where transactions may be created at (or near) the same time."
        );
    }
}
