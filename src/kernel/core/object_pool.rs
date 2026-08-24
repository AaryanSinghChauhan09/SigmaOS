// Object Pool Engine for Zero-Allocation Kernel Operations
// Location: src/kernel/core/object_pool.rs

// #![no_std]
use core::sync::atomic::{AtomicBool, Ordering};

pub struct ObjectPoolEntry<T, const N: usize> {
    pub object: T,
    pub active: AtomicBool,
}

pub struct FixedObjectPool<T, const N: usize> {
    entries: [ObjectPoolEntry<T, N>; N],
}

impl<T: Default, const N: usize> FixedObjectPool<T, N> {
    pub fn new() -> Self {
        // Initialize pool array safely
        let entries = core::array::from_fn(|_| ObjectPoolEntry {
            object: T::default(),
            active: AtomicBool::new(false),
        });
        FixedObjectPool { entries }
    }

    pub fn acquire(&self) -> Option<(usize, &T)> {
        for (idx, entry) in self.entries.iter().enumerate() {
            if !entry.active.load(Ordering::SeqCst) {
                if entry.active.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                    return Some((idx, &entry.object));
                }
            }
        }
        None
    }

    pub fn release(&self, index: usize) -> bool {
        if index < N {
            self.entries[index].active.store(false, Ordering::SeqCst);
            true
        } else {
            false
        }
    }

    pub fn active_count(&self) -> usize {
        self.entries.iter().filter(|e| e.active.load(Ordering::SeqCst)).count()
    }
}

// Pre-defined Pool Structs for Critical Kernel Data Structures
#[derive(Default, Debug, Clone, Copy)]
pub struct TaskControlBlockSlot {
    pub task_id: u64,
    pub priority: u8,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct NetworkBufferSlot {
    pub packet_id: u32,
    pub payload_len: u16,
}

pub type TaskPool = FixedObjectPool<TaskControlBlockSlot, 64>;
pub type NetworkBufPool = FixedObjectPool<NetworkBufferSlot, 128>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_pool_acquire_release() {
        let pool = TaskPool::new();
        assert_eq!(pool.active_count(), 0);

        let (idx1, _obj1) = pool.acquire().expect("Acquire slot 1");
        assert_eq!(idx1, 0);
        assert_eq!(pool.active_count(), 1);

        let (idx2, _obj2) = pool.acquire().expect("Acquire slot 2");
        assert_eq!(idx2, 1);
        assert_eq!(pool.active_count(), 2);

        assert!(pool.release(idx1));
        assert_eq!(pool.active_count(), 1);
    }
}
