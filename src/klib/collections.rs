// SigmaOS Kernel Library Collections
// Eliminates dependency on std::collections

#![allow(dead_code)]

extern crate alloc;
use alloc::collections::BTreeMap as AllocBTreeMap;
use alloc::collections::BTreeSet as AllocBTreeSet;
use alloc::collections::VecDeque as AllocVecDeque;
use core::cell::Cell;

/// Simple BTreeMap implementation for klib
pub struct BTreeMap<K, V> {
    inner: AllocBTreeMap<K, V>,
}

impl<K: Ord, V> BTreeMap<K, V> {
    pub fn new() -> Self {
        Self {
            inner: AllocBTreeMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.inner.insert(key, value)
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.inner.get(key)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.inner.get_mut(key)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.inner.remove(key)
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<K: Ord, V> Default for BTreeMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple HashSet implementation for klib (Using BTreeSet internally)
pub struct HashSet<T> {
    inner: AllocBTreeSet<T>,
}

impl<T: Ord> HashSet<T> {
    pub fn new() -> Self {
        Self {
            inner: AllocBTreeSet::new(),
        }
    }

    pub fn insert(&mut self, value: T) -> bool {
        self.inner.insert(value)
    }

    pub fn contains(&self, value: &T) -> bool {
        self.inner.contains(value)
    }

    pub fn remove(&mut self, value: &T) -> bool {
        self.inner.remove(value)
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<T: Ord> Default for HashSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple VecDeque implementation for klib
pub struct VecDeque<T> {
    inner: AllocVecDeque<T>,
}

impl<T> VecDeque<T> {
    pub fn new() -> Self {
        Self {
            inner: AllocVecDeque::new(),
        }
    }

    pub fn push_front(&mut self, value: T) {
        self.inner.push_front(value)
    }

    pub fn push_back(&mut self, value: T) {
        self.inner.push_back(value)
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.inner.pop_front()
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.inner.pop_back()
    }

    pub fn front(&self) -> Option<&T> {
        self.inner.front()
    }

    pub fn back(&self) -> Option<&T> {
        self.inner.back()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<T> Default for VecDeque<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple atomic types for klib (no_std compatible)
pub struct AtomicBool {
    value: Cell<bool>,
}

impl AtomicBool {
    pub fn new(v: bool) -> Self {
        Self {
            value: Cell::new(v),
        }
    }

    pub fn load(&self) -> bool {
        self.value.get()
    }

    pub fn store(&self, v: bool) {
        self.value.set(v);
    }

    pub fn swap(&self, v: bool) -> bool {
        self.value.replace(v)
    }
}

pub struct AtomicUsize {
    value: Cell<usize>,
}

impl AtomicUsize {
    pub fn new(v: usize) -> Self {
        Self {
            value: Cell::new(v),
        }
    }

    pub fn load(&self) -> usize {
        self.value.get()
    }

    pub fn store(&self, v: usize) {
        self.value.set(v);
    }

    pub fn fetch_add(&self, v: usize) -> usize {
        let old = self.value.get();
        self.value.set(old.wrapping_add(v));
        old
    }

    pub fn fetch_sub(&self, v: usize) -> usize {
        let old = self.value.get();
        self.value.set(old.wrapping_sub(v));
        old
    }
}

pub struct AtomicU64 {
    value: Cell<u64>,
}

impl AtomicU64 {
    pub fn new(v: u64) -> Self {
        Self {
            value: Cell::new(v),
        }
    }

    pub fn load(&self) -> u64 {
        self.value.get()
    }

    pub fn store(&self, v: u64) {
        self.value.set(v);
    }

    pub fn fetch_add(&self, v: u64) -> u64 {
        let old = self.value.get();
        self.value.set(old.wrapping_add(v));
        old
    }
}

/// Memory ordering constants (simplified for no_std)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ordering {
    Relaxed,
    Release,
    Acquire,
    AcqRel,
    SeqCst,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashmap_basic() {
        let mut map = BTreeMap::<u32, u32>::new();
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_hashset_basic() {
        let mut set = HashSet::<u32>::new();
        assert!(set.is_empty());
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn test_vecdeque_basic() {
        let deque = VecDeque::<u32>::new();
        assert!(deque.is_empty());
        assert_eq!(deque.len(), 0);
    }

    #[test]
    fn test_atomic_bool() {
        let atomic = AtomicBool::new(false);
        assert!(!atomic.load());
        atomic.store(true);
        assert!(atomic.load());
    }

    #[test]
    fn test_atomic_usize() {
        let atomic = AtomicUsize::new(0);
        assert_eq!(atomic.load(), 0);
        assert_eq!(atomic.fetch_add(5), 0);
        assert_eq!(atomic.load(), 5);
    }

    #[test]
    fn test_atomic_u64() {
        let atomic = AtomicU64::new(0);
        assert_eq!(atomic.load(), 0);
        assert_eq!(atomic.fetch_add(10), 0);
        assert_eq!(atomic.load(), 10);
    }
}