//! Custom BTreeMap implementation for SigmaOS
//! Reduces dependency on alloc::collections::BTreeMap

extern crate alloc;
use crate::klib::hash::SimpleHasher;
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::hash::{Hash, Hasher};

pub type HashMap<K, V> = BTreeMap<K, V>;

pub struct BTreeMap<K, V> {
    buckets: Vec<Option<Vec<(K, V)>>>,
    capacity: usize,
    len: usize,
}

impl<K: Eq + Hash + Clone, V: PartialEq + Clone> PartialEq for BTreeMap<K, V> {
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len {
            return false;
        }
        for bucket in &self.buckets {
            if let Some(entries) = bucket {
                for (k, v) in entries {
                    if let Some(other_v) = other.get(k) {
                        if v != other_v {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
            }
        }
        true
    }
}

impl<K: Eq + Hash + Clone, V: Eq + Clone> Eq for BTreeMap<K, V> {}

pub enum Entry<'a, K, V> {
    Occupied(OccupiedEntry<'a, K, V>),
    Vacant(VacantEntry<'a, K, V>),
}

pub struct OccupiedEntry<'a, K, V> {
    value: &'a mut V,
    _marker: core::marker::PhantomData<K>,
}

pub struct VacantEntry<'a, K, V> {
    map: &'a mut BTreeMap<K, V>,
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

    pub fn or_default(self) -> &'a mut V
    where
        V: Default,
    {
        self.or_insert_with(Default::default)
    }
}

impl<K, V> BTreeMap<K, V>
where
    K: Eq + Hash,
{
    pub fn new() -> Self {
        BTreeMap {
            buckets: Vec::new(),
            capacity: 0,
            len: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let mut map = BTreeMap::new();
        map.capacity = capacity.next_power_of_two();
        map.resize_buckets();
        map
    }

    fn resize_buckets(&mut self) {
        // Optimized by Bolt ⚡: pre-allocate with_capacity to eliminate O(log N) vector resizing overhead.
        self.buckets = Vec::with_capacity(self.capacity);
        for _ in 0..self.capacity {
            self.buckets.push(None);
        }
    }

    fn hash_key<Q: ?Sized + Hash>(&self, key: &Q) -> usize {
        if self.capacity == 0 {
            return 0;
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

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        if self.capacity == 0 || self.buckets.is_empty() {
            return None;
        }
        let hash = self.hash_key(key);
        if let Some(ref bucket) = self.buckets[hash] {
            for item in bucket.iter() {
                if item.0.borrow() == key {
                    return Some(&item.1);
                }
            }
        }
        None
    }

    // Convenience method for String keys with &str lookups
    pub fn get_str(&self, key: &str) -> Option<&V>
    where
        K: AsRef<str>,
    {
        if self.capacity == 0 || self.buckets.is_empty() {
            return None;
        }
        let hash = self.hash_key(&key);
        if let Some(ref bucket) = self.buckets[hash] {
            for item in bucket.iter() {
                if item.0.as_ref() == key {
                    return Some(&item.1);
                }
            }
        }
        None
    }

    // Convenience method for String keys with &str contains_key
    pub fn contains_key_str(&self, key: &str) -> bool
    where
        K: AsRef<str>,
    {
        self.get_str(key).is_some()
    }

    // Convenience method for String keys with &str get_mut
    pub fn get_mut_str(&mut self, key: &str) -> Option<&mut V>
    where
        K: AsRef<str>,
    {
        if self.capacity == 0 || self.buckets.is_empty() {
            return None;
        }
        let hash = self.hash_key(&key);
        if let Some(ref mut bucket) = self.buckets[hash] {
            for item in bucket.iter_mut() {
                if item.0.as_ref() == key {
                    return Some(&mut item.1);
                }
            }
        }
        None
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        if self.capacity == 0 || self.buckets.is_empty() {
            return None;
        }
        let hash = self.hash_key(key);
        if let Some(ref mut bucket) = self.buckets[hash] {
            for item in bucket.iter_mut() {
                if item.0.borrow() == key {
                    return Some(&mut item.1);
                }
            }
        }
        None
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        if self.capacity == 0 || self.buckets.is_empty() {
            return None;
        }
        let hash = self.hash_key(key);
        if let Some(ref mut bucket) = self.buckets[hash] {
            for i in 0..bucket.len() {
                if bucket[i].0.borrow() == key {
                    let (_, value) = bucket.remove(i);
                    self.len -= 1;
                    return Some(value);
                }
            }
        }
        None
    }

    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
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

    pub fn iter(&self) -> BTreeMapIter<'_, K, V> {
        BTreeMapIter {
            map: self,
            bucket_idx: 0,
            item_idx: 0,
        }
    }

    pub fn iter_mut(&mut self) -> BTreeMapIterMut<'_, K, V> {
        BTreeMapIterMut {
            map: self,
            bucket_idx: 0,
            item_idx: 0,
        }
    }

    pub fn entry(&mut self, key: K) -> Entry<'_, K, V> {
        if self.capacity == 0 || self.buckets.is_empty() {
            if self.capacity == 0 {
                self.capacity = 16;
            }
            self.resize_buckets();
        }
        let hash = self.hash_key(&key);
        if let Some(ref mut bucket) = self.buckets[hash] {
            for i in 0..bucket.len() {
                if bucket[i].0 == key {
                    let val_ptr = &mut bucket[i].1 as *mut V;
                    // SAFETY: val_ptr is a valid pointer to the value in the bucket
                    return Entry::Occupied(OccupiedEntry {
                        value: unsafe { &mut *val_ptr },
                        _marker: core::marker::PhantomData,
                    });
                }
            }
        }
        Entry::Vacant(VacantEntry { map: self, key })
    }

