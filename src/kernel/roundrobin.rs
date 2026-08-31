use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
// SigmaOS Round-Robin Scheduler
// Enhanced priority-aware round-robin with process yielding and context tracking

use crate::kernel::scheduler::{Priority, Process, ProcessState};

/// CPU register context saved during a context switch
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CpuContext {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub rsp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rip: u64,
    pub rflags: u64,
}

impl CpuContext {
    pub fn new() -> Self {
        Self::default()
    }

    /// Simulate saving a context (in a real OS this would be done in assembly)
    pub fn save_from(&mut self, rsp: u64, rip: u64) {
        self.rsp = rsp;
        self.rip = rip;
    }
}

/// Extended process entry that includes context and yields tracking
#[derive(Debug, Clone)]
pub struct ScheduledProcess {
    pub process: Process,
    pub context: CpuContext,
    pub yield_requested: bool,
    pub cpu_time_used: u64,
    pub cpu_affinity: u64,
    pub full_slice_depletions: u64,
    pub voluntary_yields: u64,
    pub interactive_score: i32,
    pub posix_rt_priority: u8, // POSIX SCHED_RR real-time priority (1..99, 99 highest)
    pub sleep_time_ms: u64,
    pub run_time_ms: u64,
}

impl ScheduledProcess {
    pub fn new(process: Process) -> Self {
        let posix_rt_priority = match process.priority {
            Priority::Realtime => 99,
            Priority::High => 70,
            Priority::Normal => 50,
            Priority::Low => 20,
            Priority::Idle => 1,
        };
        Self {
            process,
            context: CpuContext::new(),
            yield_requested: false,
            cpu_time_used: 0,
            cpu_affinity: !0, // Allowed on all CPUs by default
            full_slice_depletions: 0,
            voluntary_yields: 0,
            interactive_score: 0,
            posix_rt_priority,
            sleep_time_ms: 0,
            run_time_ms: 0,
        }
    }

    /// FreeBSD ULE inspired interactivity ratio score (0..100)
    /// Higher ratio indicates interactive (sleep-heavy) tasks that receive scheduling priority boost
    pub fn calculate_ule_interactivity_score(&self) -> u32 {
        let total = self.sleep_time_ms + self.run_time_ms;
        if total == 0 {
            return 100;
        }
        ((self.sleep_time_ms * 100) / total) as u32
    }

    /// Request this process to yield the CPU voluntarily
    pub fn request_yield(&mut self) {
        self.yield_requested = true;
    }

    pub fn boost_interactive_score(&mut self) {
        self.interactive_score = (self.interactive_score + 10).min(100);
    }

    pub fn penalize_interactive_score(&mut self) {
        self.interactive_score = (self.interactive_score - 5).max(-100);
    }

    /// Priority-based weight: higher priority gets a larger time slice multiplier
    pub fn time_slice_ticks(&self, base_slice: u64) -> u64 {
        let multiplier: u64 = match self.process.priority {
            Priority::Realtime => 8,
            Priority::High => 4,
            Priority::Normal => 2,
            Priority::Low => 1,
            Priority::Idle => 1, // Idle still gets a minimal slice
        };
        let boost = if self.interactive_score > 50 {
            2
        } else if self.interactive_score < -50 {
            1
        } else {
            1
        };
        base_slice * multiplier * boost
    }
}

/// Round-robin scheduler configuration
pub struct RoundRobinConfig {
    pub time_slice: u64,
    pub max_processes: usize,
}

impl Default for RoundRobinConfig {
    fn default() -> Self {
        Self {
            time_slice: 10, // 10ms base time slice
            max_processes: 1024,
        }
    }
}

/// Enhanced priority-aware round-robin scheduler
pub struct RoundRobinScheduler {
    processes: Vec<ScheduledProcess>,
    pub current_index: usize,
    config: RoundRobinConfig,
    current_time: u64,
}

impl RoundRobinScheduler {
    pub fn new() -> Self {
        Self {
            processes: Vec::new(),
            current_index: 0,
            config: RoundRobinConfig::default(),
            current_time: 0,
        }
    }

