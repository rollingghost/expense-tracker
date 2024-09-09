use crate::expense_cli;

use expense_cli::functions::{convert_from_system_time, generate_random_id, save_expenses};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Represents an expense in the expense tracker.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Expense {
    pub id: String,
    pub description: String,
    pub amount: f64,
    pub category: Category,
    pub added_at: String,
    pub updated_at: String,
}

/// Represents the category of an expense.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Category {
    Food,
    Transportation,
    Entertainment,
    Internet,
    UtilityBill,
    Other,
}

impl Expense {
    /// Creates a new expense with the given description, amount, and category.
    pub fn new(description: String, amount: f64, category: Category) -> Self {
        Self {
            id: generate_random_id().to_string(),
            description,
            amount,
            category,
            added_at: convert_from_system_time(SystemTime::now()),
            updated_at: convert_from_system_time(SystemTime::now()),
        }
    }

    /// Updates an expense in the expense tracker.
    ///
    /// # Arguments
    ///
    /// * `update_ready_expense` - The updated expense.
    /// * `all_expenses` - The array of all expenses.
    /// * `index` - The index of the expense to update.
    ///
    /// # Returns
    ///
    /// The updated expense.
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
