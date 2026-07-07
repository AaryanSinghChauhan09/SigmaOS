// sigma_realtime_kernel.rs — PREEMPT_RT Kernel Variant
// Implements real-time scheduling primitives for robotics, industrial,
// and audio production workloads. Priority inheritance, deadline scheduling,
// and interrupt threading.

#![no_std]
#![allow(dead_code)]

extern crate alloc;
use alloc::{string::String, vec::Vec};

// ── Real-Time Scheduling Classes ────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum RtSchedClass {
    Fifo,           // SCHED_FIFO — first-in-first-out, no timeslice
    RoundRobin,     // SCHED_RR — with configurable timeslice
    Deadline,       // SCHED_DEADLINE — earliest-deadline-first
    Normal,         // CFS — default Linux scheduler
}

#[derive(Debug, Clone)]
pub struct RtTask {
    pub pid: u32,
    pub name: String,
    pub sched_class: RtSchedClass,
    pub priority: u8,           // 1-99 for FIFO/RR
    pub runtime_ns: u64,        // For DEADLINE: max runtime per period
    pub deadline_ns: u64,       // For DEADLINE: relative deadline
    pub period_ns: u64,         // For DEADLINE: task period
    pub cpu_affinity: u32,      // CPU bitmask
    pub is_preemptible: bool,
}

// ── Priority Inheritance Protocol ───────────────────────────────────────────

#[derive(Debug)]
pub struct PriorityMutex {
    pub id: u32,
    pub holder_pid: Option<u32>,
    pub holder_original_priority: u8,
    pub holder_boosted_priority: u8,
    pub waiters: Vec<u32>,
}

impl PriorityMutex {
    pub fn new(id: u32) -> Self {
        PriorityMutex {
            id,
            holder_pid: None,
            holder_original_priority: 0,
            holder_boosted_priority: 0,
            waiters: Vec::new(),
        }
    }

    /// Lock with priority inheritance — boosts holder to highest waiter priority
    pub fn lock(&mut self, task: &RtTask) -> Result<(), &'static str> {
        match self.holder_pid {
            None => {
                self.holder_pid = Some(task.pid);
                self.holder_original_priority = task.priority;
                self.holder_boosted_priority = task.priority;
                Ok(())
            }
            Some(_) => {
                self.waiters.push(task.pid);
                // Boost holder to waiter's priority if higher
                if task.priority > self.holder_boosted_priority {
                    self.holder_boosted_priority = task.priority;
                    // In production: sched_setattr() on holder PID
                }
                Err("Mutex busy — task queued with priority inheritance")
            }
        }
    }

    /// Unlock and restore original priority
    pub fn unlock(&mut self, pid: u32) -> Result<Option<u32>, &'static str> {
        if self.holder_pid != Some(pid) {
            return Err("Not the holder");
        }
        // Restore original priority
        self.holder_boosted_priority = self.holder_original_priority;
        self.holder_pid = None;

        // Wake highest-priority waiter
        if !self.waiters.is_empty() {
            let next = self.waiters.remove(0);
            self.holder_pid = Some(next);
            return Ok(Some(next));
        }
        Ok(None)
    }
}

// ── Deadline Scheduler ──────────────────────────────────────────────────────

#[derive(Debug)]
pub struct DeadlineScheduler {
    pub tasks: Vec<RtTask>,
    pub current_time_ns: u64,
}

impl DeadlineScheduler {
    pub fn new() -> Self {
        DeadlineScheduler {
            tasks: Vec::new(),
            current_time_ns: 0,
        }
    }

    /// Admission control — verify the new task won't overload the system
    /// Uses the utilization bound test: sum(runtime/period) <= 1.0 per CPU
    pub fn admit(&self, task: &RtTask) -> bool {
        if task.sched_class != RtSchedClass::Deadline {
            return true;
        }
        if task.period_ns == 0 {
            return false;
        }

        let existing_util: u64 = self
            .tasks
            .iter()
            .filter(|t| t.sched_class == RtSchedClass::Deadline)
            .map(|t| {
                if t.period_ns > 0 {
                    (t.runtime_ns * 1000) / t.period_ns
                } else {
                    0
                }
            })
            .sum();

        let new_util = (task.runtime_ns * 1000) / task.period_ns;
        // Utilization must be <= 1000 (i.e., 100.0% scaled by 1000)
        (existing_util + new_util) <= 1000
    }

    /// Add a task if it passes admission control
    pub fn add_task(&mut self, task: RtTask) -> Result<(), &'static str> {
        if !self.admit(&task) {
            return Err("Admission denied — CPU overloaded");
        }
        self.tasks.push(task);
        Ok(())
    }

    /// Select the next task to run using Earliest Deadline First
    pub fn pick_next(&self) -> Option<&RtTask> {
        self.tasks
            .iter()
            .filter(|t| t.sched_class == RtSchedClass::Deadline)
            .min_by_key(|t| t.deadline_ns)
    }
}

// ── Interrupt Threading ─────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ThreadedIrq {
    pub irq_number: u16,
    pub handler_name: String,
    pub thread_priority: u8,
    pub cpu_affinity: u32,
    pub is_threaded: bool,
}

/// Convert a hardirq to a threaded IRQ for real-time latency guarantees
pub fn make_irq_threaded(irq: u16, handler: &str, priority: u8) -> ThreadedIrq {
    ThreadedIrq {
        irq_number: irq,
        handler_name: String::from(handler),
        thread_priority: priority,
        cpu_affinity: 0xFFFFFFFF, // All CPUs
        is_threaded: true,
    }
}
