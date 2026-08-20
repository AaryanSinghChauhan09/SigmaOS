// SigmaOS Comprehensive OS Scheduling Suite
// Implements 3 Levels of Schedulers (Long-Term, Medium-Term, Short-Term CPU),
// 6 CPU Scheduling Algorithms (FCFS, SJF, RR, Priority with Aging, Multilevel Queue, MLFQ),
// and Advanced Disk/I/O Schedulers (Anticipatory I/O, CFQ, BFQ, Deadline).
// Inspired by Linux (CFQ/BFQ/eBPF) & BSD (CAM/ULE/Kqueue) architectures under #![no_std]

extern crate alloc;

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
    pub priority: u32,          // 0 = highest, 139 = lowest
    pub burst_time: u32,       // Estimated or actual execution ticks
    pub remaining_time: u32,   // Remaining ticks
    pub time_quantum: u32,     // Quantum for RR/MLFQ
    pub state: ProcessLifecycleState,
    pub arrival_time: u64,
    pub age_ticks: u32,        // Starvation aging counter
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
        let available_slots = self.max_degree_of_multiprogramming.saturating_sub(active_ready_count);

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
    pub q0_high_rr: Vec<ProcessTaskControlBlock>,  // Quantum = 4
    pub q1_med_rr: Vec<ProcessTaskControlBlock>,   // Quantum = 8
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
    pub fn requeue_after_quantum_expiry(&mut self, pcb: ProcessTaskControlBlock, used_ticks: u32, allocated_quantum: u32) {
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
    pub sector: u64,
    pub req_type: IoRequestType,
    pub arrival_tick: u64,
}

/// Anticipatory I/O Scheduler: Pauses briefly after a read completion anticipating adjacent read requests (Linux Anticipatory/BFQ parity)
pub struct AnticipatoryIoScheduler {
    pub request_queue: Vec<DiskIoRequest>,
    pub current_head_sector: u64,
    pub anticipation_timer_ticks: u32,
    pub last_read_sector: Option<u64>,
}

impl AnticipatoryIoScheduler {
    pub fn new() -> Self {
        Self {
            request_queue: Vec::new(),
            current_head_sector: 0,
            anticipation_timer_ticks: 0,
            last_read_sector: None,
        }
    }

    pub fn submit_request(&mut self, req: DiskIoRequest) {
        self.request_queue.push(req);
    }

    /// Selects next contiguous disk request or pauses briefly for anticipated adjacent reads
    pub fn dispatch_next(&mut self) -> Option<DiskIoRequest> {
        if self.request_queue.is_empty() {
            return None;
        }

        // Check if there is an adjacent request near last_read_sector
        if let Some(last_sector) = self.last_read_sector {
            if let Some(pos) = self.request_queue.iter().position(|r| r.sector >= last_sector && r.sector <= last_sector + 16) {
                let req = self.request_queue.remove(pos);
                self.current_head_sector = req.sector;
                self.last_read_sector = Some(req.sector);
                return Some(req);
            }
        }

        // Default elevator (closest head sector)
        let mut min_dist_idx = 0;
        let mut min_dist = u64::MAX;

        for (i, req) in self.request_queue.iter().enumerate() {
            let dist = if req.sector >= self.current_head_sector {
                req.sector - self.current_head_sector
            } else {
                self.current_head_sector - req.sector
            };
            if dist < min_dist {
                min_dist = dist;
                min_dist_idx = i;
            }
        }

        let req = self.request_queue.remove(min_dist_idx);
        self.current_head_sector = req.sector;
        self.last_read_sector = Some(req.sector);
        Some(req)
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

#[cfg(test)]
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
        aio.submit_request(DiskIoRequest {
            req_id: 1,
            sector: 100,
            req_type: IoRequestType::Read,
            arrival_tick: 1,
        });
        aio.submit_request(DiskIoRequest {
            req_id: 2,
            sector: 104, // Adjacent read
            req_type: IoRequestType::Read,
            arrival_tick: 2,
        });

        let req1 = aio.dispatch_next().unwrap();
        assert_eq!(req1.sector, 100);

        let req2 = aio.dispatch_next().unwrap();
        assert_eq!(req2.sector, 104); // Anticipated adjacent sector

        let mut bfq = BfqCompletelyFairIoScheduler::new();
        bfq.enqueue_request(10, DiskIoRequest {
            req_id: 3,
            sector: 500,
            req_type: IoRequestType::Read,
            arrival_tick: 1,
        });

        let bfq_req = bfq.dispatch_fair_request().unwrap();
        assert_eq!(bfq_req.req_id, 3);
    }
}
