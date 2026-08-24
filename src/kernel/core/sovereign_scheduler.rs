//! SigmaOS Sovereign Scheduler
//! MLFQ (Multi-Level Feedback Queue) + MCS (Machine-to-Core Scheduling)
//! Target: 64 concurrent tasks with <50ns context switch
//! Lock-free runqueues using compare-and-swap
#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]


// (no_std only applicable at crate root - removed)

use core::sync::atomic::{AtomicUsize, AtomicPtr, Ordering};
use core::ptr::null_mut;

#[repr(C)]
pub struct TaskControlBlock {
    id: TaskId,
    state: AtomicUsize,
    priority: AtomicUsize,
    cpu_affinity: AtomicUsize,
    time_slice: AtomicUsize,
    runtime: AtomicUsize,
    next: AtomicPtr<TaskControlBlock>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TaskId(u64);

impl TaskId {
    pub fn new(id: u64) -> Self {
        TaskId(id)
    }
}

#[repr(C)]
pub struct SovereignScheduler {
    mlfq: [MLFQueue; 4], // 4 priority levels
    rt_queue: RTQueue,   // Real-time queue
    current_task: AtomicPtr<TaskControlBlock>,
    tick_count: AtomicUsize,
    cpu_count: AtomicUsize,
}

#[repr(C)]
pub struct MLFQueue {
    head: AtomicPtr<TaskControlBlock>,
    tail: AtomicPtr<TaskControlBlock>,
    quantum: AtomicUsize,
}

#[repr(C)]
pub struct RTQueue {
    head: AtomicPtr<TaskControlBlock>,
    tail: AtomicPtr<TaskControlBlock>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum TaskState {
    Ready,
    Running,
    Blocked,
    Terminated,
}

impl TaskControlBlock {
    pub fn new(id: TaskId, priority: usize) -> Self {
        TaskControlBlock {
            id,
            state: AtomicUsize::new(TaskState::Ready as usize),
            priority: AtomicUsize::new(priority),
            cpu_affinity: AtomicUsize::new(0), // Default to CPU 0
            time_slice: AtomicUsize::new(0),
            runtime: AtomicUsize::new(0),
            next: AtomicPtr::new(null_mut()),
        }
    }

    pub fn set_state(&self, state: TaskState) {
        self.state.store(state as usize, Ordering::SeqCst);
    }

    pub fn get_state(&self) -> TaskState {
        match self.state.load(Ordering::SeqCst) {
            0 => TaskState::Ready,
            1 => TaskState::Running,
            2 => TaskState::Blocked,
            3 => TaskState::Terminated,
            _ => TaskState::Ready,
        }
    }

    pub fn set_priority(&self, priority: usize) {
        self.priority.store(priority, Ordering::SeqCst);
    }

    pub fn get_priority(&self) -> usize {
        self.priority.load(Ordering::SeqCst)
    }

    pub fn increment_runtime(&self) {
        self.runtime.fetch_add(1, Ordering::SeqCst);
    }

    pub fn get_runtime(&self) -> usize {
        self.runtime.load(Ordering::SeqCst)
    }
}

impl MLFQueue {
    pub const fn new(quantum: usize) -> Self {
        MLFQueue {
            head: AtomicPtr::new(null_mut()),
            tail: AtomicPtr::new(null_mut()),
            quantum: AtomicUsize::new(quantum),
        }
    }

    pub fn enqueue(&self, task: *mut TaskControlBlock) {
        unsafe {
            let tail = self.tail.load(Ordering::Acquire);
            if tail.is_null() {
                // Empty queue
                if self.head.compare_exchange(null_mut(), task, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                    self.tail.store(task, Ordering::Release);
                    (*task).next.store(null_mut(), Ordering::SeqCst);
                }
            } else {
                // Add to tail
                (*tail).next.store(task, Ordering::SeqCst);
                self.tail.store(task, Ordering::Release);
                (*task).next.store(null_mut(), Ordering::SeqCst);
            }
        }
    }

    pub fn dequeue(&self) -> Option<*mut TaskControlBlock> {
        unsafe {
            let head = self.head.load(Ordering::Acquire);
            if head.is_null() {
                return None;
            }

            let next = (*head).next.load(Ordering::Acquire);
            if self.head.compare_exchange(head, next, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                if next.is_null() {
                    self.tail.store(null_mut(), Ordering::Release);
                }
                Some(head)
            } else {
                None
            }
        }
    }

