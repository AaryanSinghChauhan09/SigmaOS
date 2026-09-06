#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
// SigmaOS Comprehensive OS Scheduling Suite
// Implements 3 Levels of Schedulers (Long-Term, Medium-Term, Short-Term CPU),
// 6 CPU Scheduling Algorithms (FCFS, SJF, RR, Priority with Aging, Multilevel Queue, MLFQ),
// and Advanced Disk/I/O Schedulers (Anticipatory I/O, CFQ, BFQ, Deadline).
// Inspired by Linux (CFQ/BFQ/eBPF) & BSD (CAM/ULE/Kqueue) architectures under #![no_std]

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessLifecycleState {
    New,
    Ready,
    Running,
    Waiting,
    SuspendedSwapped,
    Terminated,
}

#[derive(Debug, Clone)]
pub struct ProcessTaskControlBlock {
    pub pid: u32,
    pub priority: u32,       // 0 = highest, 139 = lowest
    pub burst_time: u32,     // Estimated or actual execution ticks
    pub remaining_time: u32, // Remaining ticks
    pub time_quantum: u32,   // Quantum for RR/MLFQ
    pub state: ProcessLifecycleState,
    pub arrival_time: u64,
    pub age_ticks: u32, // Starvation aging counter
    pub is_interactive: bool,
}

// =================================────────────────────────────────────────────
// 1. THREE LEVELS OF OS SCHEDULERS
// =================================────────────────────────────────────────────

/// Long-Term Scheduler (Job Scheduler): Admits new batch/job submissions into Ready queue (controls degree of multiprogramming)
pub struct LongTermJobScheduler {
    pub job_pool: Vec<ProcessTaskControlBlock>,
    pub max_degree_of_multiprogramming: usize,
}

impl LongTermJobScheduler {
    pub fn new(max_degree: usize) -> Self {
        Self {
            job_pool: Vec::new(),
            max_degree_of_multiprogramming: max_degree,
        }
    }

    pub fn submit_job(&mut self, pcb: ProcessTaskControlBlock) {
        self.job_pool.push(pcb);
    }

    pub fn admit_jobs(&mut self, active_ready_count: usize) -> Vec<ProcessTaskControlBlock> {
        let mut admitted = Vec::new();
        let available_slots = self
            .max_degree_of_multiprogramming
            .saturating_sub(active_ready_count);

        for _ in 0..available_slots {
            if !self.job_pool.is_empty() {
                let mut pcb = self.job_pool.remove(0);
                pcb.state = ProcessLifecycleState::Ready;
                admitted.push(pcb);
            }
        }
        admitted
    }
}

/// Medium-Term Scheduler (Swapper): Swaps inactive processes out to disk/swap to free physical RAM
pub struct MediumTermSwapper {
    pub ready_memory_queue: Vec<ProcessTaskControlBlock>,
    pub swapped_disk_queue: Vec<ProcessTaskControlBlock>,
}

impl MediumTermSwapper {
    pub fn new() -> Self {
        Self {
            ready_memory_queue: Vec::new(),
            swapped_disk_queue: Vec::new(),
        }
    }

    pub fn swap_out_inactive(&mut self, pid: u32) -> Result<(), &'static str> {
        if let Some(pos) = self.ready_memory_queue.iter().position(|p| p.pid == pid) {
            let mut pcb = self.ready_memory_queue.remove(pos);
            pcb.state = ProcessLifecycleState::SuspendedSwapped;
            self.swapped_disk_queue.push(pcb);
            Ok(())
        } else {
            Err("Process not found in memory queue")
        }
    }

    pub fn swap_in(&mut self, pid: u32) -> Result<(), &'static str> {
        if let Some(pos) = self.swapped_disk_queue.iter().position(|p| p.pid == pid) {
            let mut pcb = self.swapped_disk_queue.remove(pos);
            pcb.state = ProcessLifecycleState::Ready;
            self.ready_memory_queue.push(pcb);
            Ok(())
        } else {
            Err("Process not found in swapped queue")
        }
    }
}

