// SigmaOS Custom Collections Library
// Zero-dependency alternatives to std::collections
// #![no_std] compliant

use core::cell::RefCell;
use core::sync::atomic::{AtomicUsize, Ordering};

/// Simple hash set implementation
pub struct SimpleHashSet<T> {
    buckets: RefCell<Vec<Vec<T>>>,
    capacity: AtomicUsize,
    len: AtomicUsize,
}

impl<T: PartialEq + Clone> SimpleHashSet<T> {
    pub fn new(capacity: usize) -> Self {
        let mut buckets = Vec::new();
        for _ in 0..capacity {
            buckets.push(Vec::new());
        }
        
        SimpleHashSet {
            buckets: RefCell::new(buckets),
            capacity: AtomicUsize::new(capacity),
            len: AtomicUsize::new(0),
        }
    }

    pub fn insert(&self, item: T) -> bool {
        let hash = self.simple_hash(&item);
        let capacity = self.capacity.load(Ordering::SeqCst);
        let bucket_idx = (hash % capacity) as usize;
        
        let mut buckets = self.buckets.borrow_mut();
        let bucket = &mut buckets[bucket_idx];
        
        if bucket.contains(&item) {
            return false;
        }
        
        bucket.push(item);
        self.len.fetch_add(1, Ordering::SeqCst);
        true
    }

    pub fn contains(&self, item: &T) -> bool {
        let hash = self.simple_hash(item);
        let capacity = self.capacity.load(Ordering::SeqCst);
        let bucket_idx = (hash % capacity) as usize;
        
        let buckets = self.buckets.borrow();
        buckets[bucket_idx].contains(item)
    }

    pub fn remove(&self, item: &T) -> bool {
        let hash = self.simple_hash(item);
        let capacity = self.capacity.load(Ordering::SeqCst);
        let bucket_idx = (hash % capacity) as usize;
        
        let mut buckets = self.buckets.borrow_mut();
        let bucket = &mut buckets[bucket_idx];
        
        if let Some(pos) = bucket.iter().position(|x| x == item) {
            bucket.remove(pos);
            self.len.fetch_sub(1, Ordering::SeqCst);
            true
        } else {
            false
        }
    }

    pub fn len(&self) -> usize {
        self.len.load(Ordering::SeqCst)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn simple_hash(&self, item: &T) -> usize {
        // Simple hash based on memory address and value
        let ptr = item as *const T as usize;
        let mut hash: usize = 5381;
        hash = hash.wrapping_mul(33).wrapping_add(ptr);
        hash
    }
}

/// Simple binary heap (priority queue) implementation
pub struct SimpleBinaryHeap<T> {
    data: RefCell<Vec<T>>,
    len: AtomicUsize,
}

impl<T: Ord + Clone> SimpleBinaryHeap<T> {
    pub fn new() -> Self {
        SimpleBinaryHeap {
            data: RefCell::new(Vec::new()),
            len: AtomicUsize::new(0),
        }
    }

    pub fn push(&self, item: T) {
        let mut data = self.data.borrow_mut();
        data.push(item);
        let idx = data.len() - 1;
        self.sift_up(&mut data, idx);
        self.len.store(data.len(), Ordering::SeqCst);
    }

    pub fn pop(&self) -> Option<T> {
        let mut data = self.data.borrow_mut();
        if data.is_empty() {
            return None;
        }
        
        let len = data.len();
        data.swap(0, len - 1);
        let item = data.pop();
        self.sift_down(&mut data, 0);
        self.len.store(data.len(), Ordering::SeqCst);
        item
    }

    pub fn peek(&self) -> Option<T> {
        let data = self.data.borrow();
        data.first().cloned()
    }

    pub fn len(&self) -> usize {
        self.len.load(Ordering::SeqCst)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn sift_up(&self, data: &mut Vec<T>, mut idx: usize) {
        while idx > 0 {
            let parent = (idx - 1) / 2;
            if data[idx] >= data[parent] {
                break;
            }
            data.swap(idx, parent);
            idx = parent;
        }
    }

    fn sift_down(&self, data: &mut Vec<T>, mut idx: usize) {
        let len = data.len();
        loop {
            let left = 2 * idx + 1;
            let right = 2 * idx + 2;
            let mut smallest = idx;

            if left < len && data[left] < data[smallest] {
                smallest = left;
            }
            if right < len && data[right] < data[smallest] {
                smallest = right;
            }

            if smallest == idx {
                break;
            }

            data.swap(idx, smallest);
            idx = smallest;
        }
    }
}

/// Simple BTree-like ordered set
pub struct SimpleOrderedSet<T> {
    data: RefCell<Vec<T>>,
    len: AtomicUsize,
}

impl<T: Ord + Clone> SimpleOrderedSet<T> {
    pub fn new() -> Self {
        SimpleOrderedSet {
            data: RefCell::new(Vec::new()),
            len: AtomicUsize::new(0),
        }
    }

