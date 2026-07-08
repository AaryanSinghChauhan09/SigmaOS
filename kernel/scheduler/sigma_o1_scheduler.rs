//! SigmaOS O(1) Work-Stealing Scheduler
//! BUG-002 Fix: Implements O(1) work-stealing scheduler instead of O(n) scan
//! Uses per-CPU run queues and multi-level feedback queue (MLFQ) for efficiency
//! Inspired by Linux CFS and work-stealing schedulers

#![no_std]
#![allow(dead_code)]

// ─── Kernel Primitive Types ────────────────────────────────────────────────────────

type U8  = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;
type I64 = i64;
type Bool = bool;
type Usize = usize;

// ─── Task Control Block ─────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TaskState {
    Ready = 0,
    Running = 1,
    Blocked = 2,
    Terminated = 3,
}

/// Task Control Block - represents a task in the scheduler
#[repr(C)]
pub struct TaskControlBlock {
    pub task_id: U64,
    pub priority: U8,
    pub state: TaskState,
    pub cpu_affinity: U8,
    pub runtime: U64,
    pub quantum_remaining: U32,
    pub next: *mut TaskControlBlock,
    pub prev: *mut TaskControlBlock,
}

// ─── Per-CPU Run Queue (O(1) operations) ───────────────────────────────────────

/// Per-CPU run queue with O(1) enqueue/dequeue operations
#[repr(C)]
pub struct PerCpuRunQueue {
    pub head: *mut TaskControlBlock,
    pub tail: *mut TaskControlBlock,
    pub count: Usize,
    pub lock: I32,
}

impl PerCpuRunQueue {
    /// Initialize an empty run queue
    pub const fn new() -> Self {
        PerCpuRunQueue {
            head: core::ptr::null_mut(),
            tail: core::ptr::null_mut(),
            count: 0,
            lock: 0,
        }
    }

    /// Enqueue task at tail (O(1))
    pub unsafe fn enqueue(&mut self, task: *mut TaskControlBlock) {
        if task.is_null() {
            return;
        }

        // Initialize task's next/prev pointers
        (*task).next = core::ptr::null_mut();
        (*task).prev = self.tail;

        if self.tail.is_null() {
            // Queue is empty
            self.head = task;
            self.tail = task;
        } else {
            // Add to tail
            (*self.tail).next = task;
            self.tail = task;
        }

        self.count += 1;
    }

    /// Dequeue task from head (O(1))
    pub unsafe fn dequeue(&mut self) -> *mut TaskControlBlock {
        if self.head.is_null() {
            return core::ptr::null_mut();
        }

        let task = self.head;
        self.head = (*task).next;

        if self.head.is_null() {
            // Queue is now empty
            self.tail = core::ptr::null_mut();
        } else {
            (*self.head).prev = core::ptr::null_mut();
        }

        (*task).next = core::ptr::null_mut();
        (*task).prev = core::ptr::null_mut();
        self.count -= 1;

        task
    }

    /// Remove specific task from queue (O(1) with prev pointer)
    pub unsafe fn remove(&mut self, task: *mut TaskControlBlock) -> Bool {
        if task.is_null() {
            return false;
        }

        let prev = (*task).prev;
        let next = (*task).next;

        if !prev.is_null() {
            (*prev).next = next;
        } else {
            // Task was head
            self.head = next;
        }

        if !next.is_null() {
            (*next).prev = prev;
        } else {
            // Task was tail
            self.tail = prev;
        }

        (*task).next = core::ptr::null_mut();
        (*task).prev = core::ptr::null_mut();
        self.count -= 1;

        true
    }

    /// Check if queue is empty (O(1))
    pub fn is_empty(&self) -> Bool {
        self.head.is_null()
    }
}

// ─── Multi-Level Feedback Queue (MLFQ) ───────────────────────────────────────────

const MLFQ_LEVELS: usize = 8;

