// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaCache - System Cache Daemon
//!
//! SigmaCache provides system cache daemon functionality for SigmaOS, inspired by memcached/redis from Linux.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// Cache entry state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheEntryState {
    /// Active
    Active,
    /// Expired
    Expired,
    /// Evicted
    Evicted,
}

impl fmt::Display for CacheEntryState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CacheEntryState::Active => write!(f, "active"),
            CacheEntryState::Expired => write!(f, "expired"),
            CacheEntryState::Evicted => write!(f, "evicted"),
        }
    }
}

/// Cache entry
#[derive(Debug, Clone)]
pub struct CacheEntry {
    /// Key
    pub key: String,
    /// Value
    pub value: String,
    /// TTL in seconds
    pub ttl: u64,
    /// State
    pub state: CacheEntryState,
}

impl CacheEntry {
    /// Create a new cache entry
    pub fn new(key: String, value: String, ttl: u64) -> Self {
        Self {
            key,
            value,
            ttl,
            state: CacheEntryState::Active,
        }
    }

    /// Set state
    pub fn with_state(mut self, state: CacheEntryState) -> Self {
        self.state = state;
        self
    }
}

/// SigmaCache - System Cache Daemon
pub struct SigmaCache {
    /// Cache entries (key -> entry)
    entries: BTreeMap<String, CacheEntry>,
    /// Running state
    running: bool,
    /// Entry counter
    entry_counter: u32,
}

impl SigmaCache {
    /// Create a new SigmaCache instance
    pub fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
            running: false,
            entry_counter: 0,
        }
    }

    /// Start the cache daemon
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the cache daemon
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the daemon is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Set a cache entry
    pub fn set(&mut self, entry: CacheEntry) -> Result<(), String> {
        if !self.running {
            return Err("SigmaCache not running".to_string());
        }

        let key = entry.key.clone();
        self.entries.insert(key.clone(), entry);
        self.entry_counter += 1;
        Ok(())
    }

    /// Get a cache entry
    pub fn get(&self, key: &str) -> Option<&CacheEntry> {
        self.entries.get(key)
    }

    /// Delete a cache entry
    pub fn delete(&mut self, key: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaCache not running".to_string());
        }

        if self.entries.remove(key).is_none() {
            return Err(format!("Key '{}' not found", key));
        }
        Ok(())
    }

    /// Expire a cache entry
    pub fn expire(&mut self, key: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaCache not running".to_string());
        }

        match self.entries.get_mut(key) {
            Some(entry) => {
                entry.state = CacheEntryState::Expired;
                Ok(())
            }
            None => Err(format!("Key '{}' not found", key)),
        }
    }

    /// Evict a cache entry
    pub fn evict(&mut self, key: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaCache not running".to_string());
        }

        match self.entries.get_mut(key) {
            Some(entry) => {
                entry.state = CacheEntryState::Evicted;
                Ok(())
            }
            None => Err(format!("Key '{}' not found", key)),
        }
    }

    /// List all entries
    pub fn list_entries(&self) -> Vec<&CacheEntry> {
        self.entries.values().collect()
    }

    /// List entries by state
    pub fn list_by_state(&self, state: CacheEntryState) -> Vec<&CacheEntry> {
        self.entries.values().filter(|e| e.state == state).collect()
    }

    /// Get entry count
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    /// Clear all entries
    pub fn clear(&mut self) -> Result<(), String> {
        if !self.running {
            return Err("SigmaCache not running".to_string());
        }

        self.entries.clear();
        Ok(())
    }

    /// Get cache statistics
    pub fn cache_stats(&self) -> BTreeMap<String, String> {
        let mut stats = BTreeMap::new();
        stats.insert("total_entries".to_string(), self.entry_count().to_string());
        stats.insert(
            "active_entries".to_string(),
            self.list_by_state(CacheEntryState::Active)
                .len()
                .to_string(),
        );
        stats.insert(
            "expired_entries".to_string(),
            self.list_by_state(CacheEntryState::Expired)
                .len()
                .to_string(),
        );
        stats.insert(
            "evicted_entries".to_string(),
            self.list_by_state(CacheEntryState::Evicted)
                .len()
                .to_string(),
        );
        stats.insert("running".to_string(), self.running.to_string());
        stats
    }
}

impl Default for SigmaCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaCache instance (placeholder for actual global state)
static mut SIGMACACHE_INSTANCE: Option<SigmaCache> = None;

/// Initialize the global SigmaCache instance
pub fn init_sigmacache() {
    unsafe {
        SIGMACACHE_INSTANCE = Some(SigmaCache::new());
    }
}

