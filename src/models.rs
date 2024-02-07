extern crate bigdecimal;
extern crate chrono;

use std::ops::Bound;

use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{Queryable, QueryableByName, Selectable};

#[derive(Debug, Queryable, Selectable, QueryableByName)]
#[diesel(table_name = crate::schema::spot_entry)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SpotEntry {
    pub network: String,
    pub pair_id: String,
    pub data_id: String,
    pub block_hash: String,
    pub block_number: i64,
    pub block_timestamp: NaiveDateTime,
    pub transaction_hash: String,
    pub price: BigDecimal,
    pub timestamp: chrono::NaiveDateTime,
    pub publisher: String,
    pub source: String,
    pub volume: BigDecimal,
    pub _cursor: i64,
}

#[derive(Debug, Queryable, Selectable, QueryableByName)]
#[diesel(table_name = crate::schema::future_entry)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FutureEntry {
    pub network: String,
    pub pair_id: String,
    pub data_id: String,
    pub block_hash: String,
    pub block_number: i64,
    pub block_timestamp: NaiveDateTime,
    pub transaction_hash: String,
    pub price: BigDecimal,
    pub timestamp: chrono::NaiveDateTime,
    pub publisher: String,
    pub source: String,
    pub volume: BigDecimal,
    pub expiration_timestamp: Option<chrono::NaiveDateTime>,
    pub _cursor: i64,
}

#[derive(Debug, Queryable, Selectable, QueryableByName)]
#[diesel(table_name = crate::schema::vrf_requests)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct VrfEntry {
    pub network: String,
    pub request_id: BigDecimal,
    pub seed: BigDecimal,
    pub created_at: NaiveDateTime,
    pub created_at_tx: String,
    pub callback_address: String,
    pub callback_fee_limit: BigDecimal,
    pub num_words: BigDecimal,
    pub requestor_address: String,
    pub updated_at: NaiveDateTime,
    pub updated_at_tx: String,
    pub status: BigDecimal,
    pub minimum_block_number: BigDecimal,
    pub _cursor: (Bound<i64>, Bound<i64>),
    pub data_id: String,
}
