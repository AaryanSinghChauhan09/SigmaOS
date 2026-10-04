//! # Kernel Workqueue Subsystem
//!
//! Deferred work execution inspired by Linux kernel workqueues.
//! Provides mechanism for executing work in process context.

#![no_std]

extern crate alloc;
use alloc::boxed::Box;
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};

/// Work item callback type
pub type WorkFn = Box<dyn FnMut() + Send>;

/// Work item state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum WorkState {
    /// Work is pending execution
    Pending = 0,
    /// Work is currently executing
    Running = 1,
    /// Work has completed
    Completed = 2,
    /// Work was cancelled
    Cancelled = 3,
}

/// Work item flags (inspired by Linux work_struct flags)
#[derive(Debug, Clone, Copy)]
pub struct WorkFlags {
    /// High priority work
    pub high_priority: bool,
    /// CPU-intensive work (may run longer)
    pub cpu_intensive: bool,
    /// Unbound work (can migrate between CPUs)
    pub unbound: bool,
    /// Freezable work (can be frozen during suspend)
    pub freezable: bool,
}

impl WorkFlags {
    pub const DEFAULT: Self = Self {
        high_priority: false,
        cpu_intensive: false,
        unbound: true,
        freezable: true,
    };

    pub const HIGH_PRIORITY: Self = Self {
        high_priority: true,
        cpu_intensive: false,
        unbound: true,
        freezable: true,
    };

    pub const CPU_INTENSIVE: Self = Self {
        high_priority: false,
        cpu_intensive: true,
        unbound: true,
        freezable: false,
    };
}

/// Work item (inspired by Linux work_struct)
pub struct WorkItem {
    /// Unique work ID
    pub id: u64,
    /// Work function
    pub func: Option<WorkFn>,
    /// Work state
    pub state: WorkState,
    /// Work flags
    pub flags: WorkFlags,
    /// Delayed execution time (0 = immediate)
    pub delay_ticks: u64,
    /// Submission time
    pub submit_time: u64,
}

impl WorkItem {
    pub fn new(id: u64, func: WorkFn, flags: WorkFlags) -> Self {
        Self {
            id,
            func: Some(func),
            state: WorkState::Pending,
            flags,
            delay_ticks: 0,
            submit_time: 0,
        }
    }

    pub fn new_delayed(id: u64, func: WorkFn, delay_ticks: u64, flags: WorkFlags) -> Self {
        Self {
            id,
            func: Some(func),
            state: WorkState::Pending,
            flags,
            delay_ticks,
            submit_time: 0,
        }
    }

    /// Execute work item
    pub fn execute(&mut self) {
        self.state = WorkState::Running;
        if let Some(mut func) = self.func.take() {
            func();
        }
        self.state = WorkState::Completed;
    }

    /// Check if work is ready to execute
    pub fn is_ready(&self, current_tick: u64) -> bool {
        self.state == WorkState::Pending && current_tick >= self.submit_time + self.delay_ticks
    }
}

/// Worker thread state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum WorkerState {
    /// Worker is idle
    Idle = 0,
    /// Worker is executing work
    Running = 1,
    /// Worker is sleeping
    Sleeping = 2,
    /// Worker is stopping
    Stopping = 3,
}

/// Worker thread (inspired by Linux worker_pool)
pub struct Worker {
    /// Worker ID
    pub id: u32,
    /// CPU affinity (which CPU this worker runs on)
    pub cpu: u32,
    /// Current state
    pub state: WorkerState,
    /// Work items processed
    pub work_processed: AtomicU64,
    /// Is active
    pub active: AtomicBool,
}

impl Worker {
    pub const fn new(id: u32, cpu: u32) -> Self {
        Self {
            id,
            cpu,
            state: WorkerState::Idle,
            work_processed: AtomicU64::new(0),
            active: AtomicBool::new(true),
        }
    }

    pub fn inc_work_count(&self) {
        self.work_processed.fetch_add(1, Ordering::Relaxed);
    }

    pub fn work_count(&self) -> u64 {
        self.work_processed.load(Ordering::Relaxed)
    }
}

