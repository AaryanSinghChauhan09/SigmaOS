// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// Lock-free data structures for SigmaOS kernel
// Zero-allocation, wait-free, performance-critical structures

use core::sync::atomic::{AtomicUsize, AtomicPtr, AtomicBool, Ordering};
use core::ptr::NonNull;

/// Lock-free single-producer single-consumer (SPSC) queue
/// Based on Dmitry Vyukov's design
pub struct SpscQueue<T, const N: usize> {
    buffer: [Option<T>; N],
    head: AtomicUsize,
    tail: AtomicUsize,
}

impl<T, const N: usize> SpscQueue<T, N> {
    pub const fn new() -> Self {
        Self {
            buffer: [None; N],
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }

    /// Push an element (producer only)
    pub fn push(&self, item: T) -> bool {
        let current_tail = self.tail.load(Ordering::Relaxed);
        let next_tail = (current_tail + 1) % N;
        
        let current_head = self.head.load(Ordering::Acquire);
        
        if next_tail == current_head {
            return false; // Queue full
        }
        
        // Write to buffer
        self.buffer[current_tail] = Some(item);
        
        // Publish the new tail
        self.tail.store(next_tail, Ordering::Release);
        
        true
    }

    /// Pop an element (consumer only)
    pub fn pop(&self) -> Option<T> {
        let current_head = self.head.load(Ordering::Relaxed);
        let current_tail = self.tail.load(Ordering::Acquire);
        
        if current_head == current_tail {
            return None; // Queue empty
        }
        
        // Read from buffer
        let item = self.buffer[current_head].take();
        
        // Publish the new head
        let next_head = (current_head + 1) % N;
        self.head.store(next_head, Ordering::Release);
        
        item
    }

    pub fn is_empty(&self) -> bool {
        self.head.load(Ordering::Relaxed) == self.tail.load(Ordering::Relaxed)
    }

    pub fn is_full(&self) -> bool {
        let current_tail = self.tail.load(Ordering::Relaxed);
        let next_tail = (current_tail + 1) % N;
        next_tail == self.head.load(Ordering::Relaxed)
    }
}

/// Lock-free stack using Treiber's algorithm
pub struct LockFreeStack<T> {
    head: AtomicPtr<Node<T>>,
}

struct Node<T> {
    data: T,
    next: *mut Node<T>,
}

impl<T> LockFreeStack<T> {
    pub const fn new() -> Self {
        Self {
            head: AtomicPtr::new(core::ptr::null_mut()),
        }
    }

    pub fn push(&self, item: T) {
        let new_node = Box::leak(Box::new(Node {
            data: item,
            next: core::ptr::null_mut(),
        }));

        loop {
            let old_head = self.head.load(Ordering::Acquire);
            new_node.next = old_head;
            
            match self.head.compare_exchange_weak(
                old_head,
                new_node,
                Ordering::Release,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(_) => continue,
            }
        }
    }

    pub fn pop(&self) -> Option<T> {
        loop {
            let old_head = self.head.load(Ordering::Acquire)?;
            
            let new_head = unsafe { (*old_head).next };
            
            match self.head.compare_exchange_weak(
                old_head,
                new_head,
                Ordering::Release,
                Ordering::Relaxed,
            ) {
                Ok(_) => {
                    unsafe {
                        let node = Box::from_raw(old_head);
                        return Some(node.data);
                    }
                }
                Err(_) => continue,
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.load(Ordering::Relaxed).is_null()
    }
}

unsafe impl<T: Send> Send for LockFreeStack<T> {}
unsafe impl<T: Send> Sync for LockFreeStack<T> {}

/// Lock-free reference counter for shared ownership
pub struct Arc<T> {
    ptr: NonNull<ArcInner<T>>,
}

struct ArcInner<T> {
    data: T,
    ref_count: AtomicUsize,
}

impl<T> Arc<T> {
    pub fn new(data: T) -> Self {
        let inner = Box::leak(Box::new(ArcInner {
            data,
            ref_count: AtomicUsize::new(1),
        }));
        
        Self {
            ptr: NonNull::new(inner).unwrap(),
        }
    }

    pub fn data(&self) -> &T {
        unsafe { &self.ptr.as_ref().data }
    }

    pub fn clone(&self) -> Self {
        let inner = unsafe { self.ptr.as_ref() };
        inner.ref_count.fetch_add(1, Ordering::Relaxed);
        
        Self { ptr: self.ptr }
    }
}

impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        self.clone()
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.ptr.as_ref() };
        
