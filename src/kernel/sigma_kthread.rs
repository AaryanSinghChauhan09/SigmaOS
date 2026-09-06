#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
//! SigmaOS Kernel Thread (kthread) Subsystem
//!
//! Sovereign kernel threading abstraction. Inspired by:
//! - Linux kthread API (kernel/kthread.c)
//! - FreeBSD kthread(9) / kproc(9)
//! - NetBSD kernel threads
//!
//! Kernel threads run in kernel context, have no user address space,
//! and are used for background tasks (page reclaim, disk I/O, timers).

#![allow(dead_code)]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

// ============================================================
// Kthread State
// ============================================================

/// Lifecycle state of a kernel thread.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KthreadState {
    /// Thread created but not yet started
    Created,
    /// Thread is running
    Running,
    /// Thread is sleeping (waiting for event)
    Sleeping,
    /// Thread is parked (suspended by kthread_park)
    Parked,
    /// Thread has been stopped and is exiting
    Stopped,
    /// Thread has exited (zombie, waiting for join)
    Zombie,
}

// ============================================================
// Kthread ID
// ============================================================

/// Unique identifier for a kernel thread.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KthreadId(u32);

// ============================================================
// Kthread Priority
// ============================================================

/// Scheduling priority for a kernel thread.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum KthreadPriority {
    /// Real-time priority (e.g., interrupt handlers)
    RealTime = 0,
    /// High priority (e.g., I/O completion)
    High = 1,
    /// Normal kernel thread priority
    Normal = 2,
    /// Low priority (background tasks)
    Low = 3,
    /// Idle (runs only when nothing else can)
    Idle = 4,
}

// ============================================================
// SigmaKthread — Kernel Thread Descriptor
// ============================================================

/// A kernel thread descriptor.
///
/// Analogous to Linux `task_struct` (kernel-only subset) or
/// BSD `proc`/`thread` structure.
///
/// # Design (Encapsulation)
/// The `should_stop` and `should_park` flags use atomic booleans
/// so that the owning code can signal the thread without a lock.
pub struct SigmaKthread {
    /// Thread identifier
    id: KthreadId,
    /// Human-readable name (shown in ps/top)
    name: String,
    /// Current lifecycle state
    state: KthreadState,
    /// Scheduling priority
    priority: KthreadPriority,
    /// Bound CPU (-1 = any CPU)
    cpu_affinity: i32,
    /// Signal: the thread should stop at next opportunity
    should_stop: AtomicBool,
    /// Signal: the thread should park (suspend) at next opportunity
    should_park: AtomicBool,
    /// Stack size in bytes
    stack_size: usize,
    /// Thread function (stored for documentation; actual execution managed by scheduler)
    func_name: String,
    /// User data pointer (opaque u64)
    data: u64,
    /// Total CPU time used in nanoseconds
    cpu_time_ns: u64,
    /// Number of times thread has woken up
    wake_count: u64,
}

impl SigmaKthread {
    /// Create a new kernel thread.
    ///
    /// # Arguments
    /// * `id` — Unique thread ID
    /// * `func_name` — Name of the thread function (for debugging)
    /// * `data` — Opaque data passed to the thread function
    /// * `name` — Human-readable thread name (e.g., "kswapd0")
    pub fn new(id: KthreadId, func_name: &str, data: u64, name: &str) -> Self {
        Self {
            id,
            name: name.into(),
            state: KthreadState::Created,
            priority: KthreadPriority::Normal,
            cpu_affinity: -1,
            should_stop: AtomicBool::new(false),
            should_park: AtomicBool::new(false),
            stack_size: 16 * 1024, // 16KB default
            func_name: func_name.into(),
            data,
            cpu_time_ns: 0,
            wake_count: 0,
        }
    }

    /// Returns whether the thread should stop.
    ///
    /// Threads MUST call this in their main loop:
    /// ```rust
    /// while !kthread.should_stop() {
    ///     // ... do work ...
    /// }
    /// ```
    #[inline]
    pub fn should_stop(&self) -> bool {
        self.should_stop.load(Ordering::Relaxed)
    }

