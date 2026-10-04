use core::sync::atomic::{AtomicU32, Ordering};
use std::collections::VecDeque;
use std::vec::Vec;

use crate::filesystem::FsError;
use crate::kernel::sched::scheduler::{RunQueue, SchedClass};
use crate::kernel::sched::Task;

/// Multi-Level Feedback Queue (MLFQ) Scheduler
///
/// Implements multiple priority queues with aging to prevent starvation.
/// Each queue has a different time slice; higher-priority queues get shorter slices.
/// Tasks are promoted/demoted based on their behavior (CPU-bound vs I/O-bound).
pub struct MlfqScheduler {
    pub nr_queues: usize,
    pub time_slices: Vec<u64>,
    /// FIFO run queues (VecDeque: O(1) pop_front). The old Vec + pop()
    /// combination ran tasks in LIFO order, starving the oldest tasks.
    pub queues: Vec<VecDeque<u64>>,
    pub current_queue: usize,
    pub aging_threshold: u64,
    pub ticks: AtomicU32,
}

impl MlfqScheduler {
    pub fn new(nr_queues: usize) -> Self {
        let mut time_slices = Vec::new();
        for i in 0..nr_queues {
            time_slices.push(1 << (nr_queues - i));
        }
        MlfqScheduler {
            nr_queues,
            time_slices,
            queues: (0..nr_queues).map(|_| VecDeque::new()).collect(),
            current_queue: 0,
            aging_threshold: 1000,
            ticks: AtomicU32::new(0),
        }
    }

    pub fn enqueue(&mut self, pid: u64, priority: usize) {
        let queue_idx = priority.min(self.nr_queues - 1);
        self.queues[queue_idx].push_back(pid);
    }

    /// Dequeue the highest-priority OLDEST task (FIFO within a level).
    pub fn dequeue(&mut self) -> Option<u64> {
        for q in 0..self.nr_queues {
            if let Some(pid) = self.queues[q].pop_front() {
                return Some(pid);
            }
        }
        None
    }

    pub fn demote(&mut self, pid: u64) {
        for q in 0..self.nr_queues {
            if let Some(pos) = self.queues[q].iter().position(|&p| p == pid) {
                self.queues[q].remove(pos);
                if q + 1 < self.nr_queues {
                    self.queues[q + 1].push_back(pid);
                } else {
                    self.queues[q].push_back(pid);
                }
                return;
            }
        }
    }

    pub fn promote(&mut self, pid: u64) {
        for q in (1..self.nr_queues).rev() {
            if let Some(pos) = self.queues[q].iter().position(|&p| p == pid) {
                self.queues[q].remove(pos);
                self.queues[q - 1].push_back(pid);
                return;
            }
        }
    }

    /// Anti-starvation aging: every WAITING task below the top level is
    /// promoted exactly one level per aging epoch.
    ///
    /// The old implementation indexed into `self.queues[q]` while `promote()`
    /// removed items from that same queue, shifting indices under the loop —
    /// skipping tasks and eventually panicking with an out-of-bounds index
    /// once enough tasks were promoted. It also double-promoted tasks because
    /// it processed queues top-down (each drain fed the next queue to be
    /// drained). Draining bottom-up in ascending order fixes both: each queue
    /// is fully evacuated before it can receive promoted tasks.
    pub fn aging(&mut self) {
        if u64::from(self.ticks.fetch_add(1, Ordering::SeqCst)) >= self.aging_threshold {
            self.ticks.store(0, Ordering::SeqCst);
            for q in 1..self.nr_queues {
                while let Some(pid) = self.queues[q].pop_front() {
                    self.queues[q - 1].push_back(pid);
                }
            }
        }
    }

    pub fn nr_running(&self) -> u32 {
        self.queues.iter().map(|q| q.len() as u32).sum()
    }
}

pub struct MlfqSchedClass {
    pub mlfq: MlfqScheduler,
}

impl MlfqSchedClass {
    pub fn new(nr_queues: usize) -> Self {
        MlfqSchedClass {
            mlfq: MlfqScheduler::new(nr_queues),
        }
    }
}

