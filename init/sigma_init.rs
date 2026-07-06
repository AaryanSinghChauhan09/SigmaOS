// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS Sovereign Init (PID 1)
//! Minimal runit-style service orchestration.
//! no_std, no alloc.

#![no_std]
#![allow(dead_code)]

type SigmaU32 = u32;
type SigmaI32 = i32;

pub const MAX_SERVICES: usize = 32;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum ServiceState {
    Down = 0,
    Starting = 1,
    Up = 2,
    Stopping = 3,
    Failed = 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ServiceDesc {
    pub name: [u8; 32],
    pub pid: SigmaU32,
    pub state: ServiceState,
    pub restart_count: SigmaU32,
    pub auto_restart: bool,
    pub active: bool,
}

static mut SERVICES: [ServiceDesc; MAX_SERVICES] = [ServiceDesc {
    name: [0; 32], pid: 0, state: ServiceState::Down,
    restart_count: 0, auto_restart: false, active: false,
}; MAX_SERVICES];

/// Register a new service descriptor.
#[no_mangle]
pub unsafe extern "C" fn sigma_init_register_service(name: *const u8, auto_restart: bool) -> SigmaI32 {
    if name.is_null() { return -1; }
    for i in 0..MAX_SERVICES {
        if !SERVICES[i].active {
            let mut j = 0;
            while j < 31 && *name.add(j) != 0 {
                SERVICES[i].name[j] = *name.add(j);
                j += 1;
            }
            SERVICES[i].name[j] = 0;
            SERVICES[i].state = ServiceState::Down;
            SERVICES[i].auto_restart = auto_restart;
            SERVICES[i].restart_count = 0;
            SERVICES[i].pid = 0;
            SERVICES[i].active = true;
            return i as SigmaI32;
        }
    }
    -1 // Table full
}

/// Start a service. (Simulates fork/exec).
#[no_mangle]
pub unsafe extern "C" fn sigma_init_start_service(idx: SigmaI32) -> SigmaI32 {
    if idx < 0 || idx as usize >= MAX_SERVICES { return -1; }
    let srv = &mut SERVICES[idx as usize];
    if !srv.active || srv.state == ServiceState::Up || srv.state == ServiceState::Starting {
        return -1;
    }
    srv.state = ServiceState::Starting;
    // In real code: sys_fork() and sys_execve(). We simulate success here.
    srv.pid = (100 + idx) as SigmaU32; // fake PID
    srv.state = ServiceState::Up;
    0
}

/// Stop a service (simulate SIGTERM).
#[no_mangle]
pub unsafe extern "C" fn sigma_init_stop_service(idx: SigmaI32) -> SigmaI32 {
    if idx < 0 || idx as usize >= MAX_SERVICES { return -1; }
    let srv = &mut SERVICES[idx as usize];
    if !srv.active || srv.state == ServiceState::Down { return -1; }
    srv.state = ServiceState::Stopping;
    // In real code: sys_kill(pid, SIGTERM).
    srv.pid = 0;
    srv.state = ServiceState::Down;
    0
}

/// Handle child exit (SIGCHLD equivalent).
#[no_mangle]
pub unsafe extern "C" fn sigma_init_handle_exit(pid: SigmaU32, exit_code: SigmaI32) {
    for i in 0..MAX_SERVICES {
        if SERVICES[i].active && SERVICES[i].pid == pid {
            SERVICES[i].pid = 0;
            if exit_code == 0 {
                SERVICES[i].state = ServiceState::Down;
            } else {
                SERVICES[i].state = ServiceState::Failed;
            }
            if SERVICES[i].auto_restart && SERVICES[i].restart_count < 5 {
                SERVICES[i].restart_count += 1;
                sigma_init_start_service(i as SigmaI32);
            }
            return;
        }
    }
}