// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS Intrusion Detection System (IDS) Hooks
//! Tracks anomaly scores per process based on syscall behavior.
//! no_std, no alloc.

#![no_std]
#![allow(dead_code)]

type SigmaU32 = u32;
type SigmaI32 = i32;

pub const MAX_IDS_TRACKED_PIDS: usize = 64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IdsState {
    pub pid: SigmaU32,
    pub anomaly_score: SigmaU32,
    pub failed_syscalls: SigmaU32,
    pub last_syscall: SigmaU32,
    pub active: bool,
}

static mut IDS_STATES: [IdsState; MAX_IDS_TRACKED_PIDS] = [IdsState {
    pid: 0, anomaly_score: 0, failed_syscalls: 0, last_syscall: 0, active: false
}; MAX_IDS_TRACKED_PIDS];

unsafe fn ids_find_or_alloc(pid: SigmaU32) -> Option<usize> {
    let mut free_slot = None;
    for i in 0..MAX_IDS_TRACKED_PIDS {
        if IDS_STATES[i].active && IDS_STATES[i].pid == pid {
            return Some(i);
        }
        if !IDS_STATES[i].active && free_slot.is_none() {
            free_slot = Some(i);
        }
    }
    
    if let Some(slot) = free_slot {
        IDS_STATES[slot].pid = pid;
        IDS_STATES[slot].anomaly_score = 0;
        IDS_STATES[slot].failed_syscalls = 0;
        IDS_STATES[slot].last_syscall = 0;
        IDS_STATES[slot].active = true;
        return Some(slot);
    }
    None
}

#[no_mangle]
pub unsafe extern "C" fn sigma_ids_init() {
    for i in 0..MAX_IDS_TRACKED_PIDS {
        IDS_STATES[i].active = false;
    }
}

/// Called on every syscall entry.
#[no_mangle]
pub unsafe extern "C" fn sigma_ids_on_syscall(pid: SigmaU32, syscall_nr: SigmaU32) {
    if let Some(idx) = ids_find_or_alloc(pid) {
        let state = &mut IDS_STATES[idx];
        
        // Simple heuristic: if repeatedly calling the same sensitive syscall, increment anomaly score
        if state.last_syscall == syscall_nr {
            // e.g. ptrace, mount, or execve
            if syscall_nr == 101 /* ptrace */ || syscall_nr == 165 /* mount */ {
                state.anomaly_score += 10;
            }
        }
        state.last_syscall = syscall_nr;
    }
}

/// Called on every syscall exit that results in an error (e.g. EPERM, EACCES).
#[no_mangle]
pub unsafe extern "C" fn sigma_ids_on_syscall_error(pid: SigmaU32) {
    if let Some(idx) = ids_find_or_alloc(pid) {
        let state = &mut IDS_STATES[idx];
        state.failed_syscalls += 1;
        state.anomaly_score += 5; // failed syscalls contribute to anomaly
    }
}

/// Check if a process should be killed due to anomalous behavior.
#[no_mangle]
pub unsafe extern "C" fn sigma_ids_check_kill(pid: SigmaU32, threshold: SigmaU32) -> SigmaI32 {
    if let Some(idx) = ids_find_or_alloc(pid) {
        let state = &IDS_STATES[idx];
        if state.anomaly_score > threshold {
            return 1; // Kill recommended
        }
    }
    0 // Safe
}

#[no_mangle]
pub unsafe extern "C" fn sigma_ids_cleanup(pid: SigmaU32) {
    for i in 0..MAX_IDS_TRACKED_PIDS {
        if IDS_STATES[i].active && IDS_STATES[i].pid == pid {
            IDS_STATES[i].active = false;
            return;
        }
    }
}
