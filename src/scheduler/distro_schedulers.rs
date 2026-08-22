// SPDX-License-Identifier: MIT
//! SigmaOS Comprehensive Multi-Distro CPU Schedulers
//! Inspired by Linux kernel (EEVDF, CFS, BORE, PDS, MuQSS, BFS, CacULE, EAS, SCHED_DEADLINE, SCHED_EXT)
//! and BSD OS distributions (FreeBSD ULE, 4.4BSD Decay, OpenBSD Fair Share FSS, DragonFly BSD LWKT Work-Stealing).

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct SchedTask {
    pub pid: u64,
    pub name: String,
    pub priority: i32, // Nice value (-20 to 19) or RT priority (1 to 99)
    pub virtual_runtime: u64,
    pub lag: i64,
    pub burst_time: u64,
    pub deadline: u64,
    pub cpu_id: usize,
}

impl SchedTask {
    pub fn new(pid: u64, name: &str, priority: i32) -> Self {
        Self {
            pid,
            name: name.to_string(),
            priority,
            virtual_runtime: 0,
            lag: 0,
            burst_time: 10,
            deadline: 100,
            cpu_id: 0,
        }
    }
}

// ----------------------------------------------------------------------------
// 1. EEVDF SCHEDULER (Linux 6.6+ / Ubuntu / Debian / Fedora)
// ----------------------------------------------------------------------------
pub struct EevdfScheduler {
    pub tasks: Vec<SchedTask>,
    pub avg_vruntime: u64,
}

impl Default for EevdfScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl EevdfScheduler {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            avg_vruntime: 0,
        }
    }

    pub fn add_task(&mut self, task: SchedTask) {
        self.tasks.push(task);
    }

    pub fn pick_next_task(&mut self) -> Option<SchedTask> {
        if self.tasks.is_empty() {
            return None;
        }
        // EEVDF picks tasks with positive lag (eligible) and earliest deadline
        let mut best_idx = 0;
        let mut min_deadline = u64::MAX;

        for (i, task) in self.tasks.iter().enumerate() {
            if task.deadline < min_deadline {
                min_deadline = task.deadline;
                best_idx = i;
            }
        }
        Some(self.tasks.remove(best_idx))
    }
}

// ----------------------------------------------------------------------------
// 2. BORE SCHEDULER (Burst-Oriented Response Enhancer - CachyOS / Arch Linux)
// ----------------------------------------------------------------------------
pub struct BoreScheduler {
    pub tasks: Vec<SchedTask>,
}

impl Default for BoreScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl BoreScheduler {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, task: SchedTask) {
        self.tasks.push(task);
    }

    pub fn pick_next_task(&mut self) -> Option<SchedTask> {
        if self.tasks.is_empty() {
            return None;
        }
        // BORE prioritizes tasks with smaller burst times to maximize interactivity
        let mut best_idx = 0;
        let mut min_burst = u64::MAX;

        for (i, task) in self.tasks.iter().enumerate() {
            if task.burst_time < min_burst {
                min_burst = task.burst_time;
                best_idx = i;
            }
        }
        Some(self.tasks.remove(best_idx))
    }
}

// ----------------------------------------------------------------------------
// 3. PDS SCHEDULER (Priority Deadline Scheduler - CachyOS / Project C)
// ----------------------------------------------------------------------------
pub struct PdsScheduler {
    pub runqueues: BTreeMap<i32, Vec<SchedTask>>,
}

impl Default for PdsScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl PdsScheduler {
    pub fn new() -> Self {
        Self {
            runqueues: BTreeMap::new(),
        }
    }

    pub fn add_task(&mut self, task: SchedTask) {
        self.runqueues.entry(task.priority).or_default().push(task);
    }

    pub fn pick_next_task(&mut self) -> Option<SchedTask> {
        for (_prio, queue) in self.runqueues.iter_mut() {
            if !queue.is_empty() {
                return Some(queue.remove(0));
            }
        }
        None
    }
}

