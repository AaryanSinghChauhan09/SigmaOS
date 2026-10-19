// SPDX-License-Identifier: MIT
// SigmaOS - Preload Application Preloading
// Inspired by Linux preload (systemd/udev preload)
// Provides application preloading for faster launch times

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};

/// Preload entry
#[derive(Debug)]
pub struct PreloadEntry {
    pub name: String,
    pub path: String,
    pub priority: u32,
    pub loaded: AtomicBool,
    pub last_loaded: AtomicU64, // timestamp
    pub load_count: AtomicU32,
}

impl PreloadEntry {
    pub fn new(name: String, path: String, priority: u32) -> Self {
        Self {
            name,
            path,
            priority,
            loaded: AtomicBool::new(false),
            last_loaded: AtomicU64::new(0),
            load_count: AtomicU32::new(0),
        }
    }

    /// Mark as loaded
    pub fn mark_loaded(&self) {
        self.loaded.store(true, Ordering::Relaxed);
        self.load_count.fetch_add(1, Ordering::Relaxed);
        // In real implementation, would set actual timestamp
        self.last_loaded.store(0, Ordering::Relaxed);
    }

    /// Mark as unloaded
    pub fn mark_unloaded(&self) {
        self.loaded.store(false, Ordering::Relaxed);
    }

    /// Is loaded
    pub fn is_loaded(&self) -> bool {
        self.loaded.load(Ordering::Relaxed)
    }

    /// Get load count
    pub fn get_load_count(&self) -> u32 {
        self.load_count.load(Ordering::Relaxed)
    }
}

/// Preload manager
pub struct PreloadManager {
    pub entries: BTreeMap<String, PreloadEntry>,
    pub enabled: AtomicBool,
    pub max_memory: u64,           // bytes
    pub current_memory: AtomicU64, // bytes
}

impl PreloadManager {
    pub fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
            enabled: AtomicBool::new(true),
            max_memory: 512 * 1024 * 1024, // 512MB default
            current_memory: AtomicU64::new(0),
        }
    }

    /// Add preload entry
    pub fn add_entry(&mut self, entry: PreloadEntry) {
        self.entries.insert(entry.name.clone(), entry);
    }

    /// Get entry
    pub fn get_entry(&self, name: &str) -> Option<&PreloadEntry> {
        self.entries.get(name)
    }

    /// Get mutable entry
    pub fn get_entry_mut(&mut self, name: &str) -> Option<&mut PreloadEntry> {
        self.entries.get_mut(name)
    }

    /// Remove entry
    pub fn remove_entry(&mut self, name: &str) -> Result<(), &'static str> {
        self.entries.remove(name).ok_or("Entry not found")?;
        Ok(())
    }

    /// Load entry
    pub fn load_entry(&mut self, name: &str) -> Result<(), &'static str> {
        if !self.enabled.load(Ordering::Relaxed) {
            return Err("Preload disabled");
        }

        let entry = self.entries.get_mut(name).ok_or("Entry not found")?;

        // Check memory limit
        let current = self.current_memory.load(Ordering::Relaxed);
        if current >= self.max_memory {
            return Err("Memory limit reached");
        }

        // In real implementation, would:
        // 1. Read binary file
        // 2. Load into memory
        // 3. Keep resident pages

        entry.mark_loaded();

        // Simulate memory usage (estimate)
        let estimated_size = 10 * 1024 * 1024; // 10MB estimate
        self.current_memory
            .fetch_add(estimated_size, Ordering::Relaxed);

        Ok(())
    }

    /// Unload entry
    pub fn unload_entry(&mut self, name: &str) -> Result<(), &'static str> {
        let entry = self.entries.get_mut(name).ok_or("Entry not found")?;

        entry.mark_unloaded();

        // Simulate memory free
        let estimated_size = 10 * 1024 * 1024; // 10MB estimate
        self.current_memory
            .fetch_sub(estimated_size, Ordering::Relaxed);

        Ok(())
    }

    /// Load high priority entries
    pub fn load_high_priority(&mut self) -> Result<u32, &'static str> {
        if !self.enabled.load(Ordering::Relaxed) {
            return Err("Preload disabled");
        }

        let mut loaded = 0;
        let mut high_priority_names: Vec<(String, u32)> = self
            .entries
            .values()
            .filter(|e| !e.is_loaded() && e.priority >= 80)
            .map(|e| (e.name.clone(), e.priority))
            .collect();

        // Sort by priority (higher priority first)
        high_priority_names.sort_by(|a, b| b.1.cmp(&a.1));

        for (name, _) in high_priority_names {
            self.load_entry(&name)?;
            loaded += 1;
        }

        Ok(loaded)
    }

    /// Unload all entries
    pub fn unload_all(&mut self) {
        for entry in self.entries.values() {
            entry.mark_unloaded();
        }
        self.current_memory.store(0, Ordering::Relaxed);
    }

    /// Enable preload
    pub fn enable(&self) {
        self.enabled.store(true, Ordering::Relaxed);
    }

    /// Disable preload
    pub fn disable(&self) {
        self.enabled.store(false, Ordering::Relaxed);
    }

    /// Set max memory
    pub fn set_max_memory(&mut self, max: u64) {
        self.max_memory = max;
    }

    /// Get current memory usage
    pub fn get_current_memory(&self) -> u64 {
        self.current_memory.load(Ordering::Relaxed)
    }

    /// Get loaded entries count
    pub fn get_loaded_count(&self) -> u32 {
        self.entries.values().filter(|e| e.is_loaded()).count() as u32
    }

    /// Get total entries count
    pub fn get_total_count(&self) -> u32 {
        self.entries.len() as u32
    }
}

