//! SigmaOS — BORE Burst-Oriented Response Enhancer Scheduler
//! SPDX-License-Identifier: MIT OR GPL-2.0
//! Inspired by Masahito Suzuki's BORE patch for Linux CFS
//! Reference: https://github.com/firelzrd/bore-scheduler
//!
//! BORE enhances desktop responsiveness by tracking per-task CPU burst history
//! and applying scheduling penalties to bursty tasks while boosting interactive
//! (low-burst) tasks. Burst score uses an EMA: score = score*7/8 + burst*1/8.

#![allow(dead_code)]

extern crate alloc;
use alloc::vec::Vec;

// ──────────────────────────────────────────────────────────
// Constants
// ──────────────────────────────────────────────────────────

/// Scale factor for burst penalty calculation
pub const BURST_PENALTY_SCALE: u64 = 256;

/// Shift used to normalise burst score before penalty lookup
pub const BURST_SCORE_SHIFT: u32 = 8;

/// Latency weight for nice-value adjustments (1 = linear)
pub const NICE_LATENCY_WEIGHT: u64 = 1;

/// Burst score threshold above which a penalty is applied (10 ms in ns)
pub const BURST_PENALTY_THRESHOLD_NS: u64 = 10_000_000;

/// Maximum burst penalty added to vruntime (5 ms in ns)
pub const MAX_BURST_PENALTY_NS: u64 = 5_000_000;

/// Maximum boost given to interactive tasks (2 ms in ns)
pub const MAX_BURST_BOOST_NS: u64 = 2_000_000;

/// Nice 0 weight (mirrors Linux CFS baseline)
pub const NICE_0_WEIGHT: u64 = 1024;

// ──────────────────────────────────────────────────────────
// Task state
// ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoreTaskState {
    Ready,
    Running,
    Blocked,
    Completed,
}

// ──────────────────────────────────────────────────────────
// BoreTask
// ──────────────────────────────────────────────────────────

/// A task managed by the BORE scheduler.
///
/// # Fields
/// * `vruntime`    – weighted virtual runtime in nanoseconds
/// * `burst_time`  – CPU time consumed in the current scheduling window (ns)
/// * `burst_score` – exponential moving average of burst_time; drives penalty
/// * `nice_value`  – Linux-style nice level (−20 = highest priority, +19 = lowest)
/// * `weight`      – derived from `nice_value` (higher weight → more CPU)
/// * `pid`         – process/thread identifier
/// * `state`       – current scheduling state
#[derive(Debug, Clone)]
pub struct BoreTask {
    pub pid: u32,
    pub vruntime: u64,
    pub burst_time: u64,
    pub burst_score: u64,
    pub nice_value: i8,
    pub weight: u64,
    pub state: BoreTaskState,
}

impl BoreTask {
    /// Create a new BORE task.
    ///
    /// `nice_value` is clamped to [−20, +19].
    pub fn new(pid: u32, nice_value: i8) -> Self {
        let nice = nice_value.clamp(-20, 19);
        let weight = bore_nice_to_weight(nice);
        Self {
            pid,
            vruntime: 0,
            burst_time: 0,
            burst_score: 0,
            nice_value: nice,
            weight,
            state: BoreTaskState::Ready,
        }
    }

    /// Effective vruntime used by the scheduler for ordering.
    ///
    /// Interactive tasks (low burst_score) get a *negative* adjustment
    /// (boost), bursty tasks get a *positive* adjustment (penalty).
    #[inline]
    pub fn effective_vruntime(&self) -> u64 {
        let penalty = bore_penalty(self.burst_score);
        // Apply signed adjustment, clamping so vruntime never wraps
        if self.burst_score >= BURST_PENALTY_THRESHOLD_NS {
            self.vruntime.saturating_add(penalty)
        } else {
            // Boost: subtract, clamped at zero
            self.vruntime.saturating_sub(penalty)
        }
    }
}

// ──────────────────────────────────────────────────────────
// Nice → weight table (Linux-compatible)
// ──────────────────────────────────────────────────────────

