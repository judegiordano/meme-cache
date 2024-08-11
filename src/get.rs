use chrono::Utc;
use serde::de::DeserializeOwned;

use crate::cache::CACHE;

pub async fn get<T: DeserializeOwned>(key: &str) -> Option<T> {
    let mut cache = CACHE.lock().await;
    if let Some(exists) = cache.get(key) {
        tracing::debug!("cache hit: {} total items", cache.len());
        let now = Utc::now().timestamp_millis();
        let diff = now - exists.set_at;
        if diff < exists.expiration_in_ms {
            let json = serde_json::from_value::<T>(exists.data.clone()).unwrap();
            return Some(json);
        }
        tracing::debug!("data stale; removing...");
        cache.remove(key);
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::{clear, get, set, size, test::ExampleData};

    #[tokio::test]
    async fn get_test() {
        clear().await;
        let data = ExampleData::default();
        set(&data.id, &data, 10_000).await;
        assert!(size().await >= 1_usize);

        let exists = get::<ExampleData>(&data.id).await;
        assert!(exists.is_some());

        let cached_data = exists.unwrap();
        assert_eq!(data, cached_data);
    }

    #[tokio::test]
    async fn get_scalar_types() {
        clear().await;
        let key = nanoid::nanoid!();
        let val = format!("some_string_{}", nanoid::nanoid!());
        set(&key, &val, 60_000).await;
        assert_eq!(get(&key).await, Some(val));

        let key = nanoid::nanoid!();
        let val = 8;
        set(&key, &val, 60_000).await;
        assert_eq!(get(&key).await, Some(val));

        let key = nanoid::nanoid!();
        let val = true;
        set(&key, &val, 60_000).await;
        assert_eq!(get(&key).await, Some(val));

        let key = nanoid::nanoid!();
        let val = 'c';
        set(&key, &val, 60_000).await;
        assert_eq!(get(&key).await, Some(val));

        let key = nanoid::nanoid!();
        let val = (1, 2);
        set(&key, &val, 60_000).await;
        assert_eq!(get(&key).await, Some(val));

        let key = nanoid::nanoid!();
        let val = vec![1, 2];
        set(&key, &val, 60_000).await;
        assert_eq!(get(&key).await, Some(val));
    }

    #[tokio::test]
    async fn get_test_large() {
        clear().await;
        let data = ExampleData::default();
        set(&data.id, &data, 100_000).await;

        // now add a bunch more
        for _ in 0..=10_000 {
            let data = ExampleData::default();
            set(&data.id, &data, 100_000).await;
        }

        let exists = get::<ExampleData>(&data.id).await;
        assert!(exists.is_some());

        let cached_data = exists.unwrap();
        assert_eq!(data, cached_data);
    }

    #[tokio::test]
    async fn expiration_test() {
        clear().await;
        let data = ExampleData::default();
        // set low expiration
        set(&data.id, &data, 1).await;
        std::thread::sleep(std::time::Duration::from_millis(1));

        assert!(size().await >= 1_usize);

        let stale = get::<ExampleData>(&data.id).await;
        assert!(stale.is_none())
    }
}
