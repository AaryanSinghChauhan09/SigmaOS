// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// Custom hash map to replace std::collections::HashMap
// Zero-allocation, performance-optimized hash table

/// Custom hash map with fixed capacity
/// Replaces std::collections::HashMap with zero-allocation alternative
pub struct SigmaHashMap<K, V, const N: usize> {
    buckets: [Option<(K, V)>; N],
    count: usize,
}

impl<K, V, const N: usize> SigmaHashMap<K, V, N>
where
    K: Copy + Eq,
    V: Copy,
{
    pub const fn new() -> Self {
        Self {
            buckets: [None; N],
            count: 0,
        }
    }

    /// Simple hash function using XOR folding
    fn hash(&self, key: K) -> usize {
        // This is a simple hash - in production, use a better hash function
        // For now, we'll use a basic approach that works for Copy types
        let bytes = unsafe {
            core::slice::from_raw_parts(&key as *const K as *const u8, core::mem::size_of::<K>())
        };
        
        let mut hash: usize = 0;
        for (i, &byte) in bytes.iter().enumerate() {
            hash ^= (byte as usize) << (i % 8);
        }
        hash % N
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.count >= N {
            return None; // Hash map full
        }

        let hash = self.hash(key);
        let mut index = hash;

        // Linear probing for collision resolution
        for _ in 0..N {
            if let Some((existing_key, existing_value)) = self.buckets[index] {
                if existing_key == key {
                    // Key exists, update value
                    let old = self.buckets[index];
                    self.buckets[index] = Some((key, value));
                    return old.map(|(_, v)| v);
                }
            } else {
                // Found empty slot
                self.buckets[index] = Some((key, value));
                self.count += 1;
                return None;
            }
            index = (index + 1) % N;
        }

        None // Hash map full (shouldn't reach here if count < N)
    }

    pub fn get(&self, key: K) -> Option<V> {
        let hash = self.hash(key);
        let mut index = hash;

        for _ in 0..N {
            if let Some((existing_key, existing_value)) = self.buckets[index] {
                if existing_key == key {
                    return Some(existing_value);
                }
            }
            index = (index + 1) % N;
        }

        None
    }

    pub fn remove(&mut self, key: K) -> Option<V> {
        let hash = self.hash(key);
        let mut index = hash;

        for _ in 0..N {
            if let Some((existing_key, existing_value)) = self.buckets[index] {
                if existing_key == key {
                    let old = self.buckets[index];
                    self.buckets[index] = None;
                    self.count -= 1;
                    return old.map(|(_, v)| v);
                }
            }
            index = (index + 1) % N;
        }

        None
    }

    pub fn contains_key(&self, key: K) -> bool {
        self.get(key).is_some()
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn capacity(&self) -> usize {
        N
    }

    pub fn clear(&mut self) {
        self.buckets = [None; N];
        self.count = 0;
    }

    pub fn iter(&self) -> SigmaHashMapIter<K, V, N> {
        SigmaHashMapIter {
            map: self,
            index: 0,
        }
    }
}

pub struct SigmaHashMapIter<'a, K, V, const N: usize> {
    map: &'a SigmaHashMap<K, V, N>,
    index: usize,
}

impl<'a, K, V, const N: usize> Iterator for SigmaHashMapIter<'a, K, V, N>
where
    K: Copy,
    V: Copy,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < N {
            let entry = self.map.buckets[self.index];
            self.index += 1;
            if let Some((k, v)) = entry {
                return Some((k, v));
            }
        }
        None
    }
}

/// Hash set built on top of SigmaHashMap
pub struct SigmaHashSet<T, const N: usize> {
    map: SigmaHashMap<T, (), N>,
}

impl<T, const N: usize> SigmaHashSet<T, N>
where
    T: Copy + Eq,
{
    pub const fn new() -> Self {
        Self {
            map: SigmaHashMap::new(),
        }
    }

    pub fn insert(&mut self, item: T) -> bool {
        self.map.insert(item, ()).is_none()
    }

    pub fn contains(&self, item: T) -> bool {
        self.map.contains_key(item)
    }

    pub fn remove(&mut self, item: T) -> bool {
        self.map.remove(item).is_some()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = T> + '_ {
        self.map.iter().map(|(k, _)| k)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashmap_insert_get() {
        let mut map: SigmaHashMap<u32, u32, 16> = SigmaHashMap::new();
        assert!(map.insert(1, 100).is_none());
        assert_eq!(map.get(1), Some(100));
    }

    #[test]
    fn test_hashmap_update() {
        let mut map: SigmaHashMap<u32, u32, 16> = SigmaHashMap::new();
        map.insert(1, 100);
        assert_eq!(map.insert(1, 200), Some(100));
        assert_eq!(map.get(1), Some(200));
    }

    #[test]
    fn test_hashmap_remove() {
        let mut map: SigmaHashMap<u32, u32, 16> = SigmaHashMap::new();
        map.insert(1, 100);
        assert_eq!(map.remove(1), Some(100));
        assert_eq!(map.get(1), None);
    }

    #[test]
    fn test_hashmap_collision() {
        let mut map: SigmaHashMap<u32, u32, 16> = SigmaHashMap::new();
        // Insert multiple items that might collide
        for i in 0..10 {
            map.insert(i, i * 10);
        }
        for i in 0..10 {
            assert_eq!(map.get(i), Some(i * 10));
        }
    }

    #[test]
    fn test_hashset() {
        let mut set: SigmaHashSet<u32, 16> = SigmaHashSet::new();
        assert!(set.insert(1));
        assert!(!set.insert(1)); // Duplicate
        assert!(set.contains(1));
        assert!(set.remove(1));
        assert!(!set.contains(1));
    }
}
