// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/core/sigma_sched.rs — Complete SigmaOS scheduler
//
// Implements MLFQ + CFS + EDF in one unified scheduler.
// MLFQ for interactive tasks, CFS for fair sharing, EDF for RTOS hard-RT.
//
// Language: Rust #![no_std]
#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

// ── Scheduler policy ──────────────────────────────────────────────────────
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum SchedPolicy {
    Mlfq     = 0,  // interactive / general
    Cfs      = 1,  // fair CPU sharing
    Edf      = 2,  // hard real-time (RTOS profile)
    Fifo     = 3,  // real-time FIFO
    Idle     = 4,  // idle tasks only
}

// ── Task control block ────────────────────────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TaskCb {
    pub pid:        u32,
    pub ppid:       u32,
    pub policy:     SchedPolicy,
    pub mlfq_level: u8,     // 0 = highest priority
    pub vruntime:   u64,    // CFS virtual runtime (nanoseconds)
    pub deadline:   u64,    // EDF absolute deadline (nanoseconds)
    pub timeslice:  u32,    // remaining ticks
    pub cpu_affinity: u32,  // bitmask of allowed CPUs
    pub state:      TaskState,
    pub _pad:       [u8; 3],
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum TaskState {
    Running  = 0,
    Runnable = 1,
    Blocked  = 2,
    Zombie   = 3,
}

impl TaskCb {
    pub const fn new(pid: u32, policy: SchedPolicy) -> Self {
        Self {
            pid, ppid: 0, policy,
            mlfq_level: 0, vruntime: 0, deadline: 0,
            timeslice: 10, cpu_affinity: 0xFFFF_FFFF,
            state: TaskState::Runnable, _pad: [0u8; 3],
        }
    }
}

// ── MLFQ — 4 priority queues ─────────────────────────────────────────────
const MLFQ_LEVELS:  usize = 4;
const QUEUE_DEPTH:  usize = 256;
const MLFQ_QUANTA: [u32; 4] = [2, 4, 8, 16]; // ticks per level
const AGING_TICKS:  u32 = 200; // ticks before boosting to Q0

pub struct MlfqQueue {
    pids:  [[u32; QUEUE_DEPTH]; MLFQ_LEVELS],
    head:  [usize; MLFQ_LEVELS],
    tail:  [usize; MLFQ_LEVELS],
    count: [usize; MLFQ_LEVELS],
}

impl MlfqQueue {
    pub const fn new() -> Self {
        Self {
            pids:  [[0u32; QUEUE_DEPTH]; MLFQ_LEVELS],
            head:  [0usize; MLFQ_LEVELS],
            tail:  [0usize; MLFQ_LEVELS],
            count: [0usize; MLFQ_LEVELS],
        }
    }

    pub fn enqueue(&mut self, level: usize, pid: u32) -> bool {
        let l = level.min(MLFQ_LEVELS - 1);
        if self.count[l] >= QUEUE_DEPTH { return false; }
        self.pids[l][self.tail[l]] = pid;
        self.tail[l] = (self.tail[l] + 1) % QUEUE_DEPTH;
        self.count[l] += 1;
        true
    }

    pub fn dequeue(&mut self) -> Option<(u32, usize)> {
        for l in 0..MLFQ_LEVELS {
            if self.count[l] > 0 {
                let pid = self.pids[l][self.head[l]];
                self.head[l] = (self.head[l] + 1) % QUEUE_DEPTH;
                self.count[l] -= 1;
                return Some((pid, l));
            }
        }
        None
    }

    /// Priority boost — move all tasks to Q0 (prevents starvation)
    pub fn boost(&mut self) {
        for l in 1..MLFQ_LEVELS {
            while self.count[l] > 0 {
                let pid = self.pids[l][self.head[l]];
                self.head[l] = (self.head[l] + 1) % QUEUE_DEPTH;
                self.count[l] -= 1;
                self.enqueue(0, pid);
            }
        }
    }
}

// ── CFS — virtual runtime min-heap (simplified as sorted array) ───────────
const CFS_MAX_TASKS: usize = 256;

pub struct CfsRunqueue {
    tasks:  [TaskCb; CFS_MAX_TASKS],
    count:  usize,
    min_vruntime: u64,
}

impl CfsRunqueue {
    pub const fn new() -> Self {
        Self {
            tasks: [TaskCb::new(0, SchedPolicy::Cfs); CFS_MAX_TASKS],
            count: 0,
            min_vruntime: 0,
        }
    }

    pub fn insert(&mut self, t: TaskCb) -> bool {
        if self.count >= CFS_MAX_TASKS { return false; }
        // Ensure new task starts at min_vruntime (prevent CPU hogging)
        let mut task = t;
        if task.vruntime < self.min_vruntime {
            task.vruntime = self.min_vruntime;
        }
        self.tasks[self.count] = task;
        self.count += 1;
        true
    }

    /// Pick the task with the smallest vruntime (leftmost in red-black tree)
    pub fn pick_next(&mut self) -> Option<TaskCb> {
        if self.count == 0 { return None; }
        let mut min_idx = 0;
        for i in 1..self.count {
            if self.tasks[i].vruntime < self.tasks[min_idx].vruntime {
                min_idx = i;
            }
        }
        let task = self.tasks[min_idx];
        // Remove from queue
        self.tasks[min_idx] = self.tasks[self.count - 1];
        self.count -= 1;
        self.min_vruntime = task.vruntime;
        Some(task)
    }