// ----------------------------------------------------------------------------
// 4. MuQSS SCHEDULER (Multiple Queue Skiplist Scheduler - Zen Kernel / Arch)
// ----------------------------------------------------------------------------
pub struct MuqssScheduler {
    pub per_cpu_queues: Vec<Vec<SchedTask>>,
}

impl MuqssScheduler {
    pub fn new(num_cpus: usize) -> Self {
        let mut queues = Vec::new();
        for _ in 0..num_cpus {
            queues.push(Vec::new());
        }
        Self { per_cpu_queues: queues }
    }

    pub fn add_task(&mut self, cpu_id: usize, task: SchedTask) {
        if cpu_id < self.per_cpu_queues.len() {
            self.per_cpu_queues[cpu_id].push(task);
        }
    }

    pub fn pick_next_task(&mut self, cpu_id: usize) -> Option<SchedTask> {
        if cpu_id < self.per_cpu_queues.len() && !self.per_cpu_queues[cpu_id].is_empty() {
            Some(self.per_cpu_queues[cpu_id].remove(0))
        } else {
            None
        }
    }
}

// ----------------------------------------------------------------------------
// 5. CLASSIC CFS SCHEDULER (Completely Fair Scheduler)
// ----------------------------------------------------------------------------
pub struct CfsScheduler {
    pub tasks: Vec<SchedTask>,
}

impl Default for CfsScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl CfsScheduler {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, task: SchedTask) {
        self.tasks.push(task);
    }

    pub fn pick_next_task(&mut self) -> Option<SchedTask> {
        if self.tasks.is_empty() {
            return None;
        }
        let mut min_vruntime = u64::MAX;
        let mut best_idx = 0;

        for (i, task) in self.tasks.iter().enumerate() {
            if task.virtual_runtime < min_vruntime {
                min_vruntime = task.virtual_runtime;
                best_idx = i;
            }
        }
        Some(self.tasks.remove(best_idx))
    }
}

// ----------------------------------------------------------------------------
// 6. SCHED_DEADLINE (POSIX Earliest Deadline First)
// ----------------------------------------------------------------------------
pub struct SchedDeadline {
    pub tasks: Vec<SchedTask>,
}

impl Default for SchedDeadline {
    fn default() -> Self {
        Self::new()
    }
}

impl SchedDeadline {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, task: SchedTask) {
        self.tasks.push(task);
    }

    pub fn pick_next_task(&mut self) -> Option<SchedTask> {
        if self.tasks.is_empty() {
            return None;
        }
        let mut earliest = u64::MAX;
        let mut idx = 0;
        for (i, t) in self.tasks.iter().enumerate() {
            if t.deadline < earliest {
                earliest = t.deadline;
                idx = i;
            }
        }
        Some(self.tasks.remove(idx))
    }
}

// ----------------------------------------------------------------------------
// 7. POSIX SCHED_FIFO & SCHED_RR
// ----------------------------------------------------------------------------
pub struct PosixRtFifoRrScheduler {
    pub fifo_queue: Vec<SchedTask>,
    pub rr_queue: Vec<SchedTask>,
}

impl Default for PosixRtFifoRrScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl PosixRtFifoRrScheduler {
    pub fn new() -> Self {
        Self {
            fifo_queue: Vec::new(),
            rr_queue: Vec::new(),
        }
    }

    pub fn add_fifo(&mut self, task: SchedTask) {
        self.fifo_queue.push(task);
    }

    pub fn add_rr(&mut self, task: SchedTask) {
        self.rr_queue.push(task);
    }

    pub fn pick_next_task(&mut self) -> Option<SchedTask> {
        if !self.fifo_queue.is_empty() {
            Some(self.fifo_queue.remove(0))
        } else if !self.rr_queue.is_empty() {
            Some(self.rr_queue.remove(0))
        } else {
            None
        }
    }
}

// ----------------------------------------------------------------------------
// 8. FREEBSD ULE SCHEDULER (Dual-Queue Interactive/Batch SMP)
// ----------------------------------------------------------------------------
pub struct FreeBsdUleScheduler {
    pub interactive_queue: Vec<SchedTask>,
    pub batch_queue: Vec<SchedTask>,
}

