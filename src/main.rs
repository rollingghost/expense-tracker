use clap::{Parser, Subcommand};
use expense_tracker::{map_category, Expense};

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
    Update,
    #[command(about = "Delete an expense.")]
    Delete,
    #[command(about = "View all expenses.")]
    View,
    #[command(about = "View summary of expenses.")]
    Summary,
    #[command(about = "Get expenses by category")]
    Category,
    #[command(about = "Export expenses to a file.")]
    Export {
        #[arg(short, long)]
        file: String,
    },
    #[command(about = "Control budget")]
    Budget,
}

fn main() {
    let args = ExpenseTracker::parse();

    match args.command {
        Commands::Add {
            description,
            amount,
            category,
        } => {
            // Create a new task
            let new_expense = Expense::new(description, amount, map_category(&category));

            print!("New expense added: {:?}", new_expense);
        }
        Commands::Update => println!("Update command"),
        Commands::Delete => println!("Delete command"),
        Commands::View => println!("View command"),
        Commands::Summary => println!("Summary command"),
        Commands::Category => println!("Category command"),
        Commands::Export { file } => println!("Export to {file}"),
        Commands::Budget => print!("Control budget"),
    }
}
