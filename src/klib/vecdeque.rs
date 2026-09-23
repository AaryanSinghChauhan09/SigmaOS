//! Custom VecDeque implementation for SigmaOS
//! Reduces dependency on std::collections::VecDeque
#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
use super::Vec;

pub struct VecDeque<T> {
    front: Vec<T>,
    back: Vec<T>,
}

impl<T> VecDeque<T> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        VecDeque {
            front: Vec::new(),
            back: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let half = capacity / 2;
        VecDeque {
            front: Vec::with_capacity(half),
            back: Vec::with_capacity(capacity - half),
        }
    }

    pub fn push_front(&mut self, item: T) {
        self.front.push(item);
    }

    pub fn push_back(&mut self, item: T) {
        self.back.push(item);
    }

    /// Optimized by Bolt ⚡: moves owned items directly from `back` to `front`
    /// after a single capacity reservation. Eliminates redundant `.clone()` loops,
    /// $O(\log N)$ intermediate reallocations, and vector buffer drops.
    pub fn pop_front(&mut self) -> Option<T> {
        if !self.front.is_empty() {
            self.front.pop()
        } else if !self.back.is_empty() {
            self.front.reserve(self.back.len());
            while let Some(item) = self.back.pop() {
                self.front.push(item);
            }
            self.front.pop()
        } else {
            None
        }
    }

    /// Optimized by Bolt ⚡: moves owned items directly from `front` to `back`
    /// after a single capacity reservation. Eliminates redundant `.clone()` loops,
    /// $O(\log N)$ intermediate reallocations, and vector buffer drops.
    pub fn pop_back(&mut self) -> Option<T> {
        if !self.back.is_empty() {
            self.back.pop()
        } else if !self.front.is_empty() {
            self.back.reserve(self.front.len());
            while let Some(item) = self.front.pop() {
                self.back.push(item);
            }
            self.back.pop()
        } else {
            None
        }
    }

    pub fn front(&self) -> Option<&T> {
        if !self.front.is_empty() {
            self.front.as_slice().last()
        } else {
            self.back.as_slice().first()
        }
    }

    pub fn back(&self) -> Option<&T> {
        if !self.back.is_empty() {
            self.back.as_slice().last()
        } else {
            self.front.as_slice().first()
        }
    }

    pub fn len(&self) -> usize {
        self.front.len() + self.back.len()
    }

    pub fn is_empty(&self) -> bool {
        self.front.is_empty() && self.back.is_empty()
    }

    pub fn iter(&self) -> VecDequeIter<'_, T> {
        VecDequeIter::<T> {
            deque: self,
            front_idx: 0,
            back_idx: 0,
            phase: IterPhase::Front,
        }
    }
}

impl<T> Default for VecDeque<T> {
    fn default() -> Self {
        Self::new()
    }
}

enum IterPhase {
    Front,
    Back,
    Done,
}

pub struct VecDequeIter<'a, T> {
    deque: &'a VecDeque<T>,
    front_idx: usize,
    back_idx: usize,
    phase: IterPhase,
}

impl<'a, T> Iterator for VecDequeIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.phase {
            IterPhase::Front => {
                if self.front_idx < self.deque.front.len() {
                    let idx = self.deque.front.len() - 1 - self.front_idx;
                    let item = &self.deque.front[idx];
                    self.front_idx += 1;
                    Some(item)
                } else {
                    self.phase = IterPhase::Back;
                    self.next()
                }
            }
            IterPhase::Back => {
                if self.back_idx < self.deque.back.len() {
                    let item = &self.deque.back[self.back_idx];
                    self.back_idx += 1;
                    Some(item)
                } else {
                    self.phase = IterPhase::Done;
                    None
                }
            }
            IterPhase::Done => None,
        }
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_vecdeque_basic() {
        let mut deque: VecDeque<i32> = VecDeque::new();
        deque.push_back(1);
        deque.push_back(2);
        deque.push_front(0);

        assert_eq!(deque.front(), Some(&0));
        assert_eq!(deque.back(), Some(&2));
        assert_eq!(deque.len(), 3);
    }

    #[test]
    fn test_vecdeque_pop() {
        let mut deque: VecDeque<i32> = VecDeque::new();
        deque.push_back(1);
        deque.push_back(2);

        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_front(), Some(2));
        assert_eq!(deque.pop_front(), None);
    }

    #[test]
    fn test_vecdeque_iter() {
        let mut deque: VecDeque<i32> = VecDeque::new();
        deque.push_back(1);
        deque.push_back(2);
        deque.push_front(0);

        let items: std::vec::Vec<i32> = deque.iter().cloned().collect();
        assert_eq!(items, std::vec![0, 1, 2]);
    }
}