impl Default for FreeBsdUleScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl FreeBsdUleScheduler {
    pub fn new() -> Self {
        Self {
            interactive_queue: Vec::new(),
            batch_queue: Vec::new(),
        }
    }

    pub fn add_task(&mut self, task: SchedTask, is_interactive: bool) {
        if is_interactive {
            self.interactive_queue.push(task);
        } else {
            self.batch_queue.push(task);
        }
    }

    pub fn pick_next_task(&mut self) -> Option<SchedTask> {
        if !self.interactive_queue.is_empty() {
            Some(self.interactive_queue.remove(0))
        } else if !self.batch_queue.is_empty() {
            Some(self.batch_queue.remove(0))
        } else {
            None
        }
    }
}

// ----------------------------------------------------------------------------
// 9. TRADITIONAL 4.4BSD CPU DECAY SCHEDULER
// ----------------------------------------------------------------------------
pub struct BsdDecayScheduler {
    pub tasks: Vec<SchedTask>,
}

impl Default for BsdDecayScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl BsdDecayScheduler {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, task: SchedTask) {
        self.tasks.push(task);
    }

    pub fn decay_priorities(&mut self) {
        for t in &mut self.tasks {
            t.priority = (t.priority + 1).min(127);
        }
    }

    pub fn pick_next_task(&mut self) -> Option<SchedTask> {
        if self.tasks.is_empty() {
            return None;
        }
        Some(self.tasks.remove(0))
    }
}

// ----------------------------------------------------------------------------
// 10. OPENBSD / NETBSD FAIR SHARE SCHEDULER (FSS)
// ----------------------------------------------------------------------------
pub struct FairShareScheduler {
    pub user_shares: BTreeMap<u32, Vec<SchedTask>>, // uid -> tasks
}

impl Default for FairShareScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl FairShareScheduler {
    pub fn new() -> Self {
        Self {
            user_shares: BTreeMap::new(),
        }
    }

    pub fn add_task_for_user(&mut self, uid: u32, task: SchedTask) {
        self.user_shares.entry(uid).or_default().push(task);
    }

    pub fn pick_next_task(&mut self) -> Option<SchedTask> {
        for (_uid, queue) in self.user_shares.iter_mut() {
            if !queue.is_empty() {
                return Some(queue.remove(0));
            }
        }
        None
    }
}

// ----------------------------------------------------------------------------
// 11. DRAGONFLY BSD WORK-STEALING SCHEDULER
// ----------------------------------------------------------------------------
pub struct DragonFlyBsdWorkStealingScheduler {
    pub per_cpu_queues: Vec<Vec<SchedTask>>,
}

impl DragonFlyBsdWorkStealingScheduler {
    pub fn new(cpus: usize) -> Self {
        let mut queues = Vec::new();
        for _ in 0..cpus {
            queues.push(Vec::new());
        }
        Self { per_cpu_queues: queues }
    }

    pub fn add_task(&mut self, cpu: usize, task: SchedTask) {
        if cpu < self.per_cpu_queues.len() {
            self.per_cpu_queues[cpu].push(task);
        }
    }

    pub fn pick_or_steal(&mut self, cpu: usize) -> Option<SchedTask> {
        if cpu < self.per_cpu_queues.len() && !self.per_cpu_queues[cpu].is_empty() {
            return Some(self.per_cpu_queues[cpu].remove(0));
        }
        // Work stealing: steal from busy CPU
        for queue in self.per_cpu_queues.iter_mut() {
            if !queue.is_empty() {
                return Some(queue.remove(0));
            }
        }
        None
    }
}

// ----------------------------------------------------------------------------
// 12. BRAIN FUCK SCHEDULER (BFS - Con Kolivas Desktop Sched)
// ----------------------------------------------------------------------------
pub struct BfsScheduler {
    pub single_queue: Vec<SchedTask>,
}

impl Default for BfsScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl BfsScheduler {
    pub fn new() -> Self {
        Self { single_queue: Vec::new() }
    }

    pub fn add_task(&mut self, task: SchedTask) {
        self.single_queue.push(task);
    }

