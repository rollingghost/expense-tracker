use crate::expense_cli;

use chrono::{DateTime, Datelike, NaiveDateTime, Utc};
use comfy_table::Table;
use expense_cli::structs_enums::{Category, Expense};
use rand::Rng;
use std::fs::OpenOptions;
use std::io::{self, stdin, stdout, Write};
use std::time::SystemTime;

/// Maps a category string to a `Category` enum variant.
///
/// # Arguments
///
/// * `category` - The category string to map.
///
/// # Returns
///
/// The corresponding `Category` enum variant.
pub fn map_category(category: &str) -> Category {
    match category {
        "food" => Category::Food,
        "transportation" => Category::Transportation,
        "entertainment" => Category::Entertainment,
        "internet" => Category::Internet,
        _ => Category::Other,
    }
}

/// Converts a `SystemTime` to a `String` using the chrono library.
///
/// # Arguments
///
/// * `system_time` - The `SystemTime` to convert.
///
/// # Returns
///
/// The converted `String`.
pub fn convert_from_system_time(system_time: SystemTime) -> String {
    let datetime: DateTime<Utc> = system_time.into();
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

/// Saves the expenses to a JSON file.
///
/// # Arguments
///
/// * `expenses` - The array of expenses to save.
///
/// # Returns
///
/// An `Ok` result if the expenses are successfully saved, or an `Err` containing a `Box<dyn std::error::Error>` otherwise.
pub fn save_expenses(expenses: &[Expense]) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "expenses.json";
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(file_path)?;

    let mut writer = io::BufWriter::new(file);
    serde_json::to_writer(&mut writer, expenses)?;
    writer.write_all(b"\n")?;

    writer.flush()?;

    Ok(())
}

/// Exports the expenses to a CSV file.
///
/// # Arguments
///
/// * `file` - The path of the CSV file to export to.
/// * `all_expenses` - The array of all expenses to export.
///
/// # Returns
///
/// An `Ok` result if the expenses are successfully exported, or an `Err` containing an `io::Error` otherwise.
pub fn export_expenses(file: &str, all_expenses: &Vec<Expense>) -> Result<(), io::Error> {
    let file_path = file;
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(file_path)?;

    let mut _writer = io::BufWriter::new(file);

    // Write the header
    writeln!(
        _writer,
        "ID,Description,Amount,Category,Added At,Updated At"
    )?;

    for expense in all_expenses {
        writeln!(
            _writer,
            "{},{},{},{:?},{},{}",
            expense.id,
            expense.description,
            expense.amount,
            expense.category,
            expense.added_at,
            expense.updated_at
        )?;
    }

    _writer.flush()?;

    Ok(())
}

/// Loads all expenses from the JSON file.
///
/// # Returns
///
/// An `Ok` result containing a vector of expenses if the expenses are successfully loaded, or an `Err` containing a `Box<dyn std::error::Error>` otherwise.
pub fn load_expenses() -> Result<Vec<Expense>, Box<dyn std::error::Error>> {
    let expense_file = "expenses.json";

    // Load existing expense from expenses.json file or use empty array if file does
    // not exist
    let expenses = std::fs::read_to_string(expense_file).unwrap_or_else(|_| "[]".to_string());

    let all_expenses: Vec<Expense> = serde_json::from_str(&expenses)?;

    Ok(all_expenses)
}

/// Searches for an expense by its ID in the array of expenses.
///
/// # Arguments
///
/// * `expenses` - The array of expenses to search in.
/// * `id` - The ID of the expense to search for.
///
/// # Returns
///
/// An `Option` containing the index of the expense if found, or `None` if not found.
pub fn search_expense_by_id(expenses: &[Expense], id: &str) -> Option<usize> {
    expenses.iter().position(|expense| expense.id == id)
}