/// Map a nice level [−20, +19] to a CPU-weight value (same as CFS).
pub fn bore_nice_to_weight(nice: i8) -> u64 {
    const WEIGHT_TABLE: [u64; 40] = [
        /* -20 */ 88761, 71755, 56483, 46273, 36291,
        /* -15 */ 29154, 23254, 18705, 14949, 11916,
        /* -10 */  9548,  7620,  6100,  4904,  3906,
        /*  -5 */  3121,  2501,  1991,  1586,  1277,
        /*   0 */  1024,   820,   655,   526,   423,
        /*   5 */   335,   272,   215,   172,   137,
        /*  10 */   110,    87,    70,    56,    45,
        /*  15 */    36,    29,    23,    18,    15,
    ];
    let idx = (nice.clamp(-20, 19) + 20) as usize;
    WEIGHT_TABLE[idx]
}

// ──────────────────────────────────────────────────────────
// Burst accounting
// ──────────────────────────────────────────────────────────

/// Update the burst score with an exponential moving average.
///
/// Formula: `burst_score = burst_score * 7/8 + burst_time * 1/8`
///
/// This decays stale burst history while integrating new burst samples.
#[inline]
pub fn bore_update_burst_score(burst_score: u64, burst_time: u64) -> u64 {
    burst_score
        .saturating_mul(7)
        .wrapping_div(8)
        .saturating_add(burst_time.wrapping_div(8))
}

/// Calculate the burst penalty (or boost magnitude) for a given burst score.
///
/// * If `burst_score` ≥ `BURST_PENALTY_THRESHOLD_NS` → penalty is proportional
///   to how far above the threshold the score is, capped at `MAX_BURST_PENALTY_NS`.
/// * If `burst_score` < threshold → boost is proportional to how far below the
///   threshold, capped at `MAX_BURST_BOOST_NS`.
///
/// The caller checks the direction (penalty vs boost) by comparing `burst_score`
/// to the threshold.
pub fn bore_penalty(burst_score: u64) -> u64 {
    if burst_score >= BURST_PENALTY_THRESHOLD_NS {
        // Penalty path
        let excess = burst_score.saturating_sub(BURST_PENALTY_THRESHOLD_NS);
        // Scale: excess * BURST_PENALTY_SCALE >> BURST_SCORE_SHIFT
        let raw = excess
            .saturating_mul(BURST_PENALTY_SCALE)
            >> BURST_SCORE_SHIFT;
        raw.min(MAX_BURST_PENALTY_NS)
    } else {
        // Boost path: reward tasks well below the threshold
        let deficit = BURST_PENALTY_THRESHOLD_NS.saturating_sub(burst_score);
        let raw = deficit
            .saturating_mul(NICE_LATENCY_WEIGHT)
            >> BURST_SCORE_SHIFT;
        raw.min(MAX_BURST_BOOST_NS)
    }
}

// ──────────────────────────────────────────────────────────
// Task tick
// ──────────────────────────────────────────────────────────

/// Called every scheduler tick while `task` is running.
///
/// * Accumulates burst time.
/// * Updates vruntime using weight-scaled delta: Δvrt = Δns * NICE_0_WEIGHT / weight.
/// * Resets burst_time at the start of each new scheduling window (not done
///   here — the runqueue is responsible for window boundaries).
pub fn bore_task_tick(task: &mut BoreTask, delta_ns: u64) {
    // Accumulate raw burst time
    task.burst_time = task.burst_time.saturating_add(delta_ns);

    // Update burst score (EMA)
    task.burst_score = bore_update_burst_score(task.burst_score, delta_ns);

    // Weight-scaled vruntime advance
    let delta_vrt = if task.weight > 0 {
        delta_ns.saturating_mul(NICE_0_WEIGHT) / task.weight
    } else {
        delta_ns
    };
    task.vruntime = task.vruntime.saturating_add(delta_vrt);
}

// ──────────────────────────────────────────────────────────
// BoreRunQueue
// ──────────────────────────────────────────────────────────

/// Run-queue for the BORE scheduler.
///
/// Tasks are kept in a `Vec` sorted by `effective_vruntime` (ascending).
/// The task at index 0 always has the lowest effective_vruntime and will be
/// scheduled next.
pub struct BoreRunQueue {
    /// Sorted list of ready tasks (lowest effective_vruntime first).
    tasks: Vec<BoreTask>,
    /// Monotonically increasing minimum vruntime of the queue.
    min_vruntime: u64,
}

