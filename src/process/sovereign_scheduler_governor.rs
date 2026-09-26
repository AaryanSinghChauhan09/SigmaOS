// SPDX-License-Identifier: MIT
// Sovereign Multi-Queue CPU Scheduler & Linux/BSD Hybrid Scheduler Governor
//
// Implements a multi-queue CPU scheduler engine inspired by Linux 6.6+ EEVDF, CachyOS BORE,
// FreeBSD ULE (UltraLight Execution), and OpenBSD SCHED_MP (per-CPU runqueues with lockless work-stealing).

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Task Scheduling Policy / Class inspired by Linux & BSD schedulers
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SchedulerClass {
    SchedDeadline = 0,   // Earliest Deadline First (EDF) - highest priority
    SchedFifo = 1,       // Real-time FIFO
    SchedRr = 2,         // Real-time Round Robin
    SchedEevdf = 3,      // Linux 6.6+ Earliest Eligible Virtual Deadline First
    SchedBore = 4,       // CachyOS Burst-Oriented Response Enhancer
    BsdUleInteractive = 5,// FreeBSD ULE Interactivity-Boosted Queue
    SchedIdle = 6,       // Low-priority background task
}

/// Schedulable Task Entity
#[derive(Debug, Clone)]
pub struct SchedulerTask {
    pub pid: u64,
    pub name: String,
    pub policy: SchedulerClass,
    pub priority: u8,            // 0..255 (lower = higher priority)
    pub virtual_runtime_ns: u64, // Virtual runtime for EEVDF
    pub virtual_deadline_ns: u64,// Virtual deadline for EEVDF
    pub burst_score: u8,         // Interactivity / burstiness score (0..100)
    pub assigned_cpu: usize,
}

/// Per-CPU Runqueue Manager
#[derive(Debug, Clone)]
pub struct CpuRunqueue {
    pub cpu_id: usize,
    pub tasks: Vec<SchedulerTask>,
    pub total_load_weight: u64,
}

impl CpuRunqueue {
    pub fn new(cpu_id: usize) -> Self {
        Self {
            cpu_id,
            tasks: Vec::new(),
            total_load_weight: 0,
        }
    }

    pub fn enqueue_task(&mut self, mut task: SchedulerTask) {
        task.assigned_cpu = self.cpu_id;
        self.total_load_weight += 256 - task.priority as u64;
        self.tasks.push(task);
    }

    pub fn pick_next_task(&mut self) -> Option<SchedulerTask> {
        if self.tasks.is_empty() {
            return None;
        }

        // Sort tasks by policy priority: Deadline > FIFO/RR > EEVDF/BORE/ULE > Idle
        let mut best_idx = 0;
        for i in 1..self.tasks.len() {
            let task = &self.tasks[i];
            let current_best = &self.tasks[best_idx];
            if (task.policy as u8) < (current_best.policy as u8) {
                best_idx = i;
            } else if task.policy == current_best.policy {
                match task.policy {
                    SchedulerClass::SchedEevdf | SchedulerClass::SchedBore | SchedulerClass::BsdUleInteractive => {
                        if task.virtual_deadline_ns < current_best.virtual_deadline_ns {
                            best_idx = i;
                        }
                    }
                    _ => {
                        if task.priority < current_best.priority {
                            best_idx = i;
                        }
                    }
                }
            }
        }

        let task = self.tasks.remove(best_idx);
        self.total_load_weight = self.total_load_weight.saturating_sub(256 - task.priority as u64);
        Some(task)
    }
}

/// Multi-Core Hybrid Scheduler Governor
#[derive(Debug, Clone)]
pub struct SovereignMultiQueueSchedulerGovernor {
    pub per_cpu_queues: Vec<CpuRunqueue>,
}

impl SovereignMultiQueueSchedulerGovernor {
    pub fn new(num_cpus: usize) -> Self {
        let mut queues = Vec::with_capacity(num_cpus);
        for i in 0..num_cpus {
            queues.push(CpuRunqueue::new(i));
        }
        Self { per_cpu_queues: queues }
    }

    pub fn submit_task(&mut self, task: SchedulerTask) {
        // Enqueue onto CPU with minimal load weight (load balancing)
        if let Some(rq) = self.per_cpu_queues.iter_mut().min_by_key(|q| q.total_load_weight) {
            rq.enqueue_task(task);
        }
    }

    /// Perform work-stealing from overloaded CPU runqueues
    pub fn work_steal_pass(&mut self) -> usize {
        let num_cpus = self.per_cpu_queues.len();
        if num_cpus < 2 {
            return 0;
        }

        let mut stolen_count = 0;
        let mut max_cpu = 0;
        let mut min_cpu = 0;

        for i in 1..num_cpus {
            if self.per_cpu_queues[i].total_load_weight > self.per_cpu_queues[max_cpu].total_load_weight {
                max_cpu = i;
            }
            if self.per_cpu_queues[i].total_load_weight < self.per_cpu_queues[min_cpu].total_load_weight {
                min_cpu = i;
            }
        }

        if max_cpu != min_cpu && self.per_cpu_queues[max_cpu].tasks.len() > 1 {
            if let Some(task) = self.per_cpu_queues[max_cpu].tasks.pop() {
                self.per_cpu_queues[max_cpu].total_load_weight = self.per_cpu_queues[max_cpu].total_load_weight.saturating_sub(256 - task.priority as u64);
                self.per_cpu_queues[min_cpu].enqueue_task(task);
                stolen_count += 1;
            }
        }

        stolen_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multi_queue_scheduler_governor() {
        let mut governor = SovereignMultiQueueSchedulerGovernor::new(1);

        governor.submit_task(SchedulerTask {
            pid: 100,
            name: "game_loop".to_string(),
            policy: SchedulerClass::SchedBore,
            priority: 20,
            virtual_runtime_ns: 1000,
            virtual_deadline_ns: 2000,
            burst_score: 95,
            assigned_cpu: 0,
        });

        governor.submit_task(SchedulerTask {
            pid: 101,
            name: "realtime_audio".to_string(),
            policy: SchedulerClass::SchedFifo,
            priority: 5,
            virtual_runtime_ns: 0,
            virtual_deadline_ns: 0,
            burst_score: 100,
            assigned_cpu: 0,
        });

        // FIFO Real-Time task should be picked before BORE task
        let next_task = governor.per_cpu_queues[0].pick_next_task().unwrap();
        assert_eq!(next_task.pid, 101);
        assert_eq!(next_task.policy, SchedulerClass::SchedFifo);
    }
}
