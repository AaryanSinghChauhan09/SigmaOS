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
}
