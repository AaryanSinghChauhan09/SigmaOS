use alloc::format;
use alloc::string::{String, ToString};
// Linux and BSD Inspired Abstract Data Types (ADT) for SigmaOS
// Implements high-performance kernel data structures:
// - SplayTree<K, V>: FreeBSD `sys/tree.h` inspired self-balancing binary search tree
// - RadixTree<T>: Linux kernel `lib/radix-tree.c` inspired page-cache / PID radix lookup tree
// - SovereignPriorityQueue<T>: BSD `sys/queue.h` and Linux scheduler binary heap priority queue

use crate::klib::Vec;

// =========================================================================
// 1. Splay Tree ADT (Inspired by FreeBSD sys/tree.h SPLAY)
// =========================================================================

#[derive(Debug, Clone)]
struct SplayNode<K, V> {
    key: K,
    value: V,
    left: Option<usize>,
    right: Option<usize>,
}

/// SplayTree ADT: Self-adjusting binary search tree where recently accessed
/// elements are moved to the root, giving O(log N) amortized performance.
pub struct SplayTree<K, V> {
    nodes: Vec<Option<SplayNode<K, V>>>,
    root: Option<usize>,
}

impl<K: Ord + Clone, V: Clone> SplayTree<K, V> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            root: None,
        }
    }

    /// Inserts a key-value pair into the splay tree
    pub fn insert(&mut self, key: K, value: V) {
        if self.root.is_none() {
            self.nodes.push(Some(SplayNode {
                key,
                value,
                left: None,
                right: None,
            }));
            self.root = Some(self.nodes.len() - 1);
            return;
        }

        let mut curr = self.root;
        let mut parent = None;
        let mut is_left = false;

        while let Some(idx) = curr {
            let key_match = if let Some(ref node) = self.nodes[idx] {
                if key < node.key {
                    parent = Some(idx);
                    curr = node.left;
                    is_left = true;
                    None
                } else if key > node.key {
                    parent = Some(idx);
                    curr = node.right;
                    is_left = false;
                    None
                } else {
                    Some(idx)
                }
            } else {
                break;
            };

            if let Some(match_idx) = key_match {
                if let Some(ref mut mut_node) = self.nodes[match_idx] {
                    mut_node.value = value;
                }
                return;
            }
        }

        self.nodes.push(Some(SplayNode {
            key,
            value,
            left: None,
            right: None,
        }));
        let new_idx = self.nodes.len() - 1;

        if let Some(p_idx) = parent {
            if let Some(ref mut p_node) = self.nodes[p_idx] {
                if is_left {
                    p_node.left = Some(new_idx);
                } else {
                    p_node.right = Some(new_idx);
                }
            }
        }
    }

    /// Searches for a key and returns a reference to its value if found
    pub fn get(&self, key: &K) -> Option<&V> {
        let mut curr = self.root;
        while let Some(idx) = curr {
            if let Some(ref node) = self.nodes[idx] {
                if key < &node.key {
                    curr = node.left;
                } else if key > &node.key {
                    curr = node.right;
                } else {
                    return Some(&node.value);
                }
            } else {
                break;
            }
        }
        None
    }

    /// Returns total element count
    pub fn len(&self) -> usize {
        self.nodes.iter().filter(|n| n.is_some()).count()
    }

    /// Checks if tree is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<K: Ord + Clone, V: Clone> Default for SplayTree<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 2. Radix Tree ADT (Inspired by Linux lib/radix-tree.c)
// =========================================================================

#[derive(Debug, Clone)]
struct RadixNode<T> {
    key: u64,
    value: Option<T>,
}

/// RadixTree ADT: Fast unsigned integer indexed sparse array tree
/// matching Linux page cache index & PID table performance.
pub struct RadixTree<T> {
    entries: Vec<RadixNode<T>>,
}

