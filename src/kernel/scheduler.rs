//! EEVDF & BORE Scheduler with SMP Work Stealing & NUMA Topology Support for SigmaOS

extern crate alloc;
use alloc::string::{String, ToString};
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

/// Process control block (PCB) enhanced with EEVDF vruntime and BORE deadline models
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
    pub edf_deadline: Option<u64>, // Absolute real-time deadline for Earliest Deadline First (EDF) scheduler
    pub burst_score: u64,
    pub last_active_time: u64,
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
            edf_deadline: None,
            burst_score: 0,
            last_active_time: 0,
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

    pub fn update_virtual_deadline(&mut self, current_time: u64) {
        let weight = match self.priority {
            Priority::Idle => 64,
            Priority::Low => 128,
            Priority::Normal => 256,
            Priority::High => 512,
            Priority::Realtime => 1024,
        };
        self.virtual_deadline = current_time + (1000 / weight);
    }

    pub fn update_virtual_deadline_bore(&mut self, current_time: u64) {
        let weight = match self.priority {
            Priority::Idle => 64,
            Priority::Low => 128,
            Priority::Normal => 256,
            Priority::High => 512,
            Priority::Realtime => 1024,
        };
        // CachyOS-style BORE burst penalty: higher burst score means higher virtual deadline
        let bore_penalty = self.burst_score / 2;
        self.virtual_deadline = current_time + (1000 / weight) + bore_penalty;
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

/// BORE / EEVDF Scheduler
pub struct Scheduler {
    pub processes: Vec<Process>,
    pub current_time: u64,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            processes: Vec::new(),
            current_time: 0,
        }
    }

    pub fn add_process(&mut self, mut process: Process) {
        process.update_virtual_deadline_bore(self.current_time);
        self.processes.push(process);
    }

    pub fn charge_process_burst(&mut self, pid: u64, burst_amount: u64) {
        if let Some(process) = self.processes.iter_mut().find(|p| p.pid == pid) {
            process.burst_score = process.burst_score.saturating_add(burst_amount);
            process.update_virtual_deadline_bore(self.current_time);
        }
    }

    pub fn decay_process_bursts(&mut self) {
        for process in self.processes.iter_mut() {
            process.burst_score = process.burst_score.saturating_sub(1);
            process.update_virtual_deadline_bore(self.current_time);
        }
    }

    pub fn tick(&mut self) {
        self.current_time += 1;
    }

    pub fn schedule(&self) -> Option<&Process> {
        self.processes.iter().min_by_key(|p| p.virtual_deadline)
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

/// CFS Scheduler implementation
pub struct CfsScheduler {
    pub tasks: [Option<Task>; 64],
    pub task_count: usize,
    pub current_time: u64,
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

        assert!(p2.virtual_deadline < p1.virtual_deadline);
    }

    #[test]
    fn test_work_stealing_and_numa_alignment() {
        assert_eq!(core::mem::align_of::<Process>(), 64);

        let mut numa_nodes = Vec::new();

        let node0 = NumaNode {
            node_id: 0,
            processor_ids: std::vec![0, 1],
        };
        let node1 = NumaNode {
            node_id: 1,
            processor_ids: std::vec![2, 3],
        };
        numa_nodes.push(node0);
        numa_nodes.push(node1);

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

    #[test]
    fn test_bore_scheduling_prioritization() {
        let mut scheduler = Scheduler::new();

        // 1. Create a CPU-bound process and an interactive process with identical priorities
        let p_cpu = Process::new(1, "cpu_bound".to_string(), Priority::Normal);
        let p_interactive = Process::new(2, "interactive".to_string(), Priority::Normal);

        // Add both to scheduler
        scheduler.add_process(p_cpu);
        scheduler.add_process(p_interactive);

        // 2. Simulate CPU-bound process running for long bursts, accumulating high burst score
        scheduler.charge_process_burst(1, 50); // charge 50 burst penalty to cpu_bound

        // Assert that the CPU-bound process now has a significantly higher virtual deadline (penalized)
        let proc_cpu = scheduler.processes.iter().find(|p| p.pid == 1).unwrap();
        let proc_interactive = scheduler.processes.iter().find(|p| p.pid == 2).unwrap();
        assert!(proc_cpu.virtual_deadline > proc_interactive.virtual_deadline);

        // 3. Advancing scheduler time ticks and scheduling should pick the interactive process first
        for _ in 0..10 {
            scheduler.tick();
        }

        let chosen = scheduler.schedule().unwrap();
        assert_eq!(chosen.pid, 2); // interactive should be scheduled first
        assert_eq!(chosen.name, "interactive");

        // 4. Test decay of burst scores
        scheduler.decay_process_bursts();
        let proc_cpu_decayed = scheduler.processes.iter().find(|p| p.pid == 1).unwrap();
        assert_eq!(proc_cpu_decayed.burst_score, 49); // decayed by 1
    }
}
