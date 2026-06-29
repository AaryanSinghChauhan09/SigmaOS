// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SigmaOS: Sovereign Adaptive Scheduler (Rust, no_std)
//! Replaces: kernel/core/SovereignAdaptiveScheduler.cpp
//! =========================================================================

#![no_std]

use core::cell::UnsafeCell;

pub const MAX_TASKS: usize = 128;
pub const EWMA_ALPHA: u64 = 30; // 30/100 weight for new sample
pub const MIN_SLICE_NS: u64 = 500_000; // 0.5 ms
pub const MAX_SLICE_NS: u64 = 20_000_000; // 20 ms

#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum TaskClass {
    Interactive = 0,
    Batch = 1,
    Realtime = 2,
    Idle = 3,
}

#[derive(Copy, Clone)]
pub struct AdaptiveTask {
    pub task_id: u32,
    pub tclass: TaskClass,
    pub deadline_ns: u64,
    pub ewma_runtime_ns: u64,
    pub last_runtime_ns: u64,
    pub allocated_slice: u64,
    pub run_count: u32,
    pub preemptions: u32,
    pub active: bool,
}

impl AdaptiveTask {
    pub const fn empty() -> Self {
        Self {
            task_id: 0,
            tclass: TaskClass::Idle,
            deadline_ns: 0,
            ewma_runtime_ns: 0,
            last_runtime_ns: 0,
            allocated_slice: 0,
            run_count: 0,
            preemptions: 0,
            active: false,
        }
    }
}

pub trait Scheduler {
    fn register(&mut self, tclass: TaskClass, deadline_ns: u64) -> u32;
    fn complete(&mut self, task_id: u32, runtime_ns: u64, preempted: bool);
    fn elect(&mut self) -> u32;
}

pub struct AdaptiveScheduler {
    tasks: [AdaptiveTask; MAX_TASKS],
    task_count: u32,
    total_preempt: u32,
    tick: u32,
}

impl AdaptiveScheduler {
    pub const fn new() -> Self {
        Self {
            tasks: [AdaptiveTask::empty(); MAX_TASKS],
            task_count: 0,
            total_preempt: 0,
            tick: 0,
        }
    }
}

struct SafeAdaptiveScheduler {
    inner: UnsafeCell<AdaptiveScheduler>,
}

unsafe impl Sync for SafeAdaptiveScheduler {}

static ADAPTIVE_SCHEDULER: SafeAdaptiveScheduler = SafeAdaptiveScheduler {
    inner: UnsafeCell::new(AdaptiveScheduler::new()),
};

fn ewma_update(old_val: u64, sample: u64) -> u64 {
    (EWMA_ALPHA * sample + (100 - EWMA_ALPHA) * old_val) / 100
}

extern "C" {
    fn sigma_log(s: *const u8);
    fn sigma_log_info(fmt: *const u8, val1: u32, val2: *const u8, val3: u32, val4: u32);
}

#[no_mangle]
pub unsafe extern "C" fn asched_init() {
    let s = &mut *ADAPTIVE_SCHEDULER.inner.get();
    s.task_count = 0;
    s.total_preempt = 0;
    s.tick = 0;
    for i in 0..MAX_TASKS {
        s.tasks[i] = AdaptiveTask::empty();
    }
    sigma_log(b"[ASCHED] Sovereign Adaptive Scheduler initialised (Rust core).\n\0".as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn asched_register(task_class: u8, deadline_ns: u64) -> u32 {
    let s = &mut *ADAPTIVE_SCHEDULER.inner.get();
    if s.task_count >= MAX_TASKS as u32 {
        return 0;
    }

    let tc = match task_class {
        1 => TaskClass::Batch,
        2 => TaskClass::Realtime,
        3 => TaskClass::Idle,
        _ => TaskClass::Interactive,
    };

    let idx = s.task_count as usize;
    let t = &mut s.tasks[idx];
    t.task_id = s.task_count + 1;
    t.tclass = tc;
    t.deadline_ns = deadline_ns;
    t.ewma_runtime_ns = match tc {
        TaskClass::Interactive => 1_000_000,
        _ => 5_000_000,
    };
    t.last_runtime_ns = 0;
    t.allocated_slice = t.ewma_runtime_ns;
    t.run_count = 0;
    t.preemptions = 0;
    t.active = true;

    s.task_count += 1;
    t.task_id
}

#[no_mangle]
pub unsafe extern "C" fn asched_complete(task_id: u32, runtime_ns: u64, preempted: bool) {
    let s = &mut *ADAPTIVE_SCHEDULER.inner.get();
    if task_id == 0 || task_id > s.task_count {
        return;
    }

    let t = &mut s.tasks[(task_id - 1) as usize];
    t.last_runtime_ns = runtime_ns;
    t.ewma_runtime_ns = ewma_update(t.ewma_runtime_ns, runtime_ns);
    t.run_count += 1;

    if preempted {
        t.preemptions += 1;
        s.total_preempt += 1;
    }

    // Class-specific recomputation
    let base = t.ewma_runtime_ns;
    let mut slice = match t.tclass {
        TaskClass::Interactive => base / 2,
        TaskClass::Batch => base * 2,
        TaskClass::Realtime => t.deadline_ns,
        TaskClass::Idle => base / 4,
    };

    if slice < MIN_SLICE_NS {
        slice = MIN_SLICE_NS;
    }
    if slice > MAX_SLICE_NS {
        slice = MAX_SLICE_NS;
    }
    t.allocated_slice = slice;
}

#[no_mangle]
pub unsafe extern "C" fn asched_elect() -> u32 {
    let s = &mut *ADAPTIVE_SCHEDULER.inner.get();
    let mut best = 0;
    let mut urgency = 0;
    s.tick = s.tick.wrapping_add(1);

    for i in 0..(s.task_count as usize) {
        let t = &s.tasks[i];
        if !t.active {
            continue;
        }

        let mut score = match t.tclass {
            TaskClass::Realtime => 1_000_000_000,
            TaskClass::Interactive => 100_000_000,
            TaskClass::Batch => 10_000_000,
            TaskClass::Idle => 1_000,
        };

        score += (s.tick.wrapping_sub(t.run_count) as u64) * 1_000;

        if score > urgency {
            urgency = score;
            best = t.task_id;
        }
    }

    best
}

#[no_mangle]
pub unsafe extern "C" fn asched_stats() {
    // Left unimplemented in Rust core if C++ printfs are handled at link-time
}
