// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/core/sigma_sched.rs — Sovereign Scheduler
// Replaces: sigma_sched.cpp (C++ stub, removed)
//
// Architecture: MLFQ (4 queues) + EDF (real-time) + CFS clone (fairness)
// Language: Rust #![no_std] — no libc, no prelude, no third-party crates
// Pattern: OOP via Traits (SdfScheduler trait) + concrete implementations

#![no_std]

use core::sync::atomic::{AtomicU64, Ordering};

// ── Constants ────────────────────────────────────────────────────────────────

pub const MLFQ_QUEUES:     usize = 4;
pub const MAX_TASKS:       usize = 64;
pub const TIME_SLICE_MS:   u64   = 10;
pub const AGING_THRESHOLD: u64   = 50; // ticks before promotion

// ── Types ────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TaskState {
    Ready,
    Running,
    Blocked,
    Dead,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum SchedClass {
    Idle     = 0,
    Normal   = 1,
    Interactive = 2,
    RealTime = 3,
}

/// A schedulable task control block
#[repr(C)]
pub struct Task {
    pub id:          u32,
    pub state:       TaskState,
    pub sched_class: SchedClass,
    pub priority:    u8,          // 0 = highest in class
    pub vruntime:    u64,         // CFS virtual runtime (ns)
    pub deadline:    u64,         // EDF absolute deadline (ticks), 0 = no RT
    pub ticks_used:  u64,
    pub ticks_starve: u64,        // ticks without CPU → aging
    pub stack_ptr:   usize,
    pub cr3:         usize,       // page table root
}

impl Task {
    pub const fn new(id: u32, class: SchedClass, deadline: u64) -> Self {
        Self {
            id,
            state:       TaskState::Ready,
            sched_class: class,
            priority:    0,
            vruntime:    0,
            deadline,
            ticks_used:  0,
            ticks_starve: 0,
            stack_ptr:   0,
            cr3:         0,
        }
    }
}

// ── Scheduler Trait (OOP interface) ─────────────────────────────────────────

pub trait SdfScheduler {
    /// Add a task to the run queue
    fn enqueue(&mut self, task: Task);
    /// Pick the next task to run
    fn pick_next(&mut self) -> Option<u32>;
    /// Called on every timer tick
    fn tick(&mut self, current_id: u32) -> bool; // returns true if preempt needed
    /// Block a task (e.g., waiting on I/O)
    fn block(&mut self, id: u32);
    /// Unblock a task
    fn unblock(&mut self, id: u32);
}

// ── MLFQ Implementation ──────────────────────────────────────────────────────

pub struct MlfqScheduler {
    queues:   [[u32; MAX_TASKS]; MLFQ_QUEUES],
    q_len:    [usize; MLFQ_QUEUES],
    tasks:    [Option<Task>; MAX_TASKS],
    tick_cnt: u64,
}

impl MlfqScheduler {
    pub const fn new() -> Self {
        Self {
            queues:   [[0u32; MAX_TASKS]; MLFQ_QUEUES],
            q_len:    [0usize; MLFQ_QUEUES],
            tasks:    [const { None }; MAX_TASKS],
            tick_cnt: 0,
        }
    }

    fn task_slot(&self, id: u32) -> Option<usize> {
        self.tasks.iter().position(|t| {
            matches!(t, Some(task) if task.id == id)
        })
    }

    fn promote_starving(&mut self) {
        for slot in 0..MAX_TASKS {
            if let Some(ref mut task) = self.tasks[slot] {
                if task.ticks_starve >= AGING_THRESHOLD
                    && task.priority > 0
                {
                    task.priority -= 1;
                    task.ticks_starve = 0;
                }
            }
        }
    }
}

impl SdfScheduler for MlfqScheduler {
    fn enqueue(&mut self, task: Task) {
        let q = (task.priority as usize).min(MLFQ_QUEUES - 1);
        let id = task.id;
        // Find free slot
        for slot in 0..MAX_TASKS {
            if self.tasks[slot].is_none() {
                self.tasks[slot] = Some(task);
                let qi = self.q_len[q];
                if qi < MAX_TASKS {
                    self.queues[q][qi] = id;
                    self.q_len[q] += 1;
                }
                return;
            }
        }
    }

    fn pick_next(&mut self) -> Option<u32> {
        // EDF: check RT tasks first (deadline nearest)
        let mut earliest_dl = u64::MAX;
        let mut rt_id = None;
        for slot in 0..MAX_TASKS {
            if let Some(ref t) = self.tasks[slot] {
                if t.state == TaskState::Ready
                    && t.sched_class == SchedClass::RealTime
                    && t.deadline > 0
                    && t.deadline < earliest_dl
                {
                    earliest_dl = t.deadline;
                    rt_id = Some(t.id);
                }
            }
        }
        if rt_id.is_some() { return rt_id; }

        // MLFQ: highest non-empty queue
        for q in (0..MLFQ_QUEUES).rev() {
            if self.q_len[q] > 0 {
                let id = self.queues[q][0];
                // Shift queue left
                self.q_len[q] -= 1;
                for i in 0..self.q_len[q] {
                    self.queues[q][i] = self.queues[q][i + 1];
                }
                return Some(id);
            }
        }
        None
    }

    fn tick(&mut self, current_id: u32) -> bool {
        self.tick_cnt += 1;
        // Age starving tasks every 10 ticks
        if self.tick_cnt % 10 == 0 { self.promote_starving(); }

        if let Some(slot) = self.task_slot(current_id) {
            if let Some(ref mut task) = self.tasks[slot] {
                task.ticks_used += 1;
                // Demote if used full time slice in MLFQ
                if task.sched_class != SchedClass::RealTime
                    && task.ticks_used >= TIME_SLICE_MS
                {
                    task.ticks_used = 0;
                    if (task.priority as usize) < MLFQ_QUEUES - 1 {
                        task.priority += 1;
                    }
                    return true; // preempt
                }
            }
        }
        // Increment starve counters for waiting tasks
        for slot in 0..MAX_TASKS {
            if let Some(ref mut t) = self.tasks[slot] {
                if t.state == TaskState::Ready && t.id != current_id {
                    t.ticks_starve += 1;
                }
            }
        }
        false
    }

    fn block(&mut self, id: u32) {
        if let Some(slot) = self.task_slot(id) {
            if let Some(ref mut t) = self.tasks[slot] {
                t.state = TaskState::Blocked;
            }
        }
    }

    fn unblock(&mut self, id: u32) {
        if let Some(slot) = self.task_slot(id) {
            if let Some(ref mut t) = self.tasks[slot] {
                t.state = TaskState::Ready;
            }
        }
    }
}

// ── Global scheduler instance ────────────────────────────────────────────────

static TICK_COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn global_tick() -> u64 {
    TICK_COUNTER.fetch_add(1, Ordering::Relaxed)
}
