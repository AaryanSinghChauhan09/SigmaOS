#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
//! SigmaOS Kernel Work Queue Subsystem
//!
//! Sovereign deferred-work infrastructure. Inspired by:
//! - Linux kernel workqueue (kernel/workqueue.c)
//! - BSD taskqueue(9) API
//!
//! Supports ordered, unordered, and CPU-affine work queues.
//! No external dependencies — pure Rust, no_std compatible.

#![allow(dead_code)]

extern crate alloc;
use alloc::collections::VecDeque;
use alloc::string::String;
use alloc::vec::Vec;

// ============================================================
// Work Item Priority
// ============================================================

/// Priority level for work queue items.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WorkPriority {
    /// Background — lowest priority, run when idle
    Background = 0,
    /// Normal — default priority
    Normal = 1,
    /// High — run before normal items
    High = 2,
    /// Critical — run immediately before anything else
    Critical = 3,
}

// ============================================================
// Work Item
// ============================================================

/// A unit of deferred kernel work.
///
/// # Design (Single Responsibility)
/// Each WorkItem encapsulates exactly one function to execute
/// with its associated data pointer. No shared state.
pub struct WorkItem {
    /// Function to execute
    func: fn(data: u64),
    /// Opaque data passed to func
    data: u64,
    /// Human-readable name for tracing/debugging
    name: String,
    /// Execution priority
    priority: WorkPriority,
    /// Delay in nanoseconds (0 = immediate)
    delay_ns: u64,
    /// Absolute deadline (set at submission time)
    scheduled_at_ns: u64,
}

impl WorkItem {
    /// Create an immediate work item.
    pub fn new(func: fn(u64), data: u64, name: &str, priority: WorkPriority) -> Self {
        Self {
            func,
            data,
            name: name.into(),
            priority,
            delay_ns: 0,
            scheduled_at_ns: 0,
        }
    }

    /// Create a delayed work item.
    pub fn new_delayed(func: fn(u64), data: u64, name: &str, delay_ns: u64) -> Self {
        Self {
            func,
            data,
            name: name.into(),
            priority: WorkPriority::Normal,
            delay_ns,
            scheduled_at_ns: 0,
        }
    }

    /// Execute the work item.
    #[inline]
    pub fn execute(&self) {
        (self.func)(self.data)
    }

    /// Returns the item name.
    pub fn name(&self) -> &str { &self.name }
    /// Returns the item priority.
    pub fn priority(&self) -> WorkPriority { self.priority }
    /// Returns the delay in ns.
    pub fn delay_ns(&self) -> u64 { self.delay_ns }
}

// ============================================================
// Work Queue Statistics
// ============================================================

/// Runtime statistics for a work queue.
#[derive(Debug, Default, Clone)]
pub struct WorkQueueStats {
    /// Items submitted
    pub submitted: u64,
    /// Items executed
    pub executed: u64,
    /// Items dropped (queue full)
    pub dropped: u64,
    /// Items still pending
    pub pending: usize,
}

// ============================================================
// SigmaWorkQueue — Main Work Queue
// ============================================================

/// Kernel work queue for deferred execution.
///
/// # Design
/// Implements a priority-ordered double-ended queue.
/// Supports both ordered (FIFO within priority) and
/// unordered (execute in any order) modes.
///
/// ## Comparison
/// | Feature | Linux workqueue | BSD taskqueue | SigmaWorkQueue |
/// |---------|----------------|---------------|----------------|
/// | Priority | Yes | No | Yes |
/// | Delay | Yes (delayed_work) | Limited | Yes |
/// | CPU binding | Yes | Yes | Yes (BoundWorkQueue) |
/// | no_std | No | No | Yes |
pub struct SigmaWorkQueue {
    /// Queue name
    name: String,
    /// Maximum number of pending items (0 = unlimited)
    max_depth: usize,
    /// Pending work items stored in priority order
    queue: VecDeque<WorkItem>,
    /// Whether this queue processes in strict FIFO order
    ordered: bool,
    /// Statistics
    stats: WorkQueueStats,
    /// Current monotonic time (for delayed work)
    now_ns: u64,
}

