// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/security/sigma_capability.rs — Linux-41 capability model
//
// Implements fine-grained Linux-compatible capabilities (CAP_NET_ADMIN,
// CAP_SYS_BOOT, CAP_SYS_MODULE, etc.) layered on top of sigma_pledge.
//
// Language: Rust #![no_std]
#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU64, Ordering};

// ── Linux capability constants (man 7 capabilities) ───────────────────────
pub const CAP_CHOWN:          u64 = 1 << 0;
pub const CAP_DAC_OVERRIDE:   u64 = 1 << 1;
pub const CAP_DAC_READ_SEARCH:u64 = 1 << 2;
pub const CAP_FOWNER:         u64 = 1 << 3;
pub const CAP_FSETID:         u64 = 1 << 4;
pub const CAP_KILL:           u64 = 1 << 5;
pub const CAP_SETGID:         u64 = 1 << 6;
pub const CAP_SETUID:         u64 = 1 << 7;
pub const CAP_SETPCAP:        u64 = 1 << 8;
pub const CAP_LINUX_IMMUTABLE:u64 = 1 << 9;
pub const CAP_NET_BIND_SERVICE:u64= 1 << 10;
pub const CAP_NET_BROADCAST:  u64 = 1 << 11;
pub const CAP_NET_ADMIN:      u64 = 1 << 12;
pub const CAP_NET_RAW:        u64 = 1 << 13;
pub const CAP_IPC_LOCK:       u64 = 1 << 14;
pub const CAP_IPC_OWNER:      u64 = 1 << 15;
pub const CAP_SYS_MODULE:     u64 = 1 << 16;
pub const CAP_SYS_RAWIO:      u64 = 1 << 17;
pub const CAP_SYS_CHROOT:     u64 = 1 << 18;
pub const CAP_SYS_PTRACE:     u64 = 1 << 19;
pub const CAP_SYS_PACCT:      u64 = 1 << 20;
pub const CAP_SYS_ADMIN:      u64 = 1 << 21;
pub const CAP_SYS_BOOT:       u64 = 1 << 22;
pub const CAP_SYS_NICE:       u64 = 1 << 23;
pub const CAP_SYS_RESOURCE:   u64 = 1 << 24;
pub const CAP_SYS_TIME:       u64 = 1 << 25;
pub const CAP_SYS_TTY_CONFIG: u64 = 1 << 26;
pub const CAP_MKNOD:          u64 = 1 << 27;
pub const CAP_LEASE:          u64 = 1 << 28;
pub const CAP_AUDIT_WRITE:    u64 = 1 << 29;
pub const CAP_AUDIT_CONTROL:  u64 = 1 << 30;
pub const CAP_SETFCAP:        u64 = 1 << 31;
pub const CAP_MAC_OVERRIDE:   u64 = 1 << 32;
pub const CAP_MAC_ADMIN:      u64 = 1 << 33;
pub const CAP_SYSLOG:         u64 = 1 << 34;
pub const CAP_WAKE_ALARM:     u64 = 1 << 35;
pub const CAP_BLOCK_SUSPEND:  u64 = 1 << 36;
pub const CAP_AUDIT_READ:     u64 = 1 << 37;
pub const CAP_PERFMON:        u64 = 1 << 38;
pub const CAP_BPF:            u64 = 1 << 39;
pub const CAP_CHECKPOINT_RESTORE:u64 = 1 << 40;

pub const CAP_ALL: u64 = (1u64 << 41) - 1;

// ── Per-process capability set ────────────────────────────────────────────
#[derive(Copy, Clone, Default)]
pub struct CapabilitySet {
    pub permitted:   u64,   // caps that may be raised to effective
    pub effective:   u64,   // caps currently active for checks
    pub inheritable: u64,   // caps passed across execve
    pub bounding:    u64,   // hard upper limit; cannot be raised
    pub ambient:     u64,   // inherited by unprivileged child
}

impl CapabilitySet {
    pub fn root() -> Self {
        // Root starts with all capabilities
        Self {
            permitted:   CAP_ALL,
            effective:   CAP_ALL,
            inheritable: 0,
            bounding:    CAP_ALL,
            ambient:     0,
        }
    }

    pub fn nobody() -> Self {
        Self::default() // all zero
    }

    /// Check if a specific capability is in the effective set
    pub fn has(&self, cap: u64) -> bool {
        self.effective & cap != 0
    }

    /// Drop a capability from all sets (cannot be re-gained)
    pub fn drop(&mut self, cap: u64) {
        self.permitted   &= !cap;
        self.effective   &= !cap;
        self.inheritable &= !cap;
        self.bounding    &= !cap;
        self.ambient     &= !cap;
    }

    /// Raise a permitted cap to effective
    pub fn raise(&mut self, cap: u64) -> bool {
        if self.permitted & cap != cap { return false; } // not in permitted
        self.effective |= cap;
        true
    }

    /// Lower effective cap (without removing from permitted)
    pub fn lower(&mut self, cap: u64) {
        self.effective &= !cap;
    }
}