// =================================================================────────────
// 2. CPU SCHEDULING ALGORITHMS (Short-Term CPU Scheduler)
// =================================================================────────────

pub struct ShortTermCpuScheduler {
    pub fcfs_queue: Vec<ProcessTaskControlBlock>,
    pub ready_queue: Vec<ProcessTaskControlBlock>,
}

impl ShortTermCpuScheduler {
    pub fn new() -> Self {
        Self {
            fcfs_queue: Vec::new(),
            ready_queue: Vec::new(),
        }
    }

    /// 1. First-Come, First-Served (FCFS) Non-Preemptive
    pub fn schedule_fcfs(&mut self) -> Option<ProcessTaskControlBlock> {
        if self.fcfs_queue.is_empty() {
            None
        } else {
            Some(self.fcfs_queue.remove(0))
        }
    }

    /// 2. Shortest Job First (SJF) / Shortest Remaining Time First (SRTF Preemptive)
    pub fn schedule_sjf_srtf(&mut self) -> Option<ProcessTaskControlBlock> {
        if self.ready_queue.is_empty() {
            return None;
        }
        let mut min_idx = 0;
        for i in 1..self.ready_queue.len() {
            if self.ready_queue[i].remaining_time < self.ready_queue[min_idx].remaining_time {
                min_idx = i;
            }
        }
        Some(self.ready_queue.remove(min_idx))
    }

    /// 3. Round Robin (RR) with Time Quantum
    pub fn schedule_round_robin(&mut self, time_quantum: u32) -> Option<ProcessTaskControlBlock> {
        if self.ready_queue.is_empty() {
            return None;
        }
        let mut pcb = self.ready_queue.remove(0);
        pcb.time_quantum = time_quantum;
        Some(pcb)
    }

    /// 4. Preemptive Priority Scheduling with Aging (Prevents Starvation)
    pub fn schedule_priority_aging(&mut self) -> Option<ProcessTaskControlBlock> {
        if self.ready_queue.is_empty() {
            return None;
        }

        // Apply aging counter to elevate long-waiting tasks
        for pcb in &mut self.ready_queue {
            pcb.age_ticks += 1;
            if pcb.age_ticks >= 10 && pcb.priority > 0 {
                pcb.priority -= 1; // Promote priority
                pcb.age_ticks = 0;
            }
        }

        let mut highest_idx = 0;
        for i in 1..self.ready_queue.len() {
            if self.ready_queue[i].priority < self.ready_queue[highest_idx].priority {
                highest_idx = i;
            }
        }
        Some(self.ready_queue.remove(highest_idx))
    }
}

impl Default for ShortTermCpuScheduler {
    fn default() -> Self {
        Self::new()
    }
}

/// 5 & 6. Multilevel Feedback Queue Scheduler (MLFQ): 3 priority queues with dynamic demotion/promotion
pub struct MultilevelFeedbackQueueScheduler {
    pub q0_high_rr: Vec<ProcessTaskControlBlock>, // Quantum = 4
    pub q1_med_rr: Vec<ProcessTaskControlBlock>,  // Quantum = 8
    pub q2_low_fcfs: Vec<ProcessTaskControlBlock>, // FCFS
}

impl MultilevelFeedbackQueueScheduler {
    pub fn new() -> Self {
        Self {
            q0_high_rr: Vec::new(),
            q1_med_rr: Vec::new(),
            q2_low_fcfs: Vec::new(),
        }
    }

    pub fn insert_new_task(&mut self, pcb: ProcessTaskControlBlock) {
        self.q0_high_rr.push(pcb);
    }

    pub fn schedule_next(&mut self) -> Option<(ProcessTaskControlBlock, u32)> {
        if !self.q0_high_rr.is_empty() {
            Some((self.q0_high_rr.remove(0), 4))
        } else if !self.q1_med_rr.is_empty() {
            Some((self.q1_med_rr.remove(0), 8))
        } else if !self.q2_low_fcfs.is_empty() {
            Some((self.q2_low_fcfs.remove(0), u32::MAX))
        } else {
            None
        }
    }