impl Default for PreloadManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global preload manager
static mut PRELOAD_MANAGER: Option<PreloadManager> = None;

/// Initialize preload manager
pub fn init_preload_manager() {
    unsafe {
        PRELOAD_MANAGER = Some(PreloadManager::new());
    }
}

/// Get global preload manager
pub fn get_preload_manager() -> Option<&'static mut PreloadManager> {
    unsafe { PRELOAD_MANAGER.as_mut() }
}

/// Add common applications to preload
pub fn add_common_preloads() {
    if let Some(manager) = get_preload_manager() {
        // Add common applications
        manager.add_entry(PreloadEntry::new(
            "firefox".to_string(),
            "/usr/bin/firefox".to_string(),
            90,
        ));
        manager.add_entry(PreloadEntry::new(
            "chromium".to_string(),
            "/usr/bin/chromium".to_string(),
            90,
        ));
        manager.add_entry(PreloadEntry::new(
            "libreoffice".to_string(),
            "/usr/bin/libreoffice".to_string(),
            80,
        ));
        manager.add_entry(PreloadEntry::new(
            "steam".to_string(),
            "/usr/bin/steam".to_string(),
            85,
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preload_entry_init() {
        let entry = PreloadEntry::new("test".to_string(), "/bin/test".to_string(), 50);
        assert_eq!(entry.name, "test");
        assert_eq!(entry.priority, 50);
        assert!(!entry.is_loaded());
    }

    #[test]
    fn test_mark_loaded() {
        let entry = PreloadEntry::new("test".to_string(), "/bin/test".to_string(), 50);
        entry.mark_loaded();
        assert!(entry.is_loaded());
        assert_eq!(entry.get_load_count(), 1);
    }

    #[test]
    fn test_preload_manager() {
        let mut manager = PreloadManager::new();
        let entry = PreloadEntry::new("test".to_string(), "/bin/test".to_string(), 50);
        manager.add_entry(entry);
        assert!(manager.get_entry("test").is_some());
    }

    #[test]
    fn test_load_entry() {
        let mut manager = PreloadManager::new();
        let entry = PreloadEntry::new("test".to_string(), "/bin/test".to_string(), 50);
        manager.add_entry(entry);
        manager.load_entry("test").unwrap();
        assert!(manager.get_entry("test").unwrap().is_loaded());
    }

    #[test]
    fn test_unload_entry() {
        let mut manager = PreloadManager::new();
        let entry = PreloadEntry::new("test".to_string(), "/bin/test".to_string(), 50);
        manager.add_entry(entry);
        manager.load_entry("test").unwrap();
        manager.unload_entry("test").unwrap();
        assert!(!manager.get_entry("test").unwrap().is_loaded());
    }

    #[test]
    fn test_load_high_priority() {
        let mut manager = PreloadManager::new();
        manager.add_entry(PreloadEntry::new(
            "low".to_string(),
            "/bin/low".to_string(),
            50,
        ));
        manager.add_entry(PreloadEntry::new(
            "high".to_string(),
            "/bin/high".to_string(),
            90,
        ));
        let loaded = manager.load_high_priority().unwrap();
        assert_eq!(loaded, 1);
        assert!(manager.get_entry("high").unwrap().is_loaded());
        assert!(!manager.get_entry("low").unwrap().is_loaded());
    }

    #[test]
    fn test_disable_preload() {
        let mut manager = PreloadManager::new();
        manager.disable();
        let entry = PreloadEntry::new("test".to_string(), "/bin/test".to_string(), 50);
        manager.add_entry(entry);
        assert!(manager.load_entry("test").is_err());
    }

    #[test]
    fn test_memory_limit() {
        let mut manager = PreloadManager::new();
        manager.set_max_memory(15 * 1024 * 1024); // 15MB
        let entry = PreloadEntry::new("test".to_string(), "/bin/test".to_string(), 50);
        manager.add_entry(entry);
        manager.load_entry("test").unwrap();
        manager.load_entry("test").unwrap(); // Should fail on second load
        assert!(manager.load_entry("test").is_err());
    }
}
