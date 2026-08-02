//! Custom HashMap implementation for SigmaOS
//! Reduces dependency on std::collections::HashMap

use crate::klib::Vec;
use core::borrow::Borrow;

pub struct HashMap<K, V> {
    buckets: Vec<Option<Vec<(K, V)>>>,
    capacity: usize,
    len: usize,
}

struct SimpleHasher {
    state: usize,
}

impl core::hash::Hasher for SimpleHasher {
    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.state = self.state.wrapping_add(byte as usize);
        }
    }

    fn finish(&self) -> u64 {
        self.state as u64
    }
}

impl<K, V> HashMap<K, V>
where
    K: PartialEq + core::hash::Hash,
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

    fn hash_key<Q: ?Sized + core::hash::Hash>(&self, key: &Q) -> usize {
        let mut hasher = SimpleHasher { state: 0 };
        key.hash(&mut hasher);
        (hasher.state) % self.capacity
    }

    pub fn insert(&mut self, key: K, value: V) {
        if self.buckets.is_empty() {
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
        K: Borrow<Q>,
        Q: core::hash::Hash + PartialEq,
    {
        if self.buckets.is_empty() {
            return None;
        }
        let hash = self.hash_key(key);
        if hash < self.buckets.len() {
            if let Some(ref bucket) = self.buckets[hash] {
                for item in bucket.iter() {
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
        K: Borrow<Q>,
        Q: core::hash::Hash + PartialEq,
    {
        if self.buckets.is_empty() {
            return None;
        }
        let hash = self.hash_key(key);
        if hash < self.buckets.len() {
            if let Some(ref mut bucket) = self.buckets[hash] {
                for item in bucket.iter_mut() {
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
        K: Borrow<Q>,
        Q: core::hash::Hash + PartialEq,
    {
        if self.buckets.is_empty() {
            return None;
        }
        let hash = self.hash_key(key);
        if hash < self.buckets.len() {
            if let Some(ref mut bucket) = self.buckets[hash] {
                for i in 0..bucket.len() {
                    if bucket[i].0.borrow() == key {
                        let (_, value) = bucket.remove(i);
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
        K: Borrow<Q>,
        Q: core::hash::Hash + PartialEq,
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

    pub fn iter_mut(&mut self) -> HashMapIterMut<'_, K, V> {
        let self_ptr = self as *mut Self;
        HashMapIterMut {
            map: unsafe { &mut *self_ptr },
            bucket_idx: 0,
            item_idx: 0,
        }
    }

    pub fn values(&self) -> HashMapValues<'_, K, V> {
        HashMapValues { iter: self.iter() }
    }

    pub fn values_mut(&mut self) -> HashMapValuesMut<'_, K, V> {
        HashMapValuesMut { iter: self.iter_mut() }
    }

    pub fn keys(&self) -> HashMapKeys<'_, K, V> {
        HashMapKeys { iter: self.iter() }
    }

    pub fn entry(&mut self, key: K) -> Entry<'_, K, V>
    where
        K: Clone,
    {
        let self_ptr = self as *mut Self;
        if unsafe { (*self_ptr).contains_key(&key) } {
            let val_ref = unsafe { (*self_ptr).get_mut(&key).unwrap() };
            Entry::Occupied(OccupiedEntry::<K, V> {
                value: val_ref,
                _marker: core::marker::PhantomData,
            })
        } else {
            Entry::Vacant(VacantEntry::<K, V> {
                map: unsafe { &mut *self_ptr },
                key,
            })
        }
    }

    fn grow(&mut self) {
        let mut old_buckets = core::mem::replace(&mut self.buckets, Vec::new());
        self.capacity *= 2;
        self.resize_buckets();
        self.len = 0;

        for i in 0..old_buckets.len() {
            if let Some(mut bucket) = old_buckets[i].take() {
                for _ in 0..bucket.len() {
                    if let Some((key, value)) = bucket.pop() {
                        self.insert(key, value);
                    }
                }
            }
        }
    }
}

impl<K, V> Clone for HashMap<K, V>
where
    K: Clone,
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
    K: core::fmt::Debug,
    V: core::fmt::Debug,
    K: PartialEq + core::hash::Hash,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<K, V> Default for HashMap<K, V>
where
    K: PartialEq + core::hash::Hash,
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
    K: PartialEq,
{
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        while self.bucket_idx < self.map.capacity {
            if self.bucket_idx < self.map.buckets.len() {
                if let Some(ref bucket) = self.map.buckets[self.bucket_idx] {
                    if self.item_idx < bucket.len() {
                        let item = (&bucket[self.item_idx].0, &bucket[self.item_idx].1);
                        self.item_idx += 1;
                        return Some(item);
                    }
                }
            }
            self.bucket_idx += 1;
            self.item_idx = 0;
        }
        None
    }
}

impl<'a, K, V> IntoIterator for &'a HashMap<K, V>
where
    K: PartialEq + core::hash::Hash,
{
    type Item = (&'a K, &'a V);
    type IntoIter = HashMapIter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
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
        while self.bucket_idx < self.map.capacity {
            if self.bucket_idx < self.map.buckets.len() {
                if let Some(ref mut bucket) = self.map.buckets[self.bucket_idx] {
                    if self.item_idx < bucket.len() {
                        let item_ptr = &mut bucket[self.item_idx] as *mut (K, V);
                        self.item_idx += 1;
                        unsafe {
                            let item_ref = &mut *item_ptr;
                            return Some((&item_ref.0, &mut item_ref.1));
                        }
                    }
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

pub enum Entry<'a, K, V> {
    Occupied(OccupiedEntry<'a, K, V>),
    Vacant(VacantEntry<'a, K, V>),
}

pub struct OccupiedEntry<'a, K, V> {
    value: &'a mut V,
    _marker: core::marker::PhantomData<&'a K>,
}

pub struct VacantEntry<'a, K, V> {
    map: &'a mut HashMap<K, V>,
    key: K,
}

impl<'a, K, V> Entry<'a, K, V>
where
    K: PartialEq + core::hash::Hash + Clone,
{
    pub fn or_insert_with<F: FnOnce() -> V>(self, default: F) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => entry.value,
            Entry::Vacant(entry) => {
                let key = entry.key;
                entry.map.insert(key.clone(), default());
                entry.map.get_mut(&key).unwrap()
            }
        }
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
