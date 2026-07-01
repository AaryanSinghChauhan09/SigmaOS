// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SigmaOS: Sovereign Process Manager (Rust, no_std)
//! Replaces: kernel/core/SovereignProcessManager.cpp
//! =========================================================================

#![no_std]

use core::cell::UnsafeCell;

// System Constants matching C++ header definitions
pub const SIGMA_MAX_PROCESSES: usize = 128;
pub const SIGMA_PROC_NAME_LEN: usize = 32;
pub const SIGMA_PROC_INVALID_PID: u32 = 0xFFFFFFFF;
pub const PAGE_SIZE: u64 = 4096;

pub const K_OK: i32 = 0;
pub const K_ERR_NOTFOUND: i32 = -1;
pub const K_ERR_PERM: i32 = -2;

#[derive(Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum ProcState {
    Created = 0,
    Ready = 1,
    Running = 2,
    Blocked = 3,
    Terminated = 4,
}

#[repr(C)]
pub struct ProcessControlBlock {
    pub pid: u32,
    pub parent_pid: u32,
    pub state: ProcState,
    pub priority: u8,
    pub is_kernel: bool,
    pub mem_allocated: u64,
    pub cpu_time_us: u64,
    pub start_tsc: u64,
    pub stack_base: u64,
    pub stack_size: u64,
    pub page_table_root: u64,
    pub name: [u8; SIGMA_PROC_NAME_LEN],
}

impl ProcessControlBlock {
    pub const fn empty() -> Self {
        Self {
            pid: SIGMA_PROC_INVALID_PID,
            parent_pid: 0,
            state: ProcState::Terminated,
            priority: 0,
            is_kernel: false,
            mem_allocated: 0,
            cpu_time_us: 0,
            start_tsc: 0,
            stack_base: 0,
            stack_size: 0,
            page_table_root: 0,
            name: [0; SIGMA_PROC_NAME_LEN],
        }
    }
}

// OOP Interface for managing lifecycles
pub trait ProcessOperations {
    fn create(&mut self, name: &str, priority: u8, is_kernel: bool) -> u32;
    fn kill(&mut self, pid: u32) -> i32;
    fn fork(&mut self, parent_pid: u32) -> u32;
}

pub struct ProcessManager {
    table: [ProcessControlBlock; SIGMA_MAX_PROCESSES],
    count: u32,
    next_pid: u32,
}

impl ProcessManager {
    pub const fn new() -> Self {
        Self {
            table: [ProcessControlBlock::empty(); SIGMA_MAX_PROCESSES],
            count: 0,
            next_pid: 1,
        }
    }

    fn alloc_pid(&mut self) -> u32 {
        let start = self.next_pid;
        loop {
            let slot = (self.next_pid as usize) % SIGMA_MAX_PROCESSES;
            if self.table[slot].state == ProcState::Terminated 
                || self.table[slot].pid == SIGMA_PROC_INVALID_PID 
            {
                let pid = self.next_pid;
                self.next_pid = self.next_pid.wrapping_add(1);
                return pid;
            }
            self.next_pid = self.next_pid.wrapping_add(1);
            if self.next_pid == start {
                break;
            }
        }
        SIGMA_PROC_INVALID_PID
    }

    fn find_pcb_mut(&mut self, pid: u32) -> Option<&mut ProcessControlBlock> {
        let slot = (pid as usize) % SIGMA_MAX_PROCESSES;
        let pcb = &mut self.table[slot];
        if pcb.pid == pid && pcb.state != ProcState::Terminated {
            Some(pcb)
        } else {
            None
        }
    }

    fn find_pcb(&self, pid: u32) -> Option<&ProcessControlBlock> {
        let slot = (pid as usize) % SIGMA_MAX_PROCESSES;
        let pcb = &self.table[slot];
        if pcb.pid == pid && pcb.state != ProcState::Terminated {
            Some(pcb)
        } else {
            None
        }
    }
}

// Global static instance (lock-free wrapper using UnsafeCell for bare-metal)
struct SafeProcessManager {
    inner: UnsafeCell<ProcessManager>,
}

unsafe impl Sync for SafeProcessManager {}

static PROCESS_MANAGER: SafeProcessManager = SafeProcessManager {
    inner: UnsafeCell::new(ProcessManager::new()),
};

extern "C" {
    fn cpu_rdtsc() -> u64;
    fn sigma_log(s: *const u8);
    fn sigma_log_info(fmt: *const u8, val1: u32, val2: *const u8, val3: u32, val4: u32);
}

// Helper to copy strings safely
fn copy_name(dest: &mut [u8; SIGMA_PROC_NAME_LEN], src: &str) {
    let bytes = src.as_bytes();
    let len = core::cmp::min(bytes.len(), SIGMA_PROC_NAME_LEN - 1);
    for i in 0..len {
        dest[i] = bytes[i];
    }
    for i in len..SIGMA_PROC_NAME_LEN {
        dest[i] = 0;
    }
}

