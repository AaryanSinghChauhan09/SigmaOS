// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS EDF Scheduler — Earliest-Deadline-First (real-time tasks)
//! Static array min-heap ordered by absolute deadline. no_std, no alloc.

#![no_std]
#![allow(dead_code)]

type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;

pub const EDF_MAX_TASKS: usize = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct EdfTask {
    pub pid:              SigmaU32,
    pub period_ns:        SigmaU64,  // task period
    pub wcet_ns:          SigmaU64,  // worst-case execution time
    pub abs_deadline_ns:  SigmaU64,  // current absolute deadline
    pub active:           bool,
}

static mut EDF_HEAP: [EdfTask; EDF_MAX_TASKS] = [EdfTask {
    pid: 0, period_ns: 0, wcet_ns: 0, abs_deadline_ns: u64::MAX, active: false,
}; EDF_MAX_TASKS];
static mut EDF_HEAP_SIZE: usize = 0;

unsafe fn edf_sift_up(mut i: usize) {
    while i > 0 {
        let p = (i - 1) / 2;
        if EDF_HEAP[i].abs_deadline_ns < EDF_HEAP[p].abs_deadline_ns {
            let tmp = EDF_HEAP[i]; EDF_HEAP[i] = EDF_HEAP[p]; EDF_HEAP[p] = tmp;
            i = p;
        } else { break; }
    }
}

unsafe fn edf_sift_down(mut i: usize) {
    let n = EDF_HEAP_SIZE;
    loop {
        let l = 2 * i + 1;
        let r = 2 * i + 2;
        let mut m = i;
        if l < n && EDF_HEAP[l].abs_deadline_ns < EDF_HEAP[m].abs_deadline_ns { m = l; }
        if r < n && EDF_HEAP[r].abs_deadline_ns < EDF_HEAP[m].abs_deadline_ns { m = r; }
        if m == i { break; }
        let tmp = EDF_HEAP[i]; EDF_HEAP[i] = EDF_HEAP[m]; EDF_HEAP[m] = tmp;
        i = m;
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_edf_init() {
    EDF_HEAP_SIZE = 0;
}

/// Admit a real-time task. `now_ns` is current monotonic time.
#[no_mangle]
pub unsafe extern "C" fn sigma_edf_admit(
    pid: SigmaU32,
    period_ns: SigmaU64,
    wcet_ns: SigmaU64,
    now_ns: SigmaU64,
) -> SigmaI32 {
    // EDF schedulability test: sum of (wcet/period) must be ≤ 1.0
    // Approximated with integer arithmetic: sum(wcet*1000/period) ≤ 1000
    let mut load = 0u64;
    for i in 0..EDF_HEAP_SIZE {
        load += EDF_HEAP[i].wcet_ns * 1000 / EDF_HEAP[i].period_ns.max(1);
    }
    load += wcet_ns * 1000 / period_ns.max(1);
    if load > 1000 { return -1; }  // not schedulable

    if EDF_HEAP_SIZE >= EDF_MAX_TASKS { return -1; }
    let task = EdfTask {
        pid,
        period_ns,
        wcet_ns,
        abs_deadline_ns: now_ns + period_ns,
        active: true,
    };
    EDF_HEAP[EDF_HEAP_SIZE] = task;
    EDF_HEAP_SIZE += 1;
    edf_sift_up(EDF_HEAP_SIZE - 1);
    0
}

/// Returns the pid of the task with the earliest absolute deadline.
#[no_mangle]
pub unsafe extern "C" fn sigma_edf_pick_next() -> SigmaU32 {
    if EDF_HEAP_SIZE == 0 { return 0; }
    EDF_HEAP[0].pid
}

/// Called when a task completes its current job. Advances deadline by one period.
#[no_mangle]
pub unsafe extern "C" fn sigma_edf_job_complete(pid: SigmaU32) {
    for i in 0..EDF_HEAP_SIZE {
        if EDF_HEAP[i].pid == pid {
            EDF_HEAP[i].abs_deadline_ns += EDF_HEAP[i].period_ns;
            edf_sift_down(i);
            return;
        }
    }
}

/// Remove a real-time task.
#[no_mangle]
pub unsafe extern "C" fn sigma_edf_remove(pid: SigmaU32) {
    for i in 0..EDF_HEAP_SIZE {
        if EDF_HEAP[i].pid == pid {
            EDF_HEAP_SIZE -= 1;
            EDF_HEAP[i] = EDF_HEAP[EDF_HEAP_SIZE];
            edf_sift_down(i);
            return;
        }
    }
}
