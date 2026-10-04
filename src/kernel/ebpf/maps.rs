// eBPF Map Types (Array, Hash, etc.)
// Key-value stores for sharing data between eBPF programs and userland

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};

/// BPF map types (Linux-compatible)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum BpfMapType {
    Array = 2,
    Hash = 1,
    PerfEventArray = 4,
    RingBuf = 27,
}

/// Map operation errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapError {
    KeyNotFound,
    ValueTooLarge,
    KeyTooLarge,
    MapFull,
    TypeMismatch,
    InvalidKey,
    NotSupported,
}

/// BPF Array Map (fixed-size, indexed by u32)
#[derive(Debug)]
pub struct BpfArrayMap {
    data: Vec<u64>,
    max_entries: u32,
}

impl BpfArrayMap {
    /// Create a new array map
    pub fn new(max_entries: u32) -> Self {
        let mut data = Vec::with_capacity(max_entries as usize);
        for _ in 0..max_entries {
            data.push(0);
        }

        Self { data, max_entries }
    }

    /// Lookup value by key (u32 index)
    pub fn lookup(&self, key: u32) -> Option<u64> {
        if key >= self.max_entries {
            return None;
        }

        Some(self.data[key as usize])
    }

    /// Update value at key
    pub fn update(&mut self, key: u32, value: u64) -> Result<(), MapError> {
        if key >= self.max_entries {
            return Err(MapError::InvalidKey);
        }

        self.data[key as usize] = value;
        Ok(())
    }

    /// Delete (reset to 0)
    pub fn delete(&mut self, key: u32) {
        if key < self.max_entries {
            self.data[key as usize] = 0;
        }
    }

    /// Get max entries
    pub fn max_entries(&self) -> u32 {
        self.max_entries
    }
}

/// BPF Hash Map (variable key/value sizes)
#[derive(Debug)]
pub struct BpfHashMap {
    inner: BTreeMap<Vec<u8>, Vec<u8>>,
    max_entries: u32,
    key_size: u32,
    value_size: u32,
}

impl BpfHashMap {
    /// Create a new hash map
    pub fn new(max_entries: u32, key_size: u32, value_size: u32) -> Self {
        Self {
            inner: BTreeMap::new(),
            max_entries,
            key_size,
            value_size,
        }
    }

    /// Lookup value by key
    pub fn lookup(&self, key: &[u8]) -> Option<&[u8]> {
        if key.len() != self.key_size as usize {
            return None;
        }

        self.inner.get(key).map(|v| v.as_slice())
    }

    /// Update or insert key-value pair
    pub fn update(&mut self, key: Vec<u8>, value: Vec<u8>) -> Result<(), MapError> {
        if key.len() != self.key_size as usize {
            return Err(MapError::KeyTooLarge);
        }

        if value.len() != self.value_size as usize {
            return Err(MapError::ValueTooLarge);
        }

        if self.inner.len() >= self.max_entries as usize && !self.inner.contains_key(&key) {
            return Err(MapError::MapFull);
        }

        self.inner.insert(key, value);
        Ok(())
    }

    /// Delete key
    pub fn delete(&mut self, key: &[u8]) -> bool {
        self.inner.remove(key).is_some()
    }

    /// Get current size
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Get max entries
    pub fn max_entries(&self) -> u32 {
        self.max_entries
    }

    /// Get key size
    pub fn key_size(&self) -> u32 {
        self.key_size
    }

    /// Get value size
    pub fn value_size(&self) -> u32 {
        self.value_size
    }
}

/// Generic BPF map enum
#[derive(Debug)]
pub enum BpfMap {
    Array(BpfArrayMap),
    Hash(BpfHashMap),
}

impl BpfMap {
    /// Get map type
    pub fn map_type(&self) -> BpfMapType {
        match self {
            BpfMap::Array(_) => BpfMapType::Array,
            BpfMap::Hash(_) => BpfMapType::Hash,
        }
    }
}

/// Global map table (manages all BPF maps)
pub struct BpfMapTable {
    maps: BTreeMap<u32, BpfMap>,
    next_id: u32,
}

impl BpfMapTable {
    /// Create a new map table
    pub fn new() -> Self {
        Self {
            maps: BTreeMap::new(),
            next_id: 1,
        }
    }

    /// Create an array map and return its FD
    pub fn create_array(&mut self, max_entries: u32) -> u32 {
        let id = self.next_id;
        self.next_id += 1;

        let map = BpfArrayMap::new(max_entries);
        self.maps.insert(id, BpfMap::Array(map));

        id
    }

