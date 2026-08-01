//! Custom HashMap implementation for SigmaOS
//! Reduces dependency on std::collections::HashMap

use crate::klib::Vec;

pub struct HashMap<K, V> {
    buckets: Vec<Option<Vec<(K, V)>>>,
    capacity: usize,
    len: usize,
}

impl<K, V> HashMap<K, V>
where
    K: PartialEq + Clone,
    V: Clone,
{
    pub fn new() -> Self {
        HashMap {
            buckets: Vec::new(),
            capacity: 16,
            len: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let mut map = HashMap::new();
        map.capacity = capacity.next_power_of_two();
        map.resize_buckets();
        map
    }

    fn resize_buckets(&mut self) {
        self.buckets = Vec::new();
        for _ in 0..self.capacity {
            self.buckets.push(None);
        }
    }

    fn hash_key(&self, key: &K) -> usize {
        // Simple hash function - in production use a proper hash
        let mut hash: usize = 0;
        let key_bytes = unsafe {
            core::slice::from_raw_parts(
                key as *const K as *const u8,
                core::mem::size_of::<K>(),
            )
        };
        for (i, &byte) in key_bytes.iter().enumerate() {
            hash = hash.wrapping_add((byte as usize) * (i + 1));
        }
        hash % self.capacity
    }

    pub fn insert(&mut self, key: K, value: V) {
        if self.len >= self.capacity * 2 {
            self.grow();
        }

        let hash = self.hash_key(&key);
        if let Some(ref mut bucket) = self.buckets[hash] {
            for item in bucket.iter_mut() {
                if item.0 == key {
                    item.1 = value;
                    return;
                }
            }
            bucket.push((key, value));
        } else {
            let mut bucket = Vec::new();
            bucket.push((key, value));
            self.buckets[hash] = Some(bucket);
        }
        self.len += 1;
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let hash = self.hash_key(key);
        if let Some(ref bucket) = self.buckets[hash] {
            for item in bucket.iter() {
                if item.0 == *key {
                    return Some(&item.1);
                }
            }
        }
        None
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let hash = self.hash_key(key);
        if let Some(ref mut bucket) = self.buckets[hash] {
            for item in bucket.iter_mut() {
                if item.0 == *key {
                    return Some(&mut item.1);
                }
            }
        }
        None
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let hash = self.hash_key(key);
        if let Some(ref mut bucket) = self.buckets[hash] {
            for i in 0..bucket.len() {
                if bucket[i].0 == *key {
                    let (_, value) = bucket.remove(i);
                    self.len -= 1;
                    return Some(value);
                }
            }
        }
        None
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn iter(&self) -> HashMapIter<'_, K, V> {
        HashMapIter {
            map: self,
            bucket_idx: 0,
            item_idx: 0,
        }
    }

    fn grow(&mut self) {
        let old_buckets = core::mem::replace(&mut self.buckets, Vec::new());
        let old_capacity = self.capacity;
        
        self.capacity *= 2;
        self.resize_buckets();
        self.len = 0;

        for bucket in old_buckets.into_iter().flatten() {
            for (key, value) in bucket {
                self.insert(key, value);
            }
        }
    }
}

impl<K, V> Default for HashMap<K, V>
where
    K: PartialEq + Clone,
    V: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

pub struct HashMapIter<'a, K, V> {
    map: &'a HashMap<K, V>,
    bucket_idx: usize,
    item_idx: usize,
}

impl<'a, K, V> Iterator for HashMapIter<'a, K, V>
where
    K: PartialEq + Clone,
    V: Clone,
{
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        while self.bucket_idx < self.map.capacity {
            if let Some(ref bucket) = self.map.buckets[self.bucket_idx] {
                if self.item_idx < bucket.len() {
                    let item = (&bucket[self.item_idx].0, &bucket[self.item_idx].1);
                    self.item_idx += 1;
                    return Some(item);
                }
            }
            self.bucket_idx += 1;
            self.item_idx = 0;
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashmap_basic() {
        let mut map = HashMap::new();
        map.insert("key1", "value1");
        map.insert("key2", "value2");
        
        assert_eq!(map.get(&"key1"), Some(&"value1"));
        assert_eq!(map.get(&"key2"), Some(&"value2"));
        assert_eq!(map.get(&"key3"), None);
    }

    #[test]
    fn test_hashmap_remove() {
        let mut map = HashMap::new();
        map.insert("key1", "value1");
        assert_eq!(map.remove(&"key1"), Some("value1"));
        assert_eq!(map.get(&"key1"), None);
    }

    #[test]
    fn test_hashmap_iter() {
        let mut map = HashMap::new();
        map.insert("key1", "value1");
        map.insert("key2", "value2");
        
        let mut count = 0;
        for (_key, _value) in map.iter() {
            count += 1;
        }
        assert_eq!(count, 2);
    }
}