#[no_mangle]
pub unsafe extern "C" fn process_manager_init() {
    let pm = &mut *PROCESS_MANAGER.inner.get();
    pm.count = 0;
    pm.next_pid = 1;

    // Create Idle Process (PID 0)
    let idle = &mut pm.table[0];
    idle.pid = 0;
    idle.parent_pid = 0;
    idle.state = ProcState::Running;
    idle.priority = 255;
    idle.is_kernel = true;
    idle.mem_allocated = 0;
    idle.cpu_time_us = 0;
    idle.start_tsc = cpu_rdtsc();
    copy_name(&mut idle.name, "sigma-idle");

    pm.count += 1;

    // Create Init Process (PID 1)
    process_create(b"sigma-init\0".as_ptr(), 0, true);

    sigma_log(b"[PROCMGR] Sovereign Process Manager initialized (Rust core).\n\0".as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn process_create(name_ptr: *const u8, priority: u8, is_kernel: bool) -> u32 {
    let pm = &mut *PROCESS_MANAGER.inner.get();
    if pm.count >= SIGMA_MAX_PROCESSES as u32 - 1 {
        return SIGMA_PROC_INVALID_PID;
    }

    let pid = pm.alloc_pid();
    if pid == SIGMA_PROC_INVALID_PID {
        return pid;
    }

    // Find length of string safely
    let mut len = 0;
    while *name_ptr.add(len) != 0 && len < 64 {
        len += 1;
    }
    let name_slice = core::slice::from_raw_parts(name_ptr, len);
    let name = core::str::from_utf8_unchecked(name_slice);

    let slot = (pid as usize) % SIGMA_MAX_PROCESSES;
    let pcb = &mut pm.table[slot];

    pcb.pid = pid;
    pcb.parent_pid = 1; // Default parent: init
    pcb.state = ProcState::Created;
    pcb.priority = priority;
    pcb.is_kernel = is_kernel;
    pcb.mem_allocated = PAGE_SIZE * 4;
    pcb.cpu_time_us = 0;
    pcb.start_tsc = cpu_rdtsc();
    pcb.stack_base = 0x00007FFF00000000u64 - (pid as u64 * PAGE_SIZE * 8);
    pcb.stack_size = PAGE_SIZE * 8;
    pcb.page_table_root = 0;
    copy_name(&mut pcb.name, name);

    pm.count += 1;
    pcb.state = ProcState::Ready;

    sigma_log_info(
        b"[PROCMGR] Created PID %u '%s' (priority=%u, kernel=%d)\n\0".as_ptr(),
        pid,
        name_ptr,
        priority as u32,
        is_kernel as u32,
    );

    pid
}

#[no_mangle]
pub unsafe extern "C" fn process_kill(pid: u32) -> i32 {
    let pm = &mut *PROCESS_MANAGER.inner.get();
    if pid == 0 {
        return K_ERR_PERM;
    }
    if let Some(pcb) = pm.find_pcb_mut(pid) {
        pcb.state = ProcState::Terminated;
        pm.count -= 1;
        K_OK
    } else {
        K_ERR_NOTFOUND
    }
}

#[no_mangle]
pub unsafe extern "C" fn process_set_state(pid: u32, new_state: ProcState) -> i32 {
    let pm = &mut *PROCESS_MANAGER.inner.get();
    if let Some(pcb) = pm.find_pcb_mut(pid) {
        pcb.state = new_state;
        K_OK
    } else {
        K_ERR_NOTFOUND
    }
}

#[no_mangle]
pub unsafe extern "C" fn process_set_priority(pid: u32, priority: u8) -> i32 {
    let pm = &mut *PROCESS_MANAGER.inner.get();
    if let Some(pcb) = pm.find_pcb_mut(pid) {
        pcb.priority = priority;
        K_OK
    } else {
        K_ERR_NOTFOUND
    }
}

#[no_mangle]
pub unsafe extern "C" fn process_getinfo(pid: u32) -> *const ProcessControlBlock {
    let pm = &*PROCESS_MANAGER.inner.get();
    if let Some(pcb) = pm.find_pcb(pid) {
        pcb as *const ProcessControlBlock
    } else {
        core::ptr::null()
    }
}

#[no_mangle]
pub unsafe extern "C" fn process_get_count() -> u32 {
    let pm = &*PROCESS_MANAGER.inner.get();
    pm.count
}

#[no_mangle]
pub unsafe extern "C" fn process_fork(parent_pid: u32) -> u32 {
    let pm = &mut *PROCESS_MANAGER.inner.get();
    let parent = match pm.find_pcb(parent_pid) {
        Some(p) => p,
        None => return SIGMA_PROC_INVALID_PID,
    };

    let child_pid = pm.alloc_pid();
    if child_pid == SIGMA_PROC_INVALID_PID {
        return child_pid;
    }

    let slot = (child_pid as usize) % SIGMA_MAX_PROCESSES;
    
    // Safety copy of parent values before mut borrow of child slot
    let parent_priority = parent.priority;
    let parent_is_kernel = parent.is_kernel;
    let parent_mem_allocated = parent.mem_allocated;
    let parent_page_table_root = parent.page_table_root;
    let mut parent_name = [0; SIGMA_PROC_NAME_LEN];
    parent_name.copy_from_slice(&parent.name);

    let child = &mut pm.table[slot];
    child.pid = child_pid;
    child.parent_pid = parent_pid;
    child.state = ProcState::Created;
    child.priority = parent_priority;
    child.is_kernel = parent_is_kernel;
    child.mem_allocated = parent_mem_allocated;
    child.cpu_time_us = 0;
    child.start_tsc = cpu_rdtsc();
    child.stack_base = 0x00007FFF00000000u64 - (child_pid as u64 * PAGE_SIZE * 8);
    child.stack_size = PAGE_SIZE * 8;
    child.page_table_root = parent_page_table_root;
    child.name = parent_name;

    pm.count += 1;
    child.state = ProcState::Ready;

    child_pid
}
