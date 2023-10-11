use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use std::time::SystemTime;
use crate::models::Storage;
pub async fn timeSinceLastUpdate(query: Storage) -> i64  { 
    let naive_datetime = NaiveDateTime::parse_from_str(query.block_timestamp, "%Y-%m-%dT%H:%M:%S").unwrap();
    let datetime: DateTime<Utc> = DateTime::from_utc(naive_datetime, Utc);
    let timestamp = datetime.timestamp();
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
    now.unwrap().as_secs()-timestamp
}