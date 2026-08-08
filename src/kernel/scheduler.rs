// SigmaOS Kernel Scheduler
// Implements EEVDF (Earliest Eligible Virtual Deadline First) scheduler

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

/// Process control block (PCB) enhanced with EEVDF vruntime and deadline models
#[derive(Debug, Clone)]
pub struct Process {
    pub pid: u64,
    pub name: String,
    pub priority: Priority,
    pub state: ProcessState,
    pub runtime: Duration,
    pub virtual_runtime: u64,  // EEVDF vruntime (ticks)
    pub virtual_deadline: u64, // EEVDF virtual deadline
    pub time_slice: Duration,
<<<<<<< HEAD
    pub edf_deadline: Option<u64>, // Absolute real-time deadline for Earliest Deadline First (EDF) scheduler
||||||| 23ef22a4a
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
            core_affinity: None,
        }
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

/// EEVDF Scheduler
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
=======
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

/// EEVDF Scheduler Engine
pub struct Scheduler {
    pub processes: Vec<Process>,
    pub current_time: u64,
    pub system_vtime: u64, // EEVDF System Virtual Time (V)
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            processes: Vec::new(),
            current_time: 0,
            system_vtime: 0,
        }
    }

    pub fn add_process(&mut self, mut process: Process) {
        // Set initial vruntime to system virtual time to prevent newly spawned process from hogging CPU
        process.virtual_runtime = self.system_vtime;
        process.update_virtual_deadline(self.system_vtime);
        self.processes.push(process);
    }

    pub fn schedule(&mut self) -> Option<&Process> {
        // 1. Filter ready processes
        let mut ready_indices = Vec::new();
        for (idx, p) in self.processes.iter().enumerate() {
            if p.state == ProcessState::Ready {
                ready_indices.push(idx);
            }
        }

        if ready_indices.is_empty() {
            return None;
        }

        // 2. Identify eligible processes (virtual_runtime <= system_vtime)
        let mut eligible_indices = Vec::new();
        for &idx in &ready_indices {
            let p = &self.processes[idx];
            if p.virtual_runtime <= self.system_vtime {
                eligible_indices.push(idx);
            }
        }

        // 3. Selection rules:
        // - Standard EEVDF: pick the eligible process with the EARLIEST virtual deadline
        // - Starvation Prevention: if no processes are currently eligible (e.g. system_vtime is lagging),
        //   fallback to selecting the process with the minimum virtual_runtime
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

        // Advance system virtual time (V) based on active threads vruntime progress
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
            // System virtual time advances gracefully
            self.system_vtime = self.system_vtime.max(avg_vtime);
        }
        self.system_vtime += 1;
    }

    pub fn execute_process_ticks(&mut self, pid: u64, ticks_executed: u64) {
        // Simulates thread execution and updates its vruntime based on priority weight:
        // vruntime_delta = executed_ticks / weight
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
        let mut scheduler = Scheduler::new();
        let mut p1 = Process::new(1, "low-prio".to_string(), Priority::Low);
        let mut p2 = Process::new(2, "high-prio".to_string(), Priority::High);

        scheduler.add_process(p1.clone());
        scheduler.add_process(p2.clone());

        p1.update_virtual_deadline(0);
        p2.update_virtual_deadline(0);

        // High priority must have a tighter/earlier virtual deadline for the same vruntime!
        assert!(p2.virtual_deadline < p1.virtual_deadline);
    }
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
}
