// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS AI Predictive Scheduler
//! Monitors task usage patterns and predicts resource needs (pre-warming).
//! no_std, no alloc.

#![no_std]
#![allow(dead_code)]

type SigmaU32 = u32;
type SigmaI32 = i32;

pub const MAX_PREDICT_TASKS: usize = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TaskPrediction {
    pub pid: SigmaU32,
    pub exec_count: SigmaU32,
    pub avg_duration_ms: SigmaU32,
    pub peak_mem_kb: SigmaU32,
    pub active: bool,
}

static mut PREDICTIONS: [TaskPrediction; MAX_PREDICT_TASKS] = [TaskPrediction {
    pid: 0, exec_count: 0, avg_duration_ms: 0, peak_mem_kb: 0, active: false
}; MAX_PREDICT_TASKS];

#[no_mangle]
pub unsafe extern "C" fn sigma_ai_sched_init() {
    for i in 0..MAX_PREDICT_TASKS {
        PREDICTIONS[i].active = false;
    }
}

unsafe fn ai_sched_find_or_alloc(pid: SigmaU32) -> Option<usize> {
    let mut free_slot = None;
    for i in 0..MAX_PREDICT_TASKS {
        if PREDICTIONS[i].active && PREDICTIONS[i].pid == pid {
            return Some(i);
        }
        if !PREDICTIONS[i].active && free_slot.is_none() {
            free_slot = Some(i);
        }
    }
    
    if let Some(slot) = free_slot {
        PREDICTIONS[slot].pid = pid;
        PREDICTIONS[slot].exec_count = 0;
        PREDICTIONS[slot].avg_duration_ms = 0;
        PREDICTIONS[slot].peak_mem_kb = 0;
        PREDICTIONS[slot].active = true;
        return Some(slot);
    }
    None
}

/// Train the model based on task execution history.
#[no_mangle]
pub unsafe extern "C" fn sigma_ai_sched_record_run(pid: SigmaU32, duration_ms: SigmaU32, mem_kb: SigmaU32) {
    if let Some(idx) = ai_sched_find_or_alloc(pid) {
        let p = &mut PREDICTIONS[idx];
        
        // Exponential moving average for duration
        if p.exec_count == 0 {
            p.avg_duration_ms = duration_ms;
        } else {
            p.avg_duration_ms = (p.avg_duration_ms * 3 + duration_ms) / 4;
        }
        
        // Track peak memory
        if mem_kb > p.peak_mem_kb {
            p.peak_mem_kb = mem_kb;
        }
        
        p.exec_count += 1;
    }
}

/// Predict memory requirements for a task before it starts (for pre-warming).
#[no_mangle]
pub unsafe extern "C" fn sigma_ai_sched_predict_mem(pid: SigmaU32) -> SigmaU32 {
    for i in 0..MAX_PREDICT_TASKS {
        if PREDICTIONS[i].active && PREDICTIONS[i].pid == pid {
            return PREDICTIONS[i].peak_mem_kb;
        }
    }
    0 // Unknown
}

/// Predict expected execution time for a task.
#[no_mangle]
pub unsafe extern "C" fn sigma_ai_sched_predict_time(pid: SigmaU32) -> SigmaU32 {
    for i in 0..MAX_PREDICT_TASKS {
        if PREDICTIONS[i].active && PREDICTIONS[i].pid == pid {
            return PREDICTIONS[i].avg_duration_ms;
        }
    }
    0 // Unknown
}
