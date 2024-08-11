use chrono::Utc;
use serde::Serialize;

use crate::cache::{Metadata, CACHE};

pub async fn set(key: impl ToString, value: impl Serialize, expiration_in_ms: i64) -> usize {
    let mut cache = CACHE.lock().await;
    cache.insert(
        key.to_string(),
        Metadata {
            expiration_in_ms,
            set_at: Utc::now().timestamp_millis(),
            data: serde_json::to_value(value).unwrap(),
        },
    );
    cache.len()
}

#[cfg(test)]
mod tests {

    use crate::{clear, size, test::ExampleData};

    use super::*;

    #[tokio::test]
    async fn set_test() {
        clear().await;
        let [one, two, three] = [
            ExampleData::default(),
            ExampleData::default(),
            ExampleData::default(),
        ];
        let inserted = set(&one.id, &one, 60_000).await;
        assert!(inserted >= 1_usize);

        let inserted = set(&two.id, &two, 60_000).await;
        assert!(inserted >= 2_usize);

        let inserted = set(&three.id, &three, 60_000).await;
        assert!(inserted >= 3_usize);
    }

    #[tokio::test]
    async fn set_scalar_types() {
        clear().await;
        let key = nanoid::nanoid!();
        let val = format!("some_string_{}", nanoid::nanoid!());
        let inserted = set(&key, val, 60_000).await;
        assert!(inserted >= 1_usize);

        let key = nanoid::nanoid!();
        let val = 8;
        let inserted = set(&key, val, 60_000).await;
        assert!(inserted >= 2_usize);

        let key = nanoid::nanoid!();
        let val = true;
        let inserted = set(&key, val, 60_000).await;
        assert!(inserted >= 3_usize);

        let key = nanoid::nanoid!();
        let val = 'c';
        let inserted = set(&key, val, 60_000).await;
        assert!(inserted >= 4_usize);

        let key = nanoid::nanoid!();
        let val = (1, 2);
        let inserted = set(&key, val, 60_000).await;
        assert!(inserted >= 5_usize);

        let key = nanoid::nanoid!();
        let val = vec![1, 2];
        let inserted = set(&key, val, 60_000).await;
        assert!(inserted >= 6_usize);
    }

    #[tokio::test]
    async fn tokio_tasks_test() {
        clear().await;
        let mut handlers = vec![];
        let tasks = 10_000;
        for _ in 0..tasks {
            let handler = tokio::task::spawn(async {
                let data = ExampleData::default();
                let inserted = set(&data.id, &data, 10_000).await;
                assert!(inserted >= 1);
            });
            handlers.push(handler);
        }
        for handler in handlers {
            assert!(handler.await.is_ok());
        }
        let cache_size = size().await;
        assert!(cache_size >= tasks);
    }
}