// ── Capability audit log ──────────────────────────────────────────────────
const CAP_LOG_SIZE: usize = 256;

#[derive(Copy, Clone)]
pub struct CapAuditEntry {
    pub timestamp_ns: u64,
    pub pid:          u32,
    pub cap:          u64,
    pub allowed:      bool,
    pub syscall_nr:   u32,
}

pub struct CapAuditLog {
    entries: [CapAuditEntry; CAP_LOG_SIZE],
    head:    usize,
    count:   u64,
}

impl CapAuditLog {
    pub const fn new() -> Self {
        Self {
            entries: [CapAuditEntry {
                timestamp_ns: 0, pid: 0, cap: 0,
                allowed: false, syscall_nr: 0,
            }; CAP_LOG_SIZE],
            head: 0,
            count: 0,
        }
    }

    pub unsafe fn record(&mut self, pid: u32, cap: u64, allowed: bool, syscall_nr: u32) {
        extern "C" { fn sigma_clock_ns() -> u64; }
        let idx = self.head % CAP_LOG_SIZE;
        self.entries[idx] = CapAuditEntry {
            timestamp_ns: sigma_clock_ns(),
            pid, cap, allowed, syscall_nr,
        };
        self.head = (self.head + 1) % CAP_LOG_SIZE;
        self.count += 1;
    }
}

// ── Per-process capability store (keyed by PID) ───────────────────────────
const MAX_CAP_ENTRIES: usize = 256;

struct CapEntry {
    pid:  u32,
    caps: CapabilitySet,
    active: bool,
}

pub struct CapabilityManager {
    entries: [CapEntry; MAX_CAP_ENTRIES],
    audit:   CapAuditLog,
}

impl CapabilityManager {
    pub const fn new() -> Self {
        Self {
            entries: [const { CapEntry { pid: 0, caps: CapabilitySet {
                permitted: 0, effective: 0,
                inheritable: 0, bounding: 0, ambient: 0,
            }, active: false } }; MAX_CAP_ENTRIES],
            audit: CapAuditLog::new(),
        }
    }

    pub fn init_process(&mut self, pid: u32, is_root: bool) {
        for e in &mut self.entries {
            if !e.active {
                e.pid    = pid;
                e.caps   = if is_root { CapabilitySet::root() } else { CapabilitySet::nobody() };
                e.active = true;
                return;
            }
        }
    }

    pub fn remove_process(&mut self, pid: u32) {
        for e in &mut self.entries {
            if e.active && e.pid == pid {
                e.active = false;
                return;
            }
        }
    }

    pub unsafe fn check(&mut self, pid: u32, cap: u64, syscall_nr: u32) -> bool {
        let allowed = self.entries.iter()
            .find(|e| e.active && e.pid == pid)
            .map(|e| e.caps.has(cap))
            .unwrap_or(false);
        self.audit.record(pid, cap, allowed, syscall_nr);
        allowed
    }

    pub fn capget(&self, pid: u32) -> Option<CapabilitySet> {
        self.entries.iter()
            .find(|e| e.active && e.pid == pid)
            .map(|e| e.caps)
    }

    pub fn capset(&mut self, pid: u32, new_caps: CapabilitySet) -> bool {
        for e in &mut self.entries {
            if e.active && e.pid == pid {
                // Can only reduce capabilities, never increase effective above permitted
                let new_eff = new_caps.effective & e.caps.permitted;
                e.caps.effective   = new_eff;
                e.caps.inheritable = new_caps.inheritable & e.caps.bounding;
                return true;
            }
        }
        false
    }

    pub fn drop_capability(&mut self, pid: u32, cap: u64) -> bool {
        for e in &mut self.entries {
            if e.active && e.pid == pid {
                e.caps.drop(cap);
                return true;
            }
        }
        false
    }

    pub fn audit_count(&self) -> u64 { self.audit.count }
}

static mut G_CAPMGR: CapabilityManager = CapabilityManager::new();

// ── C-ABI exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sigma_cap_init_process(pid: u32, is_root: bool) {
    G_CAPMGR.init_process(pid, is_root);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_cap_remove_process(pid: u32) {
    G_CAPMGR.remove_process(pid);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_cap_check(pid: u32, cap: u64, syscall_nr: u32) -> bool {
    G_CAPMGR.check(pid, cap, syscall_nr)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_capget(pid: u32, out: *mut CapabilitySet) -> i32 {
    if out.is_null() { return -14; }
    match G_CAPMGR.capget(pid) {
        Some(caps) => { *out = caps; 0 }
        None       => -3, // ESRCH
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_capset(pid: u32, new_caps: *const CapabilitySet) -> i32 {
    if new_caps.is_null() { return -14; }
    if G_CAPMGR.capset(pid, *new_caps) { 0 } else { -3 }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_cap_drop(pid: u32, cap: u64) -> i32 {
    if G_CAPMGR.drop_capability(pid, cap) { 0 } else { -3 }
}
