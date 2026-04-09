use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Expense {
    id: u32,
    amount: f64,
    description: String,
}

impl Expense {
    pub fn new(id: u32, amount: f64, description: String) -> Expense {
        Expense {
            id,
            amount,
            description,
        }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}
