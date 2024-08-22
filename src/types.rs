use chrono::Utc;
use fnv::FnvHashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::Mutex;

/// five minutes in ms (1,000 ms -> 1 sec -> 1 min * 5)
pub const DEFAULT_EXPIRATION: i64 = (1_000 * 60) * 5;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Metadata {
    pub expiration_in_ms: i64,
    pub set_at: i64,
    pub data: Value,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            expiration_in_ms: DEFAULT_EXPIRATION,
            set_at: Utc::now().timestamp_millis(),
            data: Value::default(),
        }
    }
}

pub type Cache = FnvHashMap<String, Metadata>;
pub type CacheGuard = Mutex<Cache>;
pub type Entry = (String, Metadata);