    /// Returns whether the thread should park.
    #[inline]
    pub fn should_park(&self) -> bool {
        self.should_park.load(Ordering::Relaxed)
    }

    /// Mark thread as parked (called by the thread when it detects should_park).
    pub fn do_park(&mut self) {
        self.state = KthreadState::Parked;
    }

    /// Signal this thread to stop.
    pub fn signal_stop(&self) {
        self.should_stop.store(true, Ordering::Release);
    }

    /// Signal this thread to park.
    pub fn signal_park(&self) {
        self.should_park.store(true, Ordering::Release);
    }

    /// Unpark this thread (resume from parked state).
    pub fn unpark(&mut self) {
        self.should_park.store(false, Ordering::Release);
        if self.state == KthreadState::Parked {
            self.state = KthreadState::Running;
            self.wake_count += 1;
        }
    }

    /// Transition to Running state.
    pub fn start(&mut self) {
        self.state = KthreadState::Running;
    }

    /// Mark thread as sleeping.
    pub fn sleep(&mut self) {
        self.state = KthreadState::Sleeping;
    }

    /// Wake a sleeping thread.
    pub fn wake(&mut self) {
        if self.state == KthreadState::Sleeping {
            self.state = KthreadState::Running;
            self.wake_count += 1;
        }
    }

    /// Mark thread as stopped (should be called before joining).
    pub fn mark_stopped(&mut self) {
        self.state = KthreadState::Stopped;
    }

    /// Mark thread as zombie (exited, not yet joined).
    pub fn mark_zombie(&mut self) {
        self.state = KthreadState::Zombie;
    }

    /// Charge CPU time to this thread.
    pub fn charge_cpu_ns(&mut self, ns: u64) {
        self.cpu_time_ns = self.cpu_time_ns.saturating_add(ns);
    }

    /// Set CPU affinity. -1 = any CPU.
    pub fn set_cpu_affinity(&mut self, cpu: i32) { self.cpu_affinity = cpu; }

    /// Set scheduling priority.
    pub fn set_priority(&mut self, p: KthreadPriority) { self.priority = p; }

    /// Set stack size.
    pub fn set_stack_size(&mut self, sz: usize) { self.stack_size = sz; }

    // Accessors
    pub fn id(&self) -> KthreadId { self.id }
    pub fn name(&self) -> &str { &self.name }
    pub fn state(&self) -> KthreadState { self.state }
    pub fn priority(&self) -> KthreadPriority { self.priority }
    pub fn cpu_affinity(&self) -> i32 { self.cpu_affinity }
    pub fn cpu_time_ns(&self) -> u64 { self.cpu_time_ns }
    pub fn wake_count(&self) -> u64 { self.wake_count }
    pub fn data(&self) -> u64 { self.data }
}

// ============================================================
// KthreadPool — Managed Pool of Worker Threads
// ============================================================

/// A pool of kernel worker threads for parallel background work.
///
/// Analogous to Linux `workqueue_struct` with dedicated threads,
/// or BSD `taskqueue` with worker threads.
pub struct KthreadPool {
    /// Name of this pool
    name: String,
    /// Worker threads in this pool
    threads: Vec<KthreadId>,
    /// Target number of threads
    target_size: usize,
    /// Minimum threads (never go below)
    min_threads: usize,
    /// Maximum threads (never exceed)
    max_threads: usize,
}

impl KthreadPool {
    /// Create a new thread pool.
    pub fn new(name: &str, min: usize, max: usize) -> Self {
        Self {
            name: name.into(),
            threads: Vec::new(),
            target_size: min,
            min_threads: min,
            max_threads: max,
        }
    }

    /// Register a thread in this pool.
    pub fn add_thread(&mut self, id: KthreadId) -> Result<(), &'static str> {
        if self.threads.len() >= self.max_threads {
            return Err("pool at maximum capacity");
        }
        self.threads.push(id);
        Ok(())
    }

    /// Remove a thread from the pool.
    pub fn remove_thread(&mut self, id: KthreadId) {
        self.threads.retain(|&t| t != id);
    }

    /// Returns current thread count.
    pub fn size(&self) -> usize { self.threads.len() }

    /// Returns whether the pool needs more threads.
    pub fn needs_more(&self) -> bool { self.threads.len() < self.target_size }

    /// Returns whether the pool can shrink.
    pub fn can_shrink(&self) -> bool { self.threads.len() > self.min_threads }

    pub fn name(&self) -> &str { &self.name }
    pub fn thread_ids(&self) -> &[KthreadId] { &self.threads }
}

