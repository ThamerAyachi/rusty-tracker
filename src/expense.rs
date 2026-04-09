use crate::file::{add_expense, get_expenses};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Expense {
    id: u32,
    amount: f64,
    description: String,
}

impl Expense {
    pub fn new(amount: f64, description: String) -> Expense {
        let expense = Expense {
            id: rand::random::<u32>(),
            amount,
            description,
        };
        add_expense(expense.clone()).expect("Inserting Expense failed");
        expense
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }

    pub fn list() -> Vec<Expense> {
        get_expenses()
    }
}