/// Get the global SigmaCache instance
pub fn get_sigmacache() -> Option<&'static mut SigmaCache> {
    unsafe { SIGMACACHE_INSTANCE.as_mut() }
}

/// Convenience function to set a cache entry
pub fn set_cache_entry(entry: CacheEntry) -> Result<(), String> {
    if let Some(cache) = get_sigmacache() {
        cache.set(entry)
    } else {
        Err("SigmaCache not initialized".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_entry_state_display() {
        assert_eq!(CacheEntryState::Active.to_string(), "active");
        assert_eq!(CacheEntryState::Expired.to_string(), "expired");
    }

    #[test]
    fn test_cache_entry() {
        let entry = CacheEntry::new("key1".to_string(), "value1".to_string(), 3600)
            .with_state(CacheEntryState::Expired);

        assert_eq!(entry.key, "key1");
        assert_eq!(entry.value, "value1");
        assert_eq!(entry.state, CacheEntryState::Expired);
    }

    #[test]
    fn test_sigmacache() {
        let mut cache = SigmaCache::new();
        assert!(!cache.is_running());
        cache.start();
        assert!(cache.is_running());
        cache.stop();
        assert!(!cache.is_running());
    }

    #[test]
    fn test_sigmacache_set() {
        let mut cache = SigmaCache::new();
        cache.start();

        let entry = CacheEntry::new("key1".to_string(), "value1".to_string(), 3600);

        assert!(cache.set(entry).is_ok());
        assert_eq!(cache.entry_count(), 1);
    }

    #[test]
    fn test_sigmacache_get() {
        let mut cache = SigmaCache::new();
        cache.start();

        let entry = CacheEntry::new("key1".to_string(), "value1".to_string(), 3600);

        cache.set(entry).unwrap();
        let result = cache.get("key1");

        assert!(result.is_some());
        assert_eq!(result.unwrap().value, "value1");
    }

    #[test]
    fn test_sigmacache_delete() {
        let mut cache = SigmaCache::new();
        cache.start();

        let entry = CacheEntry::new("key1".to_string(), "value1".to_string(), 3600);

        cache.set(entry).unwrap();
        assert!(cache.delete("key1").is_ok());
        assert_eq!(cache.entry_count(), 0);
    }

    #[test]
    fn test_sigmacache_expire() {
        let mut cache = SigmaCache::new();
        cache.start();

        let entry = CacheEntry::new("key1".to_string(), "value1".to_string(), 3600);

        cache.set(entry).unwrap();
        assert!(cache.expire("key1").is_ok());

        let entry = cache.get("key1").unwrap();
        assert_eq!(entry.state, CacheEntryState::Expired);
    }

    #[test]
    fn test_sigmacache_evict() {
        let mut cache = SigmaCache::new();
        cache.start();

        let entry = CacheEntry::new("key1".to_string(), "value1".to_string(), 3600);

        cache.set(entry).unwrap();
        assert!(cache.evict("key1").is_ok());

        let entry = cache.get("key1").unwrap();
        assert_eq!(entry.state, CacheEntryState::Evicted);
    }

    #[test]
    fn test_sigmacache_list_by_state() {
        let mut cache = SigmaCache::new();
        cache.start();

        let entry = CacheEntry::new("key1".to_string(), "value1".to_string(), 3600)
            .with_state(CacheEntryState::Expired);

        cache.set(entry).unwrap();

        let entries = cache.list_by_state(CacheEntryState::Expired);
        assert_eq!(entries.len(), 1);
    }

    #[test]
    fn test_sigmacache_clear() {
        let mut cache = SigmaCache::new();
        cache.start();

        let entry = CacheEntry::new("key1".to_string(), "value1".to_string(), 3600);

        cache.set(entry).unwrap();
        assert!(cache.clear().is_ok());
        assert_eq!(cache.entry_count(), 0);
    }

    #[test]
    fn test_sigmacache_stats() {
        let mut cache = SigmaCache::new();
        cache.start();

        let entry = CacheEntry::new("key1".to_string(), "value1".to_string(), 3600);

        cache.set(entry).unwrap();

        let stats = cache.cache_stats();
        assert_eq!(stats.get("total_entries"), Some(&"1".to_string()));
        assert_eq!(stats.get("active_entries"), Some(&"1".to_string()));
    }

    #[test]
    fn test_set_cache_entry_convenience() {
        init_sigmacache();

        let mut cache = get_sigmacache().unwrap();
        cache.start();

        let entry = CacheEntry::new("key1".to_string(), "value1".to_string(), 3600);

        let result = set_cache_entry(entry);
        assert!(result.is_ok());
    }
}
