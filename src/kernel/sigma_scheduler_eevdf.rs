//! # SigmaOS EEVDF Scheduler
//!
//! Earliest Eligible Virtual Deadline First (EEVDF) process scheduler for SigmaOS.
//! Inspired by Linux 6.6 EEVDF and BSD ULE (UltraLight Execution) scheduler.
//!
//! ## Design Goals
//! - Fairness: weighted-fair CPU sharing across all tasks
//! - Low latency: interactive tasks get preemptive priority
//! - Multi-CPU: per-runqueue design with load-balancing stubs
//! - No external dependencies: pure Rust, `no_std`-compatible logic
//!
//! ## Algorithm
//! Each task carries:
//! - `vruntime`: accumulated virtual runtime (normalized by weight)
//! - `eligible_time`: earliest time the task may be picked (lag-based)
//! - `deadline`: virtual deadline = eligible_time + slice / weight
//!
//! `pick_next_task` selects the *eligible* task with the smallest virtual deadline.

#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::BTreeMap;
use std::vec::Vec;

// ── Constants ────────────────────────────────────────────────────────────────

/// Default time slice in nanoseconds (4 ms, matching Linux EEVDF default).
pub const DEFAULT_SLICE_NS: u64 = 4_000_000;

/// Minimum task weight (nice +19 equivalent).
pub const MIN_WEIGHT: u64 = 1;

/// Default task weight (nice 0 equivalent).
pub const DEFAULT_WEIGHT: u64 = 1024;

/// Maximum task weight (nice -20 equivalent).
pub const MAX_WEIGHT: u64 = 88_761;

// ── SigmaTask ────────────────────────────────────────────────────────────────

/// Unique task identifier.
pub type TaskId = u64;

/// Represents a schedulable task in the SigmaOS EEVDF runqueue.
///
/// # Fields
/// - `id`: unique task identifier
/// - `vruntime`: accumulated virtual runtime in nanoseconds
/// - `eligible_time`: earliest virtual time this task may be selected
/// - `deadline`: virtual deadline = `eligible_time + slice / weight`
/// - `weight`: scheduling weight (higher = more CPU share)
/// - `slice_ns`: configured time slice in nanoseconds
/// - `on_rq`: whether the task is currently enqueued
#[derive(Debug, Clone)]
pub struct SigmaTask {
    /// Unique task identifier.
    pub id: TaskId,
    /// Accumulated virtual runtime (ns), normalised by weight.
    pub vruntime: u64,
    /// Earliest virtual time at which this task is eligible to run.
    pub eligible_time: u64,
    /// Virtual deadline: `eligible_time + slice_ns / weight`.
    pub deadline: u64,
    /// Scheduling weight (proportional to CPU share).
    pub weight: u64,
    /// Configured time slice in nanoseconds.
    pub slice_ns: u64,
    /// True when the task is on the runqueue.
    pub on_rq: bool,
    /// CPU affinity mask (bit i = allowed on CPU i).  0 = any CPU.
    pub cpu_affinity: u64,
    /// Human-readable task name (for debugging).
    pub name: std::string::String,
}

impl SigmaTask {
    /// Create a new task with the given id, name, and weight.
    ///
    /// `vruntime`, `eligible_time`, and `deadline` are initialized to
    /// the current runqueue's `min_vruntime` by [`SigmaEevdfRunqueue::enqueue_task`].
    pub fn new(id: TaskId, name: impl Into<std::string::String>, weight: u64) -> Self {
        let weight = weight.clamp(MIN_WEIGHT, MAX_WEIGHT);
        SigmaTask {
            id,
            vruntime: 0,
            eligible_time: 0,
            deadline: 0,
            weight,
            slice_ns: DEFAULT_SLICE_NS,
            on_rq: false,
            cpu_affinity: 0,
            name: name.into(),
        }
    }