/// Multi-level feedback queue scheduler
#[repr(C)]
pub struct MlfqScheduler {
    pub queues: [PerCpuRunQueue; MLFQ_LEVELS],
    pub current_level: usize,
    pub quantum: [U32; MLFQ_LEVELS],
    pub total_tasks: Usize,
}

impl MlfqScheduler {
    /// Initialize MLFQ with default quantums
    pub const fn new() -> Self {
        MlfqScheduler {
            queues: [
                PerCpuRunQueue::new(),
                PerCpuRunQueue::new(),
                PerCpuRunQueue::new(),
                PerCpuRunQueue::new(),
                PerCpuRunQueue::new(),
                PerCpuRunQueue::new(),
                PerCpuRunQueue::new(),
                PerCpuRunQueue::new(),
            ],
            current_level: 0,
            quantum: [10, 20, 40, 80, 160, 320, 640, 1280], // Exponential quantums
            total_tasks: 0,
        }
    }

    /// Add task to appropriate queue level (O(1))
    pub unsafe fn enqueue_task(&mut self, task: *mut TaskControlBlock, level: usize) {
        if level >= MLFQ_LEVELS {
            return;
        }

        (*task).quantum_remaining = self.quantum[level];
        self.queues[level].enqueue(task);
        self.total_tasks += 1;
    }

    /// Get next task to run (O(1) amortized)
    pub unsafe fn pick_next_task(&mut self) -> *mut TaskControlBlock {
        // Find highest priority non-empty queue
        for level in 0..MLFQ_LEVELS {
            if !self.queues[level].is_empty() {
                self.current_level = level;
                let task = self.queues[level].dequeue();
                if !task.is_null() {
                    self.total_tasks -= 1;
                }
                return task;
            }
        }

        core::ptr::null_mut()
    }

    /// Requeue task after time slice (O(1))
    pub unsafe fn requeue_task(&mut self, task: *mut TaskControlBlock, used_full_quantum: Bool) {
        if task.is_null() {
            return;
        }

        let new_level = if used_full_quantum {
            // Demote to lower priority queue
            if self.current_level < MLFQ_LEVELS - 1 {
                self.current_level + 1
            } else {
                MLFQ_LEVELS - 1
            }
        } else {
            // Keep at current level (didn't use full quantum)
            self.current_level
        };

        self.enqueue_task(task, new_level);
    }

    /// Priority boost: move all tasks to highest priority (O(n) but rare)
    pub unsafe fn priority_boost(&mut self) {
        // Collect all tasks
        let mut all_tasks: [*mut TaskControlBlock; 1024] = [core::ptr::null_mut(); 1024];
        let mut task_count = 0;

        for level in 0..MLFQ_LEVELS {
            while !self.queues[level].is_empty() && task_count < 1024 {
                let task = self.queues[level].dequeue();
                if !task.is_null() {
                    all_tasks[task_count] = task;
                    task_count += 1;
                }
            }
        }

        // Re-enqueue all at highest priority
        for i in 0..task_count {
            if !all_tasks[i].is_null() {
                self.enqueue_task(all_tasks[i], 0);
            }
        }

        self.current_level = 0;
    }
}

// ─── O(1) Work-Stealing Scheduler ───────────────────────────────────────────────

const MAX_CPUS: usize = 256;

/// O(1) work-stealing scheduler with per-CPU queues
#[repr(C)]
pub struct O1WorkStealingScheduler {
    pub mlfq: MlfqScheduler,
    pub cpu_count: usize,
    pub per_cpu_queues: [PerCpuRunQueue; MAX_CPUS],
    pub current_cpu: usize,
    pub initialized: Bool,
}

impl O1WorkStealingScheduler {
    /// Initialize scheduler with given CPU count
    pub const fn new() -> Self {
        O1WorkStealingScheduler {
            mlfq: MlfqScheduler::new(),
            cpu_count: 1,
            per_cpu_queues: [PerCpuRunQueue::new(); MAX_CPUS],
            current_cpu: 0,
            initialized: false,
        }
    }

