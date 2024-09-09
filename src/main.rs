use expense_tracker::main_expense_cli;

#[tokio::main]
async fn main() {
    println!("Hello world");
    main_expense_cli().await;
}
