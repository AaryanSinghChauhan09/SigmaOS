#![no_std]

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Idle,
    Low,
    Normal,
    High,
    Realtime,
}

#[derive(Debug, Clone, Copy)]
pub struct TaskId(pub u32);

#[derive(Debug, Clone, Copy)]
pub struct Task {
    pub id: TaskId,
    pub vruntime: u64,
    pub priority: u32,
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.vruntime == other.vruntime
    }
}


/// Process state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Running,
    Ready,
    Blocked,
    Terminated,
}

/// Process control block (PCB) enhanced with EEVDF vruntime and deadline models
/// Cache-line aligned to 64 bytes to prevent cache bouncing on SMP systems
#[derive(Debug, Clone)]
#[repr(C, align(64))]
pub struct Process {
    pub pid: u64,
    pub name: String,
    pub priority: Priority,
    pub state: ProcessState,
    pub runtime: Duration,
    pub virtual_runtime: u64,  // EEVDF vruntime (ticks)
    pub virtual_deadline: u64, // EEVDF virtual deadline
    pub time_slice: Duration,
}

#[derive(Debug, Clone)]
pub struct NumaNode {
    pub node_id: u32,
    pub processor_ids: Vec<u32>,
}

pub struct WorkStealingQueue {
    pub processor_id: u32,
    pub tasks: Vec<u64>, // List of process pids in the queue
}

impl WorkStealingQueue {
    pub fn new(processor_id: u32) -> Self {
        Self {
            processor_id,
            tasks: Vec::new(),
        }
    }

    pub fn push_task(&mut self, pid: u64) {
        self.tasks.push(pid);
    }

    pub fn pop_task(&mut self) -> Option<u64> {
        self.tasks.pop()
    }

    /// Steals a task from another processor's queue to balance the SMP work load
    pub fn steal_task_from(&mut self, other: &mut WorkStealingQueue) -> Option<u64> {
        if other.tasks.len() > 1 {
            // Steal the oldest task from the bottom of other's queue to minimize lock contention
            let stolen = other.tasks.remove(0);
            self.tasks.push(stolen);
            Some(stolen)
        } else {
            None
        }
    }
}

impl Process {
    pub fn new(pid: u64, name: String, priority: Priority) -> Self {
        Self {
            pid,
            name,
            priority,
            state: ProcessState::Ready,
            runtime: Duration::from_secs(0),
            virtual_runtime: 0,
            virtual_deadline: 0,
            time_slice: Duration::from_millis(10),
        }
    }

    pub fn get_weight(&self) -> u64 {
        match self.priority {
            Priority::Idle => 1,
            Priority::Low => 2,
            Priority::Normal => 4,
            Priority::High => 8,
            Priority::Realtime => 16,
        }
    }

    pub fn update_virtual_deadline(&mut self, system_vtime: u64) {
        let weight = self.get_weight();
        // deadline = vruntime + (q / w) where q is time slice slice equivalent ticks (10)
        let q = 10;
        self.virtual_deadline = self.virtual_runtime + (q / weight).max(1);
    }
}

impl Eq for Task {}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        self.vruntime.cmp(&other.vruntime)
    }
}

pub struct CfsScheduler {
    tasks: [Option<Task>; 64],
    task_count: usize,
    current_time: u64,
}

impl CfsScheduler {
    pub const fn new() -> Self {
        CfsScheduler {
            tasks: [None; 64],
            task_count: 0,
            current_time: 0,
        }
    }

    pub fn add_task(&mut self, task: Task) {
        if self.task_count < 64 {
            self.tasks[self.task_count] = Some(task);
            self.task_count += 1;
            self.sort_tasks();
        }
    }

    pub fn pick_next_task(&mut self) -> Option<Task> {
        if self.task_count > 0 {
            let task = self.tasks[0].take();
            self.tasks[0] = self.tasks[self.task_count - 1];
            self.tasks[self.task_count - 1] = None;
            self.task_count -= 1;
            self.sort_tasks();
            task
        } else {
            None
        }
    }

    fn sort_tasks(&mut self) {
        // Simple insertion sort for now since we don't have BTreeMap in no_std
        for i in 1..self.task_count {
            let mut j = i;
            while j > 0 && self.tasks[j - 1].unwrap().vruntime > self.tasks[j].unwrap().vruntime {
                self.tasks.swap(j - 1, j);
                j -= 1;
            }
        }
    }
}

impl Default for CfsScheduler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheduler_creation() {
        let scheduler = CfsScheduler::new();
        assert_eq!(scheduler.task_count, 0);
    }

    #[test]
    fn test_add_process() {
        let mut scheduler = CfsScheduler::new();
        let task = Task { id: TaskId(1), vruntime: 10, priority: 1 };
        scheduler.add_task(task);
        assert_eq!(scheduler.task_count, 1);
    }

    #[test]
    fn test_schedule() {
        let mut scheduler = CfsScheduler::new();
        let task = Task { id: TaskId(1), vruntime: 10, priority: 1 };
        scheduler.add_task(task);

        let scheduled = scheduler.pick_next_task();
        assert!(scheduled.is_some());
    }

    #[test]
    fn test_priority_ordering() {
        let p1 = Priority::Low;
        let p2 = Priority::High;
        assert!(p2 > p1);
    }

    #[test]
    fn test_eevdf_deadline_and_weight() {
        let mut p1 = Process::new(1, "low-prio".to_string(), Priority::Low);
        let mut p2 = Process::new(2, "high-prio".to_string(), Priority::High);

        p1.update_virtual_deadline(0);
        p2.update_virtual_deadline(0);

        // High priority must have a tighter/earlier virtual deadline for the same vruntime!
        assert!(p2.virtual_deadline < p1.virtual_deadline);
    }

    #[test]
    fn test_work_stealing_and_numa_alignment() {
        // 1. Assert CPU cache line alignment sizing (Process aligned to 64 bytes)
        assert_eq!(core::mem::align_of::<Process>(), 64);

        let mut numa_nodes = Vec::new();

        // 2. Setup NUMA nodes
        let node0 = NumaNode {
            node_id: 0,
            processor_ids: vec![0, 1],
        };
        let node1 = NumaNode {
            node_id: 1,
            processor_ids: vec![2, 3],
        };
        numa_nodes.push(node0);
        numa_nodes.push(node1);

        // 3. Setup work stealing run queues
        let mut q0 = WorkStealingQueue::new(0);
        let mut q1 = WorkStealingQueue::new(1);

        // Populate q1 with multiple tasks
        q1.push_task(101);
        q1.push_task(102);
        q1.push_task(103);

        // Let queue 0 steal a task from queue 1 to balance load
        assert_eq!(q0.tasks.len(), 0);
        let stolen_pid = q0.steal_task_from(&mut q1).unwrap();
        assert_eq!(stolen_pid, 101); // oldest task is stolen from bottom of deque
        assert_eq!(q0.tasks.len(), 1);
        assert_eq!(q1.tasks.len(), 2);
    }
}