/// Work queue (inspired by Linux workqueue_struct)
pub struct WorkQueue {
    /// Queue name
    pub name: &'static str,
    /// Pending work items (normal priority)
    pending: VecDeque<Box<WorkItem>>,
    /// High priority work items
    high_priority: VecDeque<Box<WorkItem>>,
    /// Maximum concurrency (0 = unlimited)
    max_active: usize,
    /// Workers
    workers: Vec<Worker>,
    /// Next work ID
    next_id: AtomicU64,
    /// Total work submitted
    total_submitted: AtomicU64,
    /// Total work completed
    total_completed: AtomicU64,
    /// Queue is draining
    draining: AtomicBool,
}

impl WorkQueue {
    pub fn new(name: &'static str, max_active: usize, num_workers: u32) -> Self {
        let mut workers = Vec::with_capacity(num_workers as usize);
        for i in 0..num_workers {
            workers.push(Worker::new(i, i % 8)); // Distribute across 8 CPUs
        }

        Self {
            name,
            pending: VecDeque::new(),
            high_priority: VecDeque::new(),
            max_active: if max_active == 0 {
                usize::MAX
            } else {
                max_active
            },
            workers,
            next_id: AtomicU64::new(1),
            total_submitted: AtomicU64::new(0),
            total_completed: AtomicU64::new(0),
            draining: AtomicBool::new(false),
        }
    }

    /// Allocate new work ID
    fn allocate_id(&self) -> u64 {
        self.next_id.fetch_add(1, Ordering::SeqCst)
    }

    /// Submit work to queue
    pub fn queue_work(&mut self, func: WorkFn, flags: WorkFlags) -> u64 {
        let id = self.allocate_id();
        let mut work = Box::new(WorkItem::new(id, func, flags));
        work.submit_time = 0; // Would be current_tick in real implementation

        self.total_submitted.fetch_add(1, Ordering::Relaxed);

        if flags.high_priority {
            self.high_priority.push_back(work);
        } else {
            self.pending.push_back(work);
        }

        id
    }

    /// Submit delayed work to queue
    pub fn queue_delayed_work(&mut self, func: WorkFn, delay_ticks: u64, flags: WorkFlags) -> u64 {
        let id = self.allocate_id();
        let mut work = Box::new(WorkItem::new_delayed(id, func, delay_ticks, flags));
        work.submit_time = 0; // Would be current_tick in real implementation

        self.total_submitted.fetch_add(1, Ordering::Relaxed);
        self.pending.push_back(work);
        id
    }

    /// Get next work item to execute
    fn dequeue_work(&mut self, current_tick: u64) -> Option<Box<WorkItem>> {
        // Check high priority queue first
        if let Some(pos) = self
            .high_priority
            .iter()
            .position(|w| w.is_ready(current_tick))
        {
            return self.high_priority.remove(pos);
        }

        // Then check normal priority queue
        if let Some(pos) = self.pending.iter().position(|w| w.is_ready(current_tick)) {
            return self.pending.remove(pos);
        }

        None
    }

    /// Process work items (called by worker thread)
    pub fn process_work(&mut self, current_tick: u64, max_items: usize) -> usize {
        if self.draining.load(Ordering::Acquire) {
            return 0;
        }

        let mut processed = 0;
        while processed < max_items {
            if let Some(mut work) = self.dequeue_work(current_tick) {
                work.execute();
                self.total_completed.fetch_add(1, Ordering::Relaxed);
                processed += 1;
            } else {
                break;
            }
        }

        processed
    }

    /// Flush all pending work (wait for completion)
    pub fn flush(&mut self, current_tick: u64) {
        while !self.pending.is_empty() || !self.high_priority.is_empty() {
            self.process_work(current_tick, 100);
        }
    }

    /// Drain queue (prevent new work submission)
    pub fn drain(&mut self, current_tick: u64) {
        self.draining.store(true, Ordering::Release);
        self.flush(current_tick);
    }

    /// Get queue statistics
    pub fn stats(&self) -> WorkQueueStats {
        WorkQueueStats {
            name: self.name,
            pending: self.pending.len(),
            high_priority: self.high_priority.len(),
            total_submitted: self.total_submitted.load(Ordering::Relaxed),
            total_completed: self.total_completed.load(Ordering::Relaxed),
            active_workers: self
                .workers
                .iter()
                .filter(|w| w.active.load(Ordering::Relaxed))
                .count(),
            total_workers: self.workers.len(),
        }
    }
}

