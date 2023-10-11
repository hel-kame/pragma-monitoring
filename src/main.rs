// main.rs
extern crate diesel;
extern crate dotenv;

use crate::diesel::ExpressionMethods;
use crate::models::Storage;
use diesel::QueryDsl;
use diesel::QueryResult;
use diesel_async::pooled_connection::deadpool::*;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::PoolableConnection;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use dotenv::dotenv;
use schema::storage::dsl::*;
use std::error::Error;
use std::{env, time::SystemTime};
mod models;
mod monitoring;
mod schema;
use prometheus::{register_gauge, Gauge};
use std::pin::Pin;
mod server;

lazy_static::lazy_static! {
    static ref TIME_SINCE_LAST_UPDATE: Gauge = register_gauge!(
        "time_since_last_update_seconds",
        "Time since the last update in seconds."
    ).unwrap();
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let pairs = vec![
        ("BTC/USD", "BINANCE"),
        ("ETH/USD", "BINANCE"),
        ("BTC/ETH", "BINANCE"),
    ];
    let server_task = tokio::spawn(server::run_metrics_server());
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    let pool = Pool::builder(config).build().unwrap();

    let tasks: Vec<_> = pairs
        .into_iter()
        .map(|(pair, srce)| {
            let pool_reference: deadpool::managed::Pool<
                AsyncDieselConnectionManager<AsyncPgConnection>,
            > = pool.clone();
            tokio::spawn(Box::pin(process_data(pool_reference, pair, srce)))
        })
        .collect();

    let results: Vec<_> = futures::future::join_all(tasks)
        .await
        .into_iter()
        .map(|task| task.unwrap()) // task.unwrap() is used to get the Result returned by process_data
        .collect();

    // Process or output the results
    for result in results {
        match result {
            Ok(data) => println!("Task succeeded with data: {:?}", data),
            Err(e) => eprintln!("Task failed with error: {:?}", e),
        }
    }
    server_task.await.unwrap();
}

async fn process_data(
    pool: deadpool::managed::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
    pair: &str,
    srce: &str,
) -> Result<u64, Box<dyn Error + Send>> {
    let mut conn = pool.get().await.unwrap();
    let result: Result<Storage, _> = storage
        .filter(pair_id.eq(pair))
        .filter(source.eq(srce))
        .order(block_timestamp.desc())
        .first(&mut conn)
        .await;
    match result {
        Ok(data) => {
            let time = monitoring::timeLastUpdate::time_since_last_update(data).await;
            TIME_SINCE_LAST_UPDATE.set(time as f64);
            Ok(time)
        }
        Err(e) => Err(Box::new(e)),
    }
}
