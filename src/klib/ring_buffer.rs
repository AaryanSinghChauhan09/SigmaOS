
// SigmaOS klib: Lock-free Ring Buffer (Circular Queue)
// Inspired by Linux kernel's kfifo and FreeBSD's ring buffer implementations
// No external dependencies - fully sovereign implementation

use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
#[allow(dead_code)]
use core::sync::atomic::{AtomicUsize, Ordering};
use std::alloc::Layout;

/// A fixed-capacity lock-free single-producer, single-consumer ring buffer.
/// Inspired by Linux kfifo and FreeBSD SPSC ring buffers.
/// Uses power-of-two size for fast modulo via bitmasking.
pub struct RingBuffer<T, const CAP: usize> {
    /// Write index (producer)
    write: AtomicUsize,
    /// Read index (consumer)
    read: AtomicUsize,
    /// Data storage
    data: UnsafeCell<[MaybeUninit<T>; CAP]>,
}

// Safety: The SPSC design ensures only one writer and one reader at a time.
unsafe impl<T: Send, const CAP: usize> Send for RingBuffer<T, CAP> {}
unsafe impl<T: Send, const CAP: usize> Sync for RingBuffer<T, CAP> {}

impl<T, const CAP: usize> RingBuffer<T, CAP> {
    /// Create a new empty ring buffer.
    /// CAP must be a power of two.
    pub const fn new() -> Self {
        assert!(
            CAP.is_power_of_two(),
            "RingBuffer capacity must be a power of two"
        );
        // SAFETY: MaybeUninit does not require initialization
        let data = unsafe { MaybeUninit::uninit().assume_init() };
        Self {
            write: AtomicUsize::new(0),
            read: AtomicUsize::new(0),
            data: UnsafeCell::new(data),
        }
    }

    /// Returns the number of elements currently in the buffer.
    #[inline]
    pub fn len(&self) -> usize {
        let w = self.write.load(Ordering::Acquire);
        let r = self.read.load(Ordering::Acquire);
        w.wrapping_sub(r)
    }

    /// Returns true if the buffer is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns true if the buffer is full.
    #[inline]
    pub fn is_full(&self) -> bool {
        self.len() == CAP
    }

    /// Returns the capacity of the buffer.
    #[inline]
    pub const fn capacity(&self) -> usize {
        CAP
    }

    /// Push an item into the ring buffer.
    /// Returns Err(item) if the buffer is full.
    pub fn push(&self, item: T) -> Result<(), T> {
        let w = self.write.load(Ordering::Relaxed);
        let r = self.read.load(Ordering::Acquire);
        if w.wrapping_sub(r) == CAP {
            return Err(item);
        }
        let idx = w & (CAP - 1);
        // SAFETY: We have exclusive write access to this slot.
        unsafe {
            (*self.data.get())[idx].write(item);
        }
        self.write.store(w.wrapping_add(1), Ordering::Release);
        Ok(())
    }

    /// Pop an item from the ring buffer.
    /// Returns None if the buffer is empty.
    pub fn pop(&self) -> Option<T> {
        let r = self.read.load(Ordering::Relaxed);
        let w = self.write.load(Ordering::Acquire);
        if r == w {
            return None;
        }
        let idx = r & (CAP - 1);
        // SAFETY: We have exclusive read access to this slot, and it was written.
        let item = unsafe { (*self.data.get())[idx].assume_init_read() };
        self.read.store(r.wrapping_add(1), Ordering::Release);
        Some(item)
    }

    /// Peek at the front element without removing it.
    pub fn peek(&self) -> Option<&T> {
        let r = self.read.load(Ordering::Relaxed);
        let w = self.write.load(Ordering::Acquire);
        if r == w {
            return None;
        }
        let idx = r & (CAP - 1);
        // SAFETY: slot is valid and initialized
        let item = unsafe { &*(*self.data.get())[idx].as_ptr() };
        Some(item)
    }

    /// Clear all elements from the buffer.
    /// This is NOT lock-free - should only be called when no concurrent accesses are occurring.
    pub fn clear(&self) {
        // Drop all existing items
        while self.pop().is_some() {}
    }
}

