//! EEVDF Scheduler with SMP Work Stealing & NUMA Topology Support for SigmaOS

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::time::Duration;

/// Process priority level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Idle = 0,
    Low = 1,
    Normal = 2,
    High = 3,
    Realtime = 4,
}

/// Task Identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TaskId(pub u32);

/// Task structure for CFS compatibility
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

    pub fn update_virtual_deadline(&mut self, _system_vtime: u64) {
        let weight = self.get_weight();
        let q = 10;
        self.virtual_deadline = self.virtual_runtime + (q / weight).max(1);
    }
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
            let stolen = other.tasks.remove(0);
            self.tasks.push(stolen);
            Some(stolen)
        } else {
            None
        }
    }
}

/// EEVDF Scheduler Engine
pub struct Scheduler {
    pub processes: Vec<Process>,
    pub current_time: u64,
    pub system_vtime: u64, // EEVDF System Virtual Time (V)
    pub numa_nodes: Vec<NumaNode>,
    pub run_queues: Vec<WorkStealingQueue>,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            processes: Vec::new(),
            current_time: 0,
            system_vtime: 0,
            numa_nodes: Vec::new(),
            run_queues: Vec::new(),
        }
    }

    pub fn add_process(&mut self, mut process: Process) {
        process.virtual_runtime = self.system_vtime;
        process.update_virtual_deadline(self.system_vtime);
        self.processes.push(process);
    }

    pub fn schedule(&mut self) -> Option<&Process> {
        let mut ready_indices = Vec::new();
        for (idx, p) in self.processes.iter().enumerate() {
            if p.state == ProcessState::Ready {
                ready_indices.push(idx);
            }
        }

        if ready_indices.is_empty() {
            return None;
        }

        let mut eligible_indices = Vec::new();
        for &idx in &ready_indices {
            let p = &self.processes[idx];
            if p.virtual_runtime <= self.system_vtime {
                eligible_indices.push(idx);
            }
        }

        let selected_idx = if !eligible_indices.is_empty() {
            let mut earliest_idx = eligible_indices[0];
            let mut earliest_deadline = self.processes[earliest_idx].virtual_deadline;

            for &idx in &eligible_indices {
                let p = &self.processes[idx];
                if p.virtual_deadline < earliest_deadline {
                    earliest_deadline = p.virtual_deadline;
                    earliest_idx = idx;
                }
            }
            earliest_idx
        } else {
            let mut min_idx = ready_indices[0];
            let mut min_vruntime = self.processes[min_idx].virtual_runtime;

            for &idx in &ready_indices {
                let p = &self.processes[idx];
                if p.virtual_runtime < min_vruntime {
                    min_vruntime = p.virtual_runtime;
                    min_idx = idx;
                }
            }
            min_idx
        };

        Some(&self.processes[selected_idx])
    }

    pub fn tick(&mut self) {
        self.current_time += 1;

        let mut active_count = 0;
        let mut total_vruntime = 0;

        for p in &self.processes {
            if p.state == ProcessState::Ready || p.state == ProcessState::Running {
                active_count += 1;
                total_vruntime += p.virtual_runtime;
            }
        }

        if active_count > 0 {
            let avg_vtime = total_vruntime / active_count;
            self.system_vtime = self.system_vtime.max(avg_vtime);
        }
        self.system_vtime += 1;
    }

    pub fn execute_process_ticks(&mut self, pid: u64, ticks_executed: u64) {
        if let Some(p) = self.processes.iter_mut().find(|p| p.pid == pid) {
            let weight = p.get_weight();
            let delta = (ticks_executed / weight).max(1);
            p.virtual_runtime = p.virtual_runtime.saturating_add(delta);
            p.update_virtual_deadline(self.system_vtime);
            p.runtime += Duration::from_millis(ticks_executed * 10);
        }
    }

    pub fn set_process_state(&mut self, pid: u64, state: ProcessState) {
        if let Some(process) = self.processes.iter_mut().find(|p| p.pid == pid) {
            process.state = state;
            if state == ProcessState::Ready {
                process.update_virtual_deadline(self.system_vtime);
            }
        }
    }

    pub fn remove_process(&mut self, pid: u64) {
        self.processes.retain(|p| p.pid != pid);
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

/// CFS Scheduler implementation
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
        let scheduler = Scheduler::new();
        assert!(scheduler.processes.is_empty());
    }

    #[test]
    fn test_add_process() {
        let mut scheduler = Scheduler::new();
        let process = Process::new(1, "test".to_string(), Priority::Normal);
        scheduler.add_process(process);
        assert_eq!(scheduler.processes.len(), 1);
    }

    #[test]
    fn test_schedule() {
        let mut scheduler = Scheduler::new();
        let process = Process::new(1, "test".to_string(), Priority::Normal);
        scheduler.add_process(process);

        for _ in 0..5 {
            scheduler.tick();
        }

        let scheduled = scheduler.schedule();
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

        assert!(p2.virtual_deadline < p1.virtual_deadline);
    }

    #[test]
    fn test_work_stealing_and_numa_alignment() {
        assert_eq!(core::mem::align_of::<Process>(), 64);

        let mut scheduler = Scheduler::new();

        let node0 = NumaNode {
            node_id: 0,
            processor_ids: vec![0, 1],
        };
        let node1 = NumaNode {
            node_id: 1,
            processor_ids: vec![2, 3],
        };
        scheduler.numa_nodes.push(node0);
        scheduler.numa_nodes.push(node1);

        let mut q0 = WorkStealingQueue::new(0);
        let mut q1 = WorkStealingQueue::new(1);

        q1.push_task(101);
        q1.push_task(102);
        q1.push_task(103);

        assert_eq!(q0.tasks.len(), 0);
        let stolen_pid = q0.steal_task_from(&mut q1).unwrap();
        assert_eq!(stolen_pid, 101);
        assert_eq!(q0.tasks.len(), 1);
        assert_eq!(q1.tasks.len(), 2);
    }
}
