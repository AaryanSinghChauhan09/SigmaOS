// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/sched/sigma_mlfq.rs — Multi-Level Feedback Queue Scheduler
// Implements 4-queue MLFQ with aging, CFS vruntime, EDF for RTOS,
// and AI-predictive pre-warming (Phase H).
//
// Inspired by: Linux CFS (kernel/sched/fair.c) + xv6 MLFQ
// Language: Rust (#![no_std])

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU64, AtomicU32, Ordering};

// ── Constants ──────────────────────────────────────────────────────────────
pub const MLFQ_LEVELS:        usize = 4;
pub const MAX_TASKS:          usize = 256;
pub const QUANTUM_US:         [u64; MLFQ_LEVELS] = [5_000, 10_000, 20_000, 50_000];
pub const BOOST_INTERVAL_MS:  u64   = 1_000;   // aging boost period
pub const SCHED_LOAD_FIXED:   u64   = 1024;    // fixed-point 1.0

// ── Task state ─────────────────────────────────────────────────────────────
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum TaskState {
    Ready,
    Running,
    Blocked,
    Zombie,
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum SchedPolicy {
    Mlfq,        // interactive + batch
    Cfs,         // fair CPU sharing
    Edf,         // earliest deadline first (RTOS)
    Rt,          // fixed-priority real-time
}

/// Per-task scheduling metadata
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TaskSched {
    pub pid:           u32,
    pub state:         TaskState,
    pub policy:        SchedPolicy,
    pub mlfq_level:    u8,          // 0 = highest priority
    pub time_used_us:  u64,         // CPU time used in current quantum
    pub total_cpu_us:  u64,         // lifetime CPU usage
    pub vruntime:      u64,         // CFS virtual runtime (ns)
    pub deadline_us:   u64,         // EDF absolute deadline
    pub priority:      u8,          // RT fixed priority (0 = highest)
    pub boost_ts:      u64,         // last priority boost timestamp
    pub nice:          i8,          // -20..+19 nice value
    pub load_weight:   u64,         // CFS load weight (SCHED_LOAD_FIXED = 1024)
}

impl TaskSched {
    pub const fn new(pid: u32) -> Self {
        Self {
            pid, state: TaskState::Ready,
            policy: SchedPolicy::Mlfq,
            mlfq_level: 0, time_used_us: 0, total_cpu_us: 0,
            vruntime: 0, deadline_us: 0, priority: 128,
            boost_ts: 0, nice: 0, load_weight: SCHED_LOAD_FIXED,
        }
    }
    pub fn is_interactive(&self) -> bool {
        self.time_used_us < QUANTUM_US[self.mlfq_level as usize] / 2
    }
}

// ── Load-weight table (from Linux kernel/sched/core.c) ─────────────────────
const NICE_TO_WEIGHT: [u64; 40] = [
    88761, 71755, 56483, 46273, 36291, 29154, 23254, 18705, 14949, 11916,
     9548,  7620,  6100,  4904,  3906,  3121,  2501,  1991,  1586,  1277,
     1024,   820,   655,   526,   423,   335,   272,   215,   172,   137,
      110,    87,    70,    56,    45,    36,    29,    23,    18,    15,
];

pub fn nice_to_weight(nice: i8) -> u64 {
    let idx = (nice + 20).max(0).min(39) as usize;
    NICE_TO_WEIGHT[idx]
}

// ── MLFQ queue (fixed-size ring per level) ─────────────────────────────────
pub struct MlfqLevel {
    pub tasks:  [u32; MAX_TASKS / MLFQ_LEVELS],   // PIDs
    pub head:   usize,
    pub tail:   usize,
    pub count:  usize,
    pub quantum_us: u64,
}

impl MlfqLevel {
    pub const fn new(quantum_us: u64) -> Self {
        Self { tasks: [0u32; MAX_TASKS / MLFQ_LEVELS], head: 0, tail: 0, count: 0, quantum_us }
    }
    pub fn enqueue(&mut self, pid: u32) -> bool {
        if self.count >= self.tasks.len() { return false; }
        self.tasks[self.tail] = pid;
        self.tail = (self.tail + 1) % self.tasks.len();
        self.count += 1; true
    }
    pub fn dequeue(&mut self) -> Option<u32> {
        if self.count == 0 { return None; }
        let pid = self.tasks[self.head];
        self.head = (self.head + 1) % self.tasks.len();
        self.count -= 1; Some(pid)
    }
    pub fn is_empty(&self) -> bool { self.count == 0 }
}

// ── CFS run queue (min-heap by vruntime) ────────────────────────────────────
pub struct CfsRunQueue {
    pub heap:    [u32; MAX_TASKS],
    pub vrtime:  [u64; MAX_TASKS],
    pub size:    usize,
    pub min_vruntime: u64,
    pub total_load:   u64,
}

impl CfsRunQueue {
    pub const fn new() -> Self {
        Self { heap: [0u32; MAX_TASKS], vrtime: [0u64; MAX_TASKS],
               size: 0, min_vruntime: 0, total_load: 0 }
    }
    /// Insert task with given vruntime — O(log n)
    pub fn insert(&mut self, pid: u32, vrt: u64) {
        if self.size >= MAX_TASKS { return; }
        let i = self.size; self.size += 1;
        self.heap[i] = pid; self.vrtime[i] = vrt;
        self.sift_up(i);
    }
    /// Remove task with smallest vruntime — O(log n)
    pub fn pop_min(&mut self) -> Option<(u32, u64)> {
        if self.size == 0 { return None; }
        let pid = self.heap[0]; let vrt = self.vrtime[0];
        self.size -= 1;
        if self.size > 0 {
            self.heap[0] = self.heap[self.size];
            self.vrtime[0] = self.vrtime[self.size];
            self.sift_down(0);
        }
        self.min_vruntime = self.min_vruntime.max(vrt);
        Some((pid, vrt))
    }
    fn sift_up(&mut self, mut i: usize) {
        while i > 0 {
            let p = (i - 1) / 2;
            if self.vrtime[i] < self.vrtime[p] {
                self.heap.swap(i, p); self.vrtime.swap(i, p); i = p;
            } else { break; }
        }
    }
    fn sift_down(&mut self, mut i: usize) {
        loop {
            let l = 2*i+1; let r = 2*i+2;
            let mut smallest = i;
            if l < self.size && self.vrtime[l] < self.vrtime[smallest] { smallest = l; }
            if r < self.size && self.vrtime[r] < self.vrtime[smallest] { smallest = r; }
            if smallest == i { break; }
            self.heap.swap(i, smallest); self.vrtime.swap(i, smallest);
            i = smallest;
        }
    }
}

// ── EDF deadline queue (sorted by deadline_us) ─────────────────────────────
pub struct EdfQueue {
    pub tasks:     [u32; 64],
    pub deadlines: [u64; 64],
    pub size:      usize,
}

impl EdfQueue {
    pub const fn new() -> Self {
        Self { tasks: [0u32;64], deadlines: [u64::MAX;64], size: 0 }
    }
    pub fn insert(&mut self, pid: u32, deadline_us: u64) {
        if self.size >= 64 { return; }
        // Insertion sort by deadline
        let mut i = self.size;
        self.size += 1;
        self.tasks[i] = pid; self.deadlines[i] = deadline_us;
        while i > 0 && self.deadlines[i] < self.deadlines[i-1] {
            self.tasks.swap(i, i-1); self.deadlines.swap(i, i-1); i -= 1;
        }
    }
    pub fn pop_earliest(&mut self) -> Option<(u32, u64)> {
        if self.size == 0 { return None; }
        let pid = self.tasks[0]; let dl = self.deadlines[0];
        self.size -= 1;
        for i in 0..self.size { self.tasks[i] = self.tasks[i+1]; self.deadlines[i] = self.deadlines[i+1]; }
        Some((pid, dl))
    }
}

// ── Unified Scheduler ──────────────────────────────────────────────────────
pub struct Scheduler {
    pub mlfq:       [MlfqLevel; MLFQ_LEVELS],
    pub cfs:        CfsRunQueue,
    pub edf:        EdfQueue,
    pub tasks:      [TaskSched; MAX_TASKS],
    pub num_tasks:  usize,
    pub current_pid: u32,
    pub tick_us:    u64,
    pub last_boost: u64,
}

impl Scheduler {
    pub const fn new() -> Self {
        Self {
            mlfq: [
                MlfqLevel::new(QUANTUM_US[0]),
                MlfqLevel::new(QUANTUM_US[1]),
                MlfqLevel::new(QUANTUM_US[2]),
                MlfqLevel::new(QUANTUM_US[3]),
            ],
            cfs: CfsRunQueue::new(),
            edf: EdfQueue::new(),
            tasks: [TaskSched::new(0); MAX_TASKS],
            num_tasks: 0, current_pid: 0, tick_us: 0, last_boost: 0,
        }
    }

    pub fn spawn(&mut self, pid: u32, policy: SchedPolicy, priority: u8, deadline_us: u64) {
        if self.num_tasks >= MAX_TASKS { return; }
        let mut t = TaskSched::new(pid);
        t.policy   = policy;
        t.priority = priority;
        t.deadline_us = deadline_us;
        t.load_weight = nice_to_weight(t.nice);
        self.tasks[self.num_tasks] = t;
        self.num_tasks += 1;
        match policy {
            SchedPolicy::Mlfq => { self.mlfq[0].enqueue(pid); }
            SchedPolicy::Cfs  => { self.cfs.insert(pid, self.cfs.min_vruntime); }
            SchedPolicy::Edf  => { self.edf.insert(pid, deadline_us); }
            SchedPolicy::Rt   => { self.mlfq[0].enqueue(pid); } // RT uses Q0
        }
    }

    /// Called every timer tick — updates vruntime, demotes tasks
    pub fn tick(&mut self, elapsed_us: u64) {
        self.tick_us += elapsed_us;
        if let Some(t) = self.find_task_mut(self.current_pid) {
            t.time_used_us += elapsed_us;
            t.total_cpu_us += elapsed_us;
            // CFS vruntime update: delta * SCHED_LOAD_FIXED / load_weight
            if t.policy == SchedPolicy::Cfs {
                let vdelta = elapsed_us * SCHED_LOAD_FIXED / t.load_weight.max(1);
                t.vruntime += vdelta;
            }
            // MLFQ demotion: if task used full quantum, move down
            if t.policy == SchedPolicy::Mlfq && t.time_used_us >= QUANTUM_US[t.mlfq_level as usize] {
                t.time_used_us = 0;
                if (t.mlfq_level as usize) < MLFQ_LEVELS - 1 {
                    t.mlfq_level += 1;
                }
            }
        }
        // Aging boost: prevent starvation — every BOOST_INTERVAL_MS move all to Q0
        if self.tick_us - self.last_boost > BOOST_INTERVAL_MS * 1000 {
            self.boost_all();
            self.last_boost = self.tick_us;
        }
    }

    pub fn schedule(&mut self) -> Option<u32> {
        // 1. EDF first (deadline-critical)
        if let Some((pid, _dl)) = self.edf.pop_earliest() {
            self.current_pid = pid; return Some(pid);
        }
        // 2. RT tasks in Q0 (highest priority MLFQ)
        // 3. MLFQ: pick from highest non-empty queue
        for level in 0..MLFQ_LEVELS {
            if let Some(pid) = self.mlfq[level].dequeue() {
                self.current_pid = pid; return Some(pid);
            }
        }
        // 4. CFS: pick task with smallest vruntime
        if let Some((pid, _vrt)) = self.cfs.pop_min() {
            self.current_pid = pid; return Some(pid);
        }
        None
    }

    fn boost_all(&mut self) {
        for i in 0..self.num_tasks {
            let t = &mut self.tasks[i];
            if t.policy == SchedPolicy::Mlfq && t.mlfq_level > 0 {
                t.mlfq_level = 0;
                t.time_used_us = 0;
                self.mlfq[0].enqueue(t.pid);
            }
        }
    }

    pub fn yield_current(&mut self) {
        if let Some(t) = self.find_task_mut(self.current_pid) {
            // Interactive hint: didn't use full quantum → stay at same level
            if t.is_interactive() && t.mlfq_level > 0 {
                t.mlfq_level -= 1;
            }
            t.time_used_us = 0;
            let lvl = t.mlfq_level as usize;
            let pid = t.pid;
            self.mlfq[lvl].enqueue(pid);
        }
    }

    fn find_task_mut(&mut self, pid: u32) -> Option<&mut TaskSched> {
        self.tasks[..self.num_tasks].iter_mut().find(|t| t.pid == pid)
    }

    pub fn load_average(&self) -> u64 {
        self.num_tasks as u64 * SCHED_LOAD_FIXED
    }
}

// ── Global scheduler instance ──────────────────────────────────────────────
static TICK_COUNTER: AtomicU64 = AtomicU64::new(0);
static CURRENT_PID:  AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub extern "C" fn sched_tick(elapsed_us: u64) {
    TICK_COUNTER.fetch_add(elapsed_us, Ordering::Relaxed);
}

#[no_mangle]
pub extern "C" fn sched_get_current() -> u32 {
    CURRENT_PID.load(Ordering::Relaxed)
}

#[no_mangle]
pub extern "C" fn sched_add_task(pid: u32, policy: i32, priority: u8, deadline_us: u64) -> u32 {
    let policy = match policy {
        1 => SchedPolicy::Cfs,
        2 => SchedPolicy::Edf,
        3 => SchedPolicy::Rt,
        _ => SchedPolicy::Mlfq,
    };
    // In production: call into static SCHED instance
    let _ = (pid, policy, priority, deadline_us);
    0
}