// ============================================================
// KthreadManager — Global Kernel Thread Registry
// ============================================================

/// Global kernel thread manager.
///
/// Tracks all running kernel threads, handles creation/destruction,
/// and provides lookup by ID or name.
pub struct KthreadManager {
    /// All threads indexed by ID
    threads: BTreeMap<KthreadId, SigmaKthread>,
    /// Next thread ID
    next_id: AtomicU32,
    /// Named thread pools
    pools: BTreeMap<String, KthreadPool>,
}

impl KthreadManager {
    /// Create a new kernel thread manager.
    pub fn new() -> Self {
        Self {
            threads: BTreeMap::new(),
            next_id: AtomicU32::new(1),
            pools: BTreeMap::new(),
        }
    }

    /// Create and register a new kernel thread.
    ///
    /// # Returns
    /// The new thread's `KthreadId`.
    pub fn create(&mut self, func_name: &str, data: u64, name: &str) -> KthreadId {
        let id = KthreadId(self.next_id.fetch_add(1, Ordering::Relaxed));
        let thread = SigmaKthread::new(id, func_name, data, name);
        self.threads.insert(id, thread);
        id
    }

    /// Start a created thread.
    pub fn start(&mut self, id: KthreadId) -> Result<(), &'static str> {
        self.threads.get_mut(&id).ok_or("thread not found")?.start();
        Ok(())
    }

    /// Signal a thread to stop and wait for it.
    pub fn stop(&mut self, id: KthreadId) -> Result<(), &'static str> {
        let thread = self.threads.get_mut(&id).ok_or("thread not found")?;
        thread.signal_stop();
        thread.mark_stopped();
        Ok(())
    }

    /// Park a thread (suspend it).
    pub fn park(&mut self, id: KthreadId) -> Result<(), &'static str> {
        let thread = self.threads.get_mut(&id).ok_or("thread not found")?;
        thread.signal_park();
        thread.do_park();
        Ok(())
    }

    /// Unpark a parked thread.
    pub fn unpark(&mut self, id: KthreadId) -> Result<(), &'static str> {
        self.threads.get_mut(&id).ok_or("thread not found")?.unpark();
        Ok(())
    }

    /// Wake a sleeping thread.
    pub fn wake(&mut self, id: KthreadId) -> Result<(), &'static str> {
        self.threads.get_mut(&id).ok_or("thread not found")?.wake();
        Ok(())
    }

    /// Destroy a zombie thread, freeing its resources.
    pub fn join(&mut self, id: KthreadId) -> Result<(), &'static str> {
        match self.threads.get(&id).map(|t| t.state()) {
            Some(KthreadState::Zombie) | Some(KthreadState::Stopped) => {
                self.threads.remove(&id);
                Ok(())
            }
            Some(_) => Err("thread is still running"),
            None => Err("thread not found"),
        }
    }

    /// Create a named thread pool.
    pub fn create_pool(&mut self, name: &str, min: usize, max: usize) {
        self.pools.insert(name.into(), KthreadPool::new(name, min, max));
    }

    /// Add thread to a pool.
    pub fn add_to_pool(&mut self, pool: &str, id: KthreadId) -> Result<(), &'static str> {
        self.pools.get_mut(pool).ok_or("pool not found")?.add_thread(id)
    }

    /// Get reference to thread.
    pub fn get(&self, id: KthreadId) -> Option<&SigmaKthread> { self.threads.get(&id) }
    /// Get mutable reference to thread.
    pub fn get_mut(&mut self, id: KthreadId) -> Option<&mut SigmaKthread> { self.threads.get_mut(&id) }

    /// Find all threads with a given name.
    pub fn find_by_name(&self, name: &str) -> Vec<KthreadId> {
        self.threads.iter()
            .filter(|(_, t)| t.name() == name)
            .map(|(&id, _)| id)
            .collect()
    }

    /// Returns total thread count.
    pub fn count(&self) -> usize { self.threads.len() }

    /// Returns count of running threads.
    pub fn running_count(&self) -> usize {
        self.threads.values().filter(|t| t.state() == KthreadState::Running).count()
    }
}

