use crate::cache::CACHE;

pub async fn remove_last() {
    let mut cache = CACHE.lock().await;
    let item = cache.last_entry().unwrap();
    let id = item.key().clone();
    cache.remove_entry(&id);
}

pub async fn remove(key: &str) -> bool {
    let mut cache = CACHE.lock().await;
    if let Some(_) = cache.remove(key) {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::{get, remove, set};

    #[tokio::test]
    async fn remove_test() {
        let key = nanoid::nanoid!();
        set(&key, &key, 600_000).await;
        // prove item was set
        assert_eq!(get(&key).await, Some(key.clone()));
        // remove item
        assert!(remove(&key).await);
        // prove item is gone
        assert_eq!(get::<String>(&key).await, None);
    }
}