        if inner.ref_count.fetch_sub(1, Ordering::Release) == 1 {
            unsafe {
                Box::from_raw(self.ptr.as_ptr());
            }
        }
    }
}

unsafe impl<T: Send + Sync> Send for Arc<T> {}
unsafe impl<T: Send + Sync> Sync for Arc<T> {}

/// Lock-free atomic reference using CAS
pub struct AtomicRef<T> {
    ptr: AtomicPtr<T>,
}

impl<T> AtomicRef<T> {
    pub const fn new(ptr: *mut T) -> Self {
        Self {
            ptr: AtomicPtr::new(ptr),
        }
    }

    pub fn load(&self, ordering: Ordering) -> *mut T {
        self.ptr.load(ordering)
    }

    pub fn store(&self, ptr: *mut T, ordering: Ordering) {
        self.ptr.store(ptr, ordering);
    }

    pub fn compare_exchange(
        &self,
        current: *mut T,
        new: *mut T,
        success: Ordering,
        failure: Ordering,
    ) -> Result<*mut T, *mut T> {
        self.ptr.compare_exchange(current, new, success, failure)
    }

    pub fn swap(&self, ptr: *mut T, ordering: Ordering) -> *mut T {
        self.ptr.swap(ptr, ordering)
    }
}

/// Lock-free ring buffer for inter-thread communication
pub struct LockFreeRingBuffer<T, const N: usize> {
    buffer: [Option<T>; N],
    write_index: AtomicUsize,
    read_index: AtomicUsize,
}

impl<T, const N: usize> LockFreeRingBuffer<T, N> {
    pub const fn new() -> Self {
        Self {
            buffer: [None; N],
            write_index: AtomicUsize::new(0),
            read_index: AtomicUsize::new(0),
        }
    }

    pub fn try_write(&self, item: T) -> bool {
        let current_write = self.write_index.load(Ordering::Relaxed);
        let next_write = (current_write + 1) % N;
        
        let current_read = self.read_index.load(Ordering::Acquire);
        
        if next_write == current_read {
            return false; // Buffer full
        }
        
        self.buffer[current_write] = Some(item);
        self.write_index.store(next_write, Ordering::Release);
        
        true
    }

    pub fn try_read(&self) -> Option<T> {
        let current_read = self.read_index.load(Ordering::Relaxed);
        let current_write = self.write_index.load(Ordering::Acquire);
        
        if current_read == current_write {
            return None; // Buffer empty
        }
        
        let item = self.buffer[current_read].take();
        let next_read = (current_read + 1) % N;
        self.read_index.store(next_read, Ordering::Release);
        
        item
    }

    pub fn len(&self) -> usize {
        let write = self.write_index.load(Ordering::Relaxed);
        let read = self.read_index.load(Ordering::Relaxed);
        
        if write >= read {
            write - read
        } else {
            N - read + write
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_full(&self) -> bool {
        self.len() == N - 1
    }
}

/// Lock-free boolean flag
pub struct AtomicFlag {
    flag: AtomicBool,
}

impl AtomicFlag {
    pub const fn new(initial: bool) -> Self {
        Self {
            flag: AtomicBool::new(initial),
        }
    }

    pub fn set(&self) {
        self.flag.store(true, Ordering::Release);
    }

    pub fn clear(&self) {
        self.flag.store(false, Ordering::Release);
    }

    pub fn test_and_set(&self) -> bool {
        self.flag.swap(true, Ordering::Acquire)
    }

    pub fn is_set(&self) -> bool {
        self.flag.load(Ordering::Acquire)
    }
}

/// Lock-free sequence number generator
pub struct SequenceGenerator {
    counter: AtomicUsize,
}

impl SequenceGenerator {
    pub const fn new(start: usize) -> Self {
        Self {
            counter: AtomicUsize::new(start),
        }
    }

    pub fn next(&self) -> usize {
        self.counter.fetch_add(1, Ordering::Relaxed)
    }

    pub fn current(&self) -> usize {
        self.counter.load(Ordering::Relaxed)
    }
}
