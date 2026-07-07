//! sigma_scheduler.rs — O(1) Priority Scheduler (no_std Rust)
//! Replaces any high-level language scheduling stubs.
//! Uses a fixed bitmap-indexed run queue — zero heap allocation.

#![no_std]
#![allow(dead_code)]

pub const MAX_PRIO: usize = 32;
pub const MAX_TASKS_PER_PRIO: usize = 16;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TaskState {
    Ready,
    Running,
    Sleeping,
    Zombie,
}

#[derive(Clone, Copy)]
pub struct TaskDescriptor {
    pub tid:        u32,
    pub priority:   u8,
    pub state:      TaskState,
    pub time_slice: u32,
    pub stack_ptr:  usize,
}

impl TaskDescriptor {
    pub const fn empty() -> Self {
        Self { tid: 0, priority: 0, state: TaskState::Zombie,
               time_slice: 0, stack_ptr: 0 }
    }
}

/// O(1) priority scheduler — bitmap tracks which priority levels are non-empty.
pub struct Scheduler {
    queues:   [[TaskDescriptor; MAX_TASKS_PER_PRIO]; MAX_PRIO],
    heads:    [usize; MAX_PRIO],
    lens:     [usize; MAX_PRIO],
    /// Bitmask: bit N set means priority level N has at least one Ready task.
    prio_bitmap: u32,
    current_tid: u32,
}

impl Scheduler {
    pub const fn new() -> Self {
        Self {
            queues:      [[TaskDescriptor::empty(); MAX_TASKS_PER_PRIO]; MAX_PRIO],
            heads:       [0; MAX_PRIO],
            lens:        [0; MAX_PRIO],
            prio_bitmap: 0,
            current_tid: 0,
        }
    }

    pub fn enqueue(&mut self, task: TaskDescriptor) -> Result<(), &'static str> {
        let p = task.priority as usize;
        if p >= MAX_PRIO { return Err("Priority out of range"); }
        if self.lens[p] >= MAX_TASKS_PER_PRIO { return Err("Queue full"); }

        let tail = (self.heads[p] + self.lens[p]) % MAX_TASKS_PER_PRIO;
        self.queues[p][tail] = task;
        self.lens[p] += 1;
        self.prio_bitmap |= 1 << p;
        Ok(())
    }

    /// Pick next task using O(1) leading-zeros on the bitmap.
    pub fn schedule_next(&mut self) -> Option<&TaskDescriptor> {
        if self.prio_bitmap == 0 { return None; }
        // Highest set bit = highest priority level
        let prio = 31 - self.prio_bitmap.leading_zeros() as usize;
        let head = self.heads[prio];
        let task = &self.queues[prio][head];
        self.current_tid = task.tid;
        Some(task)
    }

    /// Consume the head of the given priority level (task completed its slice).
    pub fn dequeue(&mut self, prio: usize) {
        if prio >= MAX_PRIO || self.lens[prio] == 0 { return; }
        self.heads[prio] = (self.heads[prio] + 1) % MAX_TASKS_PER_PRIO;
        self.lens[prio] -= 1;
        if self.lens[prio] == 0 {
            self.prio_bitmap &= !(1 << prio);
        }
    }
}
