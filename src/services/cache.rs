use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, SystemTime};

type CacheEntry = (String, SystemTime);
static CACHE: OnceLock<Mutex<HashMap<String, CacheEntry>>> = OnceLock::new();

pub fn get(key: &str, ttl: Duration) -> Option<String> {
    let map = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    let guard = map.lock().ok()?;
    let (value, saved_at) = guard.get(key)?;
    if saved_at.elapsed().ok()? > ttl {
        return None;
    }
    Some(value.clone())
}

pub fn set(key: String, value: String) {
    let map = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    if let Ok(mut guard) = map.lock() {
        guard.insert(key, (value, SystemTime::now()));
    }
}
