use std::vec::Vec;
// Atomic Bitmap, Atomic Integer & Async Procedure Call (APC) Subsystem for SigmaOS

use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

/// Thread-safe Lock-Free Atomic Bitmap for Bit Allocations
pub struct AtomicBitmap {
    pub storage: Vec<AtomicU64>,
    pub bit_capacity: usize,
}

impl AtomicBitmap {
    pub fn new(bit_capacity: usize) -> Self {
        let u64_count = (bit_capacity + 63) / 64;
        let mut storage = Vec::with_capacity(u64_count);
        for _ in 0..u64_count {
            storage.push(AtomicU64::new(0));
        }
        AtomicBitmap {
            storage,
            bit_capacity,
        }
    }

    pub fn set_bit(&self, bit_index: usize) -> bool {
        if bit_index >= self.bit_capacity {
            return false;
        }
        let word_idx = bit_index / 64;
        let bit_offset = bit_index % 64;
        let mask = 1u64 << bit_offset;

        let prev = self.storage[word_idx].fetch_or(mask, Ordering::SeqCst);
        (prev & mask) == 0
    }

    pub fn clear_bit(&self, bit_index: usize) -> bool {
        if bit_index >= self.bit_capacity {
            return false;
        }
        let word_idx = bit_index / 64;
        let bit_offset = bit_index % 64;
        let mask = !(1u64 << bit_offset);

        let prev = self.storage[word_idx].fetch_and(mask, Ordering::SeqCst);
        (prev & (1u64 << bit_offset)) != 0
    }

    pub fn test_bit(&self, bit_index: usize) -> bool {
        if bit_index >= self.bit_capacity {
            return false;
        }
        let word_idx = bit_index / 64;
        let bit_offset = bit_index % 64;
        let val = self.storage[word_idx].load(Ordering::SeqCst);
        (val & (1u64 << bit_offset)) != 0
    }
}

/// Atomic Integer Operations Wrapper with Compare-And-Swap (CAS) Loop Utilities
pub struct AtomicCounter {
    pub value: AtomicU64,
}

impl AtomicCounter {
    pub fn new(initial: u64) -> Self {
        AtomicCounter {
            value: AtomicU64::new(initial),
        }
    }

    pub fn fetch_add_cas(&self, delta: u64) -> u64 {
        let mut current = self.value.load(Ordering::SeqCst);
        loop {
            let next = current.wrapping_add(delta);
            match self.value.compare_exchange_weak(
                current,
                next,
                Ordering::SeqCst,
                Ordering::SeqCst,
            ) {
                Ok(old) => return old,
                Err(actual) => current = actual,
            }
        }
    }

    pub fn get(&self) -> u64 {
        self.value.load(Ordering::SeqCst)
    }
}

/// Lock-Free Single-Producer Single-Consumer (SPSC) Atomic Ring Buffer
pub struct AtomicRingBuffer {
    buffer: Vec<AtomicU64>,
    capacity: usize,
    head: AtomicUsize,
    tail: AtomicUsize,
}

impl AtomicRingBuffer {
    pub fn new(capacity: usize) -> Self {
        let mut buffer = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            buffer.push(AtomicU64::new(0));
        }
        Self {
            buffer,
            capacity,
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }

    pub fn push(&self, value: u64) -> Result<(), &'static str> {
        let head = self.head.load(Ordering::Relaxed);
        let tail = self.tail.load(Ordering::Acquire);
        if head.wrapping_sub(tail) >= self.capacity {
            return Err("AtomicRingBuffer: buffer full");
        }
        let index = head % self.capacity;
        self.buffer[index].store(value, Ordering::Relaxed);
        self.head.store(head.wrapping_add(1), Ordering::Release);
        Ok(())
    }

    pub fn pop(&self) -> Option<u64> {
        let tail = self.tail.load(Ordering::Relaxed);
        let head = self.head.load(Ordering::Acquire);
        if tail == head {
            return None;
        }
        let index = tail % self.capacity;
        let value = self.buffer[index].load(Ordering::Relaxed);
        self.tail.store(tail.wrapping_add(1), Ordering::Release);
        Some(value)
    }
}

/// Transactional Save-Safe State Guard for Safe State Rollback
pub struct SaveSafeStateGuard<T: Clone> {
    current_state: T,
    saved_snapshot: T,
    is_committed: bool,
}

impl<T: Clone> SaveSafeStateGuard<T> {
    pub fn new(initial_state: T) -> Self {
        Self {
            saved_snapshot: initial_state.clone(),
            current_state: initial_state,
            is_committed: false,
        }
    }

    pub fn get_state(&self) -> &T {
        &self.current_state
    }

    pub fn get_mut_state(&mut self) -> &mut T {
        &mut self.current_state
    }

    pub fn commit(&mut self) {
        self.saved_snapshot = self.current_state.clone();
        self.is_committed = true;
    }

    pub fn rollback(&mut self) {
        self.current_state = self.saved_snapshot.clone();
        self.is_committed = false;
    }
}

