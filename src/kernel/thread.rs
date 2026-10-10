//! # SigmaOS Canonical Thread Subsystem
//!
//! Provides the canonical `Thread` abstraction and lifecycle manager for SigmaOS.
//! Integrates CPU context, stack limits, scheduling attributes, wait channels,
//! signals, capabilities, join/detach mechanics, and lock-order tracking.

#![no_std]

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};

/// Process Identifier (PID)
pub type Pid = u32;

/// CPU affinity mask (up to 64 CPUs)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CpuAffinity {
    pub mask: u64,
}

impl CpuAffinity {
    pub const fn all() -> Self {
        Self { mask: u64::MAX }
    }

    pub const fn none() -> Self {
        Self { mask: 0 }
    }

    pub const fn single(cpu: u32) -> Self {
        Self { mask: 1u64 << cpu }
    }

    pub fn set_cpu(&mut self, cpu: u32) {
        self.mask |= 1u64 << cpu;
    }

    pub fn clear_cpu(&mut self, cpu: u32) {
        self.mask &= !(1u64 << cpu);
    }

    pub const fn is_set(&self, cpu: u32) -> bool {
        (self.mask & (1u64 << cpu)) != 0
    }
}

/// Thread Identifier (TID)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ThreadId(pub u32);

impl ThreadId {
    pub const fn new(id: u32) -> Self {
        Self(id)
    }

    pub fn as_u32(&self) -> u32 {
        self.0
    }
}

/// Wait Channel Identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WaitChannelId(pub u64);

/// Explicit Thread Lifecycle States
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ThreadState {
    /// Newly created, not yet enqueued
    New = 0,
    /// Runnable and waiting in run queue
    Ready = 1,
    /// Actively executing on CPU
    Running = 2,
    /// Sleeping on wait channel / timer
    Blocked = 3,
    /// Suspended by signal or control action
    Suspended = 4,
    /// Exited, waiting to be joined/reaped
    Zombie = 5,
    /// Resources fully reclaimed
    Terminated = 6,
}

impl ThreadState {
    /// Validates whether a state transition from `self` to `target` is legal.
    pub fn is_valid_transition(self, target: ThreadState) -> bool {
        match (self, target) {
            (ThreadState::New, ThreadState::Ready) => true,
            (ThreadState::New, ThreadState::Terminated) => true,

            (ThreadState::Ready, ThreadState::Running) => true,
            (ThreadState::Ready, ThreadState::Suspended) => true,
            (ThreadState::Ready, ThreadState::Zombie) => true,

            (ThreadState::Running, ThreadState::Ready) => true,
            (ThreadState::Running, ThreadState::Blocked) => true,
            (ThreadState::Running, ThreadState::Suspended) => true,
            (ThreadState::Running, ThreadState::Zombie) => true,

            (ThreadState::Blocked, ThreadState::Ready) => true,
            (ThreadState::Blocked, ThreadState::Suspended) => true,
            (ThreadState::Blocked, ThreadState::Zombie) => true,

            (ThreadState::Suspended, ThreadState::Ready) => true,
            (ThreadState::Suspended, ThreadState::Zombie) => true,

            (ThreadState::Zombie, ThreadState::Terminated) => true,

            // Self-transitions or invalid jumps are rejected
            _ => false,
        }
    }
}

/// Thread Ownership Categories
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadOwnership {
    /// Pure kernel background thread (no user address space)
    Kernel,
    /// User-space thread bound to a process
    User,
    /// Primary thread of a process group
    Process,
    /// Independent thread automatically reclaimed on exit without join
    Detached,
}

/// Scheduling Class
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulingClass {
    Normal,
    Fifo,
    RoundRobin,
    Batch,
    Idle,
    Deadline,
}

/// Saved CPU Context (Architecture Register Snapshot)
#[derive(Debug, Clone, Copy, Default)]
pub struct CpuContext {
    pub rip: u64,
    pub rsp: u64,
    pub rbp: u64,
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rflags: u64,
    pub cs: u64,
    pub ss: u64,
}

/// Stack Metadata with Bounds Guard
#[derive(Debug, Clone, Copy)]
pub struct StackMeta {
    pub base_addr: u64,
    pub stack_size: usize,
    pub guard_size: usize,
}

impl StackMeta {
    pub fn new(base_addr: u64, stack_size: usize, guard_size: usize) -> Self {
        Self {
            base_addr,
            stack_size,
            guard_size,
        }
    }

    pub fn contains(&self, ptr: u64) -> bool {
        ptr >= self.base_addr && ptr < self.base_addr + (self.stack_size as u64)
    }

