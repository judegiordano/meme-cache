use crate::{cache::CACHE, types::Entry};

// NOTE: this doesnt technically remove the oldest entry,
// just whichever one is sorted as last in the Btree
pub async fn remove_last() -> Option<Entry> {
    let mut cache = CACHE.lock().await;
    let item = cache.last_entry().unwrap();
    let id = item.key().clone();
    if let Some((key, value)) = cache.remove_entry(&id) {
        return Some((key, value));
    }
    None
}

pub async fn remove(key: &str) -> Option<Entry> {
    let mut cache = CACHE.lock().await;
    if let Some(removed) = cache.remove(key) {
        return Some((key.to_string(), removed));
    }
    None
}

pub async fn remove_oldest() -> Option<Entry> {
    let mut cache = CACHE.lock().await;
    // convert to vec for sorting
    if cache.is_empty() {
        return None;
    }
    let mut v = Vec::from_iter(cache.to_owned());
    // sort by newest first, oldest last
    v.sort_by(|(_, a), (_, b)| b.set_at.cmp(&a.set_at));
    if let Some((k, v)) = v.last() {
        cache.remove(k);
        return Some((k.to_owned(), v.to_owned()));
    }
    None
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::{
        clear, entries, get, remove, remove_last, remove_oldest, set, size, test::sleep_ms,
    };

    #[tokio::test]
    async fn remove_test() {
        clear().await;
        let key = nanoid::nanoid!();
        set(&key, &key, 600_000).await;
        // prove item was set
        assert_eq!(get(&key).await, Some(key.clone()));
        // remove item
        let removed = remove(&key).await;
        assert!(removed.is_some());
        let (removed_key, removed_value) = removed.unwrap();
        assert_eq!(removed_key, key);
        assert_eq!(removed_value.data, key);
        // prove item is gone
        assert_eq!(get::<String>(&key).await, None);
    }

    #[tokio::test]
    async fn remove_last_test() {
        clear().await;
        for i in 1..=10_000 {
            set(&i, &i, 600_000).await;
        }
        assert_eq!(size().await, 10_000_usize);

        let start = Instant::now();
        remove_last().await;
        println!("remove last operation done in: {:?}", start.elapsed());
        assert_eq!(size().await, 9_999_usize);
    }

    #[tokio::test]
    async fn remove_oldest_test() {
        clear().await;
        let key = nanoid::nanoid!();
        set(&key, &key, 600_000).await;
        sleep_ms(10);
        for i in 1..=100_000 {
            set(&i, &i, 600_000).await;
        }
        let start = Instant::now();
        let removed = remove_oldest().await;
        println!("remove oldest operation done in: {:?}", start.elapsed());
        assert!(removed.is_some());
        let (removed_key, removed_value) = removed.unwrap();
        assert_eq!(key, removed_key);
        assert_eq!(removed_value.data, key);
        for (_, metadata) in entries().await {
            assert!(metadata.set_at > removed_value.set_at)
        }
    }
}
