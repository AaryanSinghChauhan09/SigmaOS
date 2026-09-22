// Linux-inspired futex (fast userspace mutex)
// Userspace synchronization primitive for SigmaOS

use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

/// Futex operation (Linux futex.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FutexOperation {
    Wait,
    Wake,
    WakeBitset,
    LockPi,
    UnlockPi,
    TryLockPi,
    Requeue,
    RequeuePriority,
}

/// Futex wait flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FutexFlags {
    pub private: bool,
    pub clock_realtime: bool,
}

impl FutexFlags {
    pub fn new() -> Self {
        FutexFlags {
            private: false,
            clock_realtime: false,
        }
    }

    pub fn with_private(mut self) -> Self {
        self.private = true;
        self
    }

    pub fn with_clock_realtime(mut self) -> Self {
        self.clock_realtime = true;
        self
    }
}

impl Default for FutexFlags {
    fn default() -> Self {
        Self::new()
    }
}

/// Futex waiter state
#[derive(Debug)]
struct FutexWaiter {
    address: u64,
    pid: u32,
    #[allow(dead_code)]
    flags: FutexFlags,
    woken: AtomicBool,
}

impl FutexWaiter {
    pub fn new(address: u64, pid: u32, flags: FutexFlags) -> Self {
        FutexWaiter {
            address,
            pid,
            flags,
            woken: AtomicBool::new(false),
        }
    }

    pub fn wake(&self) -> bool {
        !self.woken.swap(true, Ordering::SeqCst)
    }
}

/// Futex queue for an address
#[derive(Debug, Clone)]
struct FutexQueue {
    address: u64,
    waiters: Vec<Arc<FutexWaiter>>,
}

impl FutexQueue {
    pub fn new(address: u64) -> Self {
        FutexQueue {
            address,
            waiters: Vec::new(),
        }
    }

    pub fn add_waiter(&mut self, waiter: Arc<FutexWaiter>) {
        self.waiters.push(waiter);
    }

    pub fn wake_one(&mut self) -> Option<Arc<FutexWaiter>> {
        for waiter in &self.waiters {
            if waiter.wake() {
                return Some(waiter.clone());
            }
        }
        None
    }

    pub fn wake_all(&mut self) -> Vec<Arc<FutexWaiter>> {
        let mut woken = Vec::new();
        for waiter in &self.waiters {
            if waiter.wake() {
                woken.push(waiter.clone());
            }
        }
        woken
    }

    pub fn remove_woken(&mut self) {
        self.waiters.retain(|w| !w.woken.load(Ordering::SeqCst));
    }

    pub fn waiter_count(&self) -> usize {
        self.waiters.len()
    }
}

/// Futex manager for the system
pub struct FutexManager {
    queues: BTreeMap<u64, FutexQueue>,
}

impl FutexManager {
    pub fn new() -> Self {
        FutexManager {
            queues: BTreeMap::new(),
        }
    }

    /// Wait on a futex
    pub fn wait(&mut self, address: u64, _expected_value: u32, pid: u32, flags: FutexFlags) -> Result<(), String> {
        // In a real implementation, this would check the actual memory value
        // For now, we just add the waiter to the queue

        let queue = self.queues.entry(address).or_insert_with(|| FutexQueue::new(address));
        let waiter = Arc::new(FutexWaiter::new(address, pid, flags));
        queue.add_waiter(waiter);

        Ok(())
    }

    /// Wake waiters on a futex
    pub fn wake(&mut self, address: u64, max_waiters: usize) -> Result<usize, String> {
        let queue = self.queues.get_mut(&address)
            .ok_or_else(|| format!("No waiters on address: {}", address))?;

        let mut woken_count = 0;
        for _ in 0..max_waiters {
            if queue.wake_one().is_some() {
                woken_count += 1;
            } else {
                break;
            }
        }

        queue.remove_woken();

        // Remove empty queue
        if queue.waiter_count() == 0 {
            self.queues.remove(&address);
        }

        Ok(woken_count)
    }

    /// Wake all waiters on a futex
    pub fn wake_all(&mut self, address: u64) -> Result<usize, String> {
        let queue = self.queues.get_mut(&address)
            .ok_or_else(|| format!("No waiters on address: {}", address))?;

        let woken = queue.wake_all();
        let woken_count = woken.len();

        queue.remove_woken();

        // Remove empty queue
        if queue.waiter_count() == 0 {
            self.queues.remove(&address);
        }

        Ok(woken_count)
    }