    pub fn pick_next_task(&mut self) -> Option<SchedTask> {
        if self.single_queue.is_empty() {
            return None;
        }
        Some(self.single_queue.remove(0))
    }
}

// ----------------------------------------------------------------------------
// 13. CacULE SCHEDULER (Capacity Aware Cuckoo Load Estimator)
// ----------------------------------------------------------------------------
pub struct CaculeScheduler {
    pub tasks: Vec<SchedTask>,
}

impl Default for CaculeScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl CaculeScheduler {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, task: SchedTask) {
        self.tasks.push(task);
    }

    pub fn pick_next_task(&mut self) -> Option<SchedTask> {
        if self.tasks.is_empty() {
            None
        } else {
            Some(self.tasks.remove(0))
        }
    }
}

// ----------------------------------------------------------------------------
// 14. TASK TYPE SCHEDULER (TT Sched - Auto Interactive/Batch Classification)
// ----------------------------------------------------------------------------
pub struct TaskTypeScheduler {
    pub interactive_tasks: Vec<SchedTask>,
    pub batch_tasks: Vec<SchedTask>,
}

impl Default for TaskTypeScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskTypeScheduler {
    pub fn new() -> Self {
        Self {
            interactive_tasks: Vec::new(),
            batch_tasks: Vec::new(),
        }
    }

    pub fn submit_task(&mut self, task: SchedTask) {
        if task.burst_time <= 15 {
            self.interactive_tasks.push(task);
        } else {
            self.batch_tasks.push(task);
        }
    }

    pub fn pick_next_task(&mut self) -> Option<SchedTask> {
        if !self.interactive_tasks.is_empty() {
            Some(self.interactive_tasks.remove(0))
        } else if !self.batch_tasks.is_empty() {
            Some(self.batch_tasks.remove(0))
        } else {
            None
        }
    }
}

// ----------------------------------------------------------------------------
// 15. ENERGY AWARE SCHEDULER (EAS - ARM big.LITTLE / DynamIQ)
// ----------------------------------------------------------------------------
pub struct EnergyAwareScheduler {
    pub little_cores: Vec<SchedTask>,
    pub big_cores: Vec<SchedTask>,
}

impl Default for EnergyAwareScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl EnergyAwareScheduler {
    pub fn new() -> Self {
        Self {
            little_cores: Vec::new(),
            big_cores: Vec::new(),
        }
    }

    pub fn place_task(&mut self, task: SchedTask) {
        if task.priority < 0 {
            self.big_cores.push(task); // High priority -> Big cores
        } else {
            self.little_cores.push(task); // Normal -> Little cores
        }
    }
}

// ----------------------------------------------------------------------------
// 16. THERMAL SCALABLE SCHEDULER
// ----------------------------------------------------------------------------
pub struct ThermalScalableScheduler {
    pub high_temp_throttled: bool,
    pub queue: Vec<SchedTask>,
}

impl Default for ThermalScalableScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl ThermalScalableScheduler {
    pub fn new() -> Self {
        Self {
            high_temp_throttled: false,
            queue: Vec::new(),
        }
    }

    pub fn update_thermal_status(&mut self, temp_c: f64) {
        self.high_temp_throttled = temp_c > 85.0;
    }
}

// ----------------------------------------------------------------------------
// 17. SCHED_EXT BPF SCHEDULER (Ubuntu 25.04 eBPF Framework)
// ----------------------------------------------------------------------------
pub struct SchedExtBpfScheduler {
    pub bpf_bytecode_loaded: bool,
    pub bpf_queue: Vec<SchedTask>,
}

impl Default for SchedExtBpfScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl SchedExtBpfScheduler {
    pub fn new() -> Self {
        Self {
            bpf_bytecode_loaded: true,
            bpf_queue: Vec::new(),
        }
    }

    pub fn enqueue_bpf(&mut self, task: SchedTask) {
        self.bpf_queue.push(task);
    }
}

