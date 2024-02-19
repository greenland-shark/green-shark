use chrono::Utc;

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
        let uid = now.timestamp();
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
}
