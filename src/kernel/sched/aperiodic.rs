//! SigmaOS Aperiodic & Sporadic Task Scheduling Subsystem
//! Inspired by Linux PREEMPT_RT / SCHED_DEADLINE Constant Bandwidth Server (CBS),
//! Rate-Monotonic Deferrable Server (DS), Sporadic Server (SS), and FreeBSD Taskqueues.

#![no_std]

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AperiodicPriority {
    Emergency = 0,
    Realtime = 1,
    High = 2,
    Normal = 3,
    Background = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AperiodicServerKind {
    /// Deferrable Server (DS): Fixed budget replenished periodically, budget retained if idle
    DeferrableServer {
        budget_ms: u64,
        period_ms: u64,
        current_budget_ms: u64,
        last_replenished_ms: u64,
    },
    /// Sporadic Server (SS): Capacity replenished after consumption to preserve rate-monotonic bounds
    SporadicServer {
        capacity_ms: u64,
        period_ms: u64,
        active_capacity_ms: u64,
        last_replenished_ms: u64,
    },
    /// Constant Bandwidth Server (CBS): Linux SCHED_DEADLINE style virtual deadline assignment d_k = max(t, d_{k-1}) + Q/U
    ConstantBandwidthServer {
        max_budget_ms: u64,
        period_ms: u64,
        current_budget_ms: u64,
        virtual_deadline_ms: u64,
    },
}

#[derive(Debug, Clone)]
pub struct AperiodicTask {
    pub id: u64,
    pub name: String,
    pub priority: AperiodicPriority,
    pub arrival_time_ms: u64,
    pub required_exec_ms: u64,
    pub remaining_exec_ms: u64,
    pub absolute_deadline_ms: u64,
    pub server_kind: AperiodicServerKind,
    pub is_completed: bool,
}

impl AperiodicTask {
    pub fn new(
        id: u64,
        name: &str,
        priority: AperiodicPriority,
        arrival_time_ms: u64,
        required_exec_ms: u64,
        relative_deadline_ms: u64,
        server_kind: AperiodicServerKind,
    ) -> Self {
        let absolute_deadline_ms = arrival_time_ms + relative_deadline_ms;
        Self {
            id,
            name: name.to_string(),
            priority,
            arrival_time_ms,
            required_exec_ms,
            remaining_exec_ms: required_exec_ms,
            absolute_deadline_ms,
            server_kind,
            is_completed: false,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SchedulerMetrics {
    pub total_tasks_processed: u64,
    pub deadlines_met: u64,
    pub deadlines_missed: u64,
    pub total_exec_time_ms: u64,
}

pub struct AperiodicScheduler {
    pub queue: Vec<AperiodicTask>,
    pub current_time_ms: u64,
    pub max_bandwidth_utilization: f32, // e.g. 0.30 = 30% reserved for aperiodic/sporadic tasks
    pub metrics: SchedulerMetrics,
}

impl AperiodicScheduler {
    pub fn new(max_bandwidth_utilization: f32) -> Self {
        Self {
            queue: Vec::new(),
            current_time_ms: 0,
            max_bandwidth_utilization,
            metrics: SchedulerMetrics::default(),
        }
    }

    pub fn enqueue_task(&mut self, mut task: AperiodicTask) -> Result<(), &'static str> {
        // Enforce Constant Bandwidth Server (CBS) virtual deadline calculation if CBS
        if let AperiodicServerKind::ConstantBandwidthServer {
            max_budget_ms,
            period_ms,
            ref mut virtual_deadline_ms,
            ..
        } = task.server_kind
        {
            let cbs_deadline = self.current_time_ms + period_ms;
            if cbs_deadline > *virtual_deadline_ms {
                *virtual_deadline_ms = cbs_deadline;
            }
            task.absolute_deadline_ms = *virtual_deadline_ms;
        }

        self.queue.push(task);
        self.sort_queue();
        Ok(())
    }

    pub fn tick(&mut self, delta_ms: u64) {
        self.current_time_ms += delta_ms;

        // 1. Replenish Deferrable / Sporadic Server budgets based on periodic ticks
        for task in &mut self.queue {
            match task.server_kind {
                AperiodicServerKind::DeferrableServer {
                    budget_ms,
                    period_ms,
                    ref mut current_budget_ms,
                    ref mut last_replenished_ms,
                } => {
                    if self.current_time_ms >= *last_replenished_ms + period_ms {
                        *current_budget_ms = budget_ms;
                        *last_replenished_ms = self.current_time_ms;
                    }
                }
                AperiodicServerKind::SporadicServer {
                    capacity_ms,
                    period_ms,
                    ref mut active_capacity_ms,
                    ref mut last_replenished_ms,
                } => {
                    if self.current_time_ms >= *last_replenished_ms + period_ms {
                        *active_capacity_ms = capacity_ms;
                        *last_replenished_ms = self.current_time_ms;
                    }
                }
                AperiodicServerKind::ConstantBandwidthServer {
                    max_budget_ms,
                    period_ms,
                    ref mut current_budget_ms,
                    ref mut virtual_deadline_ms,
                } => {
                    if *current_budget_ms == 0 {
                        *current_budget_ms = max_budget_ms;
                        *virtual_deadline_ms = self.current_time_ms + period_ms;
                    }
                }
            }
        }

        // 2. Dispatch task with highest priority and earliest deadline
        if let Some(task_idx) = self.select_next_task_idx() {
            let task = &mut self.queue[task_idx];
            let exec = delta_ms.min(task.remaining_exec_ms);

            task.remaining_exec_ms -= exec;
            self.metrics.total_exec_time_ms += exec;

            // Deduct server budget
            match task.server_kind {
                AperiodicServerKind::DeferrableServer {
                    ref mut current_budget_ms,
                    ..
                } => {
                    *current_budget_ms = current_budget_ms.saturating_sub(exec);
                }
                AperiodicServerKind::SporadicServer {
                    ref mut active_capacity_ms,
                    ..
                } => {
                    *active_capacity_ms = active_capacity_ms.saturating_sub(exec);
                }
                AperiodicServerKind::ConstantBandwidthServer {
                    ref mut current_budget_ms,
                    ..
                } => {
                    *current_budget_ms = current_budget_ms.saturating_sub(exec);
                }
            }

            // Check completion or deadline miss
            if task.remaining_exec_ms == 0 {
                task.is_completed = true;
                self.metrics.total_tasks_processed += 1;
                if self.current_time_ms <= task.absolute_deadline_ms {
                    self.metrics.deadlines_met += 1;
                } else {
                    self.metrics.deadlines_missed += 1;
                }
            }
        }

        // Retain uncompleted tasks
        self.queue.retain(|t| !t.is_completed);
    }

    fn select_next_task_idx(&self) -> Option<usize> {
        let mut best_idx = None;
        let mut best_priority = AperiodicPriority::Background;
        let mut best_deadline = u64::MAX;

        for (idx, task) in self.queue.iter().enumerate() {
            if task.is_completed || task.arrival_time_ms > self.current_time_ms {
                continue;
            }

            // Verify budget availability for the task's server
            let has_budget = match task.server_kind {
                AperiodicServerKind::DeferrableServer {
                    current_budget_ms, ..
                } => current_budget_ms > 0,
                AperiodicServerKind::SporadicServer {
                    active_capacity_ms, ..
                } => active_capacity_ms > 0,
                AperiodicServerKind::ConstantBandwidthServer {
                    current_budget_ms, ..
                } => current_budget_ms > 0,
            };

            if !has_budget {
                continue;
            }

            if task.priority < best_priority
                || (task.priority == best_priority && task.absolute_deadline_ms < best_deadline)
            {
                best_idx = Some(idx);
                best_priority = task.priority;
                best_deadline = task.absolute_deadline_ms;
            }
        }

        best_idx
    }

    fn sort_queue(&mut self) {
        self.queue.sort_by(|a, b| {
            a.priority
                .cmp(&b.priority)
                .then_with(|| a.absolute_deadline_ms.cmp(&b.absolute_deadline_ms))
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cbs_constant_bandwidth_server() {
        let mut sched = AperiodicScheduler::new(0.30);
        let cbs_server = AperiodicServerKind::ConstantBandwidthServer {
            max_budget_ms: 10,
            period_ms: 50,
            current_budget_ms: 10,
            virtual_deadline_ms: 0,
        };

        let task = AperiodicTask::new(
            1,
            "irq_bottom_half",
            AperiodicPriority::Emergency,
            0,
            10,
            100,
            cbs_server,
        );

        assert!(sched.enqueue_task(task).is_ok());
        assert_eq!(sched.queue[0].absolute_deadline_ms, 50); // CBS virtual deadline assigned (0 + 50)

        sched.tick(10);
        assert_eq!(sched.metrics.total_tasks_processed, 1);
        assert_eq!(sched.metrics.deadlines_met, 1);
    }

    #[test]
    fn test_deferrable_server_replenishment() {
        let mut sched = AperiodicScheduler::new(0.20);
        let ds_server = AperiodicServerKind::DeferrableServer {
            budget_ms: 5,
            period_ms: 20,
            current_budget_ms: 5,
            last_replenished_ms: 0,
        };

        let task = AperiodicTask::new(
            2,
            "usb_aperiodic_packet",
            AperiodicPriority::High,
            0,
            5,
            30,
            ds_server,
        );

        assert!(sched.enqueue_task(task).is_ok());
        sched.tick(5); // Consumes all 5ms budget
        assert_eq!(sched.metrics.deadlines_met, 1);
    }
}
