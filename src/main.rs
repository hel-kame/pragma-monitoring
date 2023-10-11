// main.rs
extern crate diesel;
extern crate dotenv;


use diesel::QueryDsl;
use diesel::QueryResult;
use diesel_async::{RunQueryDsl, AsyncPgConnection,AsyncConnection};
use dotenv::dotenv;
use std::{env,time::SystemTime};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool::*;
use diesel_async::pooled_connection::PoolableConnection;
use crate::diesel::ExpressionMethods;
use crate::models::Storage;
use std::error::Error;
use schema::storage::dsl::*;
mod schema;
mod models;
mod monitoring;
use std::pin::Pin;



#[tokio::main]
async fn main() {
    dotenv().ok();
    let pairs = vec![("BTC/USD", "BINANCE"), ("ETH/USD", "BINANCE")];
    
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    let pool = Pool::builder(config).build().unwrap();

    let tasks: Vec<_> = pairs.into_iter().map(|(pair, srce)| {
        let pool_reference:deadpool::managed::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>  = pool.clone();
        tokio::spawn(Box::pin(process_data(pool_reference, pair, srce)))
    }).collect();
    

    let results: Vec<_> = futures::future::join_all(tasks).await.into_iter()
        .map(|task| task.unwrap()) // task.unwrap() is used to get the Result returned by process_data
        .collect();

    // Process or output the results
    for result in results {
        match result {
            Ok(data) => println!("Task succeeded with data: {:?}", data),
            Err(e) => eprintln!("Task failed with error: {:?}", e),
        }
    }
    
}



async fn process_data(pool: deadpool::managed::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>, pair:&str, srce: &str) -> Result<u64,Box<dyn Error + Send>>{ 
    let mut conn = pool.get().await.unwrap(); 
    let result: Storage = storage.filter(pair_id.eq(pair))
    .order(block_timestamp.desc())
    .first(&mut conn)
    .await.unwrap();
    let time = monitoring::timeLastUpdate::time_since_last_update(result).await;
    Ok(time)
}