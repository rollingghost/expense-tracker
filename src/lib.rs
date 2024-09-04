use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Expense {
    pub id: String,
    pub description: String,
    pub amount: f64,
    pub category: Category,
    pub added_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Category {
    Food,
    Transportation,
    Entertainment,
    Other,
}

impl Expense {
    // Create new expense
    pub fn new(description: String, amount: f64, category: Category) -> Self {
        Self {
            id: "id".to_string(),
            description,
            amount,
            category,
            added_at: SystemTime::now(),
            updated_at: SystemTime::now(),
        }
    }

    // Update an expense
    pub fn update(
        update_ready_expense: Expense,
        all_expenses: &mut [Expense],
        index: usize,
    ) -> Self {
        all_expenses[index] = update_ready_expense.clone();
        save_expenses(all_expenses).unwrap();

        update_ready_expense
    }
}

pub fn map_category(category: &str) -> Category {
    match category {
        "food" => Category::Food,
        "transportation" => Category::Transportation,
        "entertainment" => Category::Entertainment,
        _ => Category::Other,
    }
}

// Convert datetime from system datetime to chrono
pub fn convert_from_system_time(_datetime: SystemTime) -> DateTime<Utc> {
    todo!()
}

// Write changes to a json file
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

// Export expenses to a CSV file
pub fn export_expenses(file: &str) -> Result<(), io::Error> {
    let file_path = file;
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(file_path)?;

    let mut _writer = io::BufWriter::new(file);

    todo!()
}

// Load all expense into a vec
pub fn load_expenses() -> Result<Vec<Expense>, Box<dyn std::error::Error>> {
    let expense_file = "expenses.json";

    // Load existing expense from expenses.json file or use empty array if file does
    // not exist
    let expenses = std::fs::read_to_string(expense_file).unwrap_or_else(|_| "[]".to_string());

    let all_expenses: Vec<Expense> = serde_json::from_str(&expenses)?;

    Ok(all_expenses)
}

// Search for a task in the array
pub fn search_expense_by_id(expenses: &[Expense], id: &str) -> Option<usize> {
    let mut index: usize = 0;

    for expense in expenses {
        if expense.id == id {
            return Some(index);
        }
        index += 1;
    }

    None
}