impl<T, const CAP: usize> Drop for RingBuffer<T, CAP> {
    fn drop(&mut self) {
        // Drop all remaining items
        self.clear();
    }
}

/// A variable-capacity ring buffer backed by heap allocation.
/// Inspired by Linux's kfifo_alloc.
pub struct HeapRingBuffer<T> {
    data: *mut core::mem::MaybeUninit<T>,
    cap: usize,
    write: usize,
    read: usize,
}

impl<T> HeapRingBuffer<T> {
    /// Create a new heap-allocated ring buffer with given capacity (rounded up to power of two).
    pub fn new(capacity: usize) -> Self {
        let cap = capacity.next_power_of_two();
        let layout = Layout::array::<core::mem::MaybeUninit<T>>(cap).unwrap();
        // SAFETY: we use the global allocator
        let data = unsafe { std::alloc::alloc(layout) as *mut core::mem::MaybeUninit<T> };
        if data.is_null() {
            panic!("HeapRingBuffer: allocation failed");
        }
        Self {
            data,
            cap,
            write: 0,
            read: 0,
        }
    }

    /// Returns the number of elements in the buffer.
    pub fn len(&self) -> usize {
        self.write.wrapping_sub(self.read)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn is_full(&self) -> bool {
        self.len() == self.cap
    }
    pub fn capacity(&self) -> usize {
        self.cap
    }

    /// Push an item. Returns Err if full.
    pub fn push(&mut self, item: T) -> Result<(), T> {
        if self.is_full() {
            return Err(item);
        }
        let idx = self.write & (self.cap - 1);
        unsafe {
            (*self.data.add(idx)).write(item);
        }
        self.write = self.write.wrapping_add(1);
        Ok(())
    }

    /// Pop an item. Returns None if empty.
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let idx = self.read & (self.cap - 1);
        let item = unsafe { (*self.data.add(idx)).assume_init_read() };
        self.read = self.read.wrapping_add(1);
        Some(item)
    }
}

impl<T> Drop for HeapRingBuffer<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
        let layout = Layout::array::<core::mem::MaybeUninit<T>>(self.cap).unwrap();
        unsafe {
            std::alloc::dealloc(self.data as *mut u8, layout);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ring_buffer_basic() {
        let buf: RingBuffer<u32, 8> = RingBuffer::new();
        assert!(buf.is_empty());
        assert_eq!(buf.capacity(), 8);

        assert!(buf.push(1).is_ok());
        assert!(buf.push(2).is_ok());
        assert!(buf.push(3).is_ok());
        assert_eq!(buf.len(), 3);

        assert_eq!(buf.pop(), Some(1));
        assert_eq!(buf.pop(), Some(2));
        assert_eq!(buf.pop(), Some(3));
        assert_eq!(buf.pop(), None);
        assert!(buf.is_empty());
    }

    #[test]
    fn test_ring_buffer_overflow() {
        let buf: RingBuffer<u32, 4> = RingBuffer::new();
        for i in 0..4 {
            assert!(buf.push(i).is_ok());
        }
        assert!(buf.is_full());
        assert!(buf.push(99).is_err());
    }

    #[test]
    fn test_ring_buffer_wrap_around() {
        let buf: RingBuffer<u32, 4> = RingBuffer::new();
        buf.push(1).unwrap();
        buf.push(2).unwrap();
        buf.pop(); // read=1
        buf.pop(); // read=2
        buf.push(3).unwrap(); // write=3, idx=3
        buf.push(4).unwrap(); // write=4, idx=0 (wrap)
        buf.push(5).unwrap(); // write=5, idx=1
        buf.push(6).unwrap(); // write=6, idx=2
        assert!(buf.is_full());
        assert_eq!(buf.pop(), Some(3));
        assert_eq!(buf.pop(), Some(4));
    }

    #[test]
    fn test_heap_ring_buffer() {
        let mut buf: HeapRingBuffer<i32> = HeapRingBuffer::new(4);
        assert!(buf.push(10).is_ok());
        assert!(buf.push(20).is_ok());
        assert_eq!(buf.pop(), Some(10));
        assert_eq!(buf.pop(), Some(20));
        assert_eq!(buf.pop(), None);
    }
}
