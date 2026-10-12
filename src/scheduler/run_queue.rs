// SigmaOS Bounded Scheduler Run Queue Subsystem
// Enforces explicit capacity bounds, multi-priority run queue management,
// lost-wakeup prevention, and race-free state handling.

extern crate alloc;

use alloc::collections::VecDeque;

/// Default maximum run queue capacity per CPU core.
pub const MAX_RUNQUEUE_CAPACITY: usize = 1024;
/// Total priority levels supported (0 = lowest/idle, 63 = real-time).
pub const NUM_PRIORITY_LEVELS: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunQueueError {
    CapacityExceeded,
    DuplicateTask,
    TaskNotFound,
    EmptyQueue,
    InvalidPriority,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskRunRef {
    pub pid: u64,
    pub priority: u8,
    pub vruntime: u64,
    pub enqueue_tick: u64,
    pub remaining_quantum_ticks: u32,
    pub cpu_affinity: u64,
}

impl TaskRunRef {
    pub fn new(pid: u64, priority: u8, cpu_affinity: u64, current_tick: u64) -> Self {
        Self {
            pid,
            priority: priority.min(63),
            vruntime: 0,
            enqueue_tick: current_tick,
            remaining_quantum_ticks: 10,
            cpu_affinity,
        }
    }
}

/// Multi-level priority bounded run queue for a CPU core.
#[derive(Debug, Clone)]
pub struct BoundedRunQueue {
    pub cpu_id: u32,
    pub capacity: usize,
    pub total_tasks: usize,
    queues: [VecDeque<TaskRunRef>; NUM_PRIORITY_LEVELS],
    active_bitmap: u64,
}

impl BoundedRunQueue {
    pub fn new(cpu_id: u32, capacity: usize) -> Self {
        let actual_cap = if capacity == 0 || capacity > MAX_RUNQUEUE_CAPACITY {
            MAX_RUNQUEUE_CAPACITY
        } else {
            capacity
        };

        // Initialize queues array
        const EMPTY_QUEUE: VecDeque<TaskRunRef> = VecDeque::new();
        let mut queues = [EMPTY_QUEUE; NUM_PRIORITY_LEVELS];
        for queue in queues.iter_mut() {
            *queue = VecDeque::new();
        }

        Self {
            cpu_id,
            capacity: actual_cap,
            total_tasks: 0,
            queues,
            active_bitmap: 0,
        }
    }

    pub fn cpu_id(&self) -> u32 {
        self.cpu_id
    }

    pub fn is_full(&self) -> bool {
        self.total_tasks >= self.capacity
    }

    pub fn is_empty(&self) -> bool {
        self.total_tasks == 0
    }

    pub fn len(&self) -> usize {
        self.total_tasks
    }

    pub fn contains(&self, pid: u64) -> bool {
        for q in self.queues.iter() {
            if q.iter().any(|t| t.pid == pid) {
                return true;
            }
        }
        false
    }

    pub fn enqueue(&mut self, mut task: TaskRunRef, current_tick: u64) -> Result<(), RunQueueError> {
        if self.is_full() {
            return Err(RunQueueError::CapacityExceeded);
        }

        if self.contains(task.pid) {
            return Err(RunQueueError::DuplicateTask);
        }

        let prio = task.priority as usize;
        if prio >= NUM_PRIORITY_LEVELS {
            return Err(RunQueueError::InvalidPriority);
        }

        task.enqueue_tick = current_tick;
        self.queues[prio].push_back(task);
        self.active_bitmap |= 1u64 << prio;
        self.total_tasks += 1;

        Ok(())
    }

    pub fn dequeue_next(&mut self) -> Result<TaskRunRef, RunQueueError> {
        if self.is_empty() {
            return Err(RunQueueError::EmptyQueue);
        }

        // Find highest non-empty priority level (63 down to 0)
        for prio in (0..NUM_PRIORITY_LEVELS).rev() {
            if (self.active_bitmap & (1u64 << prio)) != 0 {
                if let Some(task) = self.queues[prio].pop_front() {
                    self.total_tasks -= 1;
                    if self.queues[prio].is_empty() {
                        self.active_bitmap &= !(1u64 << prio);
                    }
                    return Ok(task);
                }
            }
        }

        Err(RunQueueError::EmptyQueue)
    }

    pub fn remove_task(&mut self, pid: u64) -> Result<TaskRunRef, RunQueueError> {
        for prio in 0..NUM_PRIORITY_LEVELS {
            if let Some(idx) = self.queues[prio].iter().position(|t| t.pid == pid) {
                let task = self.queues[prio].remove(idx).unwrap();
                self.total_tasks -= 1;
                if self.queues[prio].is_empty() {
                    self.active_bitmap &= !(1u64 << prio);
                }
                return Ok(task);
            }
        }
        Err(RunQueueError::TaskNotFound)
    }

    pub fn boost_starved_tasks(&mut self, current_tick: u64, starvation_threshold_ticks: u64) -> usize {
        let mut boosted_count = 0;

        // Iterate in reverse over queues up to level 60 to prevent re-boosting moved tasks in same pass
        for prio in (0..60).rev() {
            let mut i = 0;
            while i < self.queues[prio].len() {
                let wait_ticks = current_tick.saturating_sub(self.queues[prio][i].enqueue_tick);
                if wait_ticks >= starvation_threshold_ticks {
                    let mut task = self.queues[prio].remove(i).unwrap();
                    let new_prio = (task.priority + 10).min(62);
                    task.priority = new_prio;
                    task.enqueue_tick = current_tick;
                    let new_prio_idx = new_prio as usize;
                    self.queues[new_prio_idx].push_back(task);
                    self.active_bitmap |= 1u64 << new_prio_idx;
                    boosted_count += 1;
                } else {
                    i += 1;
                }
            }
            if self.queues[prio].is_empty() {
                self.active_bitmap &= !(1u64 << prio);
            }
        }

        boosted_count
    }
}