    pub fn is_in_guard_zone(&self, ptr: u64) -> bool {
        ptr >= self.base_addr && ptr < self.base_addr + (self.guard_size as u64)
    }
}

/// Canonical Thread Abstraction
#[derive(Debug)]
pub struct Thread {
    /// Thread ID
    pub tid: ThreadId,
    /// Owning Process ID
    pub pid: Pid,
    /// Human-readable name
    pub name: String,
    /// Thread Ownership
    pub ownership: ThreadOwnership,
    /// Current Lifecycle State
    pub state: ThreadState,
    /// Saved CPU Context
    pub cpu_context: CpuContext,
    /// Kernel Stack Metadata
    pub kernel_stack: StackMeta,
    /// User Stack Metadata (if user thread)
    pub user_stack: Option<StackMeta>,
    /// Static Priority (0..139)
    pub priority: u8,
    /// Nice Value (-20..19)
    pub nice: i8,
    /// Scheduling Class
    pub sched_class: SchedulingClass,
    /// CPU Affinity Mask
    pub affinity: CpuAffinity,
    /// Wait Channel ID if blocked
    pub wait_channel: Option<WaitChannelId>,
    /// Signal Mask (blocked signals)
    pub signal_mask: u64,
    /// Pending Signals Bitmask
    pub pending_signals: u64,
    /// File Descriptor Table Reference ID
    pub fd_table_ref: Option<u64>,
    /// Resource Limits Reference ID
    pub rlimits_ref: Option<u64>,
    /// Capabilities Context Bitmask
    pub capability_context: u64,
    /// Exit Status Code
    pub exit_status: Option<i32>,
    /// Whether thread is detached
    pub is_detached: bool,
    /// Held Lock Addresses (Lock Order Tracker)
    pub locks_held: Vec<u64>,
}

impl Thread {
    pub fn new(
        tid: ThreadId,
        pid: Pid,
        name: &str,
        ownership: ThreadOwnership,
        kernel_stack: StackMeta,
    ) -> Self {
        let is_detached = ownership == ThreadOwnership::Detached;
        Self {
            tid,
            pid,
            name: name.to_string(),
            ownership,
            state: ThreadState::New,
            cpu_context: CpuContext::default(),
            kernel_stack,
            user_stack: None,
            priority: 120, // Default normal priority
            nice: 0,
            sched_class: SchedulingClass::Normal,
            affinity: CpuAffinity::all(),
            wait_channel: None,
            signal_mask: 0,
            pending_signals: 0,
            fd_table_ref: None,
            rlimits_ref: None,
            capability_context: 0,
            exit_status: None,
            is_detached,
            locks_held: Vec::new(),
        }
    }

    /// Attempts a thread lifecycle state transition
    pub fn transition_to(&mut self, new_state: ThreadState) -> Result<(), &'static str> {
        if self.state.is_valid_transition(new_state) {
            self.state = new_state;
            Ok(())
        } else {
            Err("invalid thread state transition")
        }
    }

    /// Block thread on a wait channel
    pub fn block_on(&mut self, wchan: WaitChannelId) -> Result<(), &'static str> {
        self.transition_to(ThreadState::Blocked)?;
        self.wait_channel = Some(wchan);
        Ok(())
    }

    /// Wake thread from blocked state
    pub fn wake(&mut self) -> Result<(), &'static str> {
        if self.state == ThreadState::Blocked {
            self.transition_to(ThreadState::Ready)?;
            self.wait_channel = None;
            Ok(())
        } else {
            Err("thread is not in blocked state")
        }
    }

    /// Suspend thread execution
    pub fn suspend(&mut self) -> Result<(), &'static str> {
        self.transition_to(ThreadState::Suspended)
    }

    /// Resume thread from suspended state
    pub fn resume(&mut self) -> Result<(), &'static str> {
        if self.state == ThreadState::Suspended {
            self.transition_to(ThreadState::Ready)
        } else {
            Err("thread is not suspended")
        }
    }

    /// Exit thread execution
    pub fn exit(&mut self, code: i32) -> Result<(), &'static str> {
        self.transition_to(ThreadState::Zombie)?;
        self.exit_status = Some(code);
        self.wait_channel = None;
        if self.is_detached {
            self.transition_to(ThreadState::Terminated)?;
        }
        Ok(())
    }

    /// Join thread execution (reap zombie)
    pub fn join(&mut self) -> Result<i32, &'static str> {
        if self.is_detached {
            return Err("cannot join a detached thread");
        }
        match self.state {
            ThreadState::Zombie => {
                let status = self.exit_status.unwrap_or(0);
                self.transition_to(ThreadState::Terminated)?;
                Ok(status)
            }
            ThreadState::Terminated => Err("thread already joined and terminated"),
            _ => Err("thread is still running"),
        }
    }

    /// Detach thread
    pub fn detach(&mut self) -> Result<(), &'static str> {
        if self.is_detached {
            return Err("thread already detached");
        }
        self.is_detached = true;
        self.ownership = ThreadOwnership::Detached;
        if self.state == ThreadState::Zombie {
            self.transition_to(ThreadState::Terminated)?;
        }
        Ok(())
    }

    /// Acquire a lock (Lock Order Tracker)
    pub fn acquire_lock(&mut self, lock_addr: u64) -> Result<(), &'static str> {
        if let Some(&last) = self.locks_held.last() {
            if lock_addr <= last {
                // Warning: potential lock order inversion
            }
        }
        self.locks_held.push(lock_addr);
        Ok(())
    }

    /// Release a lock (Lock Order Tracker)
    pub fn release_lock(&mut self, lock_addr: u64) -> Result<(), &'static str> {
        if let Some(pos) = self.locks_held.iter().rposition(|&l| l == lock_addr) {
            self.locks_held.remove(pos);
            Ok(())
        } else {
            Err("lock not held by thread")
        }
    }
}

