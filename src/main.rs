use clap::{Parser, Subcommand};
use expense_tracker::{
    convert_from_system_time, delete_expense, export_expenses, get_budget,
    get_month_from_date_string, load_expenses, map_category, prettify_expense_display,
    prettify_expense_not_found, save_expenses, search_expense_by_id, set_budget, Expense,
};
use std::{process, time::SystemTime};

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
struct ExpenseTracker {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Add an expense.")]
    Add {
        #[arg(short, long)]
        description: String,

        #[arg(short, long)]
        amount: f64,

        #[arg(short, long, default_value = "other")]
        category: String,
    },
    #[command(about = "Update an expense.")]
    Update {
        #[arg(short, long)]
        id: String,

        #[arg(short, long, default_value = "no_update")]
        description: String,

        #[arg(short, long, default_value = "-1.00")]
        amount: f64,

        #[arg(short, long, default_value = "no_update")]
        category: String,
    },
    #[command(about = "Delete an expense.")]
    Delete {
        #[arg(short, long)]
        id: String,
    },
    #[command(about = "View all expenses.")]
    List {
        #[arg(short, long, default_value = "all")]
        id: String,

        #[arg(short, long, default_value = "all")]
        category: String,

        #[arg(short, long, default_value = "0.00")]
        amount: f64,

        #[arg(short, long, default_value = "all")]
        description: String,

        #[arg(long, default_value = "now")]
        added_at: String,
    },
    #[command(about = "View summary of expenses.")]
    Summary {
        #[arg(short, long, default_value = "all")]
        category: String,

        #[arg(short, long, default_value = "0.00")]
        amount: f64,

        #[arg(short, long, default_value = "13")]
        month: u32,
    },
    #[command(about = "Export expenses to a file.")]
    Export {
        #[arg(short, long)]
        file: String,
    },
    #[command(about = "Control budget")]
    Budget {
        #[arg(short, long)]
        budget: f64,
    },
}

fn main() {
    let args = ExpenseTracker::parse();
    let mut all_expenses = load_expenses().unwrap_or_else(|_| vec![]);
    let budget = get_budget();

    match args.command {
        Commands::Add {
            description,
            amount,
            category,
        } => {
            // Create a new task
            let new_expense = Expense::new(description, amount, map_category(&category));
            all_expenses.push(new_expense.clone());
            save_expenses(&all_expenses).unwrap();
            prettify_expense_display(&all_expenses);

            print!("{:?}", new_expense);
        }
        Commands::Update {
            id,
            description,
            amount,
            category,
        } => {
            let expense_index: usize = match search_expense_by_id(&all_expenses, id.as_str()) {
                Some(index) => index,
                None => {
                    println!("No expense was found");
                    process::exit(0);
                }
            };

            if description != "no_update" {
                all_expenses[expense_index].description = description.clone();
            }

            // Update amount. Amount should be > 0.00
            if amount <= 0.00 && amount != all_expenses[expense_index].amount && amount != -1.00 {
                println!("Amount should be greater than 0.00");
                process::exit(0);
            } else if amount != -1.00 {
                all_expenses[expense_index].amount = amount;
            }

            // Update category
            if category != "no_update" {
                all_expenses[expense_index].category = map_category(&category);
            }

            // Update updated time
            all_expenses[expense_index].updated_at = convert_from_system_time(SystemTime::now());

            save_expenses(&all_expenses).unwrap();
            prettify_expense_display(&all_expenses);
        }
        Commands::Delete { id } => {
            let expense_index = match search_expense_by_id(&all_expenses, id.as_str()) {
                Some(index) => index,
                None => {
                    prettify_expense_not_found();
                    process::exit(0);
                }
            };

            let deleted_expense = vec![delete_expense(expense_index, &mut all_expenses)];
            save_expenses(&all_expenses).unwrap();

            prettify_expense_display(&deleted_expense);
        }
        Commands::List {
            id,
            description,
            amount,
            category,
            added_at,
        } => {
            let mut filtered_expenses = all_expenses.clone();

            filtered_expenses.retain(|expense| {
                (id == "all" || expense.id == id)
                    && (description == "all" || expense.description == description)
                    && (amount == 0.00 || expense.amount == amount)
                    && (category == "all" || expense.category == map_category(&category))
                    && (added_at == "now" || expense.added_at == added_at)
            });

            prettify_expense_display(&filtered_expenses);
        }
        Commands::Summary {
            category,
            amount,
            month,
        } => {
            let mut filtered_expenses = all_expenses.clone();

            filtered_expenses.retain(|expense| {
                (category == "all" || expense.category == map_category(&category))
                    && (amount == 0.00 || expense.amount == amount)
                    && (month == 13 || get_month_from_date_string(&expense.added_at) == month)
            });

            if !(1..=12).contains(&month) && month != 13 {
                println!("Invalid month. Month should be between 1 and 12");
                process::exit(0);
            }

            // Get sum of all expenses in filtered expenses
            let total: f64 = filtered_expenses.iter().map(|expense| expense.amount).sum();

            if category != "all" {
                println!(
                    "\n\t\tTotal spent on {:?} stuff: {}\n\t\tBudget: {}\n\t\tDifference: {}\n",
                    map_category(&category),
                    total,
                    budget,
                    budget - total
                );
            } else if (1..=12).contains(&month) {
                println!(
                    "\n\t\tTotal spent on the month of {} stuff: {}\n\t\tBudget: {}\n\t\tDifference: {}\n",
                    month, total, budget, budget - total
                );
            } else {
                println!(
                    "\n\t\tTotal spent on all stuff: {}\n\t\tBudget: {}\n\t\tDifference: {}\n",
                    total,
                    budget,
                    budget - total
                );
            }

            prettify_expense_display(&filtered_expenses);
        }
        Commands::Export { file } => export_expenses(&file, &all_expenses).unwrap(),
        Commands::Budget { budget } => set_budget(budget),
    }
}
