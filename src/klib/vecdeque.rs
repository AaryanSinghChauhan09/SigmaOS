//! Custom VecDeque implementation for SigmaOS
//! Reduces dependency on std::collections::VecDeque
#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
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

impl<T> VecDeque<T>
where
    T: Clone,
{
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
            front: Vec::new(),
            back: Vec::new(),
        }
    }

    pub fn push_front(&mut self, item: T) {
        self.front.push(item);
    }

    pub fn push_back(&mut self, item: T) {
        self.back.push(item);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if !self.front.is_empty() {
            self.front.pop()
        } else if !self.back.is_empty() {
            // Move all elements from back to front
            let len = self.back.len();
            for i in 0..len {
                self.front.push(self.back[len - 1 - i].clone());
            }
            self.back = Vec::new();
            self.front.pop()
        } else {
            None
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if !self.back.is_empty() {
            self.back.pop()
        } else if !self.front.is_empty() {
            // Move all elements from front to back
            let len = self.front.len();
            for i in 0..len {
                self.back.push(self.front[len - 1 - i].clone());
            }
            self.front = Vec::new();
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
        VecDequeIter {
            deque: self,
            front_idx: 0,
            back_idx: 0,
            phase: IterPhase::Front,
        }
    }
}

impl<T> Default for VecDeque<T>
where
    T: Clone,
{
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

impl<'a, T> Iterator for VecDequeIter<'a, T>
where
    T: Clone,
{
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

#[cfg(test)]
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
        assert_eq!(items, vec![0, 1, 2]);
    }
}
