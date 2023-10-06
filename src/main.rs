// main.rs
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

mod schema;
mod models;

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    use schema::storage::dsl::*;

    let results = storage
        .load::<models::Storage>(&mut connection)
        .expect("Error loading storages");

    println!("Displaying {} storages", results.len());
    for other_storage in results {
        println!("{:?}", other_storage);
    // Now you can use connection to interact with database
    }
}