    pub fn with_config(config: RoundRobinConfig) -> Self {
        Self {
            processes: Vec::new(),
            current_index: 0,
            config,
            current_time: 0,
        }
    }

    pub fn add_process(&mut self, process: Process) -> Result<(), SchedulerError> {
        if self.processes.len() >= self.config.max_processes {
            return Err(SchedulerError::TooManyProcesses);
        }
        self.processes.push(ScheduledProcess::new(process));
        Ok(())
    }

    /// POSIX SCHED_RR scheduling with strict static priority tier preemption
    pub fn schedule_on_cpu(&mut self, cpu_id: u8) -> Option<&Process> {
        if self.processes.is_empty() {
            return None;
        }

        // Find highest POSIX RT priority level among ready tasks on this CPU
        let mut highest_prio = 0u8;
        for entry in &self.processes {
            let is_affinity_matched = (entry.cpu_affinity & (1 << cpu_id)) != 0;
            if entry.process.state == ProcessState::Ready && is_affinity_matched {
                if entry.posix_rt_priority > highest_prio {
                    highest_prio = entry.posix_rt_priority;
                }
            }
        }

        if highest_prio == 0 {
            return None;
        }

        // Round-robin selection among tasks with highest_prio
        let start_index = self.current_index;
        loop {
            let entry = &self.processes[self.current_index];
            let is_affinity_matched = (entry.cpu_affinity & (1 << cpu_id)) != 0;
            if entry.process.state == ProcessState::Ready
                && is_affinity_matched
                && entry.posix_rt_priority == highest_prio
            {
                return Some(&self.processes[self.current_index].process);
            }
            self.current_index = (self.current_index + 1) % self.processes.len();
            if self.current_index == start_index {
                return None;
            }
        }
    }

    /// Linux CFS/EEVDF inspired dynamic time slice scaling based on active task load
    /// Scales time slice quantum down when ready task count increases, bounded by min granularity (1 tick)
    pub fn dynamic_time_slice(&self, base_slice: u64) -> u64 {
        let ready_count = self.get_ready_process_count();
        if ready_count <= 1 {
            base_slice
        } else {
            let target_latency = base_slice * 4;
            (target_latency / ready_count as u64).max(1)
        }
    }

    /// POSIX sched_rr_get_interval(2) parity: returns time quantum in ticks for a given PID
    pub fn sched_rr_get_interval(&self, pid: u64) -> Option<u64> {
        self.processes
            .iter()
            .find(|e| e.process.pid == pid)
            .map(|e| e.time_slice_ticks(self.dynamic_time_slice(self.config.time_slice)))
    }

    /// FreeBSD ULE/4BSD style interactivity score decay over time
    pub fn decay_interactivity_scores(&mut self) {
        for entry in &mut self.processes {
            if entry.interactive_score > 0 {
                entry.interactive_score -= 1;
            } else if entry.interactive_score < 0 {
                entry.interactive_score += 1;
            }
        }
    }

    /// Linux/FreeBSD SMP load balancer: migrates a ready process to balance CPU load
    pub fn balance_cpu_load(&mut self, source_cpu: u8, target_cpu: u8) -> bool {
        for entry in &mut self.processes {
            let matches_source = (entry.cpu_affinity & (1 << source_cpu)) != 0;
            let matches_target = (entry.cpu_affinity & (1 << target_cpu)) != 0;
            if entry.process.state == ProcessState::Ready && matches_source && matches_target {
                // Re-pin process to target CPU
                entry.cpu_affinity = 1 << target_cpu;
                return true;
            }
        }
        false
    }

    pub fn schedule(&mut self) -> Option<&Process> {
        self.schedule_on_cpu(0)
    }

    pub fn set_cpu_affinity(&mut self, pid: u64, affinity: u64) {
        if let Some(entry) = self.processes.iter_mut().find(|e| e.process.pid == pid) {
            entry.cpu_affinity = affinity;
        }
    }