    /// Initialize scheduler
    pub unsafe fn init(&mut self, cpu_count: usize) {
        self.cpu_count = cpu_count.min(MAX_CPUS);
        self.initialized = true;
    }

    /// Add task to current CPU's queue (O(1))
    pub unsafe fn add_task(&mut self, task: *mut TaskControlBlock) {
        if !self.initialized || task.is_null() {
            return;
        }

        self.per_cpu_queues[self.current_cpu].enqueue(task);
    }

    /// Pick next task for current CPU (O(1) with work-stealing fallback)
    pub unsafe fn pick_next_task(&mut self) -> *mut TaskControlBlock {
        if !self.initialized {
            return core::ptr::null_mut();
        }

        // Try current CPU's queue first
        let task = self.per_cpu_queues[self.current_cpu].dequeue();
        if !task.is_null() {
            return task;
        }

        // Work-stealing: try to steal from other CPUs (O(1) per attempt)
        for i in 0..self.cpu_count {
            if i != self.current_cpu {
                let stolen = self.per_cpu_queues[i].dequeue();
                if !stolen.is_null() {
                    return stolen;
                }
            }
        }

        // Fall back to MLFQ
        self.mlfq.pick_next_task()
    }

    /// Requeue task after time slice (O(1))
    pub unsafe fn requeue_task(&mut self, task: *mut TaskControlBlock, used_full_quantum: Bool) {
        if !self.initialized || task.is_null() {
            return;
        }

        self.mlfq.requeue_task(task, used_full_quantum);
    }

    /// Switch to next CPU for load balancing (O(1))
    pub unsafe fn switch_cpu(&mut self) {
        self.current_cpu = (self.current_cpu + 1) % self.cpu_count;
    }

    /// Get task count for current CPU (O(1))
    pub unsafe fn get_task_count(&self) -> Usize {
        if !self.initialized {
            return 0;
        }

        self.per_cpu_queues[self.current_cpu].count
    }

    /// Get total task count across all CPUs (O(n) but rarely used)
    pub unsafe fn get_total_task_count(&self) -> Usize {
        if !self.initialized {
            return 0;
        }

        let mut total = 0;
        for i in 0..self.cpu_count {
            total += self.per_cpu_queues[i].count;
        }
        total + self.mlfq.total_tasks
    }
}

// ─── Global Scheduler Instance ─────────────────────────────────────────────────

static mut SCHEDULER: O1WorkStealingScheduler = O1WorkStealingScheduler::new();

// ─── C ABI Exports ───────────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn o1_scheduler_init(cpu_count: usize) -> I32 {
    SCHEDULER.init(cpu_count);
    0
}

#[no_mangle]
pub unsafe extern "C" fn o1_scheduler_add_task(task: *mut TaskControlBlock) -> I32 {
    SCHEDULER.add_task(task);
    0
}

#[no_mangle]
pub unsafe extern "C" fn o1_scheduler_pick_next() -> *mut TaskControlBlock {
    SCHEDULER.pick_next_task()
}

#[no_mangle]
pub unsafe extern "C" fn o1_scheduler_requeue(task: *mut TaskControlBlock, used_full_quantum: Bool) -> I32 {
    SCHEDULER.requeue_task(task, used_full_quantum);
    0
}

#[no_mangle]
pub unsafe extern "C" fn o1_scheduler_switch_cpu() {
    SCHEDULER.switch_cpu();
}

#[no_mangle]
pub unsafe extern "C" fn o1_scheduler_get_task_count() -> Usize {
    SCHEDULER.get_task_count()
}

#[no_mangle]
pub unsafe extern "C" fn o1_scheduler_get_total_count() -> Usize {
    SCHEDULER.get_total_task_count()
}

#[no_mangle]
pub unsafe extern "C" fn o1_scheduler_priority_boost() {
    SCHEDULER.mlfq.priority_boost();
}
