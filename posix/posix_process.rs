// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// posix/posix_process.rs — POSIX Process & Thread Model
//
// Implements POSIX process management: spawn, wait, exit, getpid, getppid
// Uses modern spawn abstraction instead of fork/exec.
// Integrates with SigmaOS's AI-orchestration for resource management.
//
// Language: Rust (no_std for kernel compatibility)

#![no_std]

use super::posix_base::{
    PosixProcess, ProcessState, set_errno_and_return, clear_errno,
    ESRCH, ECHILD, EPERM, EINVAL, ENOMEM,
};

type U8 = u8;
type U32 = u32;
type U64 = u64;
type I32 = i32;
type I64 = i64;
type Isize = isize;
type Usize = usize;

// ─── Process Exit Codes ─────────────────────────────────

pub const EXIT_SUCCESS: I32 = 0;
pub const EXIT_FAILURE: I32 = 1;

// ─── Wait Options ───────────────────────────────────────

pub const WNOHANG: I32 = 0x00000001;
pub const WUNTRACED: I32 = 0x00000002;
pub const WCONTINUED: I32 = 0x00000008;
pub const WEXITED: I32 = 0x00000004;
pub const WSTOPPED: I32 = 0x00000002;
pub const WNOWAIT: I32 = 0x01000000;

// ─── Wait Status Macros ─────────────────────────────────

pub const WIFEXITED(status: I32) -> bool {
    (status & 0x7F) == 0
}

pub const WEXITSTATUS(status: I32) -> I32 {
    (status >> 8) & 0xFF
}

pub const WIFSIGNALED(status: I32) -> bool {
    ((status & 0x7F) + 1) as I32 >= 2
}

pub const WTERMSIG(status: I32) -> I32 {
    status & 0x7F
}

pub const WIFSTOPPED(status: I32) -> bool {
    (status & 0xFF) == 0x7F
}

pub const WSTOPSIG(status: I32) -> I32 {
    (status >> 8) & 0xFF
}

// ─── Spawn Options ───────────────────────────────────────

#[repr(C)]
pub struct SpawnOptions {
    pub path: *const U8,
    pub argv: *const *const U8,
    pub envp: *const *const U8,
    pub stdin_fd: I32,
    pub stdout_fd: I32,
    pub stderr_fd: I32,
    pub working_dir: *const U8,
    pub uid: U32,
    pub gid: U32,
}

impl SpawnOptions {
    pub const fn new() -> Self {
        SpawnOptions {
            path: 0 as *const U8,
            argv: 0 as *const *const U8,
            envp: 0 as *const *const U8,
            stdin_fd: 0,
            stdout_fd: 1,
            stderr_fd: 2,
            working_dir: 0 as *const U8,
            uid: 0,
            gid: 0,
        }
    }
}

// ─── Process Table ─────────────────────────────────────

pub const MAX_PROCESSES: Usize = 1024;

pub struct ProcessTable {
    pub processes: [PosixProcess; MAX_PROCESSES],
    pub next_pid: I32,
    pub current_pid: I32,
}

impl ProcessTable {
    pub const fn new() -> Self {
        ProcessTable {
            processes: [PosixProcess::new(); MAX_PROCESSES],
            next_pid: 1,
            current_pid: 0,
        }
    }

    pub fn allocate_pid(&mut self) -> I32 {
        if self.next_pid >= MAX_PROCESSES as I32 {
            unsafe { set_errno_and_return(ENOMEM) };
            return -1;
        }

        let pid = self.next_pid;
        self.next_pid += 1;
        pid
    }

    pub fn get_process(&mut self, pid: I32) -> Option<&mut PosixProcess> {
        if pid > 0 && (pid as Usize) < MAX_PROCESSES {
            Some(&mut self.processes[pid as Usize])
        } else {
            None
        }
    }

    pub fn set_current_pid(&mut self, pid: I32) {
        self.current_pid = pid;
    }
}

// ─── Global Process Table ─────────────────────────────

static mut PROCESS_TABLE: ProcessTable = ProcessTable::new();

// ─── SigmaOS Process Operations (stubs) ───────────────

// These would call into SigmaOS's process manager
// For now, we provide stub implementations

unsafe fn sigma_process_spawn(options: &SpawnOptions) -> Result<U64, I32> {
    // Stub: In real implementation, this would call SigmaOS process manager
    Ok(1) // Return a handle
}

unsafe fn sigma_process_wait(pid: I32, status: *mut I32, options: I32) -> Result<I32, I32> {
    // Stub: In real implementation, this would call SigmaOS process manager
    Ok(pid)
}

unsafe fn sigma_process_exit(code: I32) -> ! {
    // Stub: In real implementation, this would call SigmaOS process manager
    loop {}
}

unsafe fn sigma_process_getpid() -> I32 {
    // Stub: In real implementation, this would call SigmaOS process manager
    1
}

unsafe fn sigma_process_getppid() -> I32 {
    // Stub: In real implementation, this would call SigmaOS process manager
    0
}

