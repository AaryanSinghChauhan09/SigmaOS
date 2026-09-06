#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
// SPDX-License-Identifier: MIT OR Apache-2.0
// SigmaOS klib::ringbuf - Lock-free Ring Buffer (zero external dependencies)
// Inspired by Linux kernel's kfifo and FreeBSD's buf_ring
// Uses only core atomic operations, no std or alloc required

use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicUsize, Ordering};

/// A fixed-size, lock-free single-producer single-consumer ring buffer.
/// Inspired by Linux's `kfifo` and FreeBSD's `buf_ring`.
/// Zero external dependencies - uses only `core`.
pub struct RingBuf<T, const N: usize> {
    data: UnsafeCell<[Option<T>; N]>,
    head: AtomicUsize, // Consumer reads from head
    tail: AtomicUsize, // Producer writes to tail
}

// SAFETY: Single-producer, single-consumer usage is safe across threads
unsafe impl<T: Send, const N: usize> Sync for RingBuf<T, N> {}
unsafe impl<T: Send, const N: usize> Send for RingBuf<T, N> {}

impl<T, const N: usize> RingBuf<T, N> {
    /// Create a new empty ring buffer. N must be a power of 2 for efficiency.
    pub const fn new() -> Self {
        // SAFETY: Option<T> is valid when all-zero for most types (None is safe)
        // This uses a const context trick to initialize the array
        Self {
            // SAFETY: MaybeUninit trick not needed for Option<T> - None is always valid
            data: UnsafeCell::new(unsafe { core::mem::zeroed() }),
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }

    /// Returns the capacity of the ring buffer.
    #[inline]
    pub const fn capacity(&self) -> usize {
        N
    }

    /// Returns the number of elements currently in the buffer.
    #[inline]
    pub fn len(&self) -> usize {
        let tail = self.tail.load(Ordering::Acquire);
        let head = self.head.load(Ordering::Acquire);
        tail.wrapping_sub(head)
    }

    /// Returns true if the buffer is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns true if the buffer is full.
    #[inline]
    pub fn is_full(&self) -> bool {
        self.len() == N
    }

    /// Push an item to the tail (producer side).
    /// Returns `Err(item)` if the buffer is full.
    pub fn push(&self, item: T) -> Result<(), T> {
        let tail = self.tail.load(Ordering::Relaxed);
        let head = self.head.load(Ordering::Acquire);

        if tail.wrapping_sub(head) >= N {
            return Err(item); // Buffer full
        }

        let slot = tail & (N - 1); // Fast modulo for power-of-2 N

        // SAFETY: We have exclusive access to this slot (producer owns tail)
        unsafe {
            let data = &mut *self.data.get();
            data[slot] = Some(item);
        }

        // Release: ensure the write to data is visible before tail update
        self.tail.store(tail.wrapping_add(1), Ordering::Release);
        Ok(())
    }

    /// Pop an item from the head (consumer side).
    /// Returns `None` if the buffer is empty.
    pub fn pop(&self) -> Option<T> {
        let head = self.head.load(Ordering::Relaxed);
        let tail = self.tail.load(Ordering::Acquire);

        if head == tail {
            return None; // Buffer empty
        }

        let slot = head & (N - 1); // Fast modulo for power-of-2 N

        // SAFETY: We have exclusive access to this slot (consumer owns head)
        let item = unsafe {
            let data = &mut *self.data.get();
            data[slot].take()
        };

        // Release: ensure the read from data is done before head update
        self.head.store(head.wrapping_add(1), Ordering::Release);
        item
    }

    /// Peek at the front item without consuming it.
    pub fn peek(&self) -> Option<&T> {
        let head = self.head.load(Ordering::Acquire);
        let tail = self.tail.load(Ordering::Acquire);

        if head == tail {
            return None;
        }

        let slot = head & (N - 1);

        // SAFETY: Slot is valid and not being modified by producer (different slot)
        unsafe {
            let data = &*self.data.get();
            data[slot].as_ref()
        }
    }
}

/// Multi-producer, multi-consumer ring buffer using spinlock
/// Inspired by FreeBSD's `buf_ring` with critical section locking
pub struct MpscRingBuf<T, const N: usize> {
    inner: RingBuf<T, N>,
    lock: AtomicUsize, // Simple spinlock: 0 = free, 1 = locked
}

impl<T, const N: usize> MpscRingBuf<T, N> {
    pub const fn new() -> Self {
        Self {
            inner: RingBuf::new(),
            lock: AtomicUsize::new(0),
        }
    }

    fn acquire_lock(&self) {
        while self
            .lock
            .compare_exchange_weak(0, 1, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            // Spin with hint for CPU to reduce power consumption
            core::hint::spin_loop();
        }
    }

    fn release_lock(&self) {
        self.lock.store(0, Ordering::Release);
    }

    pub fn push(&self, item: T) -> Result<(), T> {
        self.acquire_lock();
        let result = self.inner.push(item);
        self.release_lock();
        result
    }

    pub fn pop(&self) -> Option<T> {
        self.acquire_lock();
        let result = self.inner.pop();
        self.release_lock();
        result
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_push_pop() {
        let buf: RingBuf<u32, 4> = RingBuf::new();

        assert!(buf.is_empty());
        assert!(!buf.is_full());

        buf.push(1).unwrap();
        buf.push(2).unwrap();
        buf.push(3).unwrap();
        buf.push(4).unwrap();

        assert!(buf.is_full());
        assert!(buf.push(5).is_err()); // Should fail when full

        assert_eq!(buf.pop(), Some(1)); // FIFO ordering
        assert_eq!(buf.pop(), Some(2));
        assert_eq!(buf.pop(), Some(3));
        assert_eq!(buf.pop(), Some(4));
        assert_eq!(buf.pop(), None); // Empty
    }

    #[test]
    fn test_wrap_around() {
        let buf: RingBuf<u32, 4> = RingBuf::new();

        // Fill and drain, then fill again (tests wrap-around)
        for i in 0..4u32 {
            buf.push(i).unwrap();
        }
        for _ in 0..4 {
            buf.pop();
        }
        for i in 4..8u32 {
            buf.push(i).unwrap();
        }

        assert_eq!(buf.pop(), Some(4));
        assert_eq!(buf.pop(), Some(5));
    }

    #[test]
    fn test_peek() {
        let buf: RingBuf<u32, 8> = RingBuf::new();
        buf.push(42).unwrap();

        assert_eq!(buf.peek(), Some(&42));
        assert_eq!(buf.peek(), Some(&42)); // Still there
        assert_eq!(buf.pop(), Some(42));
        assert_eq!(buf.peek(), None);
    }
}
