use expense_tracker::main_expense_cli;
use expense_tracker::surreal_db::db::expense_interface;

#[tokio::main]
async fn main() {
    println!("Hello world");
    expense_interface().await.unwrap();
}