unsafe fn sigma_process_kill(pid: I32, signal: I32) -> Result<(), I32> {
    // Stub: In real implementation, this would call SigmaOS process manager
    Ok(())
}

// ─── POSIX spawn() ─────────────────────────────────────

/// Spawn a new process (modern alternative to fork/exec)
#[no_mangle]
pub unsafe extern "C" fn posix_spawn(options: *const SpawnOptions) -> I32 {
    clear_errno();

    if options.is_null() {
        return set_errno_and_return(EINVAL);
    }

    let opts = &*options;

    if opts.path.is_null() {
        return set_errno_and_return(EINVAL);
    }

    // Call SigmaOS process spawn
    match sigma_process_spawn(opts) {
        Ok(handle) => {
            // Allocate PID
            let process_table = &mut PROCESS_TABLE;
            let pid = process_table.allocate_pid();
            
            if pid < 0 {
                return pid;
            }

            // Set up process
            if let Some(proc) = process_table.get_process(pid) {
                proc.pid = pid;
                proc.sigma_process.handle = handle;
                proc.parent_pid = process_table.current_pid;
                proc.state = ProcessState::Running;
                proc.exit_code = 0;
            }

            pid
        }
        Err(e) => set_errno_and_return(e),
    }
}

// ─── POSIX wait() ──────────────────────────────────────

/// Wait for child process to change state
#[no_mangle]
pub unsafe extern "C" fn posix_wait(status: *mut I32) -> I32 {
    posix_waitpid(-1, status, 0)
}

// ─── POSIX waitpid() ───────────────────────────────────

/// Wait for specific child process
#[no_mangle]
pub unsafe extern "C" fn posix_waitpid(pid: I32, status: *mut I32, options: I32) -> I32 {
    clear_errno();

    // Call SigmaOS process wait
    match sigma_process_wait(pid, status, options) {
        Ok(waited_pid) => {
            // Update process table
            let process_table = &mut PROCESS_TABLE;
            if let Some(proc) = process_table.get_process(waited_pid) {
                proc.state = ProcessState::Zombie;
                if !status.is_null() {
                    *status = proc.exit_code;
                }
            }
            waited_pid
        }
        Err(e) => set_errno_and_return(e),
    }
}

// ─── POSIX exit() ──────────────────────────────────────

/// Terminate current process
#[no_mangle]
pub unsafe extern "C" fn posix_exit(code: I32) -> ! {
    let process_table = &mut PROCESS_TABLE;
    
    // Update current process state
    if let Some(proc) = process_table.get_process(process_table.current_pid) {
        proc.state = ProcessState::Dead;
        proc.exit_code = code;
    }

    sigma_process_exit(code)
}

// ─── POSIX getpid() ────────────────────────────────────

/// Get current process ID
#[no_mangle]
pub unsafe extern "C" fn posix_getpid() -> I32 {
    sigma_process_getpid()
}

// ─── POSIX getppid() ───────────────────────────────────

/// Get parent process ID
#[no_mangle]
pub unsafe extern "C" fn posix_getppid() -> I32 {
    sigma_process_getppid()
}

// ─── POSIX kill() ──────────────────────────────────────

/// Send signal to process
#[no_mangle]
pub unsafe extern "C" fn posix_kill(pid: I32, signal: I32) -> I32 {
    clear_errno();

    if pid <= 0 {
        return set_errno_and_return(ESRCH);
    }

    // Call SigmaOS process kill
    match sigma_process_kill(pid, signal) {
        Ok(()) => 0,
        Err(e) => set_errno_and_return(e),
    }
}

// ─── POSIX raise() ─────────────────────────────────────

/// Send signal to current process
#[no_mangle]
pub unsafe extern "C" fn posix_raise(signal: I32) -> I32 {
    clear_errno();

    let pid = posix_getpid();
    posix_kill(pid, signal)
}

// ─── POSIX abort() ─────────────────────────────────────

/// Abort current process with SIGABRT
#[no_mangle]
pub unsafe extern "C" fn posix_abort() -> ! {
    posix_raise(6); // SIGABRT
    posix_exit(134)
}

// ─── POSIX getpid() wrapper for C compatibility ───────

#[no_mangle]
pub extern "C" fn getpid() -> I32 {
    unsafe { posix_getpid() }
}

// ─── POSIX getppid() wrapper for C compatibility ───────

#[no_mangle]
pub extern "C" fn getppid() -> I32 {
    unsafe { posix_getppid() }
}

// ─── POSIX exit() wrapper for C compatibility ───────────

#[no_mangle]
pub extern "C" fn exit(code: I32) -> ! {
    unsafe { posix_exit(code) }
}

// ─── POSIX kill() wrapper for C compatibility ─────────

#[no_mangle]
pub extern "C" fn kill(pid: I32, signal: I32) -> I32 {
    unsafe { posix_kill(pid, signal) }
}
