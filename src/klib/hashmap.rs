//! Custom HashMap implementation for SigmaOS
//! Reduces dependency on std::collections::HashMap

use crate::klib::Vec;
use core::borrow::Borrow;
use core::hash::{Hash, Hasher};
use crate::klib::hash::SimpleHasher;

pub struct HashMap<K, V> {
    buckets: Vec<Option<Vec<(K, V)>>>,
    capacity: usize,
    len: usize,
}

pub enum Entry<'a, K, V> {
    Occupied(OccupiedEntry<'a, K, V>),
    Vacant(VacantEntry<'a, K, V>),
}

pub struct OccupiedEntry<'a, K, V> {
    value: &'a mut V,
    _marker: core::marker::PhantomData<K>,
}

pub struct VacantEntry<'a, K, V> {
    map: &'a mut HashMap<K, V>,
    key: K,
}

impl<'a, K, V> Entry<'a, K, V>
where
    K: Eq + Hash + Clone,
{
    pub fn or_insert(self, default: V) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => entry.value,
            Entry::Vacant(entry) => {
                entry.map.insert(entry.key.clone(), default);
                entry.map.get_mut(&entry.key).unwrap()
            }
        }
    }

    pub fn or_insert_with<F>(self, default: F) -> &'a mut V
    where
        F: FnOnce() -> V,
    {
        match self {
            Entry::Occupied(entry) => entry.value,
            Entry::Vacant(entry) => {
                let val = default();
                entry.map.insert(entry.key.clone(), val);
                entry.map.get_mut(&entry.key).unwrap()
            }
        }
    }
}

impl<K, V> HashMap<K, V>
where
    K: Eq + core::hash::Hash,
{
    pub fn new() -> Self {
        let mut map = HashMap {
            buckets: Vec::new(),
            capacity: 0,
            len: 0,
        };
        map.resize_buckets();
        map
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let mut map = HashMap::new();
        if capacity > 0 {
            map.capacity = capacity.next_power_of_two();
            map.resize_buckets();
        }
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
            core::slice::from_raw_parts(key as *const K as *const u8, core::mem::size_of::<K>())
        };
        for (i, &byte) in key_bytes.iter().enumerate() {
            hash = hash.wrapping_add((byte as usize) * (i + 1));
        }
        let mut hasher = SimpleHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.capacity
    }

    pub fn insert(&mut self, key: K, value: V) {
        if self.capacity == 0 || self.buckets.is_empty() {
            if self.capacity == 0 {
                self.capacity = 16;
            }
            self.resize_buckets();
        }
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

    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: core::borrow::Borrow<Q>,
        Q: PartialEq,
    {
        for bucket in self.buckets.iter() {
            if let Some(ref b) = bucket {
                for item in b.iter() {
                    if item.0.borrow() == key {
                        return Some(&item.1);
                    }
                }
            }
        }
        None
    }

    pub fn get_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: core::borrow::Borrow<Q>,
        Q: PartialEq,
    {
        for bucket in self.buckets.iter_mut() {
            if let Some(ref mut b) = bucket {
                for item in b.iter_mut() {
                    if item.0.borrow() == key {
                        return Some(&mut item.1);
                    }
                }
            }
        }
        None
    }

    pub fn remove<Q: ?Sized>(&mut self, key: &Q) -> Option<V>
    where
        K: core::borrow::Borrow<Q>,
        Q: PartialEq,
    {
        for bucket in self.buckets.iter_mut() {
            if let Some(ref mut b) = bucket {
                for i in 0..b.len() {
                    if b[i].0.borrow() == key {
                        let (_, value) = b.remove(i);
                        self.len -= 1;
                        return Some(value);
                    }
                }
            }
        }
        None
    }

    pub fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
    where
        K: core::borrow::Borrow<Q>,
        Q: PartialEq,
    {
        self.get(key).is_some()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn clear(&mut self) {
        for bucket in self.buckets.iter_mut() {
            *bucket = None;
        }
        self.len = 0;
    }

    pub fn iter(&self) -> HashMapIter<'_, K, V> {
        HashMapIter {
            map: self,
            bucket_idx: 0,
            item_idx: 0,
        }
    }

    pub fn values(&self) -> HashMapValues<'_, K, V> {
        HashMapValues { iter: self.iter() }
    }

    pub fn keys(&self) -> HashMapKeys<'_, K, V> {
        HashMapKeys { iter: self.iter() }
    }

    pub fn iter_mut(&mut self) -> HashMapIterMut<'_, K, V> {
        HashMapIterMut {
            map: self,
            bucket_idx: 0,
            item_idx: 0,
        }
    }

    pub fn entry(&mut self, key: K) -> Entry<'_, K, V>
    where
        K: Clone,
    {
        let self_ptr = self as *mut Self;
        if let Some(value) = self.get_mut(&key) {
            Entry::Occupied(OccupiedEntry {
                value,
                _marker: core::marker::PhantomData,
            })
        } else {
            unsafe {
                Entry::Vacant(VacantEntry {
                    map: &mut *self_ptr,
                    key,
                })
            }
        }
    }

    fn grow(&mut self) {
        let mut old_buckets = core::mem::replace(&mut self.buckets, Vec::new());
        self.capacity *= 2;
        self.resize_buckets();
        self.len = 0;

        while !old_buckets.is_empty() {
            if let Some(mut bucket) = old_buckets.pop().unwrap() {
                while !bucket.is_empty() {
                    let (key, value) = bucket.pop().unwrap();
                    self.insert(key, value);
                }
            }
        }
    }
}

