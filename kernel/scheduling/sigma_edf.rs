// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/scheduling/sigma_edf.rs — Earliest Deadline First (EDF) Scheduler
// Language: Rust #![no_std]
// Pattern: OOP via EdfScheduler struct implementing SdfScheduler trait

#![no_std]

pub const MAX_RT_TASKS: usize = 32;

#[derive(Clone, Copy)]
pub struct RtTask {
    pub id:       u32,
    pub deadline: u64,   // absolute deadline in ticks
    pub period:   u64,   // task period in ticks
    pub wcet:     u64,   // worst-case execution time in ticks
    pub budget:   u64,   // remaining budget this period
    pub active:   bool,
}

impl RtTask {
    pub const fn new(id: u32, deadline: u64, period: u64, wcet: u64) -> Self {
        Self { id, deadline, period, wcet, budget: wcet, active: true }
    }
    /// Utilisation: U = wcet / period. Sum must be ≤ 1.0 for schedulability.
    pub fn utilisation(&self) -> f64 {
        self.wcet as f64 / self.period.max(1) as f64
    }
}

pub struct EdfScheduler {
    tasks:    [Option<RtTask>; MAX_RT_TASKS],
    count:    usize,
    tick:     u64,
}

impl EdfScheduler {
    pub const fn new() -> Self {
        Self {
            tasks: [const { None }; MAX_RT_TASKS],
            count: 0,
            tick:  0,
        }
    }

    pub fn add_task(&mut self, task: RtTask) -> bool {
        if self.count >= MAX_RT_TASKS { return false; }
        for slot in &mut self.tasks {
            if slot.is_none() { *slot = Some(task); self.count += 1; return true; }
        }
        false
    }

    pub fn remove_task(&mut self, id: u32) {
        for slot in &mut self.tasks {
            if matches!(slot, Some(t) if t.id == id) {
                *slot = None; self.count -= 1; return;
            }
        }
    }

    /// Pick the ready task with the nearest (earliest) absolute deadline
    pub fn pick_next(&self) -> Option<u32> {
        let mut best_dl = u64::MAX;
        let mut best_id = None;
        for slot in &self.tasks {
            if let Some(ref t) = slot {
                if t.active && t.budget > 0 && t.deadline < best_dl {
                    best_dl = t.deadline;
                    best_id = Some(t.id);
                }
            }
        }
        best_id
    }

    /// Called every tick — decrement running task budget, refresh periods
    pub fn tick(&mut self, running_id: Option<u32>) -> bool {
        self.tick += 1;
        let mut preempt = false;

        for slot in &mut self.tasks {
            if let Some(ref mut t) = slot {
                // Decrement budget for running task
                if running_id == Some(t.id) && t.budget > 0 {
                    t.budget -= 1;
                    if t.budget == 0 { preempt = true; }
                }
                // Refresh task at period boundary
                if self.tick % t.period == 0 {
                    t.budget   = t.wcet;
                    t.deadline = self.tick + t.period;
                }
                // Deadline miss detection
                if self.tick > t.deadline && t.budget > 0 {
                    // Log deadline miss — in full impl: trigger FDIR
                    t.active = false;
                }
            }
        }
        preempt
    }

    /// Schedulability test (Liu & Layland bound: sum(U_i) ≤ 1)
    pub fn is_schedulable(&self) -> bool {
        let mut sum = 0.0f64;
        for slot in &self.tasks {
            if let Some(ref t) = slot { sum += t.utilisation(); }
        }
        sum <= 1.0
    }
}
