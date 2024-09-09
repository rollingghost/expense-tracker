use crate::expense_cli;

use expense_cli::structs_enums::{Category, Expense};

use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[derive(Debug, Serialize)]
pub struct DbExpense<'a> {
    id: &'a str,
    description: &'a str,
    amount: &'a f64,
    category: &'a Category,
    added_at: &'a str,
    updated_at: &'a str,
}

#[derive(Debug, Serialize)]
pub struct Name<'a> {
    first: &'a str,
    last: &'a str,
}

#[derive(Debug, Serialize)]
pub struct Person<'a> {
    title: &'a str,
    name: Name<'a>,
    marketing: bool,
}

#[derive(Debug, Serialize)]
pub struct Responsibility {
    marketing: bool,
}

#[derive(Debug, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    id: Thing,
}

pub const SURREAL_ADDRESS: &str = "127.0.0.1:8002";
pub const SURREAL_PASS: &str = "root";
pub const SURREAL_USER: &str = "root";

/// Creates a new expense in the database given expense struct
///
/// # Arguments
///
/// * `expense` - The expense or type `Expense` to add
///
/// # Returns
///
/// Return a `surreal::Result`, `OK(())` on success
pub async fn new_expense(expense: &Expense) -> surrealdb::Result<()> {
    let db = Surreal::new::<Ws>(SURREAL_ADDRESS).await?;

    //Sign in into the namespace
    db.signin(Root {
        username: SURREAL_USER,
        password: SURREAL_PASS,
    })
    .await?;

    // Select specific namespace / database
    db.use_ns("expenses").use_db("expenses").await?;

    let created: Vec<Record> = db
        .create("expenses")
        .content(DbExpense {
            id: &expense.id,
            description: &expense.description,
            amount: &expense.amount,
            category: &expense.category,
            added_at: &expense.added_at,
            updated_at: &expense.updated_at,
        })
        .await?;

    dbg!(created);

    Ok(())
}

/// Update an expense in the database given an expense `id` and updated expense struct
///
/// # Arguments
///
/// * `id` - the id of the expense to update
/// * `expense` - the updated expense struct
///
/// # Return
///
/// Return a `surreal:Result`, `OK` on success
pub async fn update_expense(id: usize, expense: &Expense) -> surrealdb::Result<()> {
    let db = Surreal::new::<Ws>(SURREAL_ADDRESS).await?;

    db.signin(Root {
        username: SURREAL_USER,
        password: SURREAL_PASS,
    })
    .await?;

    let updated: Option<Record> = db
        .update(("expenses", id.to_string()))
        .merge(DbExpense {
            id: &expense.id,
            description: &expense.description,
            amount: &expense.amount,
            category: &expense.category,
            added_at: &expense.added_at,
            updated_at: &expense.updated_at,
        })
        .await?;

    dbg!(updated);
    Ok(())
}

/// Selects a resource record from the database given an `id` of the record
///
/// # Arguments
///
/// * `id` - the id of the resource record to fetch
///
/// # Return
///
/// Returns a `surreal::Result`, `OK(resource record)` on success
pub async fn select_expense(_id: usize) -> surrealdb::Result<Expense> {
    let db = Surreal::new::<Ws>(SURREAL_ADDRESS).await?;

    db.signin(Root {
        username: SURREAL_USER,
        password: SURREAL_PASS,
    })
    .await?;

    // let record: Vec<Record> = db.select(format!("expenses:{}", id.to_string()));

    todo!()
}

/// Fetches all records of a resource from the database
///
/// # Arguments
///
/// Doesn't take any arguments `select_all_expenses()`
///
/// # Return
///
/// Returns a `surreal::Result<Vec<Record>>`, `OK(all_resource_records)` on success
pub async fn select_all_expenses() -> surrealdb::Result<Vec<Record>> {
    let db = Surreal::new::<Ws>(SURREAL_ADDRESS).await?;

    db.signin(Root {
        username: SURREAL_USER,
        password: SURREAL_PASS,
    })
    .await?;

    db.use_ns("expenses").use_db("expenses").await?;

    let all_records: Vec<Record> = db.select("expenses").await?;
    Ok(all_records)
}

/// Removes an expense from the database given the resource record id
///
/// # Arguments
///
/// * `id` - the ide of the resource record to be deleted
///
/// # Return
///
/// Returns a `surreal::Result`, `Ok(())` on success
pub async fn delete_expense(_id: usize) -> surrealdb::Result<()> {
    let db = Surreal::new::<Ws>(SURREAL_ADDRESS).await?;

    db.signin(Root {
        username: SURREAL_USER,
        password: SURREAL_PASS,
    })
    .await?;

    // db.delete(format!("expenses:", id.to_string()).to_string())
    //    .await?;
    todo!()
}
