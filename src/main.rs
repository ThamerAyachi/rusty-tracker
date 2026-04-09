mod expense;
pub mod file;

use crate::expense::Expense;
use crate::file::read_file;
use rand;
use serde_json::to_string;
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
        Some(ACTION::LIST) => println!("Listing to {}", args[1]),
        None => println!("Please provide an action (add|list)"),
    }
}

fn add_expense(args: Vec<String>) {
    if args.len() != 4 || !args[2].parse::<f64>().is_ok() {
        println!("Please provide an correct action like (add 50.50 \"Groceries\")");
        return;
    }

    let expense = Expense::new(
        rand::random::<u32>(),
        args[2].parse().unwrap(),
        args[3].clone(),
    );

    let _ = file::write_file(expense.to_string());

    println!("{}", expense.to_string());
}
