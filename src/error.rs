use std::{error::Error as StdError, fmt};

#[derive(Debug)]
pub enum MonitoringError {
    PriceError(String),
    TimeError(String),
    DatabaseError(diesel::result::Error),
    ConnectionError(String),
}

impl StdError for MonitoringError {}

impl fmt::Display for MonitoringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MonitoringError::PriceError(e) => write!(f, "Price Error: {}", e),
            MonitoringError::TimeError(e) => write!(f, "Time Error: {}", e),
            MonitoringError::DatabaseError(e) => write!(f, "Database Error: {}", e),
            MonitoringError::ConnectionError(e) => write!(f, "Connection Error: {}", e),
        }
    }
}

// Convert diesel error to our custom error
impl From<diesel::result::Error> for MonitoringError {
    fn from(err: diesel::result::Error) -> MonitoringError {
        MonitoringError::DatabaseError(err)
    }
}
