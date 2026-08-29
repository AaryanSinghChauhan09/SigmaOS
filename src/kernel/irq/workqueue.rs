#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

extern crate alloc;
/// SigmaOS Workqueue async deferred execution engine
/// Runs deferred kernel tasks in thread context (Linux kernel workqueue.c & BSD taskqueue parity)
#[cfg(test)]
use crate::klib::VecDeque;

#[cfg(not(test))]
use crate::klib::VecDeque;

use alloc::string::String;
use alloc::string::ToString;

#[derive(Clone)]
pub struct Work {
    pub name: String,
    pub func: fn(),
}

#[derive(Clone)]
pub struct DelayedWork {
    pub work: Work,
    pub remaining_ticks: u64,
}

pub struct Workqueue {
    pub name: String,
    queue: VecDeque<Work>,
    delayed_queue: VecDeque<DelayedWork>,
    pub total_executed: usize,
    pub total_cancelled: usize,
}

impl Workqueue {
    pub fn new(name: &str) -> Self {
        Workqueue {
            name: name.to_string(),
            queue: VecDeque::new(),
            delayed_queue: VecDeque::new(),
            total_executed: 0,
            total_cancelled: 0,
        }
    }

    /// Enqueues immediate work item
    pub fn queue_work(&mut self, work: Work) {
        self.queue.push_back(work);
    }

    /// Enqueues delayed work item to be executed after timer delay ticks
    pub fn queue_delayed_work(&mut self, work: Work, delay_ticks: u64) {
        if delay_ticks == 0 {
            self.queue.push_back(work);
        } else {
            self.delayed_queue.push_back(DelayedWork {
                work,
                remaining_ticks: delay_ticks,
            });
        }
    }

    /// Cancels queued or delayed work by name. Returns true if cancelled.
    pub fn cancel_work(&mut self, name: &str) -> bool {
        let mut cancelled = false;
        let q_len = self.queue.len();
        for _ in 0..q_len {
            if let Some(work) = self.queue.pop_front() {
                if work.name == name {
                    cancelled = true;
                    self.total_cancelled += 1;
                } else {
                    self.queue.push_back(work);
                }
            }
        }
        let dq_len = self.delayed_queue.len();
        for _ in 0..dq_len {
            if let Some(delayed) = self.delayed_queue.pop_front() {
                if delayed.work.name == name {
                    cancelled = true;
                    self.total_cancelled += 1;
                } else {
                    self.delayed_queue.push_back(delayed);
                }
            }
        }
        cancelled
    }

    /// Advances the workqueue timer by elapsed ticks, promoting ready delayed work items into immediate queue.
    pub fn tick(&mut self, elapsed_ticks: u64) -> usize {
        let len = self.delayed_queue.len();
        for _ in 0..len {
            if let Some(mut delayed) = self.delayed_queue.pop_front() {
                if delayed.remaining_ticks <= elapsed_ticks {
                    self.queue.push_back(delayed.work);
                } else {
                    delayed.remaining_ticks -= elapsed_ticks;
                    self.delayed_queue.push_back(delayed);
                }
            }
        }
        self.process_work()
    }

    /// Processes all currently ready work items in thread context.
    pub fn process_work(&mut self) -> usize {
        let mut count = 0;
        while let Some(work) = self.queue.pop_front() {
            (work.func)();
            count += 1;
            self.total_executed += 1;
        }
        count
    }

    /// Flushes the workqueue by force-promoting all delayed work items and processing everything.
    pub fn flush(&mut self) -> usize {
        while let Some(delayed) = self.delayed_queue.pop_front() {
            self.queue.push_back(delayed.work);
        }
        self.process_work()
    }

    pub fn pending_count(&self) -> usize {
        self.queue.len() + self.delayed_queue.len()
    }
}

// ==========================================
// Unit Tests Module
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;
    use core::sync::atomic::{AtomicUsize, Ordering};

    static COUNTER: AtomicUsize = AtomicUsize::new(0);

    fn mock_task() {
        COUNTER.fetch_add(1, Ordering::SeqCst);
    }

    #[test]
    fn test_workqueue_delayed_and_cancellation() {
        let mut wq = Workqueue::new("kworker/0");
        COUNTER.store(0, Ordering::SeqCst);

        wq.queue_work(Work {
            name: "immediate_job".to_string(),
            func: mock_task,
        });

        wq.queue_delayed_work(
            Work {
                name: "delayed_job".to_string(),
                func: mock_task,
            },
            50,
        );

        wq.queue_delayed_work(
            Work {
                name: "cancel_job".to_string(),
                func: mock_task,
            },
            100,
        );

        // Immediate work processing
        assert_eq!(wq.process_work(), 1);
        assert_eq!(COUNTER.load(Ordering::SeqCst), 1);

        // Cancel the 100-tick delayed job
        assert!(wq.cancel_work("cancel_job"));
        assert_eq!(wq.total_cancelled, 1);

        // Tick 20 ticks - delayed_job has 50 remaining, so not ready
        assert_eq!(wq.tick(20), 0);
        assert_eq!(COUNTER.load(Ordering::SeqCst), 1);

        // Tick 35 ticks - total 55 ticks, so delayed_job should run
        assert_eq!(wq.tick(35), 1);
        assert_eq!(COUNTER.load(Ordering::SeqCst), 2);

        // Flush remaining
        assert_eq!(wq.flush(), 0);
        assert_eq!(wq.total_executed, 2);
    }
}
