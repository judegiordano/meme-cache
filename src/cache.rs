use chrono::Utc;
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

pub type Cache = BTreeMap<String, Metadata>;
pub type CacheGuard = Mutex<Cache>;

lazy_static! {
    pub static ref CACHE: CacheGuard = Mutex::new(BTreeMap::default());
}

pub async fn entries() -> Cache {
    CACHE.lock().await.to_owned()
}

pub async fn clear() {
    CACHE.lock().await.clear();
}

pub async fn size() -> usize {
    CACHE.lock().await.len()
}

pub async fn purge_stale() {
    let mut cache = CACHE.lock().await;
    let now = Utc::now().timestamp_millis();
    // retain all items where the expiration in ms has not been reached
    cache.retain(|_, val| {
        let diff = now - val.set_at;
        diff <= val.expiration_in_ms
    });
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
    use nanoid::nanoid;
    use serde_json::Value;

    use crate::{
        cache::{entries, CACHE},
        clear, footprint, purge_stale, set, size,
        test::ExampleData,
    };

    #[ignore = "list entries independently"]
    #[tokio::test]
    async fn entries_test() {
        clear().await;
        let value = nanoid!();
        for i in 0..1_000 {
            set(&i, &value, 600_000).await;
        }
        let entries = entries().await;
        assert!(entries.len() == 1_000);
        entries.iter().for_each(|(_, val)| {
            assert_eq!(val.data, Value::String(value.clone()));
        });
    }

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

    #[ignore = "purge stale"]
    #[tokio::test]
    async fn purge_stale_test() {
        clear().await;
        // set half the items to expire instantly; half to have a long expiration
        for i in 0..1_000 {
            let expiration = if i % 2 == 0 { 0 } else { 600_000 };
            set(i, i, expiration).await;
        }
        assert!(size().await == 1_000);
        // wait for data to expire
        std::thread::sleep(std::time::Duration::from_millis(1));
        purge_stale().await;
        // half the data (which is stale) should now be removed
        assert!(size().await == 500);
        CACHE.lock().await.iter().for_each(|(key, value)| {
            // expiration should not be reached
            assert_eq!(value.expiration_in_ms, 600_000);
            let key = key.parse::<i32>().unwrap();
            // all keys should be odd
            assert!(key % 2 != 0);
        });
    }
}