impl Default for KthreadManager {
    fn default() -> Self { Self::new() }
}

// ============================================================
// Built-in kernel threads (like Linux kernel daemons)
// ============================================================

/// Initialize the standard SigmaOS kernel threads.
///
/// Creates analogues of Linux kernel daemons:
/// - `kswapd` — memory reclaim
/// - `kworker` — work queue processing
/// - `kcompactd` — memory compaction
/// - `ksoftirqd` — soft IRQ processing
/// - `migration` — CPU load balancing
pub fn init_kernel_threads(mgr: &mut KthreadManager) {
    let kswapd = mgr.create("sigma_kswapd", 0, "kswapd0");
    mgr.start(kswapd).ok();

    let kworker = mgr.create("sigma_kworker", 0, "kworker/0:0");
    mgr.start(kworker).ok();

    let kcompactd = mgr.create("sigma_kcompactd", 0, "kcompactd0");
    mgr.start(kcompactd).ok();

    let ksoftirqd = mgr.create("sigma_ksoftirqd", 0, "ksoftirqd/0");
    mgr.start(ksoftirqd).ok();

    let migration = mgr.create("sigma_migration", 0, "migration/0");
    mgr.start(migration).ok();
    if let Some(t) = mgr.get_mut(migration) {
        t.set_priority(KthreadPriority::RealTime);
        t.set_cpu_affinity(0);
    }

    // Create a worker pool for general kernel work
    mgr.create_pool("kworker-pool", 2, 8);
    mgr.add_to_pool("kworker-pool", kworker).ok();
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_start() {
        let mut mgr = KthreadManager::new();
        let id = mgr.create("my_func", 42, "test-thread");
        assert_eq!(mgr.get(id).unwrap().state(), KthreadState::Created);
        mgr.start(id).unwrap();
        assert_eq!(mgr.get(id).unwrap().state(), KthreadState::Running);
    }

    #[test]
    fn test_should_stop() {
        let mut mgr = KthreadManager::new();
        let id = mgr.create("worker", 0, "worker");
        mgr.start(id).unwrap();
        assert!(!mgr.get(id).unwrap().should_stop());
        mgr.stop(id).unwrap();
        assert!(mgr.get(id).unwrap().should_stop());
    }

    #[test]
    fn test_park_unpark() {
        let mut mgr = KthreadManager::new();
        let id = mgr.create("bg_work", 0, "bg");
        mgr.start(id).unwrap();
        mgr.park(id).unwrap();
        assert_eq!(mgr.get(id).unwrap().state(), KthreadState::Parked);
        mgr.unpark(id).unwrap();
        assert_eq!(mgr.get(id).unwrap().state(), KthreadState::Running);
    }

    #[test]
    fn test_init_kernel_threads() {
        let mut mgr = KthreadManager::new();
        init_kernel_threads(&mut mgr);
        assert!(mgr.count() >= 5);
        assert!(!mgr.find_by_name("kswapd0").is_empty());
        assert!(!mgr.find_by_name("migration/0").is_empty());
        let migration_id = mgr.find_by_name("migration/0")[0];
        assert_eq!(mgr.get(migration_id).unwrap().priority(), KthreadPriority::RealTime);
    }

    #[test]
    fn test_thread_pool() {
        let mut mgr = KthreadManager::new();
        mgr.create_pool("test-pool", 1, 3);
        let t1 = mgr.create("worker", 0, "w1");
        let t2 = mgr.create("worker", 0, "w2");
        let t3 = mgr.create("worker", 0, "w3");
        let t4 = mgr.create("worker", 0, "w4"); // should fail
        mgr.add_to_pool("test-pool", t1).unwrap();
        mgr.add_to_pool("test-pool", t2).unwrap();
        mgr.add_to_pool("test-pool", t3).unwrap();
        assert!(mgr.add_to_pool("test-pool", t4).is_err());
    }
}