    pub fn tick(&mut self) {
        self.current_time += 1;

        if self.processes.is_empty() {
            return;
        }

        // Safeguard current_index boundaries to prevent any out of bounds panic
        if self.current_index >= self.processes.len() {
            self.current_index = 0;
        }

        let needs_switch = {
            let entry = &mut self.processes[self.current_index];
            entry.cpu_time_used += 1;
            let slice = entry.time_slice_ticks(self.config.time_slice);
            let yielding = entry.yield_requested;
            entry.yield_requested = false;

            if yielding {
                entry.voluntary_yields += 1;
                entry.boost_interactive_score();
                true
            } else if entry.cpu_time_used % slice == 0 {
                entry.full_slice_depletions += 1;
                entry.penalize_interactive_score();
                true
            } else {
                false
            }
        };

        if needs_switch {
            self.advance_to_next_ready();
        }
    }

    /// Move to the next ready process
    fn advance_to_next_ready(&mut self) {
        if self.processes.is_empty() {
            return;
        }
        let start = self.current_index;
        loop {
            self.current_index = (self.current_index + 1) % self.processes.len();
            if self.processes[self.current_index].process.state == ProcessState::Ready {
                break;
            }
            if self.current_index == start {
                break;
            }
        }
    }

    /// Request the current running process to yield on next tick
    pub fn yield_current(&mut self) {
        if self.processes.is_empty() {
            return;
        }
        self.processes[self.current_index].request_yield();
    }

    pub fn set_process_state(&mut self, pid: u64, state: ProcessState) {
        if let Some(entry) = self.processes.iter_mut().find(|e| e.process.pid == pid) {
            entry.process.state = state;
        }
    }

    pub fn remove_process(&mut self, pid: u64) {
        self.processes.retain(|e| e.process.pid != pid);
        if self.current_index >= self.processes.len() && !self.processes.is_empty() {
            self.current_index = 0;
        }
    }

    pub fn get_process_count(&self) -> usize {
        self.processes.len()
    }

    pub fn get_ready_process_count(&self) -> usize {
        self.processes
            .iter()
            .filter(|e| e.process.state == ProcessState::Ready)
            .count()
    }

    /// Save the context of the currently running process
    pub fn save_context(&mut self, rsp: u64, rip: u64) {
        if let Some(entry) = self.processes.get_mut(self.current_index) {
            entry.context.save_from(rsp, rip);
        }
    }

    /// Restore the context of the currently scheduled process
    pub fn restore_context(&self) -> Option<CpuContext> {
        self.processes.get(self.current_index).map(|e| e.context)
    }
}

impl Default for RoundRobinScheduler {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// Multi-Queue Linux (SCHED_RR) & BSD (ULE) Inspired Round-Robin Scheduler
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedPolicy {
    SchedFifo,
    SchedRr,
    SchedOther,
}

#[derive(Debug, Clone)]
pub struct MultiQueueTask {
    pub pid: u64,
    pub name: String,
    pub policy: SchedPolicy,
    pub priority: u8, // 0..99 for RT, 100..139 for Normal
    pub rr_time_slice_ms: u32,
    pub time_slice_remaining_ms: u32,
    pub cpu_id: u8,
    pub state: ProcessState,
}

pub struct SovereignMultiQueueRoundRobin {
    pub realtime_queue: Vec<MultiQueueTask>,
    pub high_queue: Vec<MultiQueueTask>,
    pub normal_queue: Vec<MultiQueueTask>,
    pub idle_queue: Vec<MultiQueueTask>,
    pub num_cpus: u8,
    pub total_switches: u64,
}

impl SovereignMultiQueueRoundRobin {
    pub fn new(num_cpus: u8) -> Self {
        Self {
            realtime_queue: Vec::new(),
            high_queue: Vec::new(),
            normal_queue: Vec::new(),
            idle_queue: Vec::new(),
            num_cpus,
            total_switches: 0,
        }
    }

    /// Linux SCHED_RR / POSIX parity dynamic time quantum calculation (10ms to 100ms)
    pub fn calculate_sched_rr_quantum(priority: u8) -> u32 {
        if priority < 100 {
            10 + ((priority as u32) * 90 / 99)
        } else {
            20
        }
    }