    /// Compute the virtual deadline given the current eligible time.
    ///
    /// `deadline = eligible_time + (slice_ns << SCHED_FIXEDPOINT_SHIFT) / weight`
    pub fn compute_deadline(&self) -> u64 {
        self.eligible_time + (self.slice_ns << 10) / self.weight
    }

    /// Recompute and update the task's virtual deadline in place.
    pub fn refresh_deadline(&mut self) {
        self.deadline = self.compute_deadline();
    }

    /// Return `true` if the task is eligible at the given virtual clock.
    #[inline]
    pub fn is_eligible(&self, vtime: u64) -> bool {
        self.eligible_time <= vtime
    }
}

// ── SigmaEevdfRunqueue ───────────────────────────────────────────────────────

/// EEVDF runqueue for a single logical CPU.
///
/// Maintains:
/// - An ordered map of tasks keyed by `(deadline, task_id)` for O(log n) picks.
/// - `min_vruntime`: the floor used to set the eligible_time of newly-enqueued tasks.
/// - `curr`: the currently-running task id, if any.
#[derive(Debug)]
pub struct SigmaEevdfRunqueue {
    /// CPU index this runqueue belongs to.
    pub cpu: u32,
    /// Tasks keyed by (deadline, id) for ordered traversal.
    /// Value is the task itself.
    tasks: BTreeMap<(u64, TaskId), SigmaTask>,
    /// Monotonically increasing virtual clock (ns).
    pub min_vruntime: u64,
    /// Currently running task id.
    pub curr: Option<TaskId>,
    /// Number of runnable tasks.
    pub nr_running: usize,
    /// Total weight of all runnable tasks.
    pub total_weight: u64,
    /// Accumulated real time the CPU has been busy (ns).
    pub clock_task: u64,
}

impl SigmaEevdfRunqueue {
    /// Create a new, empty EEVDF runqueue for `cpu`.
    pub fn new(cpu: u32) -> Self {
        SigmaEevdfRunqueue {
            cpu,
            tasks: BTreeMap::new(),
            min_vruntime: 0,
            curr: None,
            nr_running: 0,
            total_weight: 0,
            clock_task: 0,
        }
    }

    /// Enqueue a task onto this runqueue.
    ///
    /// Sets `vruntime` to `max(task.vruntime, min_vruntime)` so a waking task
    /// does not receive an unfair head-start, and recomputes `eligible_time`
    /// and `deadline`.
    pub fn enqueue_task(&mut self, mut task: SigmaTask) {
        // Prevent tasks from claiming CPU time they haven't earned.
        task.vruntime = task.vruntime.max(self.min_vruntime);
        task.eligible_time = task.vruntime;
        task.refresh_deadline();
        task.on_rq = true;

        self.nr_running += 1;
        self.total_weight += task.weight;

        self.tasks.insert((task.deadline, task.id), task);
    }

    /// Remove a task from the runqueue by id.
    ///
    /// Returns the task if it was found, `None` otherwise.
    pub fn dequeue_task(&mut self, id: TaskId) -> Option<SigmaTask> {
        // We need to find the key first (scan by id).
        let key = self.tasks
            .iter()
            .find(|(_, t)| t.id == id)
            .map(|(k, _)| *k);

        if let Some(k) = key {
            let mut task = self.tasks.remove(&k).unwrap();
            task.on_rq = false;
            self.nr_running = self.nr_running.saturating_sub(1);
            self.total_weight = self.total_weight.saturating_sub(task.weight);
            if self.curr == Some(id) {
                self.curr = None;
            }
            Some(task)
        } else {
            None
        }
    }

    /// Pick the next task to run.
    ///
    /// Selects the eligible task with the smallest virtual deadline.
    /// A task is eligible when `eligible_time <= min_vruntime`.
    ///
    /// If no task is eligible (lag > 0 for all tasks), fall back to the
    /// task with the smallest deadline regardless of eligibility.
    pub fn pick_next_task(&mut self) -> Option<TaskId> {
        if self.tasks.is_empty() {
            return None;
        }

        // First pass: find eligible task with smallest deadline.
        let eligible = self.tasks
            .iter()
            .find(|(_, t)| t.is_eligible(self.min_vruntime))
            .map(|(k, _)| *k);

        let key = if let Some(k) = eligible {
            k
        } else {
            // Fallback: smallest deadline among all tasks (EEVDF lag compensation).
            *self.tasks.keys().next().unwrap()
        };

        self.curr = Some(key.1);
        Some(key.1)
    }

