use chrono::Utc;
use lazy_static::lazy_static;
use std::{collections::HashMap, mem::size_of_val};
use tokio::sync::Mutex;

use crate::types::{Cache, CacheGuard};

lazy_static! {
    pub static ref CACHE: CacheGuard = Mutex::new(HashMap::default());
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

pub async fn purge_stale() -> usize {
    let mut cache = CACHE.lock().await;
    let now = Utc::now().timestamp_millis();
    // retain all items where the expiration in ms has not been reached
    let mut purged_data = 0;
    cache.retain(|_, val| {
        let diff = now - val.set_at;
        if diff <= val.expiration_in_ms {
            purged_data += 1;
            return true;
        }
        false
    });
    purged_data
}

#[allow(clippy::cast_precision_loss)]
// value in bytes
pub async fn footprint() -> usize {
    let size = CACHE.lock().await.values().fold(0, |mut acc, val| {
        acc += size_of_val(val);
        acc
    });
    tracing::debug!("footprint in MB: {:?}", size as f64 * 0.000_001);
    size
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use nanoid::nanoid;
    use serde_json::Value;

    use crate::{
        cache::entries,
        clear, footprint, purge_stale, set, size,
        test::{sleep_ms, ExampleData},
    };

    #[tokio::test]
    async fn entries_test() {
        clear().await;
        let value = nanoid!();
        for i in 0..1_000 {
            set(&i, &value, 600_000).await;
        }
        let start = Instant::now();
        let entries = entries().await;
        println!("entries operation done in: {:?}", start.elapsed());
        assert!(entries.len() == 1_000);
        entries.iter().for_each(|(_, val)| {
            assert_eq!(val.data, Value::String(value.clone()));
        });
    }

    #[tokio::test]
    async fn size_test() {
        clear().await;
        let data = ExampleData::default();
        set(&data.id, &data, 10_000).await;
        assert!(size().await >= 1_usize);
    }

    #[tokio::test]
    async fn clear_test() {
        clear().await;
        let data = ExampleData::default();
        set(&data.id, &data, 10_000).await;

        assert!(size().await >= 1_usize);

        clear().await;
        assert!(size().await == 0_usize);
    }

    #[tokio::test]
    async fn footprint_test() {
        clear().await;
        for _ in 0..=1_000 {
            let data = ExampleData::default();
            set(&data.id, &data, 100_000).await;
        }
        let now = std::time::Instant::now();
        let memory_size = footprint().await;
        println!(
            "footprint: {} MB in {:?}",
            memory_size as f64 * 0.000_001,
            now.elapsed()
        );
    }

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

    #[ignore = "this creates about 4mb of memory"]
    #[tokio::test]
    async fn larger_footprint_test() {
        clear().await;
        for _ in 0..=500_000 {
            let data = ExampleData::default();
            set(&data.id, &data, 100_000).await;
        }
        let now = std::time::Instant::now();
        let memory_size = footprint().await;
        println!(
            "larger footprint: {} MB in {:?}",
            memory_size as f64 * 0.000001,
            now.elapsed()
        );
        clear().await;
    }

    #[tokio::test]
    async fn purge_stale_test() {
        clear().await;
        // set half the items to expire instantly; half to have a long expiration
        for i in 0..10_000 {
            let expiration = if i % 2 == 0 { 0 } else { 600_000 };
            set(i, i, expiration).await;
        }
        assert!(size().await == 10_000);
        // wait for data to expire
        sleep_ms(1);
        let start = Instant::now();
        let purged = purge_stale().await;
        // should have purged half
        assert_eq!(purged, 5_000_usize);
        println!("purge stale operation done in: {:?}", start.elapsed());
        // half the data (which is stale) should now be removed
        assert!(size().await == 5_000_usize);
        entries().await.iter().for_each(|(key, value)| {
            // expiration should not be reached
            assert_eq!(value.expiration_in_ms, 600_000);
            let key = key.parse::<i32>().unwrap();
            // all keys should be odd
            assert!(key % 2 != 0);
        });
    }

    #[tokio::test]
    async fn purge_nothing_test() {
        clear().await;
        for i in 0..10_000 {
            set(i, i, 600_000).await;
        }
        assert!(size().await == 10_000_usize);

        let purged = purge_stale().await;
        assert_eq!(purged, 10_000_usize);

        assert!(size().await == 10_000_usize);
    }
}
