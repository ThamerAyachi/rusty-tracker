# Rust CLI Expense Tracker

A lightweight and fast command-line interface (CLI) application to track and manage your daily expenses, built with Rust.

## Features

* **Add Expenses:** Easily add new expenses with an amount and a descriptive text.
* **List Expenses:** View all your previously recorded expenses.
* **Persistent Storage:** Data is automatically serialized and securely saved locally in an `expense.json` file.
* **Modular Architecture:** Clean codebase separated into domain (`expense.rs`), infrastructure (`file.rs`), and presentation/CLI (`main.rs`) layers.

## Prerequisites

Make sure you have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed on your machine.

## Getting Started

1. Clone the repository:
   ```bash
   git clone https://github.com/ThamerAyachi/rusty-tracker.git
   cd rust-expense-tracker