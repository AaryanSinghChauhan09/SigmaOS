// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/security/sigma_pledge.rs — sigma_pledge + sigma_unveil
// Replaces: SovereignPledge.cpp, SovereignUnveil.cpp (C++ stubs, removed)
//
// Implements: OpenBSD-inspired capability restriction
// Language: Rust #![no_std]
// Pattern: OOP via PledgePolicy / UnveilPolicy structs + Enforcer trait

#![no_std]

use core::sync::atomic::{AtomicU64, Ordering};

// ── Capability Bits (pledge allowlist) ───────────────────────────────────────

/// Each bit represents a category of allowed syscalls
#[repr(u64)]
pub enum Cap {
    /// Basic I/O: read, write, close
    Stdio    = 1 << 0,
    /// Filesystem read: open, stat, getdents
    RPath    = 1 << 1,
    /// Filesystem write: open(O_WRONLY), mkdir, unlink
    WPath    = 1 << 2,
    /// Create files: open(O_CREAT), rename
    CPath    = 1 << 3,
    /// DNS: connect to port 53 only
    Dns      = 1 << 4,
    /// Network: connect, bind, send, recv
    Inet     = 1 << 5,
    /// Process: fork, exec, wait
    Proc     = 1 << 6,
    /// Signals: kill, signal
    Signal   = 1 << 7,
    /// Memory mapping: mmap, mprotect
    Prot     = 1 << 8,
    /// Timing: clock_gettime, nanosleep
    Clock    = 1 << 9,
    /// sysctl read
    Sysctl   = 1 << 10,
    /// TTY operations: ioctl on TTY fds
    Tty      = 1 << 11,
    /// sigma_attest: PQC attestation
    Attest   = 1 << 12,
    /// GPU/display operations
    Video    = 1 << 13,
    /// Audio operations
    Audio    = 1 << 14,
    /// All capabilities (initial default, then restricted)
    All      = u64::MAX,
}

// ── PledgePolicy ─────────────────────────────────────────────────────────────

/// Per-process pledge state. Once restricted, cannot be expanded.
#[derive(Clone, Copy)]
pub struct PledgePolicy {
    pub allowed: u64, // bitmask of allowed Cap bits
    pub locked:  bool, // true = policy is immutable
}

impl PledgePolicy {
    pub const fn new_unrestricted() -> Self {
        Self { allowed: Cap::All as u64, locked: false }
    }

    /// Apply a new (more restrictive) pledge. Returns Err if expanding.
    pub fn apply(&mut self, new_caps: u64) -> Result<(), PledgeError> {
        if self.locked {
            return Err(PledgeError::AlreadyLocked);
        }
        // Can only restrict, never expand
        if new_caps & !self.allowed != 0 {
            return Err(PledgeError::CannotExpand);
        }
        self.allowed = new_caps;
        self.locked  = true; // pledge is now locked
        Ok(())
    }

    /// Check if a specific capability is allowed
    #[inline(always)]
    pub fn allows(&self, cap: Cap) -> bool {
        self.allowed & (cap as u64) != 0
    }
}

#[derive(Debug)]
pub enum PledgeError {
    AlreadyLocked,
    CannotExpand,
    InvalidCaps,
}

// ── UnveilPolicy ─────────────────────────────────────────────────────────────

const MAX_UNVEIL_ENTRIES: usize = 32;
const MAX_PATH_LEN:       usize = 256;

#[derive(Clone, Copy)]
pub struct UnveilEntry {
    pub path:    [u8; MAX_PATH_LEN],
    pub path_len: usize,
    pub perms:   UnveilPerms,
}

#[derive(Clone, Copy)]
pub struct UnveilPerms {
    pub read:    bool,
    pub write:   bool,
    pub create:  bool,
    pub execute: bool,
}

impl UnveilPerms {
    pub fn from_str(s: &[u8]) -> Self {
        let mut p = Self { read: false, write: false, create: false, execute: false };
        for &b in s {
            match b {
                b'r' => p.read    = true,
                b'w' => p.write   = true,
                b'c' => p.create  = true,
                b'x' => p.execute = true,
                _    => {}
            }
        }
        p
    }
}

pub struct UnveilPolicy {
    entries:  [Option<UnveilEntry>; MAX_UNVEIL_ENTRIES],
    count:    usize,
    locked:   bool,
}

impl UnveilPolicy {
    pub const fn new() -> Self {
        Self {
            entries: [const { None }; MAX_UNVEIL_ENTRIES],
            count:   0,
            locked:  false,
        }
    }

    pub fn unveil(&mut self, path: &[u8], perms: &[u8]) -> Result<(), UnveilError> {
        if self.locked { return Err(UnveilError::Locked); }
        if self.count >= MAX_UNVEIL_ENTRIES { return Err(UnveilError::TooManyEntries); }
        if path.len() > MAX_PATH_LEN       { return Err(UnveilError::PathTooLong); }

        let mut entry = UnveilEntry {
            path:     [0u8; MAX_PATH_LEN],
            path_len: path.len(),
            perms:    UnveilPerms::from_str(perms),
        };
        entry.path[..path.len()].copy_from_slice(path);
        self.entries[self.count] = Some(entry);
        self.count += 1;
        Ok(())
    }

    /// Lock unveil — after this, no filesystem access outside listed paths
    pub fn lock(&mut self) { self.locked = true; }

    /// Check if a path operation is permitted
    pub fn check(&self, path: &[u8], op: UnveilOp) -> bool {
        if !self.locked { return true; } // not yet restricted
        for i in 0..self.count {
            if let Some(ref e) = self.entries[i] {
                if path.starts_with(&e.path[..e.path_len]) {
                    return match op {
                        UnveilOp::Read    => e.perms.read,
                        UnveilOp::Write   => e.perms.write,
                        UnveilOp::Create  => e.perms.create,
                        UnveilOp::Execute => e.perms.execute,
                    };
                }
            }
        }
        false // deny by default once locked
    }
}

#[derive(Debug)]
pub enum UnveilError {
    Locked,
    TooManyEntries,
    PathTooLong,
}

pub enum UnveilOp { Read, Write, Create, Execute }

// ── Enforcer Trait (OOP) ─────────────────────────────────────────────────────

pub trait SecurityEnforcer {
    fn check_syscall(&self, pid: u32, syscall_nr: u32) -> bool;
    fn check_path(&self, pid: u32, path: &[u8], op: UnveilOp) -> bool;
}