/// Thread-Safe Lock-Free Atomic Variable Container
pub struct AtomicVariable {
    value: AtomicU64,
}

impl AtomicVariable {
    pub fn new(initial: u64) -> Self {
        Self {
            value: AtomicU64::new(initial),
        }
    }

    pub fn load(&self) -> u64 {
        self.value.load(Ordering::SeqCst)
    }

    pub fn store(&self, val: u64) {
        self.value.store(val, Ordering::SeqCst);
    }

    pub fn swap(&self, val: u64) -> u64 {
        self.value.swap(val, Ordering::SeqCst)
    }

    pub fn compare_and_swap(&self, current: u64, new_val: u64) -> Result<u64, u64> {
        self.value
            .compare_exchange(current, new_val, Ordering::SeqCst, Ordering::SeqCst)
    }
}

/// Async Procedure Call (APC) Queue for Kernel & User Callback Dispatching
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApcEnvironment {
    KernelMode,
    UserMode,
    SpecialKernelMode,
}

pub struct ApcItem {
    pub apc_id: usize,
    pub environment: ApcEnvironment,
    pub target_thread_id: usize,
    pub callback_param: u64,
    pub is_executed: bool,
}

pub struct AsyncProcedureCallQueue {
    pub pending_apcs: Vec<ApcItem>,
    pub next_apc_id: AtomicUsize,
}

impl AsyncProcedureCallQueue {
    pub fn new() -> Self {
        AsyncProcedureCallQueue {
            pending_apcs: Vec::new(),
            next_apc_id: AtomicUsize::new(1),
        }
    }

    pub fn queue_apc(&mut self, thread_id: usize, env: ApcEnvironment, param: u64) -> usize {
        let id = self.next_apc_id.fetch_add(1, Ordering::SeqCst);
        let item = ApcItem {
            apc_id: id,
            environment: env,
            target_thread_id: thread_id,
            callback_param: param,
            is_executed: false,
        };
        self.pending_apcs.push(item);
        id
    }

    pub fn dispatch_apcs_for_thread(&mut self, thread_id: usize, env: ApcEnvironment) -> usize {
        let mut count = 0;
        for apc in &mut self.pending_apcs {
            if apc.target_thread_id == thread_id && apc.environment == env && !apc.is_executed {
                apc.is_executed = true;
                count += 1;
            }
        }
        self.pending_apcs.retain(|a| !a.is_executed);
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atomic_bitmap_and_apc_queue() {
        let bitmap = AtomicBitmap::new(128);
        assert!(!bitmap.test_bit(42));
        assert!(bitmap.set_bit(42));
        assert!(bitmap.test_bit(42));
        assert!(bitmap.clear_bit(42));
        assert!(!bitmap.test_bit(42));

        let counter = AtomicCounter::new(10);
        assert_eq!(counter.fetch_add_cas(5), 10);
        assert_eq!(counter.get(), 15);

        let mut apc_q = AsyncProcedureCallQueue::new();
        let apc_id = apc_q.queue_apc(1001, ApcEnvironment::KernelMode, 0x11223344);
        assert_eq!(apc_id, 1);
        let count = apc_q.dispatch_apcs_for_thread(1001, ApcEnvironment::KernelMode);
        assert_eq!(count, 1);
    }

    #[test]
    fn test_atomic_ring_buffer_spsc() {
        let ring = AtomicRingBuffer::new(4);
        assert_eq!(ring.pop(), None);

        ring.push(101).unwrap();
        ring.push(102).unwrap();
        ring.push(103).unwrap();
        ring.push(104).unwrap();
        assert!(ring.push(105).is_err()); // Buffer full

        assert_eq!(ring.pop(), Some(101));
        assert_eq!(ring.pop(), Some(102));

        ring.push(105).unwrap();
        assert_eq!(ring.pop(), Some(103));
        assert_eq!(ring.pop(), Some(104));
        assert_eq!(ring.pop(), Some(105));
        assert_eq!(ring.pop(), None);
    }

    #[test]
    fn test_save_safe_state_guard_and_atomic_variable() {
        // SaveSafeStateGuard transactional rollback test
        let mut guard = SaveSafeStateGuard::new("clean_state".to_string());
        *guard.get_mut_state() = "modified_dirty_state".to_string();
        assert_eq!(guard.get_state(), "modified_dirty_state");

        guard.rollback();
        assert_eq!(guard.get_state(), "clean_state");

        *guard.get_mut_state() = "verified_state".to_string();
        guard.commit();
        *guard.get_mut_state() = "another_mutation".to_string();
        guard.rollback();
        assert_eq!(guard.get_state(), "verified_state");

        // AtomicVariable CAS test
        let var = AtomicVariable::new(500);
        assert_eq!(var.load(), 500);
        assert_eq!(var.swap(600), 500);
        assert_eq!(var.load(), 600);

        assert!(var.compare_and_swap(600, 700).is_ok());
        assert_eq!(var.load(), 700);
        assert!(var.compare_and_swap(600, 800).is_err());
    }
}
