use crate::expense::Expense;
use std::fs;
use std::fs::File;
use std::io::Write;

const FILE_PATH: &'static str = "expense.json";

fn file() -> File {
    fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(FILE_PATH)
        .unwrap()
}

fn read_file() -> Result<String, std::io::Error> {
    if fs::metadata(FILE_PATH)?.len() == 0 {
        file().write_all("[]\n".as_bytes())?;
    }
    std::fs::read_to_string(&FILE_PATH)
}

pub fn add_expense(expense: Expense) -> Result<(), std::io::Error> {
    let mut my_expenses: Vec<Expense> = get_expenses();
    my_expenses.push(expense);

    file().write_all(serde_json::to_string(&my_expenses).unwrap().as_bytes())
}

pub fn get_expenses() -> Vec<Expense> {
    if !read_file().is_ok() {
        return Vec::new();
    }
    serde_json::from_str::<Vec<Expense>>(&read_file().unwrap().as_str()).unwrap()
}
