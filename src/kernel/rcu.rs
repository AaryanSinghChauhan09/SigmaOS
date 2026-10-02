//! # RCU (Read-Copy-Update) Synchronization
//!
//! Linux-inspired RCU implementation for wait-free reads.
//! Provides efficient read-side critical sections with deferred reclamation.

#![no_std]

extern crate alloc;
use alloc::boxed::Box;
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};

/// RCU grace period number
pub type GracePeriod = u64;

/// RCU callback type
pub type RcuCallback = Box<dyn FnOnce() + Send>;

/// RCU state for each CPU
pub struct RcuCpuState {
    /// CPU ID
    pub cpu: u32,
    /// Current grace period this CPU is in
    pub gp_current: AtomicU64,
    /// Grace period this CPU has acknowledged
    pub gp_acked: AtomicU64,
    /// Number of readers in critical section
    pub readers: AtomicUsize,
    /// Is this CPU in quiescent state
    pub quiescent: AtomicBool,
}

impl RcuCpuState {
    pub const fn new(cpu: u32) -> Self {
        Self {
            cpu,
            gp_current: AtomicU64::new(0),
            gp_acked: AtomicU64::new(0),
            readers: AtomicUsize::new(0),
            quiescent: AtomicBool::new(true),
        }
    }

    /// Enter RCU read-side critical section
    pub fn read_lock(&self) {
        self.readers.fetch_add(1, Ordering::Acquire);
        self.quiescent.store(false, Ordering::Release);
    }

    /// Exit RCU read-side critical section
    pub fn read_unlock(&self) {
        let prev = self.readers.fetch_sub(1, Ordering::Release);
        if prev == 1 {
            // Last reader, enter quiescent state
            self.quiescent.store(true, Ordering::Release);
        }
    }

    /// Check if CPU is in quiescent state
    pub fn is_quiescent(&self) -> bool {
        self.quiescent.load(Ordering::Acquire)
    }

    /// Acknowledge grace period
    pub fn ack_grace_period(&self, gp: GracePeriod) {
        self.gp_acked.store(gp, Ordering::Release);
    }
}

/// RCU callback entry with grace period
struct RcuCallbackEntry {
    /// Callback function
    callback: RcuCallback,
    /// Grace period after which this can be invoked
    grace_period: GracePeriod,
}

/// RCU state (inspired by Linux kernel RCU)
pub struct RcuState {
    /// Current grace period number
    current_gp: AtomicU64,
    /// Completed grace period number
    completed_gp: AtomicU64,
    /// Per-CPU state
    cpu_states: Vec<RcuCpuState>,
    /// Pending callbacks
    callbacks: VecDeque<RcuCallbackEntry>,
    /// Number of CPUs
    num_cpus: u32,
    /// Total callbacks invoked
    total_callbacks: AtomicU64,
    /// Total grace periods
    total_gps: AtomicU64,
}

impl RcuState {
    pub fn new(num_cpus: u32) -> Self {
        let mut cpu_states = Vec::with_capacity(num_cpus as usize);
        for cpu in 0..num_cpus {
            cpu_states.push(RcuCpuState::new(cpu));
        }

        Self {
            current_gp: AtomicU64::new(1),
            completed_gp: AtomicU64::new(0),
            cpu_states,
            callbacks: VecDeque::new(),
            num_cpus,
            total_callbacks: AtomicU64::new(0),
            total_gps: AtomicU64::new(0),
        }
    }

    /// Get current grace period
    pub fn current_grace_period(&self) -> GracePeriod {
        self.current_gp.load(Ordering::Acquire)
    }

    /// Get completed grace period
    pub fn completed_grace_period(&self) -> GracePeriod {
        self.completed_gp.load(Ordering::Acquire)
    }

    /// Enter RCU read-side critical section on current CPU
    pub fn read_lock(&self, cpu: u32) {
        if let Some(state) = self.cpu_states.get(cpu as usize) {
            state.read_lock();
        }
    }

    /// Exit RCU read-side critical section on current CPU
    pub fn read_unlock(&self, cpu: u32) {
        if let Some(state) = self.cpu_states.get(cpu as usize) {
            state.read_unlock();
        }
    }

    /// Register callback to be invoked after grace period
    pub fn call_rcu(&mut self, callback: RcuCallback) {
        let gp = self.current_grace_period();
        self.callbacks.push_back(RcuCallbackEntry {
            callback,
            grace_period: gp,
        });
    }

    /// Check if all CPUs have passed through quiescent state
    fn all_cpus_quiescent(&self, gp: GracePeriod) -> bool {
        self.cpu_states.iter().all(|state| {
            state.is_quiescent() || state.gp_acked.load(Ordering::Acquire) >= gp
        })
    }

    /// Advance RCU state machine
    pub fn advance(&mut self) {
        let current = self.current_grace_period();
        let completed = self.completed_grace_period();

        // Check if current grace period is complete
        if current > completed && self.all_cpus_quiescent(current) {
            // Grace period completed
            self.completed_gp.store(current, Ordering::Release);
            self.total_gps.fetch_add(1, Ordering::Relaxed);

            // Notify all CPUs
            for state in &self.cpu_states {
                state.ack_grace_period(current);
            }

            // Start new grace period
            self.current_gp.fetch_add(1, Ordering::SeqCst);
        }

        // Invoke callbacks for completed grace periods
        self.invoke_completed_callbacks();
    }

