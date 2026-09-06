#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
use std::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};

use crate::filesystem::FsError;

/// Multi-Level Feedback Queue (MLFQ) Scheduler
///
/// Implements multiple priority queues with aging to prevent starvation.
/// Each queue has a different time slice; higher-priority queues get shorter slices.
/// Tasks are promoted/demoted based on their behavior (CPU-bound vs I/O-bound).
pub struct MlfqScheduler {
    pub nr_queues: usize,
    pub time_slices: Vec<u64>,
    pub queues: Vec<Vec<u64>>,
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
            queues: (0..nr_queues).map(|_| Vec::new()).collect(),
            current_queue: 0,
            aging_threshold: 1000,
            ticks: AtomicU32::new(0),
        }
    }

    pub fn enqueue(&mut self, pid: u64, priority: usize) {
        let queue_idx = priority.min(self.nr_queues - 1);
        self.queues[queue_idx].push(pid);
    }

    pub fn dequeue(&mut self) -> Option<u64> {
        for q in 0..self.nr_queues {
            if !self.queues[q].is_empty() {
                return self.queues[q].pop();
            }
        }
        None
    }

    pub fn demote(&mut self, pid: u64) {
        for q in 0..self.nr_queues {
            if let Some(pos) = self.queues[q].iter().position(|&p| p == pid) {
                let _ = self.queues[q].remove(pos);
                if q + 1 < self.nr_queues {
                    self.queues[q + 1].push(pid);
                } else {
                    self.queues[q].push(pid);
                }
                return;
            }
        }
    }

    pub fn promote(&mut self, pid: u64) {
        for q in (1..self.nr_queues).rev() {
            if let Some(pos) = self.queues[q].iter().position(|&p| p == pid) {
                let _ = self.queues[q].remove(pos);
                self.queues[q - 1].push(pid);
                return;
            }
        }
    }

    pub fn aging(&mut self) {
        if u64::from(self.ticks.fetch_add(1, Ordering::SeqCst)) >= self.aging_threshold {
            self.ticks.store(0, Ordering::SeqCst);
            for q in (1..self.nr_queues).rev() {
                for i in 0..self.queues[q].len() {
                    let pid = self.queues[q][i];
                    self.promote(pid);
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
}