    pub fn insert(&self, item: T) -> bool {
        let mut data = self.data.borrow_mut();
        
        // Binary search for insertion point
        let mut left = 0;
        let mut right = data.len();
        
        while left < right {
            let mid = left + (right - left) / 2;
            if data[mid] < item {
                left = mid + 1;
            } else if data[mid] > item {
                right = mid;
            } else {
                return false; // Already exists
            }
        }
        
        data.insert(left, item);
        self.len.store(data.len(), Ordering::SeqCst);
        true
    }

    pub fn contains(&self, item: &T) -> bool {
        let data = self.data.borrow();
        
        let mut left = 0;
        let mut right = data.len();
        
        while left < right {
            let mid = left + (right - left) / 2;
            if data[mid] < *item {
                left = mid + 1;
            } else if data[mid] > *item {
                right = mid;
            } else {
                return true;
            }
        }
        
        false
    }

    pub fn remove(&self, item: &T) -> bool {
        let mut data = self.data.borrow_mut();
        
        let mut left = 0;
        let mut right = data.len();
        
        while left < right {
            let mid = left + (right - left) / 2;
            if data[mid] < *item {
                left = mid + 1;
            } else if data[mid] > *item {
                right = mid;
            } else {
                data.remove(mid);
                self.len.store(data.len(), Ordering::SeqCst);
                return true;
            }
        }
        
        false
    }

    pub fn len(&self) -> usize {
        self.len.load(Ordering::SeqCst)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> Vec<T> {
        self.data.borrow().clone()
    }
}

/// Simple deque (double-ended queue) implementation
pub struct SimpleDeque<T> {
    front: RefCell<Vec<T>>,
    back: RefCell<Vec<T>>,
    len: AtomicUsize,
}

impl<T: Clone> SimpleDeque<T> {
    pub fn new() -> Self {
        SimpleDeque {
            front: RefCell::new(Vec::new()),
            back: RefCell::new(Vec::new()),
            len: AtomicUsize::new(0),
        }
    }

    pub fn push_front(&self, item: T) {
        let mut front = self.front.borrow_mut();
        front.push(item);
        self.len.fetch_add(1, Ordering::SeqCst);
    }

    pub fn push_back(&self, item: T) {
        let mut back = self.back.borrow_mut();
        back.push(item);
        self.len.fetch_add(1, Ordering::SeqCst);
    }

    pub fn pop_front(&self) -> Option<T> {
        let mut front = self.front.borrow_mut();
        if let Some(item) = front.pop() {
            self.len.fetch_sub(1, Ordering::SeqCst);
            return Some(item);
        }
        
        // If front is empty, try back (reversed)
        let mut back = self.back.borrow_mut();
        if back.is_empty() {
            return None;
        }
        
        // Move all elements from back to front
        let moved: Vec<T> = back.drain(..).rev().collect();
        front.extend(moved);
        
        if let Some(item) = front.pop() {
            self.len.fetch_sub(1, Ordering::SeqCst);
            Some(item)
        } else {
            None
        }
    }

    pub fn pop_back(&self) -> Option<T> {
        let mut back = self.back.borrow_mut();
        if let Some(item) = back.pop() {
            self.len.fetch_sub(1, Ordering::SeqCst);
            return Some(item);
        }
        
        // If back is empty, try front (reversed)
        let mut front = self.front.borrow_mut();
        if front.is_empty() {
            return None;
        }
        
        // Move all elements from front to back
        let moved: Vec<T> = front.drain(..).rev().collect();
        back.extend(moved);
        
        if let Some(item) = back.pop() {
            self.len.fetch_sub(1, Ordering::SeqCst);
            Some(item)
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.len.load(Ordering::SeqCst)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_set() {
        let set = SimpleHashSet::new(16);
        assert!(set.insert(42));
        assert!(set.contains(&42));
        assert!(!set.insert(42)); // Duplicate
        assert!(set.remove(&42));
        assert!(!set.contains(&42));
    }

    #[test]
    fn test_binary_heap() {
        let heap = SimpleBinaryHeap::new();
        heap.push(5);
        heap.push(2);
        heap.push(8);
        heap.push(1);
        
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(8));
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_ordered_set() {
        let set = SimpleOrderedSet::new();
        set.insert(5);
        set.insert(2);
        set.insert(8);
        set.insert(1);
        
        assert!(set.contains(&5));
        assert!(set.contains(&2));
        assert!(!set.contains(&10));
        
        let items = set.iter();
        assert_eq!(items, vec![1, 2, 5, 8]);
    }

    #[test]
    fn test_deque() {
        let deque = SimpleDeque::new();
        deque.push_back(1);
        deque.push_back(2);
        deque.push_front(0);
        
        assert_eq!(deque.pop_front(), Some(0));
        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_back(), Some(2));
        assert_eq!(deque.pop_back(), None);
    }
}