    /// FreeBSD ULE inspired dynamic queue migration based on interactivity score
    pub fn auto_balance_interactivity(&mut self) {
        let mut to_promote = Vec::new();
        for (i, task) in self.normal_queue.iter().enumerate() {
            if task.rr_time_slice_ms <= 10 {
                to_promote.push(i);
            }
        }
        for idx in to_promote.into_iter().rev() {
            let mut task = self.normal_queue.remove(idx);
            task.priority = 110; // Promoted to high priority queue
            self.high_queue.push(task);
        }
    }

    pub fn enqueue_task(&mut self, mut task: MultiQueueTask) {
        if task.rr_time_slice_ms == 0 {
            task.rr_time_slice_ms = Self::calculate_sched_rr_quantum(task.priority);
        }
        task.time_slice_remaining_ms = task.rr_time_slice_ms;
        if task.priority < 100 {
            self.realtime_queue.push(task);
        } else if task.priority < 120 {
            self.high_queue.push(task);
        } else if task.priority < 139 {
            self.normal_queue.push(task);
        } else {
            self.idle_queue.push(task);
        }
    }

    pub fn pick_next_task(&mut self, cpu_id: u8) -> Option<MultiQueueTask> {
        self.total_switches += 1;

        // 1. Realtime SCHED_RR / SCHED_FIFO queue
        if let Some(pos) = self
            .realtime_queue
            .iter()
            .position(|t| t.cpu_id == cpu_id && t.state == ProcessState::Ready)
        {
            let mut task = self.realtime_queue.remove(pos);
            if task.policy == SchedPolicy::SchedRr {
                // Re-enqueue at back of RT queue after picking
                let mut queued = task.clone();
                queued.time_slice_remaining_ms = queued.rr_time_slice_ms;
                self.realtime_queue.push(queued);
            }
            return Some(task);
        }

        // 2. High priority queue
        if let Some(pos) = self
            .high_queue
            .iter()
            .position(|t| t.cpu_id == cpu_id && t.state == ProcessState::Ready)
        {
            let task = self.high_queue.remove(pos);
            let mut queued = task.clone();
            self.high_queue.push(queued);
            return Some(task);
        }

        // 3. Normal queue
        if let Some(pos) = self
            .normal_queue
            .iter()
            .position(|t| t.cpu_id == cpu_id && t.state == ProcessState::Ready)
        {
            let task = self.normal_queue.remove(pos);
            let mut queued = task.clone();
            self.normal_queue.push(queued);
            return Some(task);
        }

        // 4. Idle queue
        if let Some(pos) = self
            .idle_queue
            .iter()
            .position(|t| t.cpu_id == cpu_id && t.state == ProcessState::Ready)
        {
            let task = self.idle_queue.remove(pos);
            let mut queued = task.clone();
            self.idle_queue.push(queued);
            return Some(task);
        }

        None
    }

    pub fn tick_cpu(&mut self, cpu_id: u8) {
        for queue in [
            &mut self.realtime_queue,
            &mut self.high_queue,
            &mut self.normal_queue,
        ] {
            for task in queue.iter_mut() {
                if task.cpu_id == cpu_id && task.state == ProcessState::Running {
                    if task.time_slice_remaining_ms > 0 {
                        task.time_slice_remaining_ms -= 1;
                    }
                    if task.time_slice_remaining_ms == 0 {
                        task.time_slice_remaining_ms = task.rr_time_slice_ms;
                        task.state = ProcessState::Ready;
                    }
                }
            }
        }
    }
}

impl Default for SovereignMultiQueueRoundRobin {
    fn default() -> Self {
        Self::new(4)
    }
}

/// Scheduler errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchedulerError {
    TooManyProcesses,
    ProcessNotFound,
    InvalidState,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundrobin_creation() {
        let scheduler = RoundRobinScheduler::new();
        assert_eq!(scheduler.get_process_count(), 0);
    }

    #[test]
    fn test_add_process() {
        let mut scheduler = RoundRobinScheduler::new();
        let process = Process::new(1, "test".to_string(), Priority::Normal);
        assert!(scheduler.add_process(process).is_ok());
        assert_eq!(scheduler.get_process_count(), 1);
    }