impl<K, V> Default for HashMap<K, V>
where
    K: Eq + Hash,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> Clone for HashMap<K, V>
where
    K: PartialEq + Clone,
    V: Clone,
{
    fn clone(&self) -> Self {
        HashMap {
            buckets: self.buckets.clone(),
            capacity: self.capacity,
            len: self.len,
        }
    }
}

impl<K, V> core::fmt::Debug for HashMap<K, V>
where
    K: core::fmt::Debug + Eq + core::hash::Hash,
    V: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut debug_map = f.debug_map();
        for (k, v) in self.iter() {
            debug_map.entry(k, v);
        }
        debug_map.finish()
    }
}

pub struct HashMapIter<'a, K, V> {
    map: &'a HashMap<K, V>,
    bucket_idx: usize,
    item_idx: usize,
}

impl<'a, K, V> Iterator for HashMapIter<'a, K, V>
where
    K: PartialEq,
{
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.map.capacity == 0 || self.map.buckets.is_empty() {
            return None;
        }
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

pub struct HashMapValues<'a, K, V> {
    iter: HashMapIter<'a, K, V>,
}

impl<'a, K, V> Iterator for HashMapValues<'a, K, V>
where
    K: PartialEq,
{
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(_, v)| v)
    }
}

pub struct HashMapKeys<'a, K, V> {
    iter: HashMapIter<'a, K, V>,
}

impl<'a, K, V> Iterator for HashMapKeys<'a, K, V>
where
    K: PartialEq,
{
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(k, _)| k)
    }
}

pub struct HashMapIterMut<'a, K, V> {
    map: &'a mut HashMap<K, V>,
    bucket_idx: usize,
    item_idx: usize,
}

impl<'a, K, V> Iterator for HashMapIterMut<'a, K, V>
where
    K: PartialEq,
{
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        let capacity = self.map.capacity;
        while self.bucket_idx < capacity {
            if let Some(ref mut bucket) = self.map.buckets[self.bucket_idx] {
                if self.item_idx < bucket.len() {
                    let item = &mut bucket[self.item_idx];
                    let k = &item.0 as *const K;
                    let v = &mut item.1 as *mut V;
                    self.item_idx += 1;
                    unsafe {
                        return Some((&*k, &mut *v));
                    }
                }
            }
            self.bucket_idx += 1;
            self.item_idx = 0;
        }
        None
    }
}

pub struct HashMapValuesMut<'a, K, V> {
    iter: HashMapIterMut<'a, K, V>,
}

impl<'a, K, V> Iterator for HashMapValuesMut<'a, K, V>
where
    K: PartialEq,
{
    type Item = &'a mut V;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(_, v)| v)
    }
}

impl<'a, K, V> IntoIterator for &'a HashMap<K, V>
where
    K: Eq + core::hash::Hash,
{
    type Item = (&'a K, &'a V);
    type IntoIter = HashMapIter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
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
