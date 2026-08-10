// SigmaOS Kernel Library
pub mod vec;
pub mod buddy_allocator;
pub mod paging;
pub mod uvm;

pub use vec::Vec;

#[cfg(not(target_os = "none"))]
pub use std::collections::{HashMap, HashSet};

#[cfg(target_os = "none")]
#[derive(Debug, Clone)]
pub struct HashMap<K, V> {
    data: crate::klib::Vec<(K, V)>,
}

#[cfg(target_os = "none")]
impl<K: PartialEq, V> HashMap<K, V> {
    pub fn new() -> Self {
        Self { data: crate::klib::Vec::new() }
    }
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let mut old = None;
        if let Some(idx) = self.data.iter().position(|(k, _)| k == &key) {
            old = Some(unsafe { core::ptr::replace(&mut self.data[idx].1, value) });
        } else {
            self.data.push((key, value));
        }
        old
    }
    pub fn get(&self, key: &K) -> Option<&V> {
        self.data.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.data.iter_mut().find(|(k, _)| k == key).map(|(_, v)| v)
    }
    pub fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(idx) = self.data.iter().position(|(k, _)| k == key) {
            Some(self.data.remove(idx).1)
        } else {
            None
        }
    }
    pub fn contains_key(&self, key: &K) -> bool {
        self.data.iter().any(|(k, _)| k == key)
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn clear(&mut self) {
        self.data.clear();
    }
    pub fn iter(&self) -> core::slice::Iter<'_, (K, V)> {
        self.data.iter()
    }
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, (K, V)> {
        self.data.iter_mut()
    }
    pub fn keys(&self) -> KeysIter<'_, K, V> {
        KeysIter { iter: self.data.iter() }
    }
}

#[cfg(target_os = "none")]
impl<K: PartialEq, V> Default for HashMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(target_os = "none")]
pub struct KeysIter<'a, K, V> {
    iter: core::slice::Iter<'a, (K, V)>,
}

#[cfg(target_os = "none")]
impl<'a, K, V> Iterator for KeysIter<'a, K, V> {
    type Item = &'a K;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(k, _)| k)
    }
}

#[cfg(target_os = "none")]
#[derive(Debug, Clone)]
pub struct HashSet<T> {
    data: crate::klib::Vec<T>,
}

#[cfg(target_os = "none")]
impl<T: PartialEq> HashSet<T> {
    pub fn new() -> Self {
        Self { data: crate::klib::Vec::new() }
    }
    pub fn insert(&mut self, value: T) -> bool {
        if self.data.iter().any(|v| v == &value) {
            false
        } else {
            self.data.push(value);
            true
        }
    }
    pub fn contains(&self, value: &T) -> bool {
        self.data.iter().any(|v| v == value)
    }
    pub fn remove(&mut self, value: &T) -> bool {
        if let Some(idx) = self.data.iter().position(|v| v == value) {
            self.data.remove(idx);
            true
        } else {
            false
        }
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn clear(&mut self) {
        self.data.clear();
    }
    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        self.data.iter()
    }
}

#[cfg(target_os = "none")]
impl<T: PartialEq> Default for HashSet<T> {
    fn default() -> Self {
        Self::new()
    }
}
