// main.rs
extern crate diesel;
extern crate dotenv;

use crate::diesel::ExpressionMethods;
use crate::models::SpotEntry;
use diesel::QueryDsl;
use diesel_async::pooled_connection::deadpool::*;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use dotenv::dotenv;

use schema::spot_entry::dsl::*;
use std::error::Error;
use std::env;
mod models;
mod monitoring;
mod schema;
use prometheus::{opts, register_gauge_vec, GaugeVec};
mod server;

lazy_static::lazy_static! {
    static ref TIME_SINCE_LAST_UPDATE: GaugeVec = register_gauge_vec!(
        opts!("time_since_last_updatee_seconds", "Time since the last update in seconds."),
        &["pair", "source"]
    ).unwrap();
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let pairs = vec![("BTC/USD", "CEX"), ("ETH/USD", "CEX")];
    tokio::spawn(server::run_metrics_server());
    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    let pool = Pool::builder(config).build().unwrap();
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(5));
    loop {
        interval.tick().await; // Wait for the next tick

        let tasks: Vec<_> = pairs
            .clone()
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
        for result in &results {
            match result {
                Ok(data) => println!("Task succeeded with data: {:?}", data),
                Err(e) => eprintln!("Task failed with error: {:?}", e),
            }
        }
    }
}

async fn process_data(
    pool: deadpool::managed::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
    pair: &str,
    srce: &str,
) -> Result<u64, Box<dyn Error + Send>> {
    let mut conn = pool.get().await.unwrap();
    let result: Result<SpotEntry, _> = spot_entry
        .filter(pair_id.eq(pair))
        .filter(source.eq(srce))
        .order(block_timestamp.desc())
        .first(&mut conn)
        .await;
    match result {
        Ok(data) => {
            let time = monitoring::timeLastUpdate::time_since_last_update(data).await;
            let labels = TIME_SINCE_LAST_UPDATE.with_label_values(&[pair, srce]);
            labels.set(time as f64);
            Ok(time)
        }
        Err(e) => Err(Box::new(e)),
    }
}
