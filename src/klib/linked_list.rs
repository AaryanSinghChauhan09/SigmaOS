// SigmaOS klib: Intrusive Linked List
// Inspired by Linux kernel's intrusive list implementation (include/linux/list.h)
// and BSD's TAILQ/LIST macros from sys/queue.h
// No external dependencies - fully custom implementation.

#![allow(dead_code)]

extern crate alloc;
use alloc::boxed::Box;
use core::marker::PhantomData;
use core::ptr::NonNull;

/// A doubly-linked list node.
/// Inspired by Linux's `struct list_head` and BSD's `TAILQ_ENTRY`.
pub struct ListNode<T> {
    pub value: T,
    next: Option<NonNull<ListNode<T>>>,
    prev: Option<NonNull<ListNode<T>>>,
}

/// A doubly-linked list with O(1) push/pop at both ends.
/// Fully custom implementation with no stdlib dependency.
pub struct LinkedList<T> {
    head: Option<NonNull<ListNode<T>>>,
    tail: Option<NonNull<ListNode<T>>>,
    len: usize,
    _marker: PhantomData<Box<ListNode<T>>>,
}

// SAFETY: LinkedList owns its nodes exclusively
unsafe impl<T: Send> Send for LinkedList<T> {}
unsafe impl<T: Sync> Sync for LinkedList<T> {}

impl<T> LinkedList<T> {
    /// Create a new empty linked list.
    pub const fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
            _marker: PhantomData,
        }
    }

    /// Returns the number of elements in the list.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Push an element to the front (like BSD TAILQ_INSERT_HEAD / Linux list_add).
    pub fn push_front(&mut self, value: T) {
        let node = Box::new(ListNode {
            value,
            next: self.head,
            prev: None,
        });
        let node_ptr = unsafe { NonNull::new_unchecked(Box::into_raw(node)) };

        if let Some(mut old_head) = self.head {
            unsafe {
                old_head.as_mut().prev = Some(node_ptr);
            }
        } else {
            // List was empty; tail = this new node
            self.tail = Some(node_ptr);
        }
        self.head = Some(node_ptr);
        self.len += 1;
    }

    /// Push an element to the back (like BSD TAILQ_INSERT_TAIL / Linux list_add_tail).
    pub fn push_back(&mut self, value: T) {
        let node = Box::new(ListNode {
            value,
            next: None,
            prev: self.tail,
        });
        let node_ptr = unsafe { NonNull::new_unchecked(Box::into_raw(node)) };

        if let Some(mut old_tail) = self.tail {
            unsafe {
                old_tail.as_mut().next = Some(node_ptr);
            }
        } else {
            // List was empty
            self.head = Some(node_ptr);
        }
        self.tail = Some(node_ptr);
        self.len += 1;
    }

    /// Pop an element from the front.
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.map(|head_ptr| {
            // SAFETY: head_ptr was allocated by us via Box::into_raw
            let head = unsafe { Box::from_raw(head_ptr.as_ptr()) };
            self.head = head.next;
            if let Some(mut new_head) = self.head {
                unsafe {
                    new_head.as_mut().prev = None;
                }
            } else {
                self.tail = None;
            }
            self.len -= 1;
            head.value
        })
    }

    /// Pop an element from the back.
    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.map(|tail_ptr| {
            let tail = unsafe { Box::from_raw(tail_ptr.as_ptr()) };
            self.tail = tail.prev;
            if let Some(mut new_tail) = self.tail {
                unsafe {
                    new_tail.as_mut().next = None;
                }
            } else {
                self.head = None;
            }
            self.len -= 1;
            tail.value
        })
    }

    /// Peek at the front element.
    pub fn front(&self) -> Option<&T> {
        self.head.map(|ptr| unsafe { &ptr.as_ref().value })
    }

    /// Peek at the back element.
    pub fn back(&self) -> Option<&T> {
        self.tail.map(|ptr| unsafe { &ptr.as_ref().value })
    }

    /// Iterate over elements from front to back.
    pub fn iter(&self) -> LinkedListIter<T> {
        LinkedListIter {
            current: self.head,
            _marker: PhantomData,
        }
    }

    /// Clear all elements.
    pub fn clear(&mut self) {
        while self.pop_front().is_some() {}
    }

    /// Find and remove the first element matching a predicate.
    /// Inspired by BSD's `TAILQ_REMOVE`.
    pub fn remove_first<F>(&mut self, pred: F) -> Option<T>
    where
        F: Fn(&T) -> bool,
    {
        let mut current = self.head;
        while let Some(mut cur_ptr) = current {
            let node = unsafe { cur_ptr.as_mut() };
            if pred(&node.value) {
                // Unlink this node
                if let Some(mut prev_ptr) = node.prev {
                    unsafe {
                        prev_ptr.as_mut().next = node.next;
                    }
                } else {
                    self.head = node.next;
                }
                if let Some(mut next_ptr) = node.next {
                    unsafe {
                        next_ptr.as_mut().prev = node.prev;
                    }
                } else {
                    self.tail = node.prev;
                }
                self.len -= 1;
                let boxed = unsafe { Box::from_raw(cur_ptr.as_ptr()) };
                return Some(boxed.value);
            }
            current = node.next;
        }
        None
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

/// Iterator for LinkedList
pub struct LinkedListIter<'a, T> {
    current: Option<NonNull<ListNode<T>>>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|ptr| {
            let node = unsafe { ptr.as_ref() };
            self.current = node.next;
            &node.value
        })
    }
}

