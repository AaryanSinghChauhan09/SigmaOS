#![no_std]
extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Cache manager for system cache management
/// Inspired by Linux cache systems and Omarchy cache handling
#[derive(Debug, Clone)]
pub struct SigmaCacheManager2 {
    caches: Vec<Cache>,
}

/// A cache instance
#[derive(Debug, Clone)]
pub struct Cache {
    id: String,
    name: String,
    path: String,
    cache_type: CacheType,
    state: CacheState,
    size_bytes: u64,
    max_size_bytes: u64,
}

/// Type of cache
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheType {
    FileCache,
    MemoryCache,
    DiskCache,
    NetworkCache,
    MetadataCache,
}

/// State of a cache
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheState {
    Active,
    Disabled,
    Corrupted,
    Rebuilding,
}

impl SigmaCacheManager2 {
    pub fn new() -> Self {
        Self {
            caches: Vec::new(),
        }
    }

    /// Create a new cache
    pub fn create_cache(&mut self, id: String, name: String, path: String, cache_type: CacheType, max_size_bytes: u64) -> Result<String, String> {
        if self.caches.iter().any(|c| c.id == id) {
            return Err(format!("Cache with id {} already exists", id));
        }
        
        let cache = Cache {
            id: id.clone(),
            name,
            path,
            cache_type,
            state: CacheState::Active,
            size_bytes: 0,
            max_size_bytes,
        };
        self.caches.push(cache);
        Ok(id)
    }

    /// Remove a cache by ID
    pub fn remove_cache(&mut self, id: &str) -> Result<(), String> {
        let pos = self.caches.iter().position(|c| c.id == id)
            .ok_or_else(|| format!("Cache with id {} not found", id))?;
        self.caches.remove(pos);
        Ok(())
    }

    /// Clear a cache
    pub fn clear_cache(&mut self, id: &str) -> Result<(), String> {
        let cache = self.caches.iter_mut().find(|c| c.id == id)
            .ok_or_else(|| format!("Cache with id {} not found", id))?;
        cache.size_bytes = 0;
        Ok(())
    }

    /// Disable a cache
    pub fn disable_cache(&mut self, id: &str) -> Result<(), String> {
        let cache = self.caches.iter_mut().find(|c| c.id == id)
            .ok_or_else(|| format!("Cache with id {} not found", id))?;
        cache.state = CacheState::Disabled;
        Ok(())
    }

    /// Enable a cache
    pub fn enable_cache(&mut self, id: &str) -> Result<(), String> {
        let cache = self.caches.iter_mut().find(|c| c.id == id)
            .ok_or_else(|| format!("Cache with id {} not found", id))?;
        cache.state = CacheState::Active;
        Ok(())
    }

    /// Set cache size
    pub fn set_cache_size(&mut self, id: &str, size_bytes: u64) -> Result<(), String> {
        let cache = self.caches.iter_mut().find(|c| c.id == id)
            .ok_or_else(|| format!("Cache with id {} not found", id))?;
        cache.size_bytes = size_bytes;
        Ok(())
    }

    /// Set maximum cache size
    pub fn set_max_cache_size(&mut self, id: &str, max_size_bytes: u64) -> Result<(), String> {
        let cache = self.caches.iter_mut().find(|c| c.id == id)
            .ok_or_else(|| format!("Cache with id {} not found", id))?;
        cache.max_size_bytes = max_size_bytes;
        Ok(())
    }

    /// Check if cache needs cleanup
    pub fn needs_cleanup(&self, id: &str) -> Result<bool, String> {
        let cache = self.caches.iter().find(|c| c.id == id)
            .ok_or_else(|| format!("Cache with id {} not found", id))?;
        Ok(cache.size_bytes > cache.max_size_bytes)
    }

    /// Get all caches
    pub fn get_caches(&self) -> &[Cache] {
        &self.caches
    }

    /// Get active caches
    pub fn get_active_caches(&self) -> Vec<&Cache> {
        self.caches.iter().filter(|c| c.state == CacheState::Active).collect()
    }

    /// Get cache by ID
    pub fn get_cache(&self, id: &str) -> Option<&Cache> {
        self.caches.iter().find(|c| c.id == id)
    }

    /// Get caches by type
    pub fn get_caches_by_type(&self, cache_type: CacheType) -> Vec<&Cache> {
        self.caches.iter().filter(|c| c.cache_type == cache_type).collect()
    }

    /// Get total cache size
    pub fn get_total_size(&self) -> u64 {
        self.caches.iter().map(|c| c.size_bytes).sum()
    }
}

impl Default for SigmaCacheManager2 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_cache() {
        let mut manager = SigmaCacheManager2::new();
        let result = manager.create_cache("cache1".to_string(), "File Cache".to_string(), "/var/cache".to_string(), CacheType::FileCache, 1024 * 1024 * 1024);
        assert!(result.is_ok());
        assert_eq!(manager.get_caches().len(), 1);
    }

    #[test]
    fn test_duplicate_cache() {
        let mut manager = SigmaCacheManager2::new();
        manager.create_cache("cache1".to_string(), "File Cache".to_string(), "/var/cache".to_string(), CacheType::FileCache, 1024 * 1024 * 1024).unwrap();
        let result = manager.create_cache("cache1".to_string(), "Other Cache".to_string(), "/tmp/cache".to_string(), CacheType::MemoryCache, 512 * 1024 * 1024);
        assert!(result.is_err());
    }

    #[test]
    fn test_clear_cache() {
        let mut manager = SigmaCacheManager2::new();
        manager.create_cache("cache1".to_string(), "File Cache".to_string(), "/var/cache".to_string(), CacheType::FileCache, 1024 * 1024 * 1024).unwrap();
        manager.set_cache_size("cache1", 500 * 1024 * 1024).unwrap();
        
        manager.clear_cache("cache1").unwrap();
        assert_eq!(manager.get_cache("cache1").unwrap().size_bytes, 0);
    }

    #[test]
    fn test_disable_enable_cache() {
        let mut manager = SigmaCacheManager2::new();
        manager.create_cache("cache1".to_string(), "File Cache".to_string(), "/var/cache".to_string(), CacheType::FileCache, 1024 * 1024 * 1024).unwrap();
        
        manager.disable_cache("cache1").unwrap();
        assert_eq!(manager.get_cache("cache1").unwrap().state, CacheState::Disabled);
        
        manager.enable_cache("cache1").unwrap();
        assert_eq!(manager.get_cache("cache1").unwrap().state, CacheState::Active);
    }

    #[test]
    fn test_needs_cleanup() {
        let mut manager = SigmaCacheManager2::new();
        manager.create_cache("cache1".to_string(), "File Cache".to_string(), "/var/cache".to_string(), CacheType::FileCache, 1024 * 1024 * 1024).unwrap();
        manager.set_cache_size("cache1", 2 * 1024 * 1024 * 1024).unwrap();
        
        assert!(manager.needs_cleanup("cache1").unwrap());
    }
}
