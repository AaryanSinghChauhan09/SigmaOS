// SigmaOS Kernel Scheduler
// Implements EEVDF (Earliest Eligible Virtual Deadline First) & EDF (Earliest Deadline First) hybrid real-time scheduler

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

/// Process state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Running,
    Ready,
    Blocked,
    Terminated,
}

/// Process control block
#[derive(Debug, Clone)]
pub struct Process {
    pub pid: u64,
    pub name: String,
    pub priority: Priority,
    pub state: ProcessState,
    pub runtime: Duration,
    pub virtual_deadline: u64,
    pub time_slice: Duration,
    pub edf_deadline: Option<u64>, // Absolute real-time deadline for Earliest Deadline First (EDF) scheduler
||||||| 43be3a7e8
    pub core_affinity: Option<usize>, // Core affinity for multi-core/HPC setups
}

impl Process {
    pub fn new(pid: u64, name: String, priority: Priority) -> Self {
        Self {
            pid,
            name,
            priority,
            state: ProcessState::Ready,
            runtime: Duration::from_secs(0),
            virtual_deadline: 0,
            time_slice: Duration::from_millis(10),
            edf_deadline: None,
||||||| 43be3a7e8
            core_affinity: None,
        }
    }

    pub fn with_edf(mut self, deadline: u64) -> Self {
        self.edf_deadline = Some(deadline);
        self
    }

    pub fn update_virtual_deadline(&mut self, current_time: u64) {
        // EEVDF virtual deadline calculation
        let weight = match self.priority {
            Priority::Idle => 1024,
            Priority::Low => 512,
            Priority::Normal => 256,
            Priority::High => 128,
            Priority::Realtime => 64,
        };
        self.virtual_deadline = current_time + (1000 / weight);
    }
}

/// EEVDF & EDF Hybrid Real-Time Scheduler
pub struct Scheduler {
    pub processes: Vec<Process>,
    pub current_time: u64,
    pub is_realtime_profile: bool,
    pub is_hpc_profile: bool,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            processes: Vec::new(),
            current_time: 0,
            is_realtime_profile: false,
            is_hpc_profile: false,
        }
    }

    pub fn enable_realtime_profile(&mut self, enabled: bool) {
        self.is_realtime_profile = enabled;
    }

    pub fn enable_hpc_profile(&mut self, enabled: bool) {
        self.is_hpc_profile = enabled;
    }

    pub fn add_process(&mut self, mut process: Process) {
        process.update_virtual_deadline(self.current_time);
        self.processes.push(process);
    }

    pub fn schedule(&mut self) -> Option<&Process> {
        // Find process with Earliest Deadline First (EDF) if real-time constraints are present
        let mut edf_ready_process: Option<&Process> = None;
        for p in &self.processes {
            if p.state == ProcessState::Ready && p.edf_deadline.is_some() {
                if let Some(current_best) = edf_ready_process {
                    if p.edf_deadline.unwrap() < current_best.edf_deadline.unwrap() {
                        edf_ready_process = Some(p);
                    }
                } else {
                    edf_ready_process = Some(p);
                }
            }
        }

        if let Some(edf_proc) = edf_ready_process {
            return Some(edf_proc);
        }

        // Otherwise, fall back to EEVDF earliest eligible virtual deadline
||||||| 43be3a7e8
        // Find process with earliest eligible virtual deadline
        self.schedule_on_core(0)
    }

    pub fn schedule_on_core(&self, target_core: usize) -> Option<&Process> {
        // Find process with earliest eligible virtual deadline matching core affinity
        let now = self.current_time;

        if self.is_realtime_profile {
            // Prioritize Realtime priority tasks immediately under realtime profile
            let rt_proc = self
                .processes
                .iter()
                .filter(|p| {
                    p.state == ProcessState::Ready
                        && p.priority == Priority::Realtime
                        && p.core_affinity.map_or(true, |c| c == target_core)
                })
                .min_by_key(|p| p.virtual_deadline);
            if rt_proc.is_some() {
                return rt_proc;
            }
        }

        self.processes
            .iter()
            .filter(|p| {
                p.state == ProcessState::Ready
                    && p.virtual_deadline <= now
                    && p.core_affinity.map_or(true, |c| c == target_core)
            })
            .min_by_key(|p| p.virtual_deadline)
    }

    /// Check if a higher-priority task can preempt the currently running process
    pub fn check_preemption(&self, running_pid: u64) -> bool {
        let running_proc = self.processes.iter().find(|p| p.pid == running_pid);
        let highest_ready = self
            .processes
            .iter()
            .filter(|p| p.state == ProcessState::Ready)
            .max_by_key(|p| p.priority);

        match (running_proc, highest_ready) {
            (Some(run), Some(ready)) => ready.priority > run.priority,
            (None, Some(_)) => true,
            _ => false,
        }
    }

    pub fn tick(&mut self) {
        self.current_time += 1;
    }

    pub fn set_process_state(&mut self, pid: u64, state: ProcessState) {
        if let Some(process) = self.processes.iter_mut().find(|p| p.pid == pid) {
            process.state = state;
            if state == ProcessState::Ready {
                process.update_virtual_deadline(self.current_time);
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
    fn test_edf_realtime_scheduler_tick() {
        let mut scheduler = Scheduler::new();

        // Add regular process
        let p_normal = Process::new(1, "normal".to_string(), Priority::Normal);
        scheduler.add_process(p_normal);

        // Add real-time processes with explicit EDF deadlines
        let p_rt_late = Process::new(2, "rt_late".to_string(), Priority::Realtime).with_edf(100);
        let p_rt_early = Process::new(3, "rt_early".to_string(), Priority::Realtime).with_edf(50);

        scheduler.add_process(p_rt_late);
        scheduler.add_process(p_rt_early);

        // Schedule should pick rt_early (absolute deadline 50) first, because it is the earliest real-time deadline
        let chosen = scheduler.schedule().unwrap();
        assert_eq!(chosen.pid, 3);
        assert_eq!(chosen.name, "rt_early");
    }
||||||| 43be3a7e8

    #[test]
    fn test_realtime_preemption_and_core_affinity() {
        let mut scheduler = Scheduler::new();
        scheduler.enable_realtime_profile(true);

        // Process 1: Normal task on Core 0
        let mut p1 = Process::new(1, "normal_task".to_string(), Priority::Normal);
        p1.core_affinity = Some(0);
        scheduler.add_process(p1);

        // Process 2: Realtime task on Core 1
        let mut p2 = Process::new(2, "rt_task".to_string(), Priority::Realtime);
        p2.core_affinity = Some(1);
        scheduler.add_process(p2);

        // schedule on Core 1 should pick the Realtime task (Process 2)
        let scheduled_core1 = scheduler.schedule_on_core(1);
        assert!(scheduled_core1.is_some());
        assert_eq!(scheduled_core1.unwrap().pid, 2);

        // Preemption check: Higher priority (Realtime) should preempt Normal
        assert!(scheduler.check_preemption(1));
    }
}
