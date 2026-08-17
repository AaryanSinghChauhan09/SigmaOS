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
            virtual_deadline: 0,
            time_slice: Duration::from_millis(10),
            edf_deadline: None,
            burst_score: 0,
            last_active_time: 0,
        }
    }

    pub fn with_edf(mut self, deadline: u64) -> Self {
        self.edf_deadline = Some(deadline);
        self
    }

    pub fn update_virtual_deadline(&mut self, current_time: u64) {
        // EEVDF virtual deadline calculation
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
        // CachyOS-style BORE burst penalty: higher burst score means higher virtual deadline (less eligibility)
        let bore_penalty = self.burst_score / 2;
        self.virtual_deadline = current_time + (1000 / weight) + bore_penalty;
    }
}

/// EEVDF & EDF Hybrid Real-Time Scheduler
pub struct Scheduler {
    processes: Vec<Process>,
    current_time: u64,
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
        for process in &mut self.processes {
            process.burst_score = process.burst_score.saturating_sub(1);
        }
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
        let now = self.current_time;
        self.processes
            .iter()
            .filter(|p| p.state == ProcessState::Ready && p.virtual_deadline <= now)
            .min_by_key(|p| p.virtual_deadline)
    }

    pub fn tick(&mut self) {
        self.current_time += 1;
    }

    pub fn set_process_state(&mut self, pid: u64, state: ProcessState) {
        if let Some(process) = self.processes.iter_mut().find(|p| p.pid == pid) {
            process.state = state;
            if state == ProcessState::Ready {
                process.update_virtual_deadline_bore(self.current_time);
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
