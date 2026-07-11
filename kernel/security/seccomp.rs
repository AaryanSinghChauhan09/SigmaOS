// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/security/seccomp.rs — Seccomp (Secure Computing Mode) Integration
//
// This module implements seccomp (secure computing mode) inspired by Linux seccomp.
// It allows processes to restrict themselves to a limited set of system calls,
// providing a security sandbox mechanism.
//
// Key features:
// - System call filtering with BPF-like rules
// - Multiple seccomp modes (strict, filter)
// - Per-process seccomp state
// - OOP principles with filter traits
// - No external dependencies

#![no_std]
#![allow(dead_code)]

// ─────────────────────────────────────────────────────────────────────────────
// Seccomp Modes
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SeccompMode {
    Disabled,   // No seccomp filtering
    Strict,     // Only allow read, write, exit, sigreturn
    Filter,     // Use BPF filter rules
}

// ─────────────────────────────────────────────────────────────────────────────
// System Call Numbers (simplified subset)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u32)]
pub enum SyscallNumber {
    Read = 0,
    Write = 1,
    Open = 2,
    Close = 3,
    Stat = 4,
    Fstat = 5,
    Lstat = 6,
    Poll = 7,
    Lseek = 8,
    Mmap = 9,
    Mprotect = 10,
    Munmap = 11,
    Brk = 12,
    RtSigaction = 13,
    RtSigprocmask = 14,
    RtSigreturn = 15,
    Ioctl = 16,
    Pread64 = 17,
    Pwrite64 = 18,
    Readv = 19,
    Writev = 20,
    Access = 21,
    Pipe = 22,
    Select = 23,
    SchedYield = 24,
    Mremap = 25,
    Msync = 26,
    Mincore = 27,
    Madvise = 28,
    Shmget = 29,
    Shmat = 30,
    Shmctl = 31,
    Dup = 32,
    Dup2 = 33,
    Pause = 34,
    Nanosleep = 35,
    Getitimer = 36,
    Alarm = 37,
    Setitimer = 38,
    Getpid = 39,
    Sendfile = 40,
    Socket = 41,
    Connect = 42,
    Accept = 43,
    Sendto = 44,
    Recvfrom = 45,
    Sendmsg = 46,
    Recvmsg = 47,
    Shutdown = 48,
    Bind = 49,
    Listen = 50,
    Getsockname = 51,
    Getpeername = 52,
    Socketpair = 53,
    Setsockopt = 54,
    Getsockopt = 55,
    Clone = 56,
    Fork = 57,
    Vfork = 58,
    Execve = 59,
    Exit = 60,
    Wait4 = 61,
    Kill = 62,
    Uname = 63,
}

// ─────────────────────────────────────────────────────────────────────────────
// Filter Operation Codes
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u16)]
pub enum FilterOp {
    Kill = 0,         // Kill process
    Trap = 1,         // Trap process
    Errno = 2,        // Return errno
    Trace = 3,        // Trace process
    Allow = 4,        // Allow syscall
    Log = 5,          // Log syscall
}

// ─────────────────────────────────────────────────────────────────────────────
// Filter Rule
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Copy, Clone, Debug)]
pub struct FilterRule {
    pub syscall: SyscallNumber,
    pub action: FilterOp,
    pub errno: u32,    // For Errno action
}

