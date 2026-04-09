mod expense;
pub mod file;

use crate::expense::Expense;
use std::env;

enum ACTION {
    ADD,
    LIST,
}

impl ACTION {
    fn from_str(value: &str) -> Option<Self> {
        match value {
            "add" => Some(ACTION::ADD),
            "list" => Some(ACTION::LIST),
            _ => None,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 && args.len() != 4 {
        println!("Please provide an action (add|list)");
        return;
    }

    let action = &args[1];

    match ACTION::from_str(action) {
        Some(ACTION::ADD) => add_expense(args),
        Some(ACTION::LIST) => list_expenses(),
        None => println!("Please provide an action (add|list)"),
    }
}

fn add_expense(args: Vec<String>) {
    if args.len() != 4 || !args[2].parse::<f64>().is_ok() {
        println!("Please provide an correct action like (add 50.50 \"Groceries\")");
        return;
    }

    let _ = Expense::new(args[2].parse().unwrap(), args[3].clone());
}

fn list_expenses() {
    for expense in Expense::list() {
        println!("{}", expense.to_string());
    }
}