    pub fn keys(&self) -> Keys<'_, K, V> {
        Keys { iter: self.iter() }
    }

    pub fn values(&self) -> Values<'_, K, V> {
        Values { iter: self.iter() }
    }

    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
        ValuesMut {
            iter: self.iter_mut(),
            _marker: core::marker::PhantomData,
        }
    }

    #[allow(unused_mut)]
    fn grow(&mut self) {
        let mut old_buckets = core::mem::take(&mut self.buckets);
        self.capacity *= 2;
        self.resize_buckets();
        self.len = 0;

        while let Some(opt_bucket) = old_buckets.pop() {
            if let Some(mut bucket) = opt_bucket {
                while let Some((key, value)) = bucket.pop() {
                    self.insert(key, value);
                }
            }
        }
    }
}

impl<K, V> Default for BTreeMap<K, V>
where
    K: Eq + Hash,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> Clone for BTreeMap<K, V>
where
    K: Clone,
    V: Clone,
{
    fn clone(&self) -> Self {
        Self {
            buckets: self.buckets.clone(),
            capacity: self.capacity,
            len: self.len,
        }
    }
}

impl<K, V> core::fmt::Debug for BTreeMap<K, V>
where
    K: Eq + Hash + core::fmt::Debug,
    V: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut map = f.debug_map();
        for (k, v) in self.iter() {
            map.entry(k, v);
        }
        map.finish()
    }
}

pub struct BTreeMapIter<'a, K, V> {
    map: &'a BTreeMap<K, V>,
    bucket_idx: usize,
    item_idx: usize,
}

impl<'a, K, V> Iterator for BTreeMapIter<'a, K, V> {
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

pub struct BTreeMapIterMut<'a, K, V> {
    map: &'a mut BTreeMap<K, V>,
    bucket_idx: usize,
    item_idx: usize,
}

impl<'a, K, V> Iterator for BTreeMapIterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.map.capacity == 0 || self.map.buckets.is_empty() {
            return None;
        }
        while self.bucket_idx < self.map.capacity {
            if let Some(ref mut bucket) = self.map.buckets[self.bucket_idx] {
                if self.item_idx < bucket.len() {
                    unsafe {
                        // SAFETY: item_idx < bucket.len() ensures valid pointer
                        let item_ptr = &mut bucket[self.item_idx] as *mut (K, V);
                        self.item_idx += 1;
                        return Some((&(*item_ptr).0, &mut (*item_ptr).1));
                    }
                }
            }
            self.bucket_idx += 1;
            self.item_idx = 0;
        }
        None
    }
}

impl<'a, K, V> IntoIterator for &'a BTreeMap<K, V>
where
    K: Eq + Hash,
{
    type Item = (&'a K, &'a V);
    type IntoIter = BTreeMapIter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Keys<'a, K, V> {
    iter: BTreeMapIter<'a, K, V>,
}

impl<'a, K, V> Iterator for Keys<'a, K, V> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(k, _)| k)
    }
}

pub struct Values<'a, K, V> {
    iter: BTreeMapIter<'a, K, V>,
}

impl<'a, K, V> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(_, v)| v)
    }
}

pub type HashMapValues<'a, K, V> = Values<'a, K, V>;
pub type HashMapValuesMut<'a, K, V> = ValuesMut<'a, K, V>;

pub struct ValuesMut<'a, K, V> {
    iter: BTreeMapIterMut<'a, K, V>,
    _marker: core::marker::PhantomData<K>,
}

impl<'a, K, V> Iterator for ValuesMut<'a, K, V> {
    type Item = &'a mut V;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(_, v)| v)
    }
}

impl<K, V, const N: usize> From<[(K, V); N]> for BTreeMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    fn from(arr: [(K, V); N]) -> Self {
        let mut map = BTreeMap::with_capacity(N);
        for (k, v) in arr {
            map.insert(k, v);
        }
        map
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashmap_basic() {
        let mut map = BTreeMap::new();
        map.insert("key1", "value1");
        map.insert("key2", "value2");

        assert_eq!(map.get(&"key1"), Some(&"value1"));
        assert_eq!(map.get(&"key2"), Some(&"value2"));
        assert_eq!(map.get(&"key3"), None);
    }

    #[test]
    fn test_hashmap_remove() {
        let mut map = BTreeMap::new();
        map.insert("key1", "value1");
        assert_eq!(map.remove(&"key1"), Some("value1"));
        assert_eq!(map.get(&"key1"), None);
    }

    #[test]
    fn test_hashmap_iter() {
        let mut map = BTreeMap::new();
        map.insert("key1", "value1");
        map.insert("key2", "value2");

        let mut count = 0;
        for (_key, _value) in map.iter() {
            count += 1;
        }
        assert_eq!(count, 2);
    }
}