impl FilterRule {
    pub const fn empty() -> Self {
        Self {
            syscall: SyscallNumber::Read,
            action: FilterOp::Allow,
            errno: 0,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Seccomp Filter Trait (OOP Principles)
// ─────────────────────────────────────────────────────────────────────────────

pub trait SeccompFilter {
    fn check_syscall(&self, syscall: SyscallNumber) -> FilterOp;
    fn add_rule(&mut self, rule: FilterRule) -> bool;
    fn remove_rule(&mut self, syscall: SyscallNumber) -> bool;
    fn get_rules(&self) -> &[FilterRule];
    fn set_mode(&mut self, mode: SeccompMode);
    fn get_mode(&self) -> SeccompMode;
}

// ─────────────────────────────────────────────────────────────────────────────
// Default Seccomp Filter Implementation
// ─────────────────────────────────────────────────────────────────────────────

pub struct DefaultSeccompFilter {
    mode: SeccompMode,
    rules: [FilterRule; 256],
    num_rules: usize,
}

impl DefaultSeccompFilter {
    pub const fn new() -> Self {
        Self {
            mode: SeccompMode::Disabled,
            rules: [FilterRule::empty(); 256],
            num_rules: 0,
        }
    }

    fn find_rule(&self, syscall: SyscallNumber) -> Option<usize> {
        for i in 0..self.num_rules {
            if self.rules[i].syscall == syscall {
                return Some(i);
            }
        }
        None
    }
}

impl SeccompFilter for DefaultSeccompFilter {
    fn check_syscall(&self, syscall: SyscallNumber) -> FilterOp {
        match self.mode {
            SeccompMode::Disabled => FilterOp::Allow,
            SeccompMode::Strict => {
                // Strict mode: only allow read, write, exit, sigreturn
                match syscall {
                    SyscallNumber::Read |
                    SyscallNumber::Write |
                    SyscallNumber::Exit |
                    SyscallNumber::RtSigreturn => FilterOp::Allow,
                    _ => FilterOp::Kill,
                }
            }
            SeccompMode::Filter => {
                // Filter mode: check rules
                if let Some(idx) = self.find_rule(syscall) {
                    self.rules[idx].action
                } else {
                    // Default to deny
                    FilterOp::Kill
                }
            }
        }
    }

    fn add_rule(&mut self, rule: FilterRule) -> bool {
        if self.num_rules >= 256 { return false; }
        
        // Check for duplicate
        if let Some(_) = self.find_rule(rule.syscall) {
            return false;
        }
        
        self.rules[self.num_rules] = rule;
        self.num_rules += 1;
        true
    }

    fn remove_rule(&mut self, syscall: SyscallNumber) -> bool {
        if let Some(idx) = self.find_rule(syscall) {
            // Shift remaining rules
            for i in idx..self.num_rules - 1 {
                self.rules[i] = self.rules[i + 1];
            }
            self.num_rules -= 1;
            true
        } else {
            false
        }
    }

    fn get_rules(&self) -> &[FilterRule] {
        &self.rules[..self.num_rules]
    }

    fn set_mode(&mut self, mode: SeccompMode) {
        self.mode = mode;
    }

    fn get_mode(&self) -> SeccompMode {
        self.mode
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Per-Process Seccomp State
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Copy, Clone, Debug)]
pub struct ProcessSeccompState {
    pub mode: SeccompMode,
    pub filter: Option<DefaultSeccompFilter>,
}

impl ProcessSeccompState {
    pub const fn new() -> Self {
        Self {
            mode: SeccompMode::Disabled,
            filter: None,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Seccomp Manager
// ─────────────────────────────────────────────────────────────────────────────

pub struct SeccompManager {
    process_states: [ProcessSeccompState; 1024],
    next_pid: u32,
}

impl SeccompManager {
    pub const fn new() -> Self {
        Self {
            process_states: [ProcessSeccompState::new(); 1024],
            next_pid: 1,
        }
    }

    // Initialize seccomp for a process
    pub fn init_process(&mut self, pid: u32) -> bool {
        if pid == 0 || pid >= 1024 { return false; }
        self.process_states[pid as usize] = ProcessSeccompState::new();
        true
    }

    // Set seccomp mode for a process
    pub fn set_mode(&mut self, pid: u32, mode: SeccompMode) -> bool {
        if pid == 0 || pid >= 1024 { return false; }
        
        let state = &mut self.process_states[pid as usize];
        
        // Once set to strict or filter, cannot go back to disabled
        if state.mode != SeccompMode::Disabled && mode == SeccompMode::Disabled {
            return false;
        }
        
        state.mode = mode;
        
        // Initialize filter if switching to filter mode
        if mode == SeccompMode::Filter && state.filter.is_none() {
            state.filter = Some(DefaultSeccompFilter::new());
        }
        
        true
    }

    // Add a filter rule for a process
    pub fn add_rule(&mut self, pid: u32, rule: FilterRule) -> bool {
        if pid == 0 || pid >= 1024 { return false; }
        
        let state = &mut self.process_states[pid as usize];
        
        if state.mode != SeccompMode::Filter {
            return false;
        }
        
        if let Some(ref mut filter) = state.filter {
            filter.add_rule(rule)
        } else {
            false
        }
    }

    // Remove a filter rule for a process
    pub fn remove_rule(&mut self, pid: u32, syscall: SyscallNumber) -> bool {
        if pid == 0 || pid >= 1024 { return false; }
        
        let state = &mut self.process_states[pid as usize];
        
        if state.mode != SeccompMode::Filter {
            return false;
        }
        
        if let Some(ref mut filter) = state.filter {
            filter.remove_rule(syscall)
        } else {
            false
        }
    }

    // Check if a syscall is allowed for a process
    pub fn check_syscall(&self, pid: u32, syscall: SyscallNumber) -> FilterOp {
        if pid == 0 || pid >= 1024 { return FilterOp::Allow; }
        
        let state = &self.process_states[pid as usize];
        
        match state.mode {
            SeccompMode::Disabled => FilterOp::Allow,
            SeccompMode::Strict => {
                match syscall {
                    SyscallNumber::Read |
                    SyscallNumber::Write |
                    SyscallNumber::Exit |
                    SyscallNumber::RtSigreturn => FilterOp::Allow,
                    _ => FilterOp::Kill,
                }
            }
            SeccompMode::Filter => {
                if let Some(ref filter) = state.filter {
                    filter.check_syscall(syscall)
                } else {
                    FilterOp::Kill
                }
            }
        }
    }

    // Get process seccomp mode
    pub fn get_mode(&self, pid: u32) -> SeccompMode {
        if pid == 0 || pid >= 1024 { return SeccompMode::Disabled; }
        self.process_states[pid as usize].mode
    }

    // Get process filter rules
    pub fn get_rules(&self, pid: u32) -> Option<&[FilterRule]> {
        if pid == 0 || pid >= 1024 { return None; }
        
        let state = &self.process_states[pid as usize];
        
        if let Some(ref filter) = state.filter {
            Some(filter.get_rules())
        } else {
            None
        }
    }

    // Clean up process state
    pub fn cleanup_process(&mut self, pid: u32) -> bool {
        if pid == 0 || pid >= 1024 { return false; }
        self.process_states[pid as usize] = ProcessSeccompState::new();
        true
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Global singleton
// ─────────────────────────────────────────────────────────────────────────────

static mut SECCOMP_MANAGER: SeccompManager = SeccompManager::new();

#[no_mangle]
pub unsafe extern "C" fn sigma_seccomp_init() {
    SECCOMP_MANAGER = SeccompManager::new();
}

#[no_mangle]
pub unsafe extern "C" fn sigma_seccomp_init_process(pid: u32) -> bool {
    SECCOMP_MANAGER.init_process(pid)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_seccomp_set_mode(pid: u32, mode: u8) -> bool {
    let mode = match mode {
        0 => SeccompMode::Disabled,
        1 => SeccompMode::Strict,
        2 => SeccompMode::Filter,
        _ => return false,
    };
    SECCOMP_MANAGER.set_mode(pid, mode)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_seccomp_add_rule(pid: u32, syscall: u32, action: u16, errno: u32) -> bool {
    let syscall = match syscall {
        0 => SyscallNumber::Read,
        1 => SyscallNumber::Write,
        2 => SyscallNumber::Open,
        3 => SyscallNumber::Close,
        4 => SyscallNumber::Stat,
        5 => SyscallNumber::Fstat,
        6 => SyscallNumber::Lstat,
        7 => SyscallNumber::Poll,
        8 => SyscallNumber::Lseek,
        9 => SyscallNumber::Mmap,
        10 => SyscallNumber::Mprotect,
        11 => SyscallNumber::Munmap,
        12 => SyscallNumber::Brk,
        13 => SyscallNumber::RtSigaction,
        14 => SyscallNumber::RtSigprocmask,
        15 => SyscallNumber::RtSigreturn,
        16 => SyscallNumber::Ioctl,
        17 => SyscallNumber::Pread64,
        18 => SyscallNumber::Pwrite64,
        19 => SyscallNumber::Readv,
        20 => SyscallNumber::Writev,
        21 => SyscallNumber::Access,
        22 => SyscallNumber::Pipe,
        23 => SyscallNumber::Select,
        24 => SyscallNumber::SchedYield,
        25 => SyscallNumber::Mremap,
        26 => SyscallNumber::Msync,
        27 => SyscallNumber::Mincore,
        28 => SyscallNumber::Madvise,
        29 => SyscallNumber::Shmget,
        30 => SyscallNumber::Shmat,
        31 => SyscallNumber::Shmctl,
        32 => SyscallNumber::Dup,
        33 => SyscallNumber::Dup2,
        34 => SyscallNumber::Pause,
        35 => SyscallNumber::Nanosleep,
        36 => SyscallNumber::Getitimer,
        37 => SyscallNumber::Alarm,
        38 => SyscallNumber::Setitimer,
        39 => SyscallNumber::Getpid,
        40 => SyscallNumber::Sendfile,
        41 => SyscallNumber::Socket,
        42 => SyscallNumber::Connect,
        43 => SyscallNumber::Accept,
        44 => SyscallNumber::Sendto,
        45 => SyscallNumber::Recvfrom,
        46 => SyscallNumber::Sendmsg,
        47 => SyscallNumber::Recvmsg,
        48 => SyscallNumber::Shutdown,
        49 => SyscallNumber::Bind,
        50 => SyscallNumber::Listen,
        51 => SyscallNumber::Getsockname,
        52 => SyscallNumber::Getpeername,
        53 => SyscallNumber::Socketpair,
        54 => SyscallNumber::Setsockopt,
        55 => SyscallNumber::Getsockopt,
        56 => SyscallNumber::Clone,
        57 => SyscallNumber::Fork,
        58 => SyscallNumber::Vfork,
        59 => SyscallNumber::Execve,
        60 => SyscallNumber::Exit,
        61 => SyscallNumber::Wait4,
        62 => SyscallNumber::Kill,
        63 => SyscallNumber::Uname,
        _ => return false,
    };
    
    let action = match action {
        0 => FilterOp::Kill,
        1 => FilterOp::Trap,
        2 => FilterOp::Errno,
        3 => FilterOp::Trace,
        4 => FilterOp::Allow,
        5 => FilterOp::Log,
        _ => return false,
    };
    
    let rule = FilterRule {
        syscall,
        action,
        errno,
    };
    
    SECCOMP_MANAGER.add_rule(pid, rule)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_seccomp_check_syscall(pid: u32, syscall: u32) -> u16 {
    let syscall = match syscall {
        0 => SyscallNumber::Read,
        1 => SyscallNumber::Write,
        2 => SyscallNumber::Open,
        3 => SyscallNumber::Close,
        4 => SyscallNumber::Stat,
        5 => SyscallNumber::Fstat,
        6 => SyscallNumber::Lstat,
        7 => SyscallNumber::Poll,
        8 => SyscallNumber::Lseek,
        9 => SyscallNumber::Mmap,
        10 => SyscallNumber::Mprotect,
        11 => SyscallNumber::Munmap,
        12 => SyscallNumber::Brk,
        13 => SyscallNumber::RtSigaction,
        14 => SyscallNumber::RtSigprocmask,
        15 => SyscallNumber::RtSigreturn,
        16 => SyscallNumber::Ioctl,
        17 => SyscallNumber::Pread64,
        18 => SyscallNumber::Pwrite64,
        19 => SyscallNumber::Readv,
        20 => SyscallNumber::Writev,
        21 => SyscallNumber::Access,
        22 => SyscallNumber::Pipe,
        23 => SyscallNumber::Select,
        24 => SyscallNumber::SchedYield,
        25 => SyscallNumber::Mremap,
        26 => SyscallNumber::Msync,
        27 => SyscallNumber::Mincore,
        28 => SyscallNumber::Madvise,
        29 => SyscallNumber::Shmget,
        30 => SyscallNumber::Shmat,
        31 => SyscallNumber::Shmctl,
        32 => SyscallNumber::Dup,
        33 => SyscallNumber::Dup2,
        34 => SyscallNumber::Pause,
        35 => SyscallNumber::Nanosleep,
        36 => SyscallNumber::Getitimer,
        37 => SyscallNumber::Alarm,
        38 => SyscallNumber::Setitimer,
        39 => SyscallNumber::Getpid,
        40 => SyscallNumber::Sendfile,
        41 => SyscallNumber::Socket,
        42 => SyscallNumber::Connect,
        43 => SyscallNumber::Accept,
        44 => SyscallNumber::Sendto,
        45 => SyscallNumber::Recvfrom,
        46 => SyscallNumber::Sendmsg,
        47 => SyscallNumber::Recvmsg,
        48 => SyscallNumber::Shutdown,
        49 => SyscallNumber::Bind,
        50 => SyscallNumber::Listen,
        51 => SyscallNumber::Getsockname,
        52 => SyscallNumber::Getpeername,
        53 => SyscallNumber::Socketpair,
        54 => SyscallNumber::Setsockopt,
        55 => SyscallNumber::Getsockopt,
        56 => SyscallNumber::Clone,
        57 => SyscallNumber::Fork,
        58 => SyscallNumber::Vfork,
        59 => SyscallNumber::Execve,
        60 => SyscallNumber::Exit,
        61 => SyscallNumber::Wait4,
        62 => SyscallNumber::Kill,
        63 => SyscallNumber::Uname,
        _ => return 0, // Allow by default for unknown syscalls
    };
    
    match SECCOMP_MANAGER.check_syscall(pid, syscall) {
        FilterOp::Kill => 0,
        FilterOp::Trap => 1,
        FilterOp::Errno => 2,
        FilterOp::Trace => 3,
        FilterOp::Allow => 4,
        FilterOp::Log => 5,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_seccomp_get_mode(pid: u32) -> u8 {
    match SECCOMP_MANAGER.get_mode(pid) {
        SeccompMode::Disabled => 0,
        SeccompMode::Strict => 1,
        SeccompMode::Filter => 2,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_seccomp_cleanup_process(pid: u32) -> bool {
    SECCOMP_MANAGER.cleanup_process(pid)
}
