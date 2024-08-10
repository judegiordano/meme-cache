use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::BTreeMap, mem::size_of_val};
use tokio::sync::Mutex;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Metadata {
    pub expiration_in_ms: i64,
    pub set_at: i64,
    pub data: Value,
}

lazy_static! {
    pub static ref CACHE: Mutex<BTreeMap<String, Metadata>> = Mutex::new(BTreeMap::default());
}

pub async fn clear() {
    CACHE.lock().await.clear();
}

pub async fn size() -> usize {
    CACHE.lock().await.len()
}

// value in bytes
pub async fn footprint() -> usize {
    CACHE.lock().await.iter().fold(0, |mut acc, val| {
        acc += size_of_val(&val);
        acc
    })
}

#[cfg(test)]
mod tests {
    use crate::{clear, set, size, test::ExampleData};

    use super::footprint;

    #[tokio::test]
    async fn size_test() {
        let data = ExampleData::default();
        set(&data.id, &data, 10_000).await;
        assert!(size().await >= 1_usize);
    }

    #[ignore = "clear should happen last"]
    #[tokio::test]
    async fn clear_test() {
        let data = ExampleData::default();
        set(&data.id, &data, 10_000).await;

        assert!(size().await >= 1_usize);

        clear().await;
        assert!(size().await == 0_usize);
    }

    #[ignore = "memory footprint"]
    #[tokio::test]
    async fn footprint_test() {
        for _ in 0..=1_000 {
            let data = ExampleData::default();
            set(&data.id, &data, 100_000).await;
        }
        let now = std::time::Instant::now();
        let memory_size = footprint().await;
        println!(
            "footprint: {} MB in {:?}",
            memory_size as f64 * 0.000001,
            now.elapsed()
        );
    }

    #[ignore = "scalar memory footprint"]
    #[tokio::test]
    async fn scalar_footprint_test() {
        clear().await;
        for i in 0..=1_000 {
            set(i, i, 600_000).await;
        }
        let now = std::time::Instant::now();
        let memory_size = footprint().await;
        println!(
            "footprint: {} MB in {:?}",
            memory_size as f64 * 0.000001,
            now.elapsed()
        );
    }
}
