use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExtStruct {
    #[serde(with = "ts_seconds")]
    pub timestamp: DateTime<Utc>,
    pub bits: u32,
}
