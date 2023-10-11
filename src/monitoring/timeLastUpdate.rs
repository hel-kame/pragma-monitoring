use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use std::time::SystemTime;
use crate::models::Storage;
pub async fn time_since_last_update(query: Storage) -> u64  { 
    let datetime: DateTime<Utc> = TimeZone::from_utc_datetime(&Utc,&query.block_timestamp);
    let timestamp = datetime.timestamp();
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
    now.unwrap().as_secs()- timestamp as u64
}