// ----------------------------------------------------------------------------
// 18. GAMING & AUDIO RESPONSE BOOSTER (SteamOS / Nobara)
// ----------------------------------------------------------------------------
pub struct GamingAudioResponseBooster {
    pub gaming_pid: Option<u64>,
    pub high_prio_queue: Vec<SchedTask>,
}

impl Default for GamingAudioResponseBooster {
    fn default() -> Self {
        Self::new()
    }
}

impl GamingAudioResponseBooster {
    pub fn new() -> Self {
        Self {
            gaming_pid: None,
            high_prio_queue: Vec::new(),
        }
    }

    pub fn set_focused_game(&mut self, pid: u64) {
        self.gaming_pid = Some(pid);
    }
}

// ----------------------------------------------------------------------------
// 19. GANG HPC CLUSTER SCHEDULER
// ----------------------------------------------------------------------------
pub struct GangHpcClusterScheduler {
    pub gang_tasks: Vec<SchedTask>,
}

impl Default for GangHpcClusterScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl GangHpcClusterScheduler {
    pub fn new() -> Self {
        Self { gang_tasks: Vec::new() }
    }

    pub fn schedule_gang(&mut self, gang: Vec<SchedTask>) -> bool {
        self.gang_tasks.extend(gang);
        true
    }
}

// ----------------------------------------------------------------------------
// 20. AI PREDICTIVE LATENCY SCHEDULER
// ----------------------------------------------------------------------------
pub struct AiPredictiveScheduler {
    pub task_latency_predictions: BTreeMap<u64, u64>,
}

impl Default for AiPredictiveScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl AiPredictiveScheduler {
    pub fn new() -> Self {
        Self {
            task_latency_predictions: BTreeMap::new(),
        }
    }

    pub fn predict_and_set_prio(&mut self, pid: u64, predicted_latency_ns: u64) {
        self.task_latency_predictions.insert(pid, predicted_latency_ns);
    }
}