    /// Update virtual runtime after a task has run for `delta_ns` real nanoseconds.
    ///
    /// `vruntime_delta = delta_ns * DEFAULT_WEIGHT / task.weight`
    /// (heavier tasks accumulate vruntime more slowly = more CPU share).
    pub fn update_curr(&mut self, delta_ns: u64) {
        let id = match self.curr {
            Some(id) => id,
            None => return,
        };

        // Find and update the task.
        let key = self.tasks
            .iter()
            .find(|(_, t)| t.id == id)
            .map(|(k, _)| *k);

        if let Some(k) = key {
            let mut task = self.tasks.remove(&k).unwrap();

            let vdelta = (delta_ns * DEFAULT_WEIGHT) / task.weight;
            task.vruntime += vdelta;
            task.eligible_time = task.vruntime;
            task.refresh_deadline();

            // Advance the virtual clock.
            self.min_vruntime = self.min_vruntime.max(task.vruntime);
            self.clock_task += delta_ns;

            self.tasks.insert((task.deadline, task.id), task);
        }
    }

    /// Return `true` if the currently-running task should be preempted.
    ///
    /// Preemption is warranted when:
    /// 1. There is a different eligible task whose deadline < current task's deadline.
    /// 2. The current task has exhausted its time slice.
    pub fn should_preempt(&self, elapsed_ns: u64) -> bool {
        let curr_id = match self.curr {
            Some(id) => id,
            None => return false,
        };

        // Find current task's deadline.
        let curr_deadline = self.tasks
            .iter()
            .find(|(_, t)| t.id == curr_id)
            .map(|(k, _)| k.0)
            .unwrap_or(u64::MAX);

        // Slice exhaustion check.
        let curr_slice = self.tasks
            .iter()
            .find(|(_, t)| t.id == curr_id)
            .map(|(_, t)| t.slice_ns)
            .unwrap_or(DEFAULT_SLICE_NS);

        if elapsed_ns >= curr_slice {
            return true;
        }

        // Earlier-deadline eligible task check.
        for (k, t) in &self.tasks {
            if t.id == curr_id {
                continue;
            }
            if t.is_eligible(self.min_vruntime) && k.0 < curr_deadline {
                return true;
            }
        }

        false
    }

    /// Return the number of runnable tasks.
    pub fn len(&self) -> usize {
        self.nr_running
    }

    /// Return `true` if the runqueue is empty.
    pub fn is_empty(&self) -> bool {
        self.nr_running == 0
    }

    /// Peek at the currently-running task (immutable).
    pub fn current_task(&self) -> Option<&SigmaTask> {
        let id = self.curr?;
        self.tasks.values().find(|t| t.id == id)
    }
}

// ── Multi-CPU Load Balancer ──────────────────────────────────────────────────

/// Per-CPU runqueue collection and load-balancing engine.
///
/// Load balancing is triggered when one CPU's runqueue length exceeds another's
/// by more than [`IMBALANCE_THRESHOLD`].
pub struct SigmaLoadBalancer {
    /// One runqueue per logical CPU.
    pub runqueues: Vec<SigmaEevdfRunqueue>,
}

/// Minimum ratio difference before a load migration is triggered.
const IMBALANCE_THRESHOLD: usize = 2;

impl SigmaLoadBalancer {
    /// Create a load balancer for `nr_cpus` logical processors.
    pub fn new(nr_cpus: u32) -> Self {
        let runqueues = (0..nr_cpus).map(SigmaEevdfRunqueue::new).collect();
        SigmaLoadBalancer { runqueues }
    }

