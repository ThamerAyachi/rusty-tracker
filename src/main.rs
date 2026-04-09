use std::env;
use std::io::Write;
use rand;
use serde::Serialize;

#[derive(Serialize)]
struct Expense {
    id: u32,
    amount: f64,
    description: String,
}

enum ACTION {
    ADD,
    LIST
}

impl ACTION {
    fn from_str(value: &str) -> Option<Self> {
        match value {
            "add" => Some(ACTION::ADD),
            "list" => Some(ACTION::LIST),
            _ => None
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Please provide an action (add|list)");
    }

    let action = &args[1];

    match ACTION::from_str(action) {
        Some(ACTION::ADD) => add_expense(args),
        Some(ACTION::LIST) => println!("Listing to {}", args[1]),
        None => println!("Please provide an action (add|list)")
    }
}

fn add_expense(args: Vec<String>) {
    if args.len() != 4 || !args[2].parse::<f64>().is_ok() {
        println!("Please provide an correct action like (add 50.50 \"Groceries\")");
        return;
    }

    let expense = Expense {
        id: rand::random::<u32>(),
        amount: args[2].parse().unwrap(),
        description: args[3].clone()
    };

    let json = serde_json::to_string_pretty(&expense).unwrap();
    
    let mut file = std::fs::File::create("expense.json").unwrap();
    
    file.write_all(json.as_bytes()).unwrap();
    
    println!("{}", json);
}
