// SigmaOS Kernel Scheduler
// Implements EEVDF (Earliest Eligible Virtual Deadline First) & EDF (Earliest Deadline First) hybrid real-time scheduler

extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;
use core::cmp::Ordering;
use core::time::Duration;

/// Process priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Idle = 0,
    Low = 1,
    Normal = 2,
    High = 3,
    Realtime = 4,
}

#[derive(Debug, Clone, Copy)]
pub struct Task {
    pub id: u64,
    pub priority: u8,
    pub vruntime: u64,
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
        for i in 1..self.task_count {
            let mut j = i;
            while j > 0 && self.tasks[j - 1].unwrap().vruntime > self.tasks[j].unwrap().vruntime {
                self.tasks.swap(j - 1, j);
                j -= 1;
            }
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
        Self {
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

    }