impl SigmaWorkQueue {
    /// Create a new unbounded work queue.
    pub fn new(name: &str, ordered: bool) -> Self {
        Self {
            name: name.into(),
            max_depth: 0,
            queue: VecDeque::new(),
            ordered,
            stats: WorkQueueStats::default(),
            now_ns: 0,
        }
    }

    /// Create a bounded work queue with a maximum depth.
    pub fn new_bounded(name: &str, ordered: bool, max_depth: usize) -> Self {
        Self {
            name: name.into(),
            max_depth,
            queue: VecDeque::new(),
            ordered,
            stats: WorkQueueStats::default(),
            now_ns: 0,
        }
    }

    /// Submit a work item to the queue.
    ///
    /// Returns `Err` if the queue is full (bounded queues only).
    pub fn submit(&mut self, mut item: WorkItem) -> Result<(), &'static str> {
        if self.max_depth > 0 && self.queue.len() >= self.max_depth {
            self.stats.dropped += 1;
            return Err("work queue full");
        }
        item.scheduled_at_ns = self.now_ns;
        // Insert in priority order (higher priority first) for unordered queues
        if !self.ordered {
            let pos = self.queue.iter().position(|i| i.priority < item.priority)
                .unwrap_or(self.queue.len());
            self.queue.insert(pos, item);
        } else {
            self.queue.push_back(item);
        }
        self.stats.submitted += 1;
        self.stats.pending = self.queue.len();
        Ok(())
    }

    /// Process one pending work item.
    ///
    /// Returns `true` if an item was executed, `false` if queue empty.
    pub fn process_one(&mut self) -> bool {
        // Skip delayed items not yet ready
        if let Some(item) = self.queue.front() {
            if item.delay_ns > 0 && self.now_ns < item.scheduled_at_ns + item.delay_ns {
                return false;
            }
        }
        if let Some(item) = self.queue.pop_front() {
            item.execute();
            self.stats.executed += 1;
            self.stats.pending = self.queue.len();
            true
        } else {
            false
        }
    }

    /// Flush all pending (ready) work items.
    ///
    /// # Returns
    /// Number of items executed.
    pub fn flush(&mut self) -> usize {
        let mut count = 0;
        while self.process_one() { count += 1; }
        count
    }

    /// Advance the internal clock (for delayed work).
    pub fn advance_time(&mut self, delta_ns: u64) {
        self.now_ns = self.now_ns.saturating_add(delta_ns);
    }

    /// Drain all work regardless of delay (used during shutdown).
    pub fn drain(&mut self) -> usize {
        let count = self.queue.len();
        for item in self.queue.drain(..) {
            item.execute();
        }
        self.stats.executed += count as u64;
        self.stats.pending = 0;
        count
    }

    /// Returns the queue name.
    pub fn name(&self) -> &str { &self.name }
    /// Returns pending item count.
    pub fn pending(&self) -> usize { self.queue.len() }
    /// Returns statistics.
    pub fn stats(&self) -> &WorkQueueStats { &self.stats }
    /// Returns whether queue is empty.
    pub fn is_empty(&self) -> bool { self.queue.is_empty() }
}

// ============================================================
// BoundWorkQueue — CPU-Affine Work Queue
// ============================================================

/// A CPU-affine work queue that executes on a specific CPU core.
///
/// Wraps `SigmaWorkQueue` with CPU binding metadata.
/// Useful for cache-hot operations that benefit from locality.
pub struct BoundWorkQueue {
    /// The underlying work queue
    inner: SigmaWorkQueue,
    /// Target CPU core ID
    cpu_id: usize,
    /// Whether the CPU is currently active
    cpu_online: bool,
}

impl BoundWorkQueue {
    /// Create a work queue bound to `cpu_id`.
    pub fn new(name: &str, cpu_id: usize) -> Self {
        Self {
            inner: SigmaWorkQueue::new(name, true),
            cpu_id,
            cpu_online: true,
        }
    }

    /// Set CPU online status.
    pub fn set_cpu_online(&mut self, online: bool) { self.cpu_online = online; }

    /// Returns the bound CPU ID.
    pub fn cpu_id(&self) -> usize { self.cpu_id }

    /// Flush work if CPU is online.
    pub fn flush_if_online(&mut self) -> usize {
        if self.cpu_online { self.inner.flush() } else { 0 }
    }