    /// Create a hash map and return its FD
    pub fn create_hash(&mut self, max_entries: u32, key_size: u32, value_size: u32) -> u32 {
        let id = self.next_id;
        self.next_id += 1;

        let map = BpfHashMap::new(max_entries, key_size, value_size);
        self.maps.insert(id, BpfMap::Hash(map));

        id
    }

    /// Get map by FD (immutable)
    pub fn get(&self, fd: u32) -> Option<&BpfMap> {
        self.maps.get(&fd)
    }

    /// Get map by FD (mutable)
    pub fn get_mut(&mut self, fd: u32) -> Option<&mut BpfMap> {
        self.maps.get_mut(&fd)
    }

    /// Remove map
    pub fn remove(&mut self, fd: u32) -> Option<BpfMap> {
        self.maps.remove(&fd)
    }

    /// Get number of maps
    pub fn len(&self) -> usize {
        self.maps.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.maps.is_empty()
    }
}

impl Default for BpfMapTable {
    fn default() -> Self {
        Self::new()
    }
}

/// Global atomic counter for unique map IDs
pub static GLOBAL_MAP_ID: AtomicU64 = AtomicU64::new(1);

/// Generate a unique map ID
pub fn next_map_id() -> u64 {
    GLOBAL_MAP_ID.fetch_add(1, Ordering::SeqCst)
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_map_creation() {
        let map = BpfArrayMap::new(10);
        assert_eq!(map.max_entries(), 10);
    }

    #[test]
    fn test_array_map_lookup_update() {
        let mut map = BpfArrayMap::new(10);

        // Update
        map.update(5, 42).unwrap();

        // Lookup
        assert_eq!(map.lookup(5), Some(42));
        assert_eq!(map.lookup(0), Some(0));
    }

    #[test]
    fn test_array_map_out_of_bounds() {
        let mut map = BpfArrayMap::new(10);

        // Out of bounds update
        assert_eq!(map.update(10, 42).unwrap_err(), MapError::InvalidKey);

        // Out of bounds lookup
        assert_eq!(map.lookup(10), None);
    }

    #[test]
    fn test_hash_map_creation() {
        let map = BpfHashMap::new(100, 4, 8);
        assert_eq!(map.max_entries(), 100);
        assert_eq!(map.key_size(), 4);
        assert_eq!(map.value_size(), 8);
    }

    #[test]
    fn test_hash_map_insert_lookup() {
        let mut map = BpfHashMap::new(100, 4, 8);

        let key = vec![1, 2, 3, 4];
        let value = vec![10, 20, 30, 40, 50, 60, 70, 80];

        map.update(key.clone(), value.clone()).unwrap();

        let result = map.lookup(&key).unwrap();
        assert_eq!(result, value.as_slice());
    }

    #[test]
    fn test_hash_map_delete() {
        let mut map = BpfHashMap::new(100, 4, 8);

        let key = vec![1, 2, 3, 4];
        let value = vec![10, 20, 30, 40, 50, 60, 70, 80];

        map.update(key.clone(), value).unwrap();
        assert!(map.lookup(&key).is_some());

        assert!(map.delete(&key));
        assert!(map.lookup(&key).is_none());
    }

    #[test]
    fn test_hash_map_size_validation() {
        let mut map = BpfHashMap::new(100, 4, 8);

        // Wrong key size
        let key = vec![1, 2, 3];
        let value = vec![10, 20, 30, 40, 50, 60, 70, 80];
        assert_eq!(
            map.update(key, value.clone()).unwrap_err(),
            MapError::KeyTooLarge
        );

        // Wrong value size
        let key = vec![1, 2, 3, 4];
        let value = vec![10, 20];
        assert_eq!(map.update(key, value).unwrap_err(), MapError::ValueTooLarge);
    }

    #[test]
    fn test_map_table() {
        let mut table = BpfMapTable::new();

        // Create array map
        let fd1 = table.create_array(10);
        assert_eq!(fd1, 1);

        // Create hash map
        let fd2 = table.create_hash(100, 4, 8);
        assert_eq!(fd2, 2);

        // Get maps
        assert!(table.get(fd1).is_some());
        assert!(table.get(fd2).is_some());

        // Check count
        assert_eq!(table.len(), 2);
    }

    #[test]
    fn test_global_map_id() {
        let id1 = next_map_id();
        let id2 = next_map_id();
        assert!(id2 > id1);
    }
}