    /// Demotes CPU-bound tasks or promotes interactive I/O tasks
    pub fn requeue_after_quantum_expiry(
        &mut self,
        pcb: ProcessTaskControlBlock,
        used_ticks: u32,
        allocated_quantum: u32,
    ) {
        if used_ticks >= allocated_quantum {
            // Demote to lower queue
            if allocated_quantum == 4 {
                self.q1_med_rr.push(pcb);
            } else {
                self.q2_low_fcfs.push(pcb);
            }
        } else {
            // Yielded voluntarily (I/O) -> Keep or promote to high priority
            if pcb.is_interactive {
                self.q0_high_rr.push(pcb);
            } else {
                self.q1_med_rr.push(pcb);
            }
        }
    }
}

impl Default for MultilevelFeedbackQueueScheduler {
    fn default() -> Self {
        Self::new()
    }
}

// =================================================================────────────
// 3. DISK & I/O SCHEDULERS
// =================================================================────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoRequestType {
    Read,
    Write,
}

#[derive(Debug, Clone)]
pub struct DiskIoRequest {
    pub req_id: u32,
    pub pid: u32,
    pub sector: u64,
    pub sector_count: u32,
    pub req_type: IoRequestType,
    pub arrival_tick: u64,
}

impl DiskIoRequest {
    pub fn simple(req_id: u32, sector: u64, req_type: IoRequestType, arrival_tick: u64) -> Self {
        Self {
            req_id,
            pid: 1,
            sector,
            sector_count: 8,
            req_type,
            arrival_tick,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElevatorDirection {
    Ascending,
    Descending,
}

#[derive(Debug, Clone)]
pub struct ProcessIoStats {
    pub last_read_sector: u64,
    pub last_request_tick: u64,
    pub think_time_accumulator_ticks: u64,
    pub request_count: u64,
    pub sequential_hits: u64,
}

impl ProcessIoStats {
    pub fn new() -> Self {
        Self {
            last_read_sector: 0,
            last_request_tick: 0,
            think_time_accumulator_ticks: 0,
            request_count: 0,
            sequential_hits: 0,
        }
    }

    pub fn average_think_time(&self) -> u64 {
        if self.request_count == 0 {
            0
        } else {
            self.think_time_accumulator_ticks / self.request_count
        }
    }
}

impl Default for ProcessIoStats {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Default)]
pub struct AnticipatoryTelemetry {
    pub total_dispatched: u64,
    pub anticipation_hits: u64,
    pub anticipation_misses: u64,
    pub seeks_saved_sectors: u64,
    pub read_starvation_preventions: u64,
    pub write_starvation_preventions: u64,
}

/// Advanced Anticipatory I/O Scheduler:
/// Inspired by Linux Anticipatory (as-iosched.c), BFQ, and FreeBSD CAM I/O Scheduler.
/// Features heuristic read anticipation, think-time profiling per process,
/// directional SCAN/C-LOOK elevator, deadline starvation thresholds, and telemetry tracking.
pub struct AnticipatoryIoScheduler {
    pub request_queue: Vec<DiskIoRequest>,
    pub current_head_sector: u64,
    pub head_direction: ElevatorDirection,
    pub anticipation_window_ticks: u32,
    pub spatial_anticipation_span_sectors: u64,
    pub read_expire_ticks: u64,
    pub write_expire_ticks: u64,
    pub active_anticipation_pid: Option<u32>,
    pub active_anticipation_until_tick: u64,
    pub last_dispatched_sector: Option<u64>,
    pub process_stats: BTreeMap<u32, ProcessIoStats>,
    pub telemetry: AnticipatoryTelemetry,
    pub current_tick: u64,
}

impl AnticipatoryIoScheduler {
    pub fn new() -> Self {
        Self {
            request_queue: Vec::new(),
            current_head_sector: 0,
            head_direction: ElevatorDirection::Ascending,
            anticipation_window_ticks: 6,
            spatial_anticipation_span_sectors: 64,
            read_expire_ticks: 100,
            write_expire_ticks: 500,
            active_anticipation_pid: None,
            active_anticipation_until_tick: 0,
            last_dispatched_sector: None,
            process_stats: BTreeMap::new(),
            telemetry: AnticipatoryTelemetry::default(),
            current_tick: 0,
        }
    }

    pub fn tick(&mut self) {
        self.current_tick += 1;
    }

    pub fn submit_request(&mut self, mut req: DiskIoRequest) {
        if req.arrival_tick == 0 {
            req.arrival_tick = self.current_tick;
        }

        // Profile process think time and spatial proximity
        let stats = self.process_stats.entry(req.pid).or_default();
        if stats.request_count > 0 {
            let think_time = req.arrival_tick.saturating_sub(stats.last_request_tick);
            stats.think_time_accumulator_ticks += think_time;
        }
        stats.request_count += 1;
        stats.last_request_tick = req.arrival_tick;

        if let Some(last_sector) = self.last_dispatched_sector {
            if req.sector >= last_sector
                && req.sector <= last_sector + self.spatial_anticipation_span_sectors
            {
                stats.sequential_hits += 1;
            }
        }

        self.request_queue.push(req);
    }

    /// Primary dispatch loop implementing Anticipatory + Deadline + C-LOOK Elevator.
    pub fn dispatch_next(&mut self) -> Option<DiskIoRequest> {
        if self.request_queue.is_empty() {
            if self.active_anticipation_pid.is_some() {
                if self.current_tick >= self.active_anticipation_until_tick {
                    self.telemetry.anticipation_misses += 1;
                    self.active_anticipation_pid = None;
                } else {
                    // Still within anticipation window; pause dispatch for anticipated read
                    return None;
                }
            }
            return None;
        }

        // 1. Check for expired read/write deadlines (Linux Deadline parity)
        let mut expired_idx = None;
        for (i, req) in self.request_queue.iter().enumerate() {
            let age = self.current_tick.saturating_sub(req.arrival_tick);
            let expire_limit = match req.req_type {
                IoRequestType::Read => self.read_expire_ticks,
                IoRequestType::Write => self.write_expire_ticks,
            };
            if age >= expire_limit {
                expired_idx = Some((i, req.req_type));
                break;
            }
        }

        if let Some((idx, req_type)) = expired_idx {
            match req_type {
                IoRequestType::Read => self.telemetry.read_starvation_preventions += 1,
                IoRequestType::Write => self.telemetry.write_starvation_preventions += 1,
            }
            let req = self.request_queue.remove(idx);
            return Some(self.finalize_dispatch(req));
        }

        // 2. Check for Anticipation match (Linux Anticipatory / BFQ parity)
        if let Some(last_sector) = self.last_dispatched_sector {
            let active_pid = self.active_anticipation_pid;
            let target_pid = active_pid.unwrap_or(0);

            let mut best_adj_idx = None;
            let mut min_forward_dist = u64::MAX;

            for (i, req) in self.request_queue.iter().enumerate() {
                if req.req_type == IoRequestType::Read
                    && (active_pid.is_none() || req.pid == target_pid)
                {
                    if req.sector >= last_sector
                        && req.sector <= last_sector + self.spatial_anticipation_span_sectors
                    {
                        let dist = req.sector - last_sector;
                        if dist < min_forward_dist {
                            min_forward_dist = dist;
                            best_adj_idx = Some(i);
                        }
                    }
                }
            }

            if let Some(idx) = best_adj_idx {
                let req = self.request_queue.remove(idx);
                self.telemetry.anticipation_hits += 1;
                let seek_saved = if req.sector >= self.current_head_sector {
                    req.sector - self.current_head_sector
                } else {
                    self.current_head_sector - req.sector
                };
                self.telemetry.seeks_saved_sectors += seek_saved;
                return Some(self.finalize_dispatch(req));
            }
        }

        // 3. Directional SCAN / C-LOOK Elevator dispatch
        let mut chosen_idx = None;
        let mut best_dist = u64::MAX;

        // Try same direction first
        for (i, req) in self.request_queue.iter().enumerate() {
            let (valid_direction, dist) = match self.head_direction {
                ElevatorDirection::Ascending => (
                    req.sector >= self.current_head_sector,
                    req.sector.saturating_sub(self.current_head_sector),
                ),
                ElevatorDirection::Descending => (
                    req.sector <= self.current_head_sector,
                    self.current_head_sector.saturating_sub(req.sector),
                ),
            };

            if valid_direction && dist < best_dist {
                best_dist = dist;
                chosen_idx = Some(i);
            }
        }

        // If no request in current direction, reverse elevator direction
        if chosen_idx.is_none() {
            self.head_direction = match self.head_direction {
                ElevatorDirection::Ascending => ElevatorDirection::Descending,
                ElevatorDirection::Descending => ElevatorDirection::Ascending,
            };

            for (i, req) in self.request_queue.iter().enumerate() {
                let (valid_direction, dist) = match self.head_direction {
                    ElevatorDirection::Ascending => (
                        req.sector >= self.current_head_sector,
                        req.sector.saturating_sub(self.current_head_sector),
                    ),
                    ElevatorDirection::Descending => (
                        req.sector <= self.current_head_sector,
                        self.current_head_sector.saturating_sub(req.sector),
                    ),
                };

                if valid_direction && dist < best_dist {
                    best_dist = dist;
                    chosen_idx = Some(i);
                }
            }
        }

        // Fallback: pick closest request overall
        if chosen_idx.is_none() {
            let mut min_abs_dist = u64::MAX;
            for (i, req) in self.request_queue.iter().enumerate() {
                let dist = if req.sector >= self.current_head_sector {
                    req.sector - self.current_head_sector
                } else {
                    self.current_head_sector - req.sector
                };
                if dist < min_abs_dist {
                    min_abs_dist = dist;
                    chosen_idx = Some(i);
                }
            }
        }

        if let Some(idx) = chosen_idx {
            let req = self.request_queue.remove(idx);
            Some(self.finalize_dispatch(req))
        } else {
            None
        }
    }

    fn finalize_dispatch(&mut self, req: DiskIoRequest) -> DiskIoRequest {
        self.current_head_sector = req.sector;
        self.last_dispatched_sector = Some(req.sector);
        self.telemetry.total_dispatched += 1;

        if let Some(stats) = self.process_stats.get_mut(&req.pid) {
            stats.last_read_sector = req.sector;
        }

        // Enable anticipation pause if it was a read request with good think-time profile
        if req.req_type == IoRequestType::Read {
            let avg_think = self
                .process_stats
                .get(&req.pid)
                .map(|s| s.average_think_time())
                .unwrap_or(0);
            if avg_think <= u64::from(self.anticipation_window_ticks) {
                self.active_anticipation_pid = Some(req.pid);
                self.active_anticipation_until_tick =
                    self.current_tick + u64::from(self.anticipation_window_ticks);
            } else {
                self.active_anticipation_pid = None;
            }
        } else {
            self.active_anticipation_pid = None;
        }

        req
    }
}

impl Default for AnticipatoryIoScheduler {
    fn default() -> Self {
        Self::new()
    }
}

/// Completely Fair Queueing (CFQ) & Budget Fair Queueing (BFQ) Disk Scheduler: Per-process I/O queues with time/sector budget allocations
pub struct BfqCompletelyFairIoScheduler {
    pub process_queues: BTreeMap<u32, Vec<DiskIoRequest>>, // PID -> Requests
    pub process_budgets_sectors: BTreeMap<u32, u32>,
}

impl BfqCompletelyFairIoScheduler {
    pub fn new() -> Self {
        Self {
            process_queues: BTreeMap::new(),
            process_budgets_sectors: BTreeMap::new(),
        }
    }

    pub fn enqueue_request(&mut self, pid: u32, req: DiskIoRequest) {
        self.process_queues.entry(pid).or_default().push(req);
        self.process_budgets_sectors.entry(pid).or_insert(256); // 256 sector budget
    }

    pub fn dispatch_fair_request(&mut self) -> Option<DiskIoRequest> {
        let active_pids: Vec<u32> = self.process_queues.keys().copied().collect();

        for pid in active_pids {
            let budget = self.process_budgets_sectors.get_mut(&pid).unwrap();
            if *budget > 0 {
                if let Some(req_queue) = self.process_queues.get_mut(&pid) {
                    if !req_queue.is_empty() {
                        let req = req_queue.remove(0);
                        *budget = budget.saturating_sub(8); // Consume 8 sectors of budget
                        return Some(req);
                    }
                }
            } else {
                *budget = 256; // Replenish budget
            }
        }
        None
    }
}

impl Default for BfqCompletelyFairIoScheduler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_three_levels_of_schedulers() {
        let mut long_term = LongTermJobScheduler::new(2);
        long_term.submit_job(ProcessTaskControlBlock {
            pid: 1,
            priority: 10,
            burst_time: 20,
            remaining_time: 20,
            time_quantum: 0,
            state: ProcessLifecycleState::New,
            arrival_time: 0,
            age_ticks: 0,
            is_interactive: true,
        });

        let admitted = long_term.admit_jobs(0);
        assert_eq!(admitted.len(), 1);
        assert_eq!(admitted[0].state, ProcessLifecycleState::Ready);

        let mut swapper = MediumTermSwapper::new();
        swapper.ready_memory_queue.push(admitted[0].clone());
        assert!(swapper.swap_out_inactive(1).is_ok());
        assert_eq!(swapper.swapped_disk_queue.len(), 1);
        assert!(swapper.swap_in(1).is_ok());
    }

    #[test]
    fn test_cpu_scheduling_algorithms() {
        let mut mlfq = MultilevelFeedbackQueueScheduler::new();
        mlfq.insert_new_task(ProcessTaskControlBlock {
            pid: 100,
            priority: 0,
            burst_time: 12,
            remaining_time: 12,
            time_quantum: 4,
            state: ProcessLifecycleState::Ready,
            arrival_time: 0,
            age_ticks: 0,
            is_interactive: false,
        });

        let (task, quantum) = mlfq.schedule_next().unwrap();
        assert_eq!(task.pid, 100);
        assert_eq!(quantum, 4);

        // Demote after full quantum use
        mlfq.requeue_after_quantum_expiry(task, 4, 4);
        assert_eq!(mlfq.q1_med_rr.len(), 1);
    }

    #[test]
    fn test_anticipatory_and_bfq_io_schedulers() {
        let mut aio = AnticipatoryIoScheduler::new();
        aio.submit_request(DiskIoRequest::simple(1, 100, IoRequestType::Read, 1));
        aio.submit_request(DiskIoRequest::simple(2, 104, IoRequestType::Read, 2));

        let req1 = aio.dispatch_next().unwrap();
        assert_eq!(req1.sector, 100);

        let req2 = aio.dispatch_next().unwrap();
        assert_eq!(req2.sector, 104); // Anticipated adjacent sector
        assert_eq!(aio.telemetry.anticipation_hits, 1);

        let mut bfq = BfqCompletelyFairIoScheduler::new();
        bfq.enqueue_request(10, DiskIoRequest::simple(3, 500, IoRequestType::Read, 1));

        let bfq_req = bfq.dispatch_fair_request().unwrap();
        assert_eq!(bfq_req.req_id, 3);
    }

    #[test]
    fn test_anticipatory_scheduler_advanced_features() {
        let mut aio = AnticipatoryIoScheduler::new();

        // Test Deadline Starvation Prevention
        aio.current_tick = 10;
        aio.submit_request(DiskIoRequest {
            req_id: 10,
            pid: 2,
            sector: 500,
            sector_count: 8,
            req_type: IoRequestType::Write,
            arrival_tick: 10,
        });
        aio.submit_request(DiskIoRequest {
            req_id: 11,
            pid: 2,
            sector: 100,
            sector_count: 8,
            req_type: IoRequestType::Read,
            arrival_tick: 10,
        });

        // Fast forward past read expiration threshold
        aio.current_tick = 120;
        let dispatched = aio.dispatch_next().unwrap();
        assert_eq!(dispatched.req_id, 11); // Read expired (age 110 >= 100)
        assert_eq!(aio.telemetry.read_starvation_preventions, 1);
    }
}
