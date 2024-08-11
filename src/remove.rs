use crate::cache::CACHE;

// NOTE: this doesnt technically remove the oldest entry,
// just whichever one is sorted as last in the Btree
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
    use crate::{clear, get, remove, remove_last, set, size};

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

    #[tokio::test]
    async fn remove_last_test() {
        clear().await;
        for i in 1..=100 {
            set(&i, &i, 600_000).await;
        }
        assert_eq!(size().await, 100);
        remove_last().await;
        assert_eq!(size().await, 99);
    }
}