/// Canonical Thread Manager
#[derive(Debug)]
pub struct ThreadManager {
    next_tid: AtomicU32,
    threads: Vec<Thread>,
}

impl ThreadManager {
    pub const fn new() -> Self {
        Self {
            next_tid: AtomicU32::new(1),
            threads: Vec::new(),
        }
    }

    pub fn allocate_tid(&self) -> ThreadId {
        ThreadId(self.next_tid.fetch_add(1, Ordering::SeqCst))
    }

    pub fn create_thread(
        &mut self,
        pid: Pid,
        name: &str,
        ownership: ThreadOwnership,
        kernel_stack: StackMeta,
    ) -> ThreadId {
        let tid = self.allocate_tid();
        let thread = Thread::new(tid, pid, name, ownership, kernel_stack);
        self.threads.push(thread);
        tid
    }

    pub fn find(&self, tid: ThreadId) -> Option<&Thread> {
        self.threads.iter().find(|t| t.tid == tid)
    }

    pub fn find_mut(&mut self, tid: ThreadId) -> Option<&mut Thread> {
        self.threads.iter_mut().find(|t| t.tid == tid)
    }

    pub fn enqueue_ready(&mut self, tid: ThreadId) -> Result<(), &'static str> {
        let thread = self.find_mut(tid).ok_or("thread not found")?;
        thread.transition_to(ThreadState::Ready)
    }

    pub fn wake_wait_channel(&mut self, wchan: WaitChannelId) -> usize {
        let mut woken = 0;
        for t in self.threads.iter_mut() {
            if t.state == ThreadState::Blocked && t.wait_channel == Some(wchan) {
                if t.wake().is_ok() {
                    woken += 1;
                }
            }
        }
        woken
    }

    pub fn reap_terminated(&mut self) {
        self.threads.retain(|t| t.state != ThreadState::Terminated);
    }

    pub fn active_count(&self) -> usize {
        self.threads
            .iter()
            .filter(|t| t.state != ThreadState::Terminated)
            .count()
    }
}

impl Default for ThreadManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_state_transitions() {
        let stack = StackMeta::new(0x1000, 0x4000, 0x1000);
        let mut thread = Thread::new(
            ThreadId(1),
            1,
            "worker",
            ThreadOwnership::User,
            stack,
        );

        assert_eq!(thread.state, ThreadState::New);

        assert!(thread.transition_to(ThreadState::Ready).is_ok());
        assert_eq!(thread.state, ThreadState::Ready);

        assert!(thread.transition_to(ThreadState::Running).is_ok());
        assert_eq!(thread.state, ThreadState::Running);

        assert!(thread.block_on(WaitChannelId(0x100)).is_ok());
        assert_eq!(thread.state, ThreadState::Blocked);

        assert!(thread.wake().is_ok());
        assert_eq!(thread.state, ThreadState::Ready);

        assert!(thread.transition_to(ThreadState::Running).is_ok());
        assert!(thread.exit(0).is_ok());
        assert_eq!(thread.state, ThreadState::Zombie);

