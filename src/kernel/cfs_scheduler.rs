//! CFS (Completely Fair Scheduler)
//!
//! Inspired by Linux CFS and EEVDF schedulers.
//! Provides fair CPU time distribution using red-black tree.
//!
//! # Features
//! - Virtual runtime tracking (vruntime)
//! - Red-black tree for O(log n) operations
//! - Per-CPU run queues
//! - Nice value support (-20 to +19)
//! - Load balancing across CPUs
//!
//! # Linux Inspiration
//! - `kernel/sched/fair.c` - CFS implementation
//! - `kernel/sched/core.c` - Scheduler core
//! - Linux 6.6+ EEVDF (Earliest Eligible Virtual Deadline First)
//!
//! # FreeBSD Inspiration
//! - `sys/kern/sched_ule.c` - ULE scheduler

#![cfg_attr(not(any(feature = "standalone_test", test)), no_std)]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::cmp::Ordering;

/// Scheduler time slice (nanoseconds)
const SCHED_LATENCY_NS: u64 = 6_000_000; // 6ms target latency
const MIN_GRANULARITY_NS: u64 = 750_000; // 0.75ms minimum granularity

/// Nice level weights (from Linux kernel)
/// Maps nice values (-20 to +19) to CPU weight
const NICE_WEIGHTS: [u32; 40] = [
    88761, 71755, 56483, 46273, 36291, // -20 to -16
    29154, 23254, 18705, 14949, 11916, // -15 to -11
    9548, 7620, 6100, 4904, 3906, // -10 to -6
    3121, 2501, 1991, 1586, 1277, // -5 to -1
    1024, 820, 655, 526, 423, // 0 to 4
    335, 272, 215, 172, 137, // 5 to 9
    110, 87, 70, 56, 45, // 10 to 14
    36, 29, 23, 18, 15, // 15 to 19
];

/// Convert nice value to weight
fn nice_to_weight(nice: i8) -> u32 {
    let index = (nice + 20).clamp(0, 39) as usize;
    NICE_WEIGHTS[index]
}

/// Task state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Running,
    Runnable,
    Sleeping,
    Stopped,
    Zombie,
}

/// Scheduling entity (task or thread)
#[derive(Debug, Clone)]
pub struct SchedEntity {
    /// Task ID
    pub id: usize,
    /// Virtual runtime (nanoseconds)
    pub vruntime: u64,
    /// Actual runtime (nanoseconds)
    pub runtime: u64,
    /// Nice value (-20 to +19)
    pub nice: i8,
    /// CPU weight (derived from nice)
    pub weight: u32,
    /// Task state
    pub state: TaskState,
    /// Last time task ran
    pub last_run: u64,
}

impl SchedEntity {
    pub fn new(id: usize, nice: i8) -> Self {
        Self {
            id,
            vruntime: 0,
            runtime: 0,
            nice,
            weight: nice_to_weight(nice),
            state: TaskState::Runnable,
            last_run: 0,
        }
    }

    /// Update virtual runtime
    /// Linux: `kernel/sched/fair.c:update_curr()`
    pub fn update_vruntime(&mut self, delta_ns: u64) {
        // vruntime = runtime * (NICE_0_WEIGHT / weight)
        // This makes higher priority tasks accumulate vruntime slower
        let delta_vruntime = (delta_ns as u128 * 1024 / self.weight as u128) as u64;
        self.vruntime += delta_vruntime;
        self.runtime += delta_ns;
    }
}

impl PartialOrd for SchedEntity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SchedEntity {
    fn cmp(&self, other: &Self) -> Ordering {
        // Primary order: vruntime (lower = higher priority).
        // Tie-break on task id so distinct tasks with identical vruntime
        // remain distinct set members (new tasks are placed at min_vruntime,
        // so ties are the common case, not the exception).
        self.vruntime
            .cmp(&other.vruntime)
            .then_with(|| self.id.cmp(&other.id))
    }
}

impl PartialEq for SchedEntity {
    fn eq(&self, other: &Self) -> bool {
        // Consistent with Ord: identity is (vruntime, id). The old id-only
        // equality disagreed with the ordering contract, which BTreeSet
        // requires to be strictly consistent (Ord == Equal <=> Eq).
        self.vruntime == other.vruntime && self.id == other.id
    }
}

impl Eq for SchedEntity {}

/// Per-CPU run queue
pub struct CfsRunQueue {
    /// Runnable tasks ordered by (vruntime, id) — red-black tree semantics
    /// via BTreeMap. Keying on the composite (vruntime, id) fixes a task-loss
    /// bug: the old vruntime-only key made two tasks sharing one vruntime
    /// overwrite each other, silently dropping one from the queue (new
    /// tasks are placed at min_vruntime, so ties are the common case).
    tasks: BTreeMap<(u64, usize), SchedEntity>,
    /// Secondary index: task id -> vruntime, enabling O(log n) dequeue by id
    /// instead of a full O(n) scan over the tree.
    id_index: BTreeMap<usize, u64>,
    /// Minimum vruntime (for new task placement)
    min_vruntime: u64,
    /// Number of runnable tasks
    nr_running: usize,
    /// Total weight of all runnable tasks
    load_weight: u64,
    /// Currently running task
    current: Option<SchedEntity>,
}

