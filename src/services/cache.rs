use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::Duration;
#[cfg(not(target_arch = "wasm32"))]
use std::time::{SystemTime, UNIX_EPOCH};

type CacheEntry = (String, u64);
static CACHE: OnceLock<Mutex<HashMap<String, CacheEntry>>> = OnceLock::new();

pub fn get(key: &str, ttl: Duration) -> Option<String> {
    let map = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    let guard = map.lock().ok()?;
    let (value, saved_at) = guard.get(key)?;
    if now_millis().saturating_sub(*saved_at) > ttl.as_millis() as u64 {
        return None;
    }
    Some(value.clone())
}

pub fn set(key: String, value: String) {
    let map = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    if let Ok(mut guard) = map.lock() {
        guard.insert(key, (value, now_millis()));
    }
}

fn now_millis() -> u64 {
    #[cfg(target_arch = "wasm32")]
    {
        return web_sys::window()
            .and_then(|window| window.performance())
            .map(|performance| performance.now() as u64)
            .unwrap_or_default();
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }
}