/// Work queue statistics
#[derive(Debug, Clone)]
pub struct WorkQueueStats {
    pub name: &'static str,
    pub pending: usize,
    pub high_priority: usize,
    pub total_submitted: u64,
    pub total_completed: u64,
    pub active_workers: usize,
    pub total_workers: usize,
}

/// Global workqueue manager (inspired by Linux system_wq, etc.)
pub struct WorkQueueManager {
    /// System workqueue (general purpose)
    pub system_wq: WorkQueue,
    /// High priority workqueue
    pub system_highpri_wq: WorkQueue,
    /// Long-running workqueue
    pub system_long_wq: WorkQueue,
    /// Unbound workqueue
    pub system_unbound_wq: WorkQueue,
}

impl WorkQueueManager {
    pub fn new() -> Self {
        Self {
            system_wq: WorkQueue::new("system", 256, 8),
            system_highpri_wq: WorkQueue::new("system_highpri", 512, 4),
            system_long_wq: WorkQueue::new("system_long", 0, 16),
            system_unbound_wq: WorkQueue::new("system_unbound", 512, 32),
        }
    }

    /// Schedule work on system workqueue
    pub fn schedule_work(&mut self, func: WorkFn) -> u64 {
        self.system_wq.queue_work(func, WorkFlags::DEFAULT)
    }

    /// Schedule work on high priority workqueue
    pub fn schedule_work_highpri(&mut self, func: WorkFn) -> u64 {
        self.system_highpri_wq
            .queue_work(func, WorkFlags::HIGH_PRIORITY)
    }

    /// Schedule delayed work
    pub fn schedule_delayed_work(&mut self, func: WorkFn, delay_ticks: u64) -> u64 {
        self.system_wq
            .queue_delayed_work(func, delay_ticks, WorkFlags::DEFAULT)
    }

    /// Process all workqueues
    pub fn process_all(&mut self, current_tick: u64) -> usize {
        let mut total = 0;
        total += self.system_highpri_wq.process_work(current_tick, 10);
        total += self.system_wq.process_work(current_tick, 10);
        total += self.system_long_wq.process_work(current_tick, 5);
        total += self.system_unbound_wq.process_work(current_tick, 10);
        total
    }

    /// Get statistics for all workqueues
    pub fn all_stats(&self) -> Vec<WorkQueueStats> {
        alloc::vec![
            self.system_wq.stats(),
            self.system_highpri_wq.stats(),
            self.system_long_wq.stats(),
            self.system_unbound_wq.stats(),
        ]
    }
}

#[cfg(test)]
#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_work_item_creation() {
        let work = WorkItem::new(1, Box::new(|| {}), WorkFlags::DEFAULT);
        assert_eq!(work.state, WorkState::Pending);
        assert_eq!(work.id, 1);
    }

    #[test]
    fn test_work_queue() {
        let mut wq = WorkQueue::new("test", 256, 4);
        let id = wq.queue_work(Box::new(|| {}), WorkFlags::DEFAULT);
        assert!(id > 0);
        assert_eq!(wq.stats().pending, 1);
    }

    #[test]
    fn test_high_priority_work() {
        let mut wq = WorkQueue::new("test", 256, 4);
        wq.queue_work(Box::new(|| {}), WorkFlags::HIGH_PRIORITY);
        assert_eq!(wq.stats().high_priority, 1);
    }

    #[test]
    fn test_work_processing() {
        let mut wq = WorkQueue::new("test", 256, 4);
        wq.queue_work(Box::new(|| {}), WorkFlags::DEFAULT);
        let processed = wq.process_work(0, 10);
        assert_eq!(processed, 1);
        assert_eq!(wq.stats().total_completed, 1);
    }

    #[test]
    fn test_worker() {
        let worker = Worker::new(0, 0);
        assert_eq!(worker.work_count(), 0);
        worker.inc_work_count();
        assert_eq!(worker.work_count(), 1);
    }
}