impl CfsRunQueue {
    pub fn new() -> Self {
        Self {
            tasks: BTreeMap::new(),
            id_index: BTreeMap::new(),
            min_vruntime: 0,
            nr_running: 0,
            load_weight: 0,
            current: None,
        }
    }

    /// Add task to run queue
    /// Linux: `kernel/sched/fair.c:enqueue_entity()`
    pub fn enqueue(&mut self, mut task: SchedEntity) {
        // New tasks start at min_vruntime to avoid starvation
        if task.vruntime == 0 {
            task.vruntime = self.min_vruntime;
        }

        self.load_weight += task.weight as u64;
        self.nr_running += 1;

        // Insert into red-black tree keyed by (vruntime, id)
        self.id_index.insert(task.id, task.vruntime);
        self.tasks.insert((task.vruntime, task.id), task);
    }

    /// Remove task from run queue
    /// Linux: `kernel/sched/fair.c:dequeue_entity()`
    /// O(log n) via the id -> vruntime secondary index.
    pub fn dequeue(&mut self, task_id: usize) -> Option<SchedEntity> {
        let vruntime = self.id_index.remove(&task_id)?;
        let task = self.tasks.remove(&(vruntime, task_id))?;
        self.load_weight -= task.weight as u64;
        self.nr_running -= 1;
        Some(task)
    }

    /// Pick next task to run
    /// Linux: `kernel/sched/fair.c:pick_next_entity()`
    pub fn pick_next(&mut self) -> Option<SchedEntity> {
        // Get task with minimum vruntime (leftmost in tree)
        let first_key = *self.tasks.keys().next()?;
        let task = self.tasks.remove(&first_key)?;
        self.id_index.remove(&task.id);
        self.nr_running -= 1;
        self.load_weight -= task.weight as u64;
        Some(task)
    }

    /// Update min_vruntime
    /// Linux: `kernel/sched/fair.c:update_min_vruntime()`
    pub fn update_min_vruntime(&mut self) {
        if let Some(&(vruntime, _)) = self.tasks.keys().next() {
            self.min_vruntime = self.min_vruntime.max(vruntime);
        }

        if let Some(ref current) = self.current {
            self.min_vruntime = self.min_vruntime.max(current.vruntime);
        }
    }

    /// Calculate time slice for task
    /// Linux: `kernel/sched/fair.c:sched_slice()`
    pub fn calc_time_slice(&self, task: &SchedEntity) -> u64 {
        if self.nr_running == 0 {
            return SCHED_LATENCY_NS;
        }

        // Time slice = target_latency * (task_weight / total_weight)
        let slice =
            (SCHED_LATENCY_NS as u128 * task.weight as u128 / self.load_weight as u128) as u64;

        // Ensure minimum granularity
        slice.max(MIN_GRANULARITY_NS)
    }

    /// Get statistics
    pub fn stats(&self) -> CfsStats {
        CfsStats {
            nr_running: self.nr_running,
            min_vruntime: self.min_vruntime,
            load_weight: self.load_weight,
        }
    }
}

/// CFS scheduler statistics
#[derive(Debug, Clone, Copy)]
pub struct CfsStats {
    pub nr_running: usize,
    pub min_vruntime: u64,
    pub load_weight: u64,
}

/// Global CFS scheduler
pub struct CfsScheduler {
    /// Per-CPU run queues
    run_queues: Vec<CfsRunQueue>,
    /// Global time (nanoseconds)
    global_time: u64,
}

impl CfsScheduler {
    /// Create new CFS scheduler
    pub fn new(num_cpus: usize) -> Self {
        let mut run_queues = Vec::new();
        for _ in 0..num_cpus {
            run_queues.push(CfsRunQueue::new());
        }

        Self {
            run_queues,
            global_time: 0,
        }
    }

    /// Add task to scheduler
    pub fn add_task(&mut self, task: SchedEntity, cpu: usize) {
        if cpu < self.run_queues.len() {
            self.run_queues[cpu].enqueue(task);
        }
    }

    /// Remove task from scheduler
    pub fn remove_task(&mut self, task_id: usize, cpu: usize) -> Option<SchedEntity> {
        if cpu < self.run_queues.len() {
            self.run_queues[cpu].dequeue(task_id)
        } else {
            None
        }
    }

    /// Schedule next task on CPU
    /// Linux: `kernel/sched/core.c:schedule()`
    pub fn schedule(&mut self, cpu: usize) -> Option<SchedEntity> {
        if cpu >= self.run_queues.len() {
            return None;
        }

        let rq = &mut self.run_queues[cpu];

        // Put current task back if it's still runnable
        if let Some(mut current) = rq.current.take() {
            if current.state == TaskState::Runnable {
                rq.enqueue(current);
            }
        }

        // Pick next task
        let next = rq.pick_next()?;
        rq.current = Some(next.clone());
        rq.update_min_vruntime();

        Some(next)
    }