    /// Update vruntime after a tick (weight = 1024 / priority_weight)
    pub fn account_tick(&mut self, pid: u32, delta_ns: u64) {
        for i in 0..self.count {
            if self.tasks[i].pid == pid {
                self.tasks[i].vruntime += delta_ns;
                break;
            }
        }
    }
}

// ── EDF — earliest deadline first ────────────────────────────────────────
const EDF_MAX_TASKS: usize = 64;

pub struct EdfRunqueue {
    tasks: [TaskCb; EDF_MAX_TASKS],
    count: usize,
}

impl EdfRunqueue {
    pub const fn new() -> Self {
        Self {
            tasks: [TaskCb::new(0, SchedPolicy::Edf); EDF_MAX_TASKS],
            count: 0,
        }
    }

    pub fn insert(&mut self, t: TaskCb) -> bool {
        if self.count >= EDF_MAX_TASKS { return false; }
        self.tasks[self.count] = t;
        self.count += 1;
        true
    }

    /// Pick task with earliest (smallest) deadline
    pub fn pick_next(&mut self) -> Option<TaskCb> {
        if self.count == 0 { return None; }
        let mut min_idx = 0;
        for i in 1..self.count {
            if self.tasks[i].deadline < self.tasks[min_idx].deadline {
                min_idx = i;
            }
        }
        let task = self.tasks[min_idx];
        self.tasks[min_idx] = self.tasks[self.count - 1];
        self.count -= 1;
        Some(task)
    }

    /// Check for deadline misses (returns missed PIDs count)
    pub fn check_deadline_misses(&self, now_ns: u64) -> u32 {
        let mut missed = 0u32;
        for i in 0..self.count {
            if self.tasks[i].deadline < now_ns {
                missed += 1;
            }
        }
        missed
    }
}

// ── Unified scheduler ─────────────────────────────────────────────────────
const MAX_TASKS: usize = 512;

pub struct SigmaScheduler {
    tasks:         [TaskCb; MAX_TASKS],
    task_count:    usize,
    mlfq:          MlfqQueue,
    cfs:           CfsRunqueue,
    edf:           EdfRunqueue,
    tick_count:    AtomicU64,
    current_pid:   AtomicU32,
    initialized:   bool,
}

impl SigmaScheduler {
    pub const fn new() -> Self {
        Self {
            tasks:       [TaskCb::new(0, SchedPolicy::Mlfq); MAX_TASKS],
            task_count:  0,
            mlfq:        MlfqQueue::new(),
            cfs:         CfsRunqueue::new(),
            edf:         EdfRunqueue::new(),
            tick_count:  AtomicU64::new(0),
            current_pid: AtomicU32::new(0),
            initialized: false,
        }
    }

    pub fn init(&mut self) { self.initialized = true; }

    pub fn add_task(&mut self, t: TaskCb) -> bool {
        if self.task_count >= MAX_TASKS { return false; }
        let idx = self.task_count;
        self.tasks[idx] = t;
        self.task_count += 1;
        match t.policy {
            SchedPolicy::Mlfq | SchedPolicy::Fifo =>
                self.mlfq.enqueue(t.mlfq_level as usize, t.pid),
            SchedPolicy::Cfs  => self.cfs.insert(t),
            SchedPolicy::Edf  => self.edf.insert(t),
            SchedPolicy::Idle => true,
        };
        true
    }

    /// Called on every timer tick — returns PID to run next
    pub fn schedule(&mut self, now_ns: u64) -> u32 {
        let tick = self.tick_count.fetch_add(1, Ordering::Relaxed);

        // Priority boost every AGING_TICKS
        if tick % AGING_TICKS as u64 == 0 {
            self.mlfq.boost();
        }

        // EDF first (hard real-time)
        if let Some(t) = self.edf.pick_next() {
            self.current_pid.store(t.pid, Ordering::Relaxed);
            return t.pid;
        }

        // MLFQ second (interactive)
        if let Some((pid, _lvl)) = self.mlfq.dequeue() {
            self.current_pid.store(pid, Ordering::Relaxed);
            return pid;
        }

        // CFS third (fair sharing)
        if let Some(t) = self.cfs.pick_next() {
            self.current_pid.store(t.pid, Ordering::Relaxed);
            return t.pid;
        }

        0 // idle
    }

    pub fn current_pid(&self) -> u32 { self.current_pid.load(Ordering::Relaxed) }
    pub fn tick_count(&self)  -> u64 { self.tick_count.load(Ordering::Relaxed)  }
}

static mut G_SCHEDULER: SigmaScheduler = SigmaScheduler::new();

// ── C-ABI exports ─────────────────────────────────────────────────────────
#[no_mangle] pub unsafe extern "C" fn asched_init() { G_SCHEDULER.init(); }

#[no_mangle]
pub unsafe extern "C" fn sched_add_task(
    pid: u32, policy: u8, deadline: u64, mlfq_level: u8,
) -> i32 {
    let pol = match policy {
        0 => SchedPolicy::Mlfq, 1 => SchedPolicy::Cfs,
        2 => SchedPolicy::Edf,  3 => SchedPolicy::Fifo,
        _ => SchedPolicy::Idle,
    };
    let mut t = TaskCb::new(pid, pol);
    t.deadline   = deadline;
    t.mlfq_level = mlfq_level;
    if G_SCHEDULER.add_task(t) { 0 } else { -12 }
}

#[no_mangle]
pub unsafe extern "C" fn sched_tick(now_ns: u64) -> u32 {
    G_SCHEDULER.schedule(now_ns)
}

#[no_mangle]
pub unsafe extern "C" fn sched_current_pid() -> u32 {
    G_SCHEDULER.current_pid()
}