    /// Find the CPU with the fewest runnable tasks.
    pub fn least_loaded_cpu(&self) -> u32 {
        self.runqueues
            .iter()
            .enumerate()
            .min_by_key(|(_, rq)| rq.nr_running)
            .map(|(i, _)| i as u32)
            .unwrap_or(0)
    }

    /// Find the CPU with the most runnable tasks.
    pub fn most_loaded_cpu(&self) -> u32 {
        self.runqueues
            .iter()
            .enumerate()
            .max_by_key(|(_, rq)| rq.nr_running)
            .map(|(i, _)| i as u32)
            .unwrap_or(0)
    }

    /// Attempt to migrate one task from the busiest CPU to the idlest CPU.
    ///
    /// Returns `Some((src_cpu, dst_cpu, task_id))` if a migration was performed.
    pub fn balance(&mut self) -> Option<(u32, u32, TaskId)> {
        let src = self.most_loaded_cpu() as usize;
        let dst = self.least_loaded_cpu() as usize;

        if src == dst {
            return None;
        }

        let imbalance = self.runqueues[src].nr_running
            .saturating_sub(self.runqueues[dst].nr_running);

        if imbalance < IMBALANCE_THRESHOLD {
            return None;
        }

        // Pick a migratable task from src (not currently running, affinity allows dst).
        let dst_cpu = dst as u32;
        let candidate_key = {
            let src_rq = &self.runqueues[src];
            src_rq.tasks
                .iter()
                .filter(|(_, t)| {
                    Some(t.id) != src_rq.curr
                        && (t.cpu_affinity == 0 || t.cpu_affinity & (1u64 << dst_cpu) != 0)
                })
                .map(|(k, _)| *k)
                .next()
        };

        let (k, task_id) = candidate_key.map(|k| (k, k.1))?;

        // Move the task.
        let task = self.runqueues[src].tasks.remove(&k)?;
        self.runqueues[src].nr_running = self.runqueues[src].nr_running.saturating_sub(1);
        self.runqueues[src].total_weight =
            self.runqueues[src].total_weight.saturating_sub(task.weight);

        self.runqueues[dst].enqueue_task(task);

        Some((src as u32, dst_cpu, task_id))
    }
}

// ── Unit Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_task(id: TaskId, weight: u64) -> SigmaTask {
        SigmaTask::new(id, format!("task-{}", id), weight)
    }

    #[test]
    fn test_enqueue_dequeue() {
        let mut rq = SigmaEevdfRunqueue::new(0);
        rq.enqueue_task(make_task(1, DEFAULT_WEIGHT));
        assert_eq!(rq.len(), 1);
        let t = rq.dequeue_task(1);
        assert!(t.is_some());
        assert_eq!(rq.len(), 0);
    }

    #[test]
    fn test_pick_next_task() {
        let mut rq = SigmaEevdfRunqueue::new(0);
        rq.enqueue_task(make_task(1, DEFAULT_WEIGHT));
        rq.enqueue_task(make_task(2, DEFAULT_WEIGHT * 2));
        let picked = rq.pick_next_task();
        assert!(picked.is_some());
    }

    #[test]
    fn test_should_preempt_on_slice_exhaustion() {
        let mut rq = SigmaEevdfRunqueue::new(0);
        rq.enqueue_task(make_task(1, DEFAULT_WEIGHT));
        rq.pick_next_task();
        // Elapsed >= slice → must preempt.
        assert!(rq.should_preempt(DEFAULT_SLICE_NS));
    }

    #[test]
    fn test_load_balancer() {
        let mut lb = SigmaLoadBalancer::new(2);
        // Load CPU 0 with 4 tasks, CPU 1 empty.
        for i in 0..4u64 {
            lb.runqueues[0].enqueue_task(make_task(i, DEFAULT_WEIGHT));
        }
        let result = lb.balance();
        assert!(result.is_some());
        let (src, dst, _) = result.unwrap();
        assert_eq!(src, 0);
        assert_eq!(dst, 1);
    }
}
