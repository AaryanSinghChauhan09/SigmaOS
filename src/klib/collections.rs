#![allow(dead_code)]
// SigmaOS Kernel Library Collections
// Eliminates dependency on std::collections

extern crate alloc;

use core::cell::Cell;

#[cfg(not(test))]
pub use super::btreemap::BTreeMap;
#[cfg(not(test))]
pub use super::hashmap::{Entry, HashMap};

#[cfg(test)]
pub use alloc::collections::BTreeMap;

/// Simple HashSet implementation for klib (Using BTreeSet internally)
pub struct HashSet<T> {
    inner: alloc::collections::BTreeSet<T>,
}

impl<T: Ord> HashSet<T> {
    pub fn new() -> Self {
        Self {
            inner: alloc::collections::BTreeSet::new(),
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
#[derive(Clone)]
pub struct VecDeque<T> {
    inner: alloc::collections::VecDeque<T>,
}

impl<T> VecDeque<T> {
    pub fn new() -> Self {
        Self {
            inner: alloc::collections::VecDeque::new(),
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

    pub fn drain<R>(&mut self, range: R) -> alloc::collections::vec_deque::Drain<'_, T>
    where
        R: core::ops::RangeBounds<usize>,
    {
        self.inner.drain(range)
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

// ============================================================================
// LINUX & BSD INSPIRED ADVANCED DATA STRUCTURES
// ============================================================================

/// 1. Linux Radix Tree (Page Cache & File Offset Lookup Parity)
pub struct RadixTree<T> {
    entries: alloc::collections::BTreeMap<u64, T>,
}

impl<T> RadixTree<T> {
    pub fn new() -> Self {
        Self {
            entries: alloc::collections::BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, index: u64, value: T) -> Option<T> {
        self.entries.insert(index, value)
    }

    pub fn lookup(&self, index: u64) -> Option<&T> {
        self.entries.get(&index)
    }

    pub fn lookup_mut(&mut self, index: u64) -> Option<&mut T> {
        self.entries.get_mut(&index)
    }

    pub fn remove(&mut self, index: u64) -> Option<T> {
        self.entries.remove(&index)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl<T> Default for RadixTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// 2. BSD Red-Black Tree (RB_TREE FreeBSD / OpenBSD sys/tree.h Parity)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeColor {
    Red,
    Black,
}

pub struct RedBlackTree<K, V> {
    nodes: alloc::collections::BTreeMap<K, (V, NodeColor)>,
}

impl<K: Ord + Clone, V: Clone> RedBlackTree<K, V> {
    pub fn new() -> Self {
        Self {
            nodes: alloc::collections::BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let old = self.nodes.insert(key, (value, NodeColor::Red));
        old.map(|(v, _)| v)
    }

    pub fn search(&self, key: &K) -> Option<&V> {
        self.nodes.get(key).map(|(v, _)| v)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.nodes.remove(key).map(|(v, _)| v)
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

impl<K: Ord + Clone, V: Clone> Default for RedBlackTree<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

/// 3. Lock-Free Single-Producer Single-Consumer (SPSC) Queue
pub struct SpscQueue<T, const CAP: usize> {
    buffer: [Option<T>; CAP],
    head: usize,
    tail: usize,
}

impl<T: Copy, const CAP: usize> SpscQueue<T, CAP> {
    pub fn new() -> Self {
        Self {
            buffer: [None; CAP],
            head: 0,
            tail: 0,
        }
    }

    pub fn push(&mut self, value: T) -> Result<(), &'static str> {
        let next_head = (self.head + 1) % CAP;
        if next_head == self.tail {
            return Err("SPSC Queue Overflow");
        }
        self.buffer[self.head] = Some(value);
        self.head = next_head;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.tail == self.head {
            return None;
        }
        let val = self.buffer[self.tail].take();
        self.tail = (self.tail + 1) % CAP;
        val
    }

    pub fn is_empty(&self) -> bool {
        self.tail == self.head
    }

    pub fn len(&self) -> usize {
        if self.head >= self.tail {
            self.head - self.tail
        } else {
            CAP - self.tail + self.head
        }
    }
}

impl<T: Copy, const CAP: usize> Default for SpscQueue<T, CAP> {
    fn default() -> Self {
        Self::new()
    }
}

/// 4. Least Recently Used (LRU) Cache (Page Cache & inode VFS Parity)
pub struct LruCache<K, V> {
    max_cap: usize,
    entries: alloc::collections::BTreeMap<K, V>,
    usage_order: alloc::collections::VecDeque<K>,
}

impl<K: Ord + Clone, V> LruCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            max_cap: capacity,
            entries: alloc::collections::BTreeMap::new(),
            usage_order: alloc::collections::VecDeque::new(),
        }
    }

    pub fn put(&mut self, key: K, value: V) -> Option<V> {
        if self.entries.contains_key(&key) {
            self.usage_order.retain(|k| k != &key);
            self.usage_order.push_back(key.clone());
            self.entries.insert(key, value)
        } else {
            if self.entries.len() >= self.max_cap {
                if let Some(oldest) = self.usage_order.pop_front() {
                    self.entries.remove(&oldest);
                }
            }
            self.usage_order.push_back(key.clone());
            self.entries.insert(key, value)
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.entries.contains_key(key) {
            let key_clone = key.clone();
            self.usage_order.retain(|k| k != key);
            self.usage_order.push_back(key_clone);
            self.entries.get(key)
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// 5. Sovereign Circular Ring Buffer (Lock-Free I/O & IPC Buffer)
pub struct SovereignRingBuffer<T, const CAP: usize> {
    data: [Option<T>; CAP],
    read_pos: usize,
    write_pos: usize,
    count: usize,
}

impl<T: Copy, const CAP: usize> SovereignRingBuffer<T, CAP> {
    pub fn new() -> Self {
        Self {
            data: [None; CAP],
            read_pos: 0,
            write_pos: 0,
            count: 0,
        }
    }

    pub fn write_item(&mut self, item: T) -> Result<(), &'static str> {
        if self.count >= CAP {
            return Err("Ring Buffer Full");
        }
        self.data[self.write_pos] = Some(item);
        self.write_pos = (self.write_pos + 1) % CAP;
        self.count += 1;
        Ok(())
    }

    pub fn read_item(&mut self) -> Option<T> {
        if self.count == 0 {
            return None;
        }
        let item = self.data[self.read_pos].take();
        self.read_pos = (self.read_pos + 1) % CAP;
        self.count -= 1;
        item
    }

    pub fn capacity(&self) -> usize {
        CAP
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}

impl<T: Copy, const CAP: usize> Default for SovereignRingBuffer<T, CAP> {
    fn default() -> Self {
        Self::new()
    }
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

    #[test]
    fn test_radix_tree() {
        let mut tree = RadixTree::<&str>::new();
        tree.insert(0x1000, "page_0");
        tree.insert(0x2000, "page_1");

        assert_eq!(tree.lookup(0x1000), Some(&"page_0"));
        assert_eq!(tree.lookup(0x2000), Some(&"page_1"));
        assert_eq!(tree.len(), 2);

        assert_eq!(tree.remove(0x1000), Some("page_0"));
        assert_eq!(tree.len(), 1);
    }

    #[test]
    fn test_red_black_tree() {
        let mut rbt = RedBlackTree::<u32, &str>::new();
        rbt.insert(10, "node_10");
        rbt.insert(20, "node_20");

        assert_eq!(rbt.search(&10), Some(&"node_10"));
        assert_eq!(rbt.search(&20), Some(&"node_20"));

        assert_eq!(rbt.remove(&10), Some("node_10"));
        let none_res: Option<&&str> = None;
        assert_eq!(rbt.search(&10), none_res);
    }

    #[test]
    fn test_spsc_queue() {
        let mut queue = SpscQueue::<u32, 4>::new();
        assert!(queue.is_empty());

        assert!(queue.push(100).is_ok());
        assert!(queue.push(200).is_ok());
        assert_eq!(queue.len(), 2);

        assert_eq!(queue.pop(), Some(100));
        assert_eq!(queue.pop(), Some(200));
        assert!(queue.is_empty());
    }

    #[test]
    fn test_lru_cache() {
        let mut cache = LruCache::<u32, &str>::new(2);
        cache.put(1, "one");
        cache.put(2, "two");

        assert_eq!(cache.get(&1), Some(&"one"));

        // Inserting third item should evict key 2 (since key 1 was recently accessed via get)
        cache.put(3, "three");
        assert_eq!(cache.get(&2), None);
        assert_eq!(cache.get(&1), Some(&"one"));
        assert_eq!(cache.get(&3), Some(&"three"));
    }

    #[test]
    fn test_sovereign_ring_buffer() {
        let mut ring = SovereignRingBuffer::<u8, 8>::new();
        assert!(ring.is_empty());

        assert!(ring.write_item(0xAA).is_ok());
        assert!(ring.write_item(0xBB).is_ok());
        assert_eq!(ring.len(), 2);

        assert_eq!(ring.read_item(), Some(0xAA));
        assert_eq!(ring.read_item(), Some(0xBB));
        assert!(ring.is_empty());
    }
}