impl<T: core::fmt::Debug> core::fmt::Debug for LinkedList<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut list = f.debug_list();
        for item in self.iter() {
            list.entry(item);
        }
        list.finish()
    }
}

/// A singly-linked list optimized for stack-like usage.
/// Inspired by BSD's `SLIST_*` macros and Linux's `hlist`.
pub struct SList<T> {
    head: Option<NonNull<SListNode<T>>>,
    len: usize,
}

struct SListNode<T> {
    value: T,
    next: Option<NonNull<SListNode<T>>>,
}

unsafe impl<T: Send> Send for SList<T> {}

impl<T> SList<T> {
    pub const fn new() -> Self {
        Self { head: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn push(&mut self, value: T) {
        let node = Box::new(SListNode {
            value,
            next: self.head,
        });
        let ptr = unsafe { NonNull::new_unchecked(Box::into_raw(node)) };
        self.head = Some(ptr);
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.map(|ptr| {
            let node = unsafe { Box::from_raw(ptr.as_ptr()) };
            self.head = node.next;
            self.len -= 1;
            node.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.map(|ptr| unsafe { &ptr.as_ref().value })
    }
}

impl<T> Drop for SList<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doubly_linked_list() {
        let mut list: LinkedList<i32> = LinkedList::new();
        assert!(list.is_empty());

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.len(), 3);
        assert_eq!(list.front(), Some(&1));
        assert_eq!(list.back(), Some(&3));

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(2));
        assert!(list.is_empty());
    }

    #[test]
    fn test_push_front() {
        let mut list: LinkedList<u32> = LinkedList::new();
        list.push_front(3);
        list.push_front(2);
        list.push_front(1);
        let collected: alloc::vec::Vec<&u32> = list.iter().collect();
        assert_eq!(collected, [&1, &2, &3]);
    }

    #[test]
    fn test_remove_first() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_back(10);
        list.push_back(20);
        list.push_back(30);
        let removed = list.remove_first(|&x| x == 20);
        assert_eq!(removed, Some(20));
        assert_eq!(list.len(), 2);
        let collected: alloc::vec::Vec<&i32> = list.iter().collect();
        assert_eq!(collected, [&10, &30]);
    }

    #[test]
    fn test_slist_stack() {
        let mut stack: SList<i32> = SList::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }
}
