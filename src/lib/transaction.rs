use crate::frequency::Frequency;
use chrono::Utc;

use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Currency {
    NZD,
    GBP,
    BRL,
}

type Amount = (Currency, f32);

#[derive(Clone, Debug, Serialize, Deserialize)]
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

    pub fn amount_value(&self) -> f32 {
        self.amount.1
    }
}