impl SchedClass for MlfqSchedClass {
    fn enqueue_task(&self, rq: &mut RunQueue, _task: &mut Task) -> Result<(), FsError> {
        rq.nr_running.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    fn dequeue_task(&self, rq: &mut RunQueue, _task: &mut Task) -> Result<(), FsError> {
        rq.nr_running.fetch_sub(1, Ordering::SeqCst);
        Ok(())
    }

    fn yield_task(&self, _rq: &mut RunQueue, _task: &mut Task) -> Result<(), FsError> {
        Ok(())
    }

    fn check_preempt_curr(&self, _rq: &mut RunQueue, _task: &Task) -> bool {
        false
    }

    fn pick_next_task(&self, _rq: &mut RunQueue) -> Option<u64> {
        None
    }

    fn put_prev_task(&self, _rq: &mut RunQueue, _task: &mut Task) {}

    fn set_curr_task(&self, _rq: &mut RunQueue, _task: &mut Task) {}

    fn task_tick(&self, _rq: &mut RunQueue, _task: &mut Task) -> Result<(), FsError> {
        Ok(())
    }

    fn task_fork(
        &self,
        _rq: &mut RunQueue,
        _child: &mut Task,
        _parent: &Task,
    ) -> Result<(), FsError> {
        Ok(())
    }

    fn task_dead(&self, _rq: &mut RunQueue, _task: &mut Task) {}

    fn prio_changed(&self, _rq: &mut RunQueue, _task: &mut Task) {}
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_mlfq_enqueue_dequeue() {
        let mut mlfq = MlfqScheduler::new(3);
        mlfq.enqueue(100, 0);
        mlfq.enqueue(200, 1);
        assert_eq!(mlfq.dequeue(), Some(100));
        assert_eq!(mlfq.dequeue(), Some(200));
    }

    #[test]
    fn test_mlfq_demote() {
        let mut mlfq = MlfqScheduler::new(3);
        mlfq.enqueue(100, 0);
        mlfq.demote(100);
        assert!(mlfq.queues[1].contains(&100));
    }

    #[test]
    fn test_mlfq_promote() {
        let mut mlfq = MlfqScheduler::new(3);
        mlfq.enqueue(100, 2);
        mlfq.promote(100);
        assert!(mlfq.queues[1].contains(&100));
    }

    #[test]
    fn test_mlfq_nr_running() {
        let mut mlfq = MlfqScheduler::new(2);
        assert_eq!(mlfq.nr_running(), 0);
        mlfq.enqueue(100, 0);
        mlfq.enqueue(200, 1);
        assert_eq!(mlfq.nr_running(), 2);
    }

    /// Regression: within a single level, the OLDEST task must run first.
    /// The old Vec::pop() dequeue ran LIFO and starved the oldest tasks.
    #[test]
    fn test_mlfq_fifo_within_level() {
        let mut mlfq = MlfqScheduler::new(3);
        mlfq.enqueue(100, 0);
        mlfq.enqueue(200, 0);
        mlfq.enqueue(300, 0);
        assert_eq!(mlfq.dequeue(), Some(100));
        assert_eq!(mlfq.dequeue(), Some(200));
        assert_eq!(mlfq.dequeue(), Some(300));
    }

    /// Regression: aging must not panic (the old code mutated the queue it
    /// was indexing into) and must promote every waiting task exactly one
    /// level per epoch — no double promotion, no task left behind.
    #[test]
    fn test_mlfq_aging_promotes_exactly_one_level() {
        let mut mlfq = MlfqScheduler::new(4);
        mlfq.aging_threshold = 1;
        mlfq.enqueue(10, 3);
        mlfq.enqueue(20, 3);
        mlfq.enqueue(30, 2);
        mlfq.enqueue(40, 1);

        mlfq.aging(); // tick 0 -> 1 (previous value 0 < threshold: no-op)
        mlfq.aging(); // previous value 1 >= 1: threshold trips

        // Every task moved up exactly one level.
        assert!(mlfq.queues[0].contains(&40), "level-1 task -> level 0");
        assert!(mlfq.queues[1].contains(&30), "level-2 task -> level 1");
        assert!(
            mlfq.queues[2].contains(&10) && mlfq.queues[2].contains(&20),
            "level-3 tasks -> level 2 exactly once"
        );
        assert!(mlfq.queues[3].is_empty());
        assert_eq!(mlfq.nr_running(), 4, "no task may be lost during aging");
    }
}