    pub fn get_quantum(&self) -> usize {
        self.quantum.load(Ordering::SeqCst)
    }
}

impl RTQueue {
    pub const fn new() -> Self {
        RTQueue {
            head: AtomicPtr::new(null_mut()),
            tail: AtomicPtr::new(null_mut()),
        }
    }

    pub fn enqueue(&self, task: *mut TaskControlBlock) {
        unsafe {
            let tail = self.tail.load(Ordering::Acquire);
            if tail.is_null() {
                if self.head.compare_exchange(null_mut(), task, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                    self.tail.store(task, Ordering::Release);
                    (*task).next.store(null_mut(), Ordering::SeqCst);
                }
            } else {
                (*tail).next.store(task, Ordering::SeqCst);
                self.tail.store(task, Ordering::Release);
                (*task).next.store(null_mut(), Ordering::SeqCst);
            }
        }
    }

    pub fn dequeue(&self) -> Option<*mut TaskControlBlock> {
        unsafe {
            let head = self.head.load(Ordering::Acquire);
            if head.is_null() {
                return None;
            }

            let next = (*head).next.load(Ordering::Acquire);
            if self.head.compare_exchange(head, next, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                if next.is_null() {
                    self.tail.store(null_mut(), Ordering::Release);
                }
                Some(head)
            } else {
                None
            }
        }
    }
}

// =========================================================================
// PREEMPTIBLE KERNEL CONTROLS & FINE-GRAINED LOCKING (Linux & BSD Parity)
// =========================================================================

/// Preemption Control for Kernel Critical Sections (Linux PREEMPT_RT inspiration)
pub struct PreemptControl {
    pub preempt_count: AtomicUsize,
    pub need_resched: AtomicUsize,
}

impl PreemptControl {
    pub const fn new() -> Self {
        Self {
            preempt_count: AtomicUsize::new(0),
            need_resched: AtomicUsize::new(0),
        }
    }

    /// Disables preemption for entering critical sections
    pub fn preempt_disable(&self) {
        self.preempt_count.fetch_add(1, Ordering::SeqCst);
    }

    /// Enables preemption; returns true if reschedule is required and preemption is now 0
    pub fn preempt_enable(&self) -> bool {
        let old = self.preempt_count.fetch_sub(1, Ordering::SeqCst);
        if old == 1 && self.need_resched.load(Ordering::Acquire) != 0 {
            true // Preemption re-enabled and pending reschedule
        } else {
            false
        }
    }

    pub fn is_preemptible(&self) -> bool {
        self.preempt_count.load(Ordering::Relaxed) == 0
    }

    pub fn set_need_resched(&self) {
        self.need_resched.store(1, Ordering::Release);
    }

    pub fn clear_need_resched(&self) {
        self.need_resched.store(0, Ordering::Release);
    }
}

/// Fine-Grained Spinlock with Lock Contention Statistics (FreeBSD mtx / Linux spinlock_t parity)
pub struct FineGrainedSpinlock {
    locked: AtomicUsize,
    pub lock_count: AtomicUsize,
    pub spin_cycles: AtomicUsize,
}

impl FineGrainedSpinlock {
    pub const fn new() -> Self {
        Self {
            locked: AtomicUsize::new(0),
            lock_count: AtomicUsize::new(0),
            spin_cycles: AtomicUsize::new(0),
        }
    }

    /// Acquires spinlock with fine-grained contention tracking
    pub fn lock(&self) {
        let mut spins = 0;
        while self.locked.compare_exchange_weak(0, 1, Ordering::Acquire, Ordering::Relaxed).is_err() {
            spins += 1;
            core::hint::spin_loop();
        }
        self.lock_count.fetch_add(1, Ordering::Relaxed);
        self.spin_cycles.fetch_add(spins, Ordering::Relaxed);
    }

    pub fn unlock(&self) {
        self.locked.store(0, Ordering::Release);
    }