    /// Update current task's runtime
    pub fn update_current(&mut self, cpu: usize, delta_ns: u64) {
        if cpu < self.run_queues.len() {
            if let Some(ref mut current) = self.run_queues[cpu].current {
                current.update_vruntime(delta_ns);
            }
        }
        self.global_time += delta_ns;
    }

    /// Load balance across CPUs (simplified)
    /// Linux: `kernel/sched/fair.c:load_balance()`
    pub fn load_balance(&mut self) {
        // Find most and least loaded CPUs
        let mut max_load = 0;
        let mut max_cpu = 0;
        let mut min_load = usize::MAX;
        let mut min_cpu = 0;

        for (i, rq) in self.run_queues.iter().enumerate() {
            let load = rq.nr_running;
            if load > max_load {
                max_load = load;
                max_cpu = i;
            }
            if load < min_load {
                min_load = load;
                min_cpu = i;
            }
        }

        // Balance if difference is significant
        if max_load > min_load + 1 {
            // In real implementation, would migrate tasks
        }
    }

    /// Get global statistics
    pub fn global_stats(&self) -> GlobalSchedStats {
        let total_tasks: usize = self.run_queues.iter().map(|rq| rq.nr_running).sum();
        let total_weight: u64 = self.run_queues.iter().map(|rq| rq.load_weight).sum();

        GlobalSchedStats {
            num_cpus: self.run_queues.len(),
            total_tasks,
            total_weight,
            global_time: self.global_time,
        }
    }
}

/// Global scheduler statistics
#[derive(Debug, Clone, Copy)]
pub struct GlobalSchedStats {
    pub num_cpus: usize,
    pub total_tasks: usize,
    pub total_weight: u64,
    pub global_time: u64,
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nice_to_weight() {
        assert_eq!(nice_to_weight(0), 1024); // Nice 0 is baseline
        assert!(nice_to_weight(-20) > nice_to_weight(0)); // Lower nice = higher weight
        assert!(nice_to_weight(19) < nice_to_weight(0)); // Higher nice = lower weight
    }

    #[test]
    fn test_sched_entity_creation() {
        let task = SchedEntity::new(1, 0);
        assert_eq!(task.id, 1);
        assert_eq!(task.nice, 0);
        assert_eq!(task.weight, 1024);
    }

    #[test]
    fn test_vruntime_update() {
        let mut task = SchedEntity::new(1, 0);
        task.update_vruntime(1_000_000); // 1ms
        assert!(task.vruntime > 0);
        assert_eq!(task.runtime, 1_000_000);
    }

    #[test]
    fn test_runqueue_operations() {
        let mut rq = CfsRunQueue::new();
        let task = SchedEntity::new(1, 0);

        rq.enqueue(task);
        assert_eq!(rq.nr_running, 1);

        let picked = rq.pick_next();
        assert!(picked.is_some());
        assert_eq!(rq.nr_running, 0);
    }

    #[test]
    fn test_scheduler_creation() {
        let sched = CfsScheduler::new(4);
        assert_eq!(sched.run_queues.len(), 4);
    }

    /// Regression: two tasks entering the queue with identical vruntime
    /// (the common case — new tasks are placed at min_vruntime) must both
    /// remain runnable. The old vruntime-keyed BTreeMap silently dropped
    /// one of them.
    #[test]
    fn test_equal_vruntime_no_task_loss() {
        let mut rq = CfsRunQueue::new();
        let t1 = SchedEntity::new(1, 0);
        let t2 = SchedEntity::new(2, 0);
        rq.enqueue(t1);
        rq.enqueue(t2);
        assert_eq!(rq.nr_running, 2, "both equal-vruntime tasks must survive");

        let first = rq.pick_next().expect("first task");
        let second = rq.pick_next().expect("second task must not be lost");
        assert_ne!(first.id, second.id);
        assert_eq!(rq.nr_running, 0);
    }

    /// Dequeue-by-id must be O(log n) via the secondary index and must
    /// remove exactly the requested task, not any other equal-vruntime one.
    #[test]
    fn test_dequeue_by_id_removes_requested_task() {
        let mut rq = CfsRunQueue::new();
        for id in 1..=8 {
            rq.enqueue(SchedEntity::new(id, 0));
        }
        assert_eq!(rq.nr_running, 8);

        let removed = rq.dequeue(5).expect("task 5 must dequeue");
        assert_eq!(removed.id, 5);
        assert_eq!(rq.nr_running, 7);

        // All other tasks still schedulable.
        let mut ids = vec![];
        while let Some(t) = rq.pick_next() {
            ids.push(t.id);
        }
        assert_eq!(ids.len(), 7);
        assert!(!ids.contains(&5));
    }
}