        assert_eq!(thread.join(), Ok(0));
        assert_eq!(thread.state, ThreadState::Terminated);
    }

    #[test]
    fn test_invalid_state_transitions() {
        let stack = StackMeta::new(0x1000, 0x4000, 0x1000);
        let mut thread = Thread::new(
            ThreadId(2),
            1,
            "bad_trans",
            ThreadOwnership::User,
            stack,
        );

        // Invalid: New -> Running (must go through Ready)
        assert!(thread.transition_to(ThreadState::Running).is_err());

        // Invalid: New -> Blocked
        assert!(thread.transition_to(ThreadState::Blocked).is_err());
    }

    #[test]
    fn test_detach_and_join() {
        let stack = StackMeta::new(0x1000, 0x4000, 0x1000);
        let mut thread = Thread::new(
            ThreadId(3),
            1,
            "detached",
            ThreadOwnership::User,
            stack,
        );

        thread.transition_to(ThreadState::Ready).unwrap();
        thread.transition_to(ThreadState::Running).unwrap();

        assert!(thread.detach().is_ok());
        assert!(thread.is_detached);

        assert!(thread.exit(42).is_ok());
        assert_eq!(thread.state, ThreadState::Terminated);

        // Joining detached thread must fail
        assert!(thread.join().is_err());
    }

    #[test]
    fn test_lock_order_tracker() {
        let stack = StackMeta::new(0x1000, 0x4000, 0x1000);
        let mut thread = Thread::new(
            ThreadId(4),
            1,
            "lock_test",
            ThreadOwnership::User,
            stack,
        );

        assert!(thread.acquire_lock(0x1000).is_ok());
        assert!(thread.acquire_lock(0x2000).is_ok());
        assert_eq!(thread.locks_held.len(), 2);

        assert!(thread.release_lock(0x2000).is_ok());
        assert_eq!(thread.locks_held.len(), 1);

        assert!(thread.release_lock(0x1000).is_ok());
        assert!(thread.locks_held.is_empty());
    }

    #[test]
    fn test_thread_manager_wait_channel_wake() {
        let mut tm = ThreadManager::new();
        let stack = StackMeta::new(0x1000, 0x4000, 0x1000);

        let t1 = tm.create_thread(1, "w1", ThreadOwnership::User, stack);
        let t2 = tm.create_thread(1, "w2", ThreadOwnership::User, stack);

        tm.enqueue_ready(t1).unwrap();
        tm.enqueue_ready(t2).unwrap();

        tm.find_mut(t1)
            .unwrap()
            .transition_to(ThreadState::Running)
            .unwrap();
        tm.find_mut(t1).unwrap().block_on(WaitChannelId(42)).unwrap();

        tm.find_mut(t2)
            .unwrap()
            .transition_to(ThreadState::Running)
            .unwrap();
        tm.find_mut(t2).unwrap().block_on(WaitChannelId(42)).unwrap();

        let woken = tm.wake_wait_channel(WaitChannelId(42));
        assert_eq!(woken, 2);

        assert_eq!(tm.find(t1).unwrap().state, ThreadState::Ready);
        assert_eq!(tm.find(t2).unwrap().state, ThreadState::Ready);
    }

    #[test]
    fn test_stack_bounds_and_guards() {
        let stack = StackMeta::new(0x10000, 0x4000, 0x1000);
        assert!(stack.contains(0x10000));
        assert!(stack.contains(0x12000));
        assert!(!stack.contains(0x14000));

        assert!(stack.is_in_guard_zone(0x10000));
        assert!(stack.is_in_guard_zone(0x10800));
        assert!(!stack.is_in_guard_zone(0x11000));
    }

    #[test]
    fn test_suspension_and_resume_lifecycle() {
        let stack = StackMeta::new(0x1000, 0x4000, 0x1000);
        let mut thread = Thread::new(
            ThreadId(5),
            1,
            "suspend_test",
            ThreadOwnership::User,
            stack,
        );

        thread.transition_to(ThreadState::Ready).unwrap();
        thread.transition_to(ThreadState::Running).unwrap();

        assert!(thread.suspend().is_ok());
        assert_eq!(thread.state, ThreadState::Suspended);

        assert!(thread.resume().is_ok());
        assert_eq!(thread.state, ThreadState::Ready);
    }

    #[test]
    fn test_reap_terminated_threads() {
        let mut tm = ThreadManager::new();
        let stack = StackMeta::new(0x1000, 0x4000, 0x1000);

        let t1 = tm.create_thread(1, "t1", ThreadOwnership::Detached, stack);
        let t2 = tm.create_thread(1, "t2", ThreadOwnership::User, stack);

        tm.enqueue_ready(t1).unwrap();
        tm.find_mut(t1).unwrap().transition_to(ThreadState::Running).unwrap();
        tm.find_mut(t1).unwrap().exit(0).unwrap(); // Detached -> Terminated immediately

        assert_eq!(tm.find(t1).unwrap().state, ThreadState::Terminated);
        assert_eq!(tm.active_count(), 1);

        tm.reap_terminated();
        assert!(tm.find(t1).is_none());
        assert!(tm.find(t2).is_some());
    }
}