    /// Requeue waiters from one address to another
    pub fn requeue(&mut self, src_address: u64, dst_address: u64, max_waiters: usize) -> Result<usize, String> {
        // First, collect PIDs to requeue
        let to_requeue_pids = {
            let src_queue = self.queues.get(&src_address)
                .ok_or_else(|| format!("No waiters on source address: {}", src_address))?;

            let mut pids = Vec::new();
            let mut count = 0;

            for waiter in &src_queue.waiters {
                if count < max_waiters && !waiter.woken.load(Ordering::SeqCst) {
                    pids.push(waiter.pid);
                    count += 1;
                }
            }
            pids
        };

        let requeued_count = to_requeue_pids.len();

        // Remove from source
        if let Some(src_queue) = self.queues.get_mut(&src_address) {
            src_queue.waiters.retain(|w| !to_requeue_pids.contains(&w.pid));

            // Clean up empty source queue
            if src_queue.waiter_count() == 0 {
                self.queues.remove(&src_address);
            }
        }

        // Add to destination
        let dst_queue = self.queues.entry(dst_address).or_insert_with(|| FutexQueue::new(dst_address));
        for pid in to_requeue_pids {
            let new_waiter = Arc::new(FutexWaiter::new(dst_address, pid, FutexFlags::new()));
            dst_queue.add_waiter(new_waiter);
        }

        Ok(requeued_count)
    }

    /// Get waiter count for an address
    pub fn waiter_count(&self, address: u64) -> usize {
        self.queues.get(&address)
            .map(|q| q.waiter_count())
            .unwrap_or(0)
    }

    /// Get total queue count
    pub fn queue_count(&self) -> usize {
        self.queues.len()
    }
}

impl Default for FutexManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_futex_flags_creation() {
        let flags = FutexFlags::new();
        assert!(!flags.private);
        assert!(!flags.clock_realtime);
    }

    #[test]
    fn test_futex_flags_with_private() {
        let flags = FutexFlags::new().with_private();
        assert!(flags.private);
    }

    #[test]
    fn test_futex_flags_with_clock_realtime() {
        let flags = FutexFlags::new().with_clock_realtime();
        assert!(flags.clock_realtime);
    }

    #[test]
    fn test_futex_waiter_creation() {
        let waiter = FutexWaiter::new(0x1000, 1, FutexFlags::new());
        assert_eq!(waiter.address, 0x1000);
        assert_eq!(waiter.pid, 1);
    }

    #[test]
    fn test_futex_waiter_wake() {
        let waiter = FutexWaiter::new(0x1000, 1, FutexFlags::new());
        assert!(waiter.wake());
        assert!(!waiter.wake());
    }

    #[test]
    fn test_futex_queue_creation() {
        let queue = FutexQueue::new(0x1000);
        assert_eq!(queue.address, 0x1000);
        assert_eq!(queue.waiter_count(), 0);
    }

    #[test]
    fn test_futex_queue_add_waiter() {
        let mut queue = FutexQueue::new(0x1000);
        let waiter = Arc::new(FutexWaiter::new(0x1000, 1, FutexFlags::new()));
        queue.add_waiter(waiter);
        assert_eq!(queue.waiter_count(), 1);
    }

    #[test]
    fn test_futex_queue_wake_one() {
        let mut queue = FutexQueue::new(0x1000);
        let waiter = Arc::new(FutexWaiter::new(0x1000, 1, FutexFlags::new()));
        queue.add_waiter(waiter.clone());

        let woken = queue.wake_one();
        assert!(woken.is_some());
        assert_eq!(queue.waiter_count(), 1);
    }

    #[test]
    fn test_futex_queue_wake_all() {
        let mut queue = FutexQueue::new(0x1000);
        let waiter1 = Arc::new(FutexWaiter::new(0x1000, 1, FutexFlags::new()));
        let waiter2 = Arc::new(FutexWaiter::new(0x1000, 2, FutexFlags::new()));
        queue.add_waiter(waiter1);
        queue.add_waiter(waiter2);

        let woken = queue.wake_all();
        assert_eq!(woken.len(), 2);
    }

    #[test]
    fn test_futex_manager_creation() {
        let manager = FutexManager::new();
        assert_eq!(manager.queue_count(), 0);
    }

    #[test]
    fn test_futex_manager_wait() {
        let mut manager = FutexManager::new();
        assert!(manager.wait(0x1000, 42, 1, FutexFlags::new()).is_ok());
        assert_eq!(manager.waiter_count(0x1000), 1);
    }

    #[test]
    fn test_futex_manager_wake() {
        let mut manager = FutexManager::new();
        manager.wait(0x1000, 42, 1, FutexFlags::new()).unwrap();

        let woken = manager.wake(0x1000, 1).unwrap();
        assert_eq!(woken, 1);
    }

    #[test]
    fn test_futex_manager_wake_all() {
        let mut manager = FutexManager::new();
        manager.wait(0x1000, 42, 1, FutexFlags::new()).unwrap();
        manager.wait(0x1000, 42, 2, FutexFlags::new()).unwrap();

        let woken = manager.wake_all(0x1000).unwrap();
        assert_eq!(woken, 2);
    }

    #[test]
    fn test_futex_manager_requeue() {
        let mut manager = FutexManager::new();
        manager.wait(0x1000, 42, 1, FutexFlags::new()).unwrap();
        manager.wait(0x1000, 42, 2, FutexFlags::new()).unwrap();

        let requeued = manager.requeue(0x1000, 0x2000, 1).unwrap();
        assert_eq!(requeued, 1);
        assert_eq!(manager.waiter_count(0x1000), 1);
        assert_eq!(manager.waiter_count(0x2000), 1);
    }
}