impl BoreRunQueue {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            min_vruntime: 0,
        }
    }

    /// Sort the queue by effective vruntime (ascending).
    fn sort(&mut self) {
        self.tasks
            .sort_by_key(|t| t.effective_vruntime());
    }

    /// Number of runnable tasks.
    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    pub fn min_vruntime(&self) -> u64 {
        self.min_vruntime
    }
}

/// Enqueue a task into the BORE run-queue.
///
/// Sets the task vruntime to at least `min_vruntime` so newly woken tasks
/// don't receive an unfair CPU-time windfall.
pub fn bore_task_enqueue(rq: &mut BoreRunQueue, mut task: BoreTask) {
    // Normalise vruntime against the queue's minimum
    if task.vruntime < rq.min_vruntime {
        task.vruntime = rq.min_vruntime;
    }
    task.state = BoreTaskState::Ready;
    rq.tasks.push(task);
    rq.sort();
}

/// Dequeue a task by PID. Returns `None` if not found.
pub fn bore_task_dequeue(rq: &mut BoreRunQueue, pid: u32) -> Option<BoreTask> {
    if let Some(idx) = rq.tasks.iter().position(|t| t.pid == pid) {
        let task = rq.tasks.remove(idx);
        // Update min_vruntime if the removed task was the minimum
        if let Some(front) = rq.tasks.first() {
            rq.min_vruntime = rq.min_vruntime.max(front.vruntime);
        }
        Some(task)
    } else {
        None
    }
}

/// Pick the next task to run: the one with the lowest `effective_vruntime`.
///
/// Returns `None` if the queue is empty.
/// The task is **not** removed from the queue here; that is the responsibility
/// of the caller (e.g., call `bore_task_dequeue` after deciding to run it).
pub fn bore_pick_next_task(rq: &BoreRunQueue) -> Option<&BoreTask> {
    rq.tasks.first()
}

/// Advance the scheduler clock by `delta_ns` nanoseconds.
///
/// Updates `min_vruntime` monotonically based on the front of the queue.
pub fn bore_advance_clock(rq: &mut BoreRunQueue, delta_ns: u64) {
    rq.min_vruntime = rq.min_vruntime.saturating_add(delta_ns);
    // Also clamp to the actual minimum in the queue
    if let Some(front) = rq.tasks.first() {
        rq.min_vruntime = rq.min_vruntime.min(front.effective_vruntime());
    }
}