    /// Invoke callbacks whose grace periods have completed
    fn invoke_completed_callbacks(&mut self) {
        let completed = self.completed_grace_period();
        let mut to_invoke = Vec::new();

        // Collect callbacks ready to invoke
        while let Some(entry) = self.callbacks.front() {
            if entry.grace_period <= completed {
                to_invoke.push(self.callbacks.pop_front().unwrap());
            } else {
                break;
            }
        }

        // Invoke callbacks
        for entry in to_invoke {
            (entry.callback)();
            self.total_callbacks.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Synchronize RCU (wait for grace period)
    pub fn synchronize(&mut self) {
        let target_gp = self.current_grace_period();
        while self.completed_grace_period() < target_gp {
            self.advance();
        }
    }

    /// Force quiescent state on all CPUs
    pub fn force_quiescent_state(&self) {
        for state in &self.cpu_states {
            if state.readers.load(Ordering::Acquire) == 0 {
                state.quiescent.store(true, Ordering::Release);
            }
        }
    }

    /// Get RCU statistics
    pub fn stats(&self) -> RcuStats {
        RcuStats {
            current_gp: self.current_grace_period(),
            completed_gp: self.completed_grace_period(),
            pending_callbacks: self.callbacks.len(),
            total_callbacks: self.total_callbacks.load(Ordering::Relaxed),
            total_gps: self.total_gps.load(Ordering::Relaxed),
            num_cpus: self.num_cpus,
        }
    }
}

/// RCU statistics
#[derive(Debug, Clone, Copy)]
pub struct RcuStats {
    pub current_gp: GracePeriod,
    pub completed_gp: GracePeriod,
    pub pending_callbacks: usize,
    pub total_callbacks: u64,
    pub total_gps: u64,
    pub num_cpus: u32,
}

/// RCU-protected pointer wrapper
pub struct RcuPointer<T> {
    ptr: AtomicUsize,
    _phantom: core::marker::PhantomData<T>,
}

impl<T> RcuPointer<T> {
    pub const fn new_null() -> Self {
        Self {
            ptr: AtomicUsize::new(0),
            _phantom: core::marker::PhantomData,
        }
    }

    pub fn new(ptr: *mut T) -> Self {
        Self {
            ptr: AtomicUsize::new(ptr as usize),
            _phantom: core::marker::PhantomData,
        }
    }

    /// Load pointer (read-side)
    pub fn load(&self) -> *mut T {
        self.ptr.load(Ordering::Acquire) as *mut T
    }

    /// Store pointer (write-side, requires RCU synchronization)
    pub fn store(&self, ptr: *mut T) {
        self.ptr.store(ptr as usize, Ordering::Release);
    }

    /// Exchange pointer atomically
    pub fn swap(&self, ptr: *mut T) -> *mut T {
        self.ptr.swap(ptr as usize, Ordering::AcqRel) as *mut T
    }
}

unsafe impl<T: Send> Send for RcuPointer<T> {}
unsafe impl<T: Sync> Sync for RcuPointer<T> {}

/// RCU read guard (RAII for read-side critical section)
pub struct RcuReadGuard<'a> {
    rcu: &'a RcuState,
    cpu: u32,
}

impl<'a> RcuReadGuard<'a> {
    pub fn new(rcu: &'a RcuState, cpu: u32) -> Self {
        rcu.read_lock(cpu);
        Self { rcu, cpu }
    }
}

impl<'a> Drop for RcuReadGuard<'a> {
    fn drop(&mut self) {
        self.rcu.read_unlock(self.cpu);
    }
}

/// Global RCU state
static mut GLOBAL_RCU: Option<RcuState> = None;

/// Initialize global RCU
pub fn init_rcu(num_cpus: u32) {
    unsafe {
        GLOBAL_RCU = Some(RcuState::new(num_cpus));
    }
}

/// Get global RCU state
pub fn global_rcu() -> Option<&'static mut RcuState> {
    unsafe { GLOBAL_RCU.as_mut() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rcu_cpu_state() {
        let state = RcuCpuState::new(0);
        assert!(state.is_quiescent());

        state.read_lock();
        assert!(!state.is_quiescent());
        assert_eq!(state.readers.load(Ordering::Relaxed), 1);

        state.read_unlock();
        assert!(state.is_quiescent());
    }

    #[test]
    fn test_rcu_grace_period() {
        let mut rcu = RcuState::new(4);
        let gp1 = rcu.current_grace_period();
        assert_eq!(gp1, 1);

        // Mark all CPUs as quiescent
        rcu.force_quiescent_state();
        rcu.advance();

        assert_eq!(rcu.completed_grace_period(), 1);
        assert_eq!(rcu.current_grace_period(), 2);
    }

    #[test]
    fn test_rcu_callback() {
        let mut rcu = RcuState::new(2);
        let mut invoked = false;
        let callback = Box::new(|| {
            // Callback would set invoked = true
        });

        rcu.call_rcu(callback);
        assert_eq!(rcu.callbacks.len(), 1);
    }

    #[test]
    fn test_rcu_synchronize() {
        let mut rcu = RcuState::new(2);
        rcu.force_quiescent_state();
        rcu.synchronize();
        assert!(rcu.completed_grace_period() >= 1);
    }

    #[test]
    fn test_rcu_pointer() {
        let value = Box::into_raw(Box::new(42));
        let ptr = RcuPointer::new(value);
        assert_eq!(ptr.load(), value);

        let new_value = Box::into_raw(Box::new(100));
        let old = ptr.swap(new_value);
        assert_eq!(old, value);
        assert_eq!(ptr.load(), new_value);

        // Cleanup
        unsafe {
            let _ = Box::from_raw(ptr.load());
        }
    }
}
