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
    pub burst_score: u64, // CachyOS-style Burst-Oriented Response Enhancer (BORE) metric
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
    fn test_bore_burst_penalty_and_decay() {
        let mut scheduler = Scheduler::new();
        let p1 = Process::new(10, "interactive".to_string(), Priority::Normal);
        let p2 = Process::new(11, "cpu_bound".to_string(), Priority::Normal);

        scheduler.add_process(p1);
        scheduler.add_process(p2);

        // Charge p2 with burst amount (simulating a CPU burst)
        scheduler.charge_process_burst(11, 20);

        // Retrieve processes and check their burst scores and deadlines
        let proc1 = scheduler.processes.iter().find(|p| p.pid == 10).unwrap();
        let proc2 = scheduler.processes.iter().find(|p| p.pid == 11).unwrap();

        assert_eq!(proc1.burst_score, 0);
        assert_eq!(proc2.burst_score, 20);

        // p2 virtual deadline should be penalized (larger than p1's virtual deadline)
        assert!(proc2.virtual_deadline > proc1.virtual_deadline);

        // Decay the bursts
        for _ in 0..10 {
            scheduler.decay_process_bursts();
        }

        // Recover proc2 reference and check updated burst_score
        let proc2_decayed = scheduler.processes.iter().find(|p| p.pid == 11).unwrap();
        assert_eq!(proc2_decayed.burst_score, 10);
    }
}