/// Displays the expenses in a table
///
/// # Arguments
///
/// * `expenses` - The array of expenses to display
///
/// # Returns
///
/// A table of the expenses
pub fn prettify_expense_display(expenses: &[Expense]) {
    let mut table = Table::new();
    let mut expense_number = 1;
    table.set_header(vec![
        "No",
        "ID",
        "Description",
        "Amount",
        "Category",
        "Last Updated",
    ]);
    if !expenses.is_empty() {
        for expense in expenses {
            table.add_row(vec![
                expense_number.to_string(),
                expense.id.clone(),
                expense.description.clone(),
                expense.amount.to_string(),
                format!("{:?}", expense.category),
                expense.updated_at.clone(),
            ]);
            expense_number += 1;
        }
    } else {
        table.add_row(vec!["No expenses found"; 5]);
    }

    println!("{}", table);
}

/// Generates a random ID for an expense.
///
/// # Returns
///
/// A random ID.
pub fn generate_random_id() -> u64 {
    let mut range = rand::thread_rng();
    range.gen_range(1000..9999)
}

/// Deletes an expense from the array of all expenses.
///
/// # Arguments
///
/// * `index` - The index of the expense to delete.
/// * `all_expenses` - The array of all expenses.
///
/// # Returns
///
/// The deleted expense.
pub fn delete_expense(index: usize, all_expenses: &mut Vec<Expense>) -> Expense {
    all_expenses.remove(index)
}

/// Displays a message when an expense is not found.
///
/// # Returns
///
/// A message indicating that no expenses were found.
pub fn prettify_expense_not_found() {
    let mut table = Table::new();
    let no_expense_found = "No expenses found".to_string();

    table.set_header(vec![
        "No",
        "ID",
        "Description",
        "Amount",
        "Category",
        "Last Updated",
    ]);

    table.add_row(vec![
        1.to_string(),
        no_expense_found.clone(),
        no_expense_found.clone(),
        no_expense_found.clone(),
        no_expense_found.clone(),
        no_expense_found.clone(),
    ]);

    println!("{}", table);
}

/// Get the month from a date string
///
/// # Arguments
///
/// * `datetime` - The date string to extract the month from
///
/// # Returns
///
/// The month as a u32
pub fn get_month_from_date_string(datetime: &str) -> u32 {
    let datetime = NaiveDateTime::parse_from_str(datetime, "%Y-%m-%d %H:%M:%S")
        .expect("Unable to parse the date");
    datetime.month()
}

/// Set the monthly budget
///
/// # Arguments
///
/// * `budget` - The budget to set
///
/// # Returns
///
/// The budget as a f64
pub fn set_budget(budget: f64) {
    let budget_file = "budget.json";
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(budget_file)
        .expect("Unable to open the file");

    let mut writer = io::BufWriter::new(file);
    serde_json::to_writer(&mut writer, &budget).expect("Unable to write to the file");

    writer.flush().expect("Unable to flush the writer");
}

/// Get the monthly budget
///
/// # Returns
///
/// The budget as a f64
pub fn get_budget() -> f64 {
    let budget_file = "budget.json";
    let budget = std::fs::read_to_string(budget_file).unwrap_or_else(|_| "0.00".to_string());

    let budget: f64 = serde_json::from_str(&budget).expect("Unable to parse the budget");

    budget
}

/// Clear all expense
///
/// # Arguments
/// No arguments
///
/// # Returns
///
/// Result `Ok` if the expenses were successfully cleared and `Err` std::error::Error on error
pub fn clear_all_expenses() -> Result<(), Box<dyn std::error::Error>> {
    let mut confirm_clear = "n".to_string();
    print!("\n >> << Are you sure you want to clear all expenses? [y][N] << >> ");
    stdout()
        .flush()
        .expect("Hard to display something here. Try agin.");
    stdin()
        .read_line(&mut confirm_clear)
        .expect("Could not read you terminal. Try again");

    match confirm_clear.as_str() {
        "y" | "Y" => {
            let blank_expenses = [];
            save_expenses(&blank_expenses)?;
        }

        "n" | "N" => println!("User quit!"),

        _ => {
            println!("\n\tMaybe that is not the command you wanted to run: Try [y] or [N]\n");
        }
    }
    Ok(())
}
