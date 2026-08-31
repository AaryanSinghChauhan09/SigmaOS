# ⏱️ Predictive Scheduler Shard Blueprint (SovereignSched)

Inspired by **Linux's Completely Fair Scheduler (CFS)**, **RT-PREEMPT**, and modern predictive scheduling, this document defines a complete, functional, `#![no_std]` predictive Multi-Level Feedback Queue (MLFQ) scheduler. It uses Object-Oriented Programming (OOP) principles, contains zero external dependencies, and implements dynamic task scaling and wait times.

***

## 🏗️ Component Implementation Source Code

```rust
// SovereignSched: Predictive MLFQ + CFS Thread Scheduler
// Zero-dependency, #![no_std] compliant, OOP-centric

use core::cell::RefCell;

/// Task execution state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Ready,
    Running,
    Blocked,
    Terminated,
}

/// Task priorities (conforms to MLFQ queue mappings)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    RealTime = 0, // Highest priority, strict round-robin/FIFO
    Interactive = 1, // High priority, CFS-like virtual runtime tracking
    Batch = 2,       // Standard throughput queue
    Idle = 3,        // Background execution
}

/// Struct representing a single schedulable thread of execution
#[derive(Debug, Clone)]
pub struct Task {
    pub tid: u32,
    pub name: &'static str,
    pub state: TaskState,
    pub priority: Priority,
    pub virtual_runtime: u64, // Virtual runtime for completely fair scheduling
    pub cpu_time_used: u64,   // Elapsed CPU execution cycles
    pub wait_time: u64,       // Cycle count task has spent waiting in Ready queue
    pub time_slice: u64,      // Cycles allowed in active execution slot
}

impl Task {
    pub fn new(tid: u32, name: &'static str, priority: Priority) -> Self {
        let time_slice = match priority {
            Priority::RealTime => 10,
            Priority::Interactive => 20,
            Priority::Batch => 40,
            Priority::Idle => 80,
        };

        Self {
            tid,
            name,
            state: TaskState::Ready,
            priority,
            virtual_runtime: 0,
            cpu_time_used: 0,
            wait_time: 0,
            time_slice,
        }
    }

    /// Increments cpu cycles and recalculates CFS virtual runtime based on priority weighting
    pub fn tick(&mut self, cycles: u64) {
        self.cpu_time_used += cycles;

        // CFS Weighting factor: RealTime has low penalty, Idle has high penalty
        let weight = match self.priority {
            Priority::RealTime => 1,
            Priority::Interactive => 2,
            Priority::Batch => 4,
            Priority::Idle => 16,
        };
        self.virtual_runtime += cycles * weight;
    }
}

/// Object-Oriented Scheduler Queue Manager
pub struct PredictiveScheduler {
    pub ready_queues: [RefCell<[Option<Task>; 32]>; 4], // 4 priority levels, max 32 tasks per queue
    pub active_task: Option<u32>,
    pub total_cycles_run: u64,
}

impl PredictiveScheduler {
    pub fn new() -> Self {
        // Initialize fixed-size static task arrays to preserve zero-heap allocation guarantees
        const EMPTY_TASK: Option<Task> = None;
        Self {
            ready_queues: [
                RefCell::new([EMPTY_TASK; 32]),
                RefCell::new([EMPTY_TASK; 32]),
                RefCell::new([EMPTY_TASK; 32]),
                RefCell::new([EMPTY_TASK; 32]),
            ],
            active_task: None,
            total_cycles_run: 0,
        }
    }

    /// Registers a new task into the appropriate MLFQ queue
    pub fn add_task(&self, task: Task) -> Result<(), &'static str> {
        let q_idx = task.priority as usize;
        let mut queue = self.ready_queues[q_idx].borrow_mut();

        for slot in queue.iter_mut() {
            if slot.is_none() {
                *slot = Some(task);
                return Ok(());
            }
        }
        Err("Scheduler queue limit reached - no available task slots")
    }

    /// Promotes starved tasks that have waited too long in lower-priority queues (Prevents Priority Inversion)
    pub fn prevent_starvation(&self, starvation_threshold: u64) {
        for q_idx in 1..4 {
            let mut lower_queue = self.ready_queues[q_idx].borrow_mut();
            for i in 0..32 {
                if let Some(ref mut task) = lower_queue[i] {
                    if task.wait_time > starvation_threshold {
                        let mut promoted_task = task.clone();
                        promoted_task.priority = match promoted_task.priority {
                            Priority::Interactive => Priority::RealTime,
                            Priority::Batch => Priority::Interactive,
                            Priority::Idle => Priority::Batch,
                            Priority::RealTime => Priority::RealTime,
                        };
                        promoted_task.wait_time = 0;
                        promoted_task.time_slice = match promoted_task.priority {
                            Priority::RealTime => 10,
                            Priority::Interactive => 20,
                            _ => 40,
                        };

                        // Attempt to place in higher queue
                        if self.add_task(promoted_task).is_ok() {
                            lower_queue[i] = None; // Prune from lower queue
                        }
                    }
                }
            }
        }
    }

    /// Selects the next task to run based on priority queues (MLFQ) and lowest virtual runtime (CFS)
    pub fn schedule(&mut self) -> Option<Task> {
        // First check starvation and promote tasks
        self.prevent_starvation(100);

        // Walk through queues starting from highest priority
        for q_idx in 0..4 {
            let mut queue = self.ready_queues[q_idx].borrow_mut();
            let mut best_slot_idx: Option<usize> = None;

            for (idx, slot) in queue.iter().enumerate() {
                if let Some(ref task) = slot {
                    if task.state == TaskState::Ready {
                        match best_slot_idx {
                            None => best_slot_idx = Some(idx),
                            Some(best_idx) => {
                                // Within same queue, select task with lowest virtual runtime (CFS Principle)
                                if task.virtual_runtime < queue[best_idx].as_ref().unwrap().virtual_runtime {
                                    best_slot_idx = Some(idx);
                                }
                            }
                        }
                    }
                }
            }

            if let Some(best_idx) = best_slot_idx {
                let mut selected_task = queue[best_idx].take().unwrap();
                selected_task.state = TaskState::Running;
                self.active_task = Some(selected_task.tid);
                return Some(selected_task);
            }
        }

        self.active_task = None;
        None
    }

    /// Returns a running task back to Ready state, tracking wait times of other tasks
    pub fn yield_task(&mut self, mut task: Task, executed_cycles: u64) -> Result<(), &'static str> {
        task.tick(executed_cycles);
        self.total_cycles_run += executed_cycles;

        if task.cpu_time_used >= task.time_slice {
            // Task exhausted time slice; demote to lower queue
            task.priority = match task.priority {
                Priority::RealTime => Priority::Interactive,
                Priority::Interactive => Priority::Batch,
                _ => Priority::Idle,
            };
            task.cpu_time_used = 0;
            task.time_slice = match task.priority {
                Priority::Interactive => 20,
                Priority::Batch => 40,
                _ => 80,
            };
        }

        task.state = TaskState::Ready;

        // Increment wait times for all other ready tasks in queues
        for q_idx in 0..4 {
            let mut queue = self.ready_queues[q_idx].borrow_mut();
            for slot in queue.iter_mut() {
                if let Some(ref mut ready_task) = slot {
                    if ready_task.state == TaskState::Ready {
                        ready_task.wait_time += executed_cycles;
                    }
                }
            }
        }

        self.add_task(task)
    }
}
```