    /// Submit work to this CPU queue.
    pub fn submit(&mut self, item: WorkItem) -> Result<(), &'static str> {
        self.inner.submit(item)
    }

    /// Access inner queue.
    pub fn inner(&self) -> &SigmaWorkQueue { &self.inner }
    pub fn inner_mut(&mut self) -> &mut SigmaWorkQueue { &mut self.inner }
}

// ============================================================
// WorkQueueSystem — Global Coordinator
// ============================================================

/// System-wide work queue coordinator.
///
/// Manages multiple work queues across the kernel.
/// Provides named queue lookup and global flush.
pub struct WorkQueueSystem {
    queues: Vec<SigmaWorkQueue>,
    bound_queues: Vec<BoundWorkQueue>,
}

impl WorkQueueSystem {
    /// Create a new empty system.
    pub fn new() -> Self {
        Self { queues: Vec::new(), bound_queues: Vec::new() }
    }

    /// Register a new work queue.
    pub fn register(&mut self, q: SigmaWorkQueue) {
        self.queues.push(q);
    }

    /// Register a CPU-bound queue.
    pub fn register_bound(&mut self, q: BoundWorkQueue) {
        self.bound_queues.push(q);
    }

    /// Flush all queues. Returns total items executed.
    pub fn flush_all(&mut self) -> usize {
        let mut total = 0;
        for q in &mut self.queues { total += q.flush(); }
        for q in &mut self.bound_queues { total += q.flush_if_online(); }
        total
    }

    /// Find a queue by name and submit work.
    pub fn submit_to(&mut self, queue_name: &str, item: WorkItem) -> Result<(), &'static str> {
        for q in &mut self.queues {
            if q.name() == queue_name {
                return q.submit(item);
            }
        }
        Err("queue not found")
    }
}

impl Default for WorkQueueSystem {
    fn default() -> Self { Self::new() }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static EXEC_COUNT: AtomicU64 = AtomicU64::new(0);
    fn work_fn(_: u64) { EXEC_COUNT.fetch_add(1, Ordering::Relaxed); }

    #[test]
    fn test_basic_submit_flush() {
        EXEC_COUNT.store(0, Ordering::Relaxed);
        let mut wq = SigmaWorkQueue::new("test", false);
        wq.submit(WorkItem::new(work_fn, 0, "item1", WorkPriority::Normal)).unwrap();
        wq.submit(WorkItem::new(work_fn, 0, "item2", WorkPriority::High)).unwrap();
        let count = wq.flush();
        assert_eq!(count, 2);
        assert_eq!(EXEC_COUNT.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn test_priority_ordering() {
        let mut wq = SigmaWorkQueue::new("prio-test", false);
        wq.submit(WorkItem::new(work_fn, 1, "bg", WorkPriority::Background)).unwrap();
        wq.submit(WorkItem::new(work_fn, 2, "critical", WorkPriority::Critical)).unwrap();
        wq.submit(WorkItem::new(work_fn, 3, "normal", WorkPriority::Normal)).unwrap();
        // First item should be Critical
        let item = wq.queue.front().unwrap();
        assert_eq!(item.priority(), WorkPriority::Critical);
    }

    #[test]
    fn test_bounded_queue_drops() {
        let mut wq = SigmaWorkQueue::new_bounded("bounded", false, 2);
        wq.submit(WorkItem::new(work_fn, 0, "a", WorkPriority::Normal)).unwrap();
        wq.submit(WorkItem::new(work_fn, 0, "b", WorkPriority::Normal)).unwrap();
        assert!(wq.submit(WorkItem::new(work_fn, 0, "c", WorkPriority::Normal)).is_err());
        assert_eq!(wq.stats().dropped, 1);
    }

    #[test]
    fn test_delayed_work() {
        EXEC_COUNT.store(0, Ordering::Relaxed);
        let mut wq = SigmaWorkQueue::new("delayed", true);
        wq.submit(WorkItem::new_delayed(work_fn, 0, "delayed", 10_000_000)).unwrap();
        wq.process_one(); // Should not execute — delay not met
        assert_eq!(EXEC_COUNT.load(Ordering::Relaxed), 0);
        wq.advance_time(15_000_000); // Advance past delay
        wq.process_one();
        assert_eq!(EXEC_COUNT.load(Ordering::Relaxed), 1);
    }
}
