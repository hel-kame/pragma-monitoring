// main.rs
extern crate diesel;
extern crate dotenv;

use diesel::query_dsl::methods::FilterDsl;
use diesel_async::{RunQueryDsl, AsyncPgConnection};
use dotenv::dotenv;
use std::{env,time::SystemTime};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool::*;
use diesel_async::pooled_connection::PoolableConnection;
use crate::models::Storage;
mod schema;
mod models;
mod monitoring;
use schema::storage::dsl::*;



#[tokio::main]
async fn main() {
    dotenv().ok();
    let pairs = vec!["BTC/USD"];
    
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    let pool = Pool::builder(config).build().unwrap();

    let tasks: Vec<_> = pairs.into_iter().map(|pair| {
        let pool_reference = &pool;
        tokio::spawn(process_data(pool_reference, pair))
    }).collect();

    //let results: Vec<_> = futures::future::join_all(tasks).await.into_iter().map(|task| task.unwrap()).collect();

    
}



async fn process_data(pool: &Pool<AsyncDieselConnectionManager<AsyncPgConnection>>, pair:&str, source : &str) -> Result<(), ()> { 
    let mut conn = pool.get().await.unwrap();
    let query = storage.filter::<Storage>(&mut conn).await.unwrap();
    let time = monitoring::timeLastUpdate::timeSinceLastUpdate(query);
    Ok(())
}