    #[test]
    fn test_schedule() {
        let mut scheduler = RoundRobinScheduler::new();
        let process = Process::new(1, "test".to_string(), Priority::Normal);
        scheduler.add_process(process).unwrap();
        assert!(scheduler.schedule().is_some());
    }

    #[test]
    fn test_tick_switches_process() {
        let mut scheduler = RoundRobinScheduler::new();
        let p1 = Process::new(1, "test1".to_string(), Priority::Normal);
        let p2 = Process::new(2, "test2".to_string(), Priority::Normal);
        scheduler.add_process(p1).unwrap();
        scheduler.add_process(p2).unwrap();

        let initial_index = scheduler.current_index;
        // Normal priority multiplier is 2x base 10 = 20 ticks per slice
        for _ in 0..20 {
            scheduler.tick();
        }
        assert_ne!(scheduler.current_index, initial_index);
    }

    #[test]
    fn test_yield_current() {
        let mut scheduler = RoundRobinScheduler::new();
        let p1 = Process::new(1, "test1".to_string(), Priority::High);
        let p2 = Process::new(2, "test2".to_string(), Priority::Normal);
        scheduler.add_process(p1).unwrap();
        scheduler.add_process(p2).unwrap();

        let initial_index = scheduler.current_index;
        scheduler.yield_current();
        scheduler.tick(); // triggers the switch
        assert_ne!(scheduler.current_index, initial_index);
    }

    #[test]
    fn test_context_save_restore() {
        let mut scheduler = RoundRobinScheduler::new();
        let process = Process::new(1, "test".to_string(), Priority::Normal);
        scheduler.add_process(process).unwrap();

        scheduler.save_context(0xDEADBEEF, 0xCAFEBABE);
        let ctx = scheduler.restore_context().unwrap();
        assert_eq!(ctx.rsp, 0xDEAD_BEEF);
        assert_eq!(ctx.rip, 0xCAFE_BABE);
    }

    #[test]
    fn test_remove_process() {
        let mut scheduler = RoundRobinScheduler::new();
        let process = Process::new(1, "test".to_string(), Priority::Normal);
        scheduler.add_process(process).unwrap();
        scheduler.remove_process(1);
        assert_eq!(scheduler.get_process_count(), 0);
    }

    #[test]
    fn test_max_processes() {
        let config = RoundRobinConfig {
            time_slice: 10,
            max_processes: 2,
        };
        let mut scheduler = RoundRobinScheduler::with_config(config);

        assert!(scheduler
            .add_process(Process::new(1, "test1".to_string(), Priority::Normal))
            .is_ok());
        assert!(scheduler
            .add_process(Process::new(2, "test2".to_string(), Priority::Normal))
            .is_ok());
        assert!(scheduler
            .add_process(Process::new(3, "test3".to_string(), Priority::Normal))
            .is_err());
    }

    #[test]
    fn test_cpu_affinity() {
        let mut scheduler = RoundRobinScheduler::new();
        let p1 = Process::new(1, "cpu1_only".to_string(), Priority::Normal);
        let p2 = Process::new(2, "cpu2_only".to_string(), Priority::Normal);
        scheduler.add_process(p1).unwrap();
        scheduler.add_process(p2).unwrap();

        // Limit process 1 to CPU 0 (bit 0) and process 2 to CPU 1 (bit 1)
        scheduler.set_cpu_affinity(1, 1 << 0);
        scheduler.set_cpu_affinity(2, 1 << 1);

        // On CPU 0, only process 1 should be scheduled
        let scheduled_on_cpu0 = scheduler.schedule_on_cpu(0).unwrap();
        assert_eq!(scheduled_on_cpu0.pid, 1);

        // On CPU 1, only process 2 should be scheduled
        let scheduled_on_cpu1 = scheduler.schedule_on_cpu(1).unwrap();
        assert_eq!(scheduled_on_cpu1.pid, 2);
    }

    #[test]
    fn test_interactive_boosting() {
        let mut scheduler = RoundRobinScheduler::new();
        let p1 = Process::new(1, "sleeper_interactive".to_string(), Priority::Normal);
        scheduler.add_process(p1).unwrap();

        // Voluntarily yield boosts interactive score
        scheduler.yield_current();
        scheduler.tick(); // trigger voluntary yield

        let p = &scheduler.processes[0];
        assert!(p.interactive_score > 0);
        assert_eq!(p.voluntary_yields, 1);
        assert_eq!(p.full_slice_depletions, 0);

        // Artificially deplete its time slice to test penalty
        for _ in 0..100 {
            scheduler.tick();
        }
        let p_penalized = &scheduler.processes[0];
        assert!(p_penalized.interactive_score < 0);
        assert!(p_penalized.full_slice_depletions > 0);
    }

    #[test]
    fn test_sovereign_multi_queue_round_robin() {
        let mut mq_rr = SovereignMultiQueueRoundRobin::new(2);

        let rt_task = MultiQueueTask {
            pid: 10,
            name: "audio_dsp".to_string(),
            policy: SchedPolicy::SchedRr,
            priority: 10, // Realtime
            rr_time_slice_ms: 5,
            time_slice_remaining_ms: 5,
            cpu_id: 0,
            state: ProcessState::Ready,
        };

        let normal_task = MultiQueueTask {
            pid: 20,
            name: "compiler".to_string(),
            policy: SchedPolicy::SchedOther,
            priority: 120, // Normal
            rr_time_slice_ms: 20,
            time_slice_remaining_ms: 20,
            cpu_id: 0,
            state: ProcessState::Ready,
        };

        mq_rr.enqueue_task(rt_task);
        mq_rr.enqueue_task(normal_task);

        assert_eq!(mq_rr.realtime_queue.len(), 1);
        assert_eq!(mq_rr.normal_queue.len(), 1);

        // Realtime SCHED_RR task should be picked first
        let picked = mq_rr.pick_next_task(0).unwrap();
        assert_eq!(picked.pid, 10);
        assert_eq!(picked.policy, SchedPolicy::SchedRr);
    }

    #[test]
    fn test_dynamic_time_slice_scaling() {
        let mut scheduler = RoundRobinScheduler::new();
        let p1 = Process::new(1, "task1".to_string(), Priority::Normal);
        scheduler.add_process(p1).unwrap();

        // Single process ready -> base slice
        assert_eq!(scheduler.dynamic_time_slice(10), 10);

        // Add 3 more processes -> ready count = 4
        scheduler
            .add_process(Process::new(2, "task2".to_string(), Priority::Normal))
            .unwrap();
        scheduler
            .add_process(Process::new(3, "task3".to_string(), Priority::Normal))
            .unwrap();
        scheduler
            .add_process(Process::new(4, "task4".to_string(), Priority::Normal))
            .unwrap();

        // 4 ready tasks -> target_latency (40) / 4 = 10
        assert_eq!(scheduler.dynamic_time_slice(10), 10);
    }

    #[test]
    fn test_ule_interactivity_score_and_quantum() {
        let mut process = Process::new(1, "interactive_gui".to_string(), Priority::Normal);
        let mut scheduled = ScheduledProcess::new(process);
        scheduled.sleep_time_ms = 90;
        scheduled.run_time_ms = 10;

        // 90% sleep ratio -> 90 interactivity score
        assert_eq!(scheduled.calculate_ule_interactivity_score(), 90);

        // Check Linux SCHED_RR quantum calculation
        let rt_quantum = SovereignMultiQueueRoundRobin::calculate_sched_rr_quantum(50);
        assert!(rt_quantum > 10 && rt_quantum < 100);

        // Test auto balance interactivity queue promotion
        let mut mq_rr = SovereignMultiQueueRoundRobin::new(1);
        let normal_task = MultiQueueTask {
            pid: 101,
            name: "quick_response".to_string(),
            policy: SchedPolicy::SchedOther,
            priority: 125,
            rr_time_slice_ms: 10,
            time_slice_remaining_ms: 10,
            cpu_id: 0,
            state: ProcessState::Ready,
        };
        mq_rr.enqueue_task(normal_task);
        assert_eq!(mq_rr.normal_queue.len(), 1);

        mq_rr.auto_balance_interactivity();
        assert_eq!(mq_rr.normal_queue.len(), 0);
        assert_eq!(mq_rr.high_queue.len(), 1);
    }
}