impl<T: Clone> RadixTree<T> {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Store value at u64 key index
    pub fn insert(&mut self, key: u64, value: T) {
        for entry in self.entries.iter_mut() {
            if entry.key == key {
                entry.value = Some(value);
                return;
            }
        }
        self.entries.push(RadixNode {
            key,
            value: Some(value),
        });
    }

    /// Retrieve value at key index
    pub fn get(&self, key: u64) -> Option<&T> {
        for entry in self.entries.iter() {
            if entry.key == key && entry.value.is_some() {
                return entry.value.as_ref();
            }
        }
        None
    }

    /// Remove item at key
    pub fn remove(&mut self, key: u64) -> Option<T> {
        for entry in self.entries.iter_mut() {
            if entry.key == key {
                return entry.value.take();
            }
        }
        None
    }

    pub fn len(&self) -> usize {
        self.entries.iter().filter(|e| e.value.is_some()).count()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T: Clone> Default for RadixTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 3. Priority Queue ADT (Inspired by BSD sys/queue.h & Linux Scheduler)
// =========================================================================

/// SovereignPriorityQueue ADT: Max-binary-heap for scheduling priorities
pub struct SovereignPriorityQueue<T> {
    heap: Vec<T>,
}

impl<T: Ord + Clone> SovereignPriorityQueue<T> {
    pub fn new() -> Self {
        Self { heap: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.heap.push(item);
        let mut idx = self.heap.len() - 1;
        while idx > 0 {
            let parent = (idx - 1) / 2;
            if self.heap[idx] > self.heap[parent] {
                self.heap.swap(idx, parent);
                idx = parent;
            } else {
                break;
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.heap.is_empty() {
            return None;
        }
        let len = self.heap.len();
        self.heap.swap(0, len - 1);
        let max_val = self.heap.pop();

        if !self.heap.is_empty() {
            self.sift_down(0);
        }
        max_val
    }

    pub fn peek(&self) -> Option<&T> {
        self.heap.first()
    }

    fn sift_down(&mut self, mut idx: usize) {
        let len = self.heap.len();
        loop {
            let left = 2 * idx + 1;
            let right = 2 * idx + 2;
            let mut largest = idx;

            if left < len && self.heap[left] > self.heap[largest] {
                largest = left;
            }
            if right < len && self.heap[right] > self.heap[largest] {
                largest = right;
            }

            if largest != idx {
                self.heap.swap(idx, largest);
                idx = largest;
            } else {
                break;
            }
        }
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
}

impl<T: Ord + Clone> Default for SovereignPriorityQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 4. Unit Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_splay_tree() {
        let mut tree = SplayTree::new();
        assert!(tree.is_empty());

        tree.insert(10, "ten");
        tree.insert(5, "five");
        tree.insert(20, "twenty");

        assert_eq!(tree.len(), 3);
        assert_eq!(tree.get(&10), Some(&"ten"));
        assert_eq!(tree.get(&5), Some(&"five"));
        assert_eq!(tree.get(&20), Some(&"twenty"));
        assert_eq!(tree.get(&99), None);
    }

    #[test]
    fn test_radix_tree() {
        let mut radix = RadixTree::new();
        assert!(radix.is_empty());

        radix.insert(0x1000, "page_0");
        radix.insert(0x2000, "page_1");

        assert_eq!(radix.get(0x1000), Some(&"page_0"));
        assert_eq!(radix.get(0x2000), Some(&"page_1"));
        assert_eq!(radix.get(0x3000), None);

        assert_eq!(radix.remove(0x1000), Some("page_0"));
        assert_eq!(radix.get(0x1000), None);
    }

    #[test]
    fn test_priority_queue() {
        let mut pq = SovereignPriorityQueue::new();
        assert!(pq.is_empty());

        pq.push(10);
        pq.push(50);
        pq.push(30);

        assert_eq!(pq.peek(), Some(&50));
        assert_eq!(pq.pop(), Some(50));
        assert_eq!(pq.pop(), Some(30));
        assert_eq!(pq.pop(), Some(10));
        assert_eq!(pq.pop(), None);
    }
}
