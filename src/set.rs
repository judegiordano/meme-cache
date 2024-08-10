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

    use crate::{size, test::ExampleData};

    use super::*;

    #[tokio::test]
    async fn set_test() {
        let [one, two, three] = [
            ExampleData::default(),
            ExampleData::default(),
            ExampleData::default(),
        ];
        let inserted = set(&one.id, &one, 10_000).await;
        assert!(inserted >= 1_usize);

        let inserted = set(&two.id, &two, 10_000).await;
        assert!(inserted >= 2_usize);

        let inserted = set(&three.id, &three, 10_000).await;
        assert!(inserted >= 3_usize);
    }

    #[tokio::test]
    async fn threading_test() {
        let mut handlers = vec![];
        let tasks = 1_000;
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
