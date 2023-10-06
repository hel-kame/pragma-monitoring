extern crate chrono;
extern crate bigdecimal;
extern crate uuid;

use chrono::NaiveDateTime;
use uuid::Uuid;
use diesel::Queryable;

#[derive(Queryable, Debug)]
pub struct Storage {
    pub id: Uuid,
    pub network: String,
    pub data_type: String,
    pub block_hash: String,
    pub block_number: i32,
    pub block_timestamp: NaiveDateTime,
    pub transaction_hash: String,
    pub source: Option<String>,
    pub price: Option<f32>,
}
