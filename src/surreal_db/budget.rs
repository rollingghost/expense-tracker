// Handling the budget allocation
use crate::surreal_db;

use serde::Serialize;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

use surreal_db::db::Record;
use surreal_db::db::SURREAL_ADDRESS;
use surreal_db::db::SURREAL_PASS;
use surreal_db::db::SURREAL_USER;

#[derive(Debug, Serialize)]
pub struct Budget {
    yearly: f64,
    monthly: f64,
    daily: f64,
}

pub async fn get_client() -> surrealdb::Result<Surreal<Client>> {
    let db = Surreal::new::<Ws>(SURREAL_ADDRESS).await?;

    db.signin(Root {
        username: SURREAL_USER,
        password: SURREAL_PASS,
    })
    .await?;

    db.use_ns("expenses").use_db("expenses").await?;

    Ok(db)
}

pub async fn set_new_budget(_budget: Budget) -> surrealdb::Result<()> {
    let _db = get_client().await?;

    // db.create("budget").content(budget).await?;

    // Ok(())

    todo!()
}

pub async fn get_budget() -> surrealdb::Result<Vec<Record>> {
    let client = get_client().await?;

    let records: Vec<Record> = client.select("budget").await?;

    Ok(records)
}

pub async fn update_budget() -> surrealdb::Result<()> {
    todo!()
}
