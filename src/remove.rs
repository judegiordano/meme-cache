use crate::cache::{Metadata, CACHE};

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

pub async fn remove_oldest() -> (String, Metadata) {
    let mut cache = CACHE.lock().await;
    // convert to vec for sorting
    let mut v = Vec::from_iter(cache.to_owned());
    // sort by newest first, oldest last
    v.sort_by(|(_, a), (_, b)| b.set_at.cmp(&a.set_at));
    let (k, v) = v.last().unwrap();
    cache.remove(k);
    (k.clone(), v.clone())
}

#[cfg(test)]
mod tests {
    use crate::{clear, entries, get, remove, remove_last, remove_oldest, set, size};

    #[tokio::test]
    async fn remove_test() {
        clear().await;
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

    #[tokio::test]
    async fn remove_oldest_test() {
        clear().await;
        for i in 1..=10 {
            std::thread::sleep(std::time::Duration::from_millis(10));
            set(&i, &i, 600_000).await;
        }
        let (_, value) = remove_oldest().await;
        for (_, metadata) in entries().await {
            assert!(metadata.set_at > value.set_at)
        }
    }
}
