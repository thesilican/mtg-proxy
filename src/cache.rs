use anyhow::{anyhow, Result};
use log::info;
use std::{
    collections::HashMap,
    ops::Add,
    sync::{Arc, Mutex, MutexGuard},
    time::{Duration, Instant},
};
use uuid::Uuid;

struct CacheEntry {
    data: Vec<u8>,
    expires: Instant,
}

/// A general purpose binary cache with UUID keys and TTL cache eviction.
#[derive(Clone)]
pub struct Cache {
    cache: Arc<Mutex<HashMap<Uuid, CacheEntry>>>,
}

impl Cache {
    pub fn new() -> Self {
        Cache {
            cache: Default::default(),
        }
    }
    fn lock(&self) -> Result<MutexGuard<'_, HashMap<Uuid, CacheEntry>>> {
        match self.cache.lock() {
            Ok(x) => Ok(x),
            Err(err) => Err(anyhow!("Encountered error locking mutex: {err}")),
        }
    }
    pub fn get(&self, key: Uuid) -> Result<Option<Vec<u8>>> {
        let lock = self.lock()?;
        match lock.get(&key) {
            Some(val) => Ok(Some(val.data.clone())),
            None => Ok(None),
        }
    }
    pub fn insert(&self, key: Uuid, val: &[u8], ttl: Duration) -> Result<()> {
        let mut lock = self.lock()?;
        if lock.contains_key(&key) {
            info!("Cache: Dropping key entry {key}");
        }
        let val = CacheEntry {
            data: val.to_vec(),
            expires: Instant::now().add(ttl),
        };
        lock.insert(key, val);
        Ok(())
    }
    pub fn prune(&self) -> Result<()> {
        let mut lock = self.lock()?;
        let now = Instant::now();
        let before = lock.len();
        lock.retain(|_, val| val.expires > now);
        let pruned = before - lock.len();
        if pruned > 0 {
            info!("Cache: Pruned {pruned} cache entries");
        }
        Ok(())
    }
}
