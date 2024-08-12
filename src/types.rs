use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use tokio::sync::Mutex;

const FIVE_MINUTES_IN_MS: i64 = (1_000 * 60) * 5;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Metadata {
    pub expiration_in_ms: i64,
    pub set_at: i64,
    pub data: Value,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            expiration_in_ms: FIVE_MINUTES_IN_MS,
            set_at: Utc::now().timestamp_millis(),
            data: Default::default(),
        }
    }
}

pub type Cache = BTreeMap<String, Metadata>;
pub type CacheGuard = Mutex<Cache>;
pub type Entry = (String, Metadata);