// ──────────────────────────────────────────────────────────
// Unit tests
// ──────────────────────────────────────────────────────────

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    // Helper: build a task with a pre-set burst_score
    fn task_with_burst(pid: u32, burst_score: u64) -> BoreTask {
        let mut t = BoreTask::new(pid, 0);
        t.burst_score = burst_score;
        t
    }

    // ── bore_penalty ────────────────────────────────────────

    #[test]
    fn test_penalty_zero_for_zero_burst() {
        // A brand-new task has burst_score=0, well below threshold → boost
        let p = bore_penalty(0);
        // Boost, not penalty; result is non-zero (represents the boost magnitude)
        assert!(p > 0, "Expected a boost magnitude for zero burst score");
    }

    #[test]
    fn test_penalty_increases_with_burst() {
        let p_low  = bore_penalty(BURST_PENALTY_THRESHOLD_NS + 1_000_000);
        let p_high = bore_penalty(BURST_PENALTY_THRESHOLD_NS + 50_000_000);
        assert!(p_high > p_low, "Higher burst should incur larger penalty");
    }

    #[test]
    fn test_penalty_capped_at_max() {
        let p = bore_penalty(u64::MAX / 2);
        assert!(p <= MAX_BURST_PENALTY_NS,
            "Penalty must be capped at MAX_BURST_PENALTY_NS");
    }

    #[test]
    fn test_boost_capped_at_max() {
        let b = bore_penalty(0);
        assert!(b <= MAX_BURST_BOOST_NS,
            "Boost must be capped at MAX_BURST_BOOST_NS");
    }

    // ── effective_vruntime ordering ──────────────────────────

    #[test]
    fn test_interactive_task_runs_before_bursty() {
        // Interactive task: burst_score below threshold
        let interactive = task_with_burst(1, 0);
        // Bursty task: burst_score well above threshold
        let bursty = task_with_burst(2, BURST_PENALTY_THRESHOLD_NS * 5);

        // Give both identical raw vruntime so only burst_score matters
        assert!(
            interactive.effective_vruntime() < bursty.effective_vruntime(),
            "Interactive task must have lower effective_vruntime than bursty task"
        );
    }

    #[test]
    fn test_bursty_task_penalised() {
        let bursty = task_with_burst(1, BURST_PENALTY_THRESHOLD_NS * 10);
        // effective_vruntime should be greater than raw vruntime
        assert!(
            bursty.effective_vruntime() > bursty.vruntime,
            "Bursty task should have effective_vruntime > raw vruntime"
        );
    }

    // ── bore_update_burst_score (EMA) ────────────────────────

    #[test]
    fn test_burst_score_ema_decay() {
        let score = 8_000_000_u64; // 8 ms
        // If burst_time is 0, score should decay
        let new_score = bore_update_burst_score(score, 0);
        assert!(new_score < score, "Burst score should decay when burst_time=0");
    }

    #[test]
    fn test_burst_score_ema_increases_with_burst() {
        let score = 0_u64;
        let new_score = bore_update_burst_score(score, 16_000_000); // 16 ms burst
        assert!(new_score > score, "Burst score should grow when burst is added");
    }

    // ── bore_task_tick ───────────────────────────────────────

    #[test]
    fn test_task_tick_advances_vruntime() {
        let mut task = BoreTask::new(1, 0);
        let vrt_before = task.vruntime;
        bore_task_tick(&mut task, 1_000_000); // 1 ms tick
        assert!(task.vruntime > vrt_before, "vruntime must increase after tick");
    }

    #[test]
    fn test_task_tick_accumulates_burst() {
        let mut task = BoreTask::new(1, 0);
        bore_task_tick(&mut task, 5_000_000); // 5 ms
        assert!(task.burst_time > 0, "burst_time must accumulate");
        assert!(task.burst_score > 0, "burst_score must grow");
    }

    // ── BoreRunQueue operations ──────────────────────────────

    #[test]
    fn test_enqueue_dequeue_round_trip() {
        let mut rq = BoreRunQueue::new();
        bore_task_enqueue(&mut rq, BoreTask::new(42, 0));
        assert_eq!(rq.len(), 1);
        let t = bore_task_dequeue(&mut rq, 42).expect("Should find pid 42");
        assert_eq!(t.pid, 42);
        assert!(rq.is_empty());
    }

    #[test]
    fn test_dequeue_missing_pid_returns_none() {
        let mut rq = BoreRunQueue::new();
        bore_task_enqueue(&mut rq, BoreTask::new(1, 0));
        assert!(bore_task_dequeue(&mut rq, 99).is_none());
    }

    #[test]
    fn test_pick_next_returns_interactive_first() {
        let mut rq = BoreRunQueue::new();

        // Bursty task enqueued first
        let bursty = task_with_burst(2, BURST_PENALTY_THRESHOLD_NS * 8);
        bore_task_enqueue(&mut rq, bursty);

        // Interactive task enqueued second
        bore_task_enqueue(&mut rq, BoreTask::new(1, 0)); // burst_score = 0

        let next = bore_pick_next_task(&rq).expect("Queue not empty");
        assert_eq!(next.pid, 1, "Interactive task should be picked first");
    }

    #[test]
    fn test_empty_queue_pick_returns_none() {
        let rq = BoreRunQueue::new();
        assert!(bore_pick_next_task(&rq).is_none());
    }

    #[test]
    fn test_nice_weight_ordering() {
        // Higher priority (lower nice) should have higher weight
        assert!(bore_nice_to_weight(-20) > bore_nice_to_weight(0));
        assert!(bore_nice_to_weight(0)  > bore_nice_to_weight(19));
    }

    #[test]
    fn test_vruntime_normalised_on_enqueue() {
        let mut rq = BoreRunQueue::new();
        rq.min_vruntime = 1_000_000;

        let mut late = BoreTask::new(1, 0);
        late.vruntime = 0; // below min_vruntime
        bore_task_enqueue(&mut rq, late);

        // vruntime should have been bumped to at least min_vruntime
        assert!(
            rq.tasks[0].vruntime >= 1_000_000,
            "vruntime must be normalised to min_vruntime on enqueue"
        );
    }
}