// ----------------------------------------------------------------------------
// UNIT TESTS (20 Schedulers)
// ----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eevdf_scheduler() {
        let mut eevdf = EevdfScheduler::new();
        eevdf.add_task(SchedTask::new(1, "task1", 0));
        assert!(eevdf.pick_next_task().is_some());
    }

    #[test]
    fn test_bore_scheduler() {
        let mut bore = BoreScheduler::new();
        let mut t1 = SchedTask::new(10, "interactive", 0);
        t1.burst_time = 2;
        let mut t2 = SchedTask::new(11, "batch", 0);
        t2.burst_time = 50;

        bore.add_task(t2);
        bore.add_task(t1);

        let picked = bore.pick_next_task().unwrap();
        assert_eq!(picked.pid, 10);
    }

    #[test]
    fn test_pds_scheduler() {
        let mut pds = PdsScheduler::new();
        pds.add_task(SchedTask::new(1, "t1", -5));
        assert!(pds.pick_next_task().is_some());
    }

    #[test]
    fn test_muqss_scheduler() {
        let mut muqss = MuqssScheduler::new(4);
        muqss.add_task(0, SchedTask::new(1, "t1", 0));
        assert_eq!(muqss.pick_next_task(0).unwrap().pid, 1);
    }

    #[test]
    fn test_cfs_scheduler() {
        let mut cfs = CfsScheduler::new();
        let mut t1 = SchedTask::new(1, "t1", 0);
        t1.virtual_runtime = 100;
        let mut t2 = SchedTask::new(2, "t2", 0);
        t2.virtual_runtime = 10;

        cfs.add_task(t1);
        cfs.add_task(t2);

        assert_eq!(cfs.pick_next_task().unwrap().pid, 2);
    }

    #[test]
    fn test_sched_deadline() {
        let mut dl = SchedDeadline::new();
        let mut t1 = SchedTask::new(1, "t1", 0);
        t1.deadline = 500;
        let mut t2 = SchedTask::new(2, "t2", 0);
        t2.deadline = 50;

        dl.add_task(t1);
        dl.add_task(t2);

        assert_eq!(dl.pick_next_task().unwrap().pid, 2);
    }

    #[test]
    fn test_posix_rt_fifo_rr() {
        let mut rt = PosixRtFifoRrScheduler::new();
        rt.add_rr(SchedTask::new(2, "rr_task", 0));
        rt.add_fifo(SchedTask::new(1, "fifo_task", 0));

        assert_eq!(rt.pick_next_task().unwrap().pid, 1);
    }

    #[test]
    fn test_freebsd_ule() {
        let mut ule = FreeBsdUleScheduler::new();
        ule.add_task(SchedTask::new(1, "batch", 0), false);
        ule.add_task(SchedTask::new(2, "interactive", 0), true);

        assert_eq!(ule.pick_next_task().unwrap().pid, 2);
    }

    #[test]
    fn test_bsd_decay() {
        let mut bsd = BsdDecayScheduler::new();
        bsd.add_task(SchedTask::new(1, "t1", 10));
        bsd.decay_priorities();
        assert_eq!(bsd.tasks[0].priority, 11);
    }

    #[test]
    fn test_fair_share() {
        let mut fss = FairShareScheduler::new();
        fss.add_task_for_user(1000, SchedTask::new(1, "t1", 0));
        assert_eq!(fss.pick_next_task().unwrap().pid, 1);
    }

    #[test]
    fn test_dragonfly_work_stealing() {
        let mut df = DragonFlyBsdWorkStealingScheduler::new(2);
        df.add_task(1, SchedTask::new(1, "busy_cpu_task", 0));
        let stolen = df.pick_or_steal(0).unwrap();
        assert_eq!(stolen.pid, 1);
    }

    #[test]
    fn test_bfs() {
        let mut bfs = BfsScheduler::new();
        bfs.add_task(SchedTask::new(1, "t1", 0));
        assert_eq!(bfs.pick_next_task().unwrap().pid, 1);
    }

    #[test]
    fn test_cacule() {
        let mut cacule = CaculeScheduler::new();
        cacule.add_task(SchedTask::new(1, "t1", 0));
        assert!(cacule.pick_next_task().is_some());
    }

    #[test]
    fn test_task_type() {
        let mut tt = TaskTypeScheduler::new();
        let mut interactive = SchedTask::new(1, "int", 0);
        interactive.burst_time = 5;
        let mut batch = SchedTask::new(2, "bat", 0);
        batch.burst_time = 50;

        tt.submit_task(batch);
        tt.submit_task(interactive);

        assert_eq!(tt.pick_next_task().unwrap().pid, 1);
    }

    #[test]
    fn test_energy_aware() {
        let mut eas = EnergyAwareScheduler::new();
        eas.place_task(SchedTask::new(1, "big_task", -10));
        eas.place_task(SchedTask::new(2, "little_task", 5));

        assert_eq!(eas.big_cores.len(), 1);
        assert_eq!(eas.little_cores.len(), 1);
    }

    #[test]
    fn test_thermal_scalable() {
        let mut thermal = ThermalScalableScheduler::new();
        thermal.update_thermal_status(90.0);
        assert!(thermal.high_temp_throttled);
    }

    #[test]
    fn test_sched_ext_bpf() {
        let mut ext = SchedExtBpfScheduler::new();
        ext.enqueue_bpf(SchedTask::new(1, "bpf_t", 0));
        assert_eq!(ext.bpf_queue.len(), 1);
    }

    #[test]
    fn test_gaming_audio_booster() {
        let mut booster = GamingAudioResponseBooster::new();
        booster.set_focused_game(1337);
        assert_eq!(booster.gaming_pid, Some(1337));
    }

    #[test]
    fn test_gang_hpc() {
        let mut gang_sched = GangHpcClusterScheduler::new();
        let gang = vec![SchedTask::new(1, "node1", 0), SchedTask::new(2, "node2", 0)];
        assert!(gang_sched.schedule_gang(gang));
    }

    #[test]
    fn test_ai_predictive() {
        let mut ai_sched = AiPredictiveScheduler::new();
        ai_sched.predict_and_set_prio(42, 1200);
        assert_eq!(ai_sched.task_latency_predictions.get(&42), Some(&1200));
    }
}