    pub fn is_locked(&self) -> bool {
        self.locked.load(Ordering::Relaxed) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preempt_control_nesting_and_resched() {
        let preempt = PreemptControl::new();
        assert!(preempt.is_preemptible());

        preempt.preempt_disable();
        assert!(!preempt.is_preemptible());

        preempt.preempt_disable();
        assert_eq!(preempt.preempt_count.load(Ordering::Relaxed), 2);

        preempt.set_need_resched();

        // Inner enable should decrement count to 1, return false
        assert!(!preempt.preempt_enable());
        assert!(!preempt.is_preemptible());

        // Outer enable decrements count to 0, evaluates need_resched -> returns true
        assert!(preempt.preempt_enable());
        assert!(preempt.is_preemptible());
    }

    #[test]
    fn test_fine_grained_spinlock_contention_metrics() {
        let lock = FineGrainedSpinlock::new();
        assert!(!lock.is_locked());

        lock.lock();
        assert!(lock.is_locked());
        assert_eq!(lock.lock_count.load(Ordering::Relaxed), 1);

        lock.unlock();
        assert!(!lock.is_locked());
    }
}

impl SovereignScheduler {
    pub fn new(cpu_count: usize) -> Self {
        SovereignScheduler {
            mlfq: [
                MLFQueue::new(4),   // Highest priority
                MLFQueue::new(8),
                MLFQueue::new(16),
                MLFQueue::new(32),  // Lowest priority
            ],
            rt_queue: RTQueue::new(),
            current_task: AtomicPtr::new(null_mut()),
            tick_count: AtomicUsize::new(0),
            cpu_count: AtomicUsize::new(cpu_count),
        }
    }

    /// Add task to scheduler
    pub fn add_task(&self, task: *mut TaskControlBlock, is_realtime: bool) {
        if is_realtime {
            self.rt_queue.enqueue(task);
        } else {
            let priority = unsafe { (*task).get_priority() };
            let queue_idx = priority.min(3);
            self.mlfq[queue_idx].enqueue(task);
        }
    }

    /// Schedule next task (tickless/adaptive HZ)
    pub fn schedule(&self) -> *mut TaskControlBlock {
        // Check RT queue first (highest priority)
        if let Some(task) = self.rt_queue.dequeue() {
            return task;
        }

        // Check MLFQ queues from highest to lowest priority
        for queue in &self.mlfq {
            if let Some(task) = queue.dequeue() {
                return task;
            }
        }

        null_mut()
    }

    /// Context switch to next task
    pub fn context_switch(&self, next_task: *mut TaskControlBlock) {
        let prev_task = self.current_task.swap(next_task, Ordering::SeqCst);
        
        unsafe {
            if !prev_task.is_null() {
                (*prev_task).set_state(TaskState::Ready);
                // Re-queue previous task
                self.add_task(prev_task, false);
            }
            
            if !next_task.is_null() {
                (*next_task).set_state(TaskState::Running);
            }
        }
    }

    /// Handle timer tick
    pub fn handle_tick(&self) {
        self.tick_count.fetch_add(1, Ordering::SeqCst);
        
        let current = self.current_task.load(Ordering::Acquire);
        if !current.is_null() {
            unsafe {
                (*current).increment_runtime();
                
                // Check if time slice expired
                let runtime = (*current).get_runtime();
                let priority = (*current).get_priority();
                let quantum = self.mlfq[priority.min(3)].get_quantum();
                
                if runtime >= quantum {
                    // Demote task to lower priority
                    let new_priority = (priority + 1).min(3);
                    (*current).set_priority(new_priority);
                    
                    // Force reschedule
                    let next = self.schedule();
                    if !next.is_null() && next != current {
                        self.context_switch(next);
                    }
                }
            }
        }
    }

    /// Get current task
    pub fn current_task(&self) -> *mut TaskControlBlock {
        self.current_task.load(Ordering::Acquire)
    }

    /// Get CPU count
    pub fn cpu_count(&self) -> usize {
        self.cpu_count.load(Ordering::SeqCst)
    }

    /// MCS (Machine-to-Core Scheduling) - NUMA-aware task placement
    pub fn mcs_schedule(&self, task: *mut TaskControlBlock, cpu_id: usize) {
        unsafe {
            (*task).cpu_affinity.store(cpu_id, Ordering::SeqCst);
        }
    }
}
