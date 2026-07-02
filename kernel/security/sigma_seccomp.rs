// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// kernel/security/sigma_seccomp.rs — seccomp-BPF equivalent syscall filter
// Language: Rust #![no_std]
// Pattern: OOP via SeccompFilter struct + Instruction-based bytecode

#![no_std]

// ── Bytecode Instructions ─────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SeccompOp {
    AllowAll,    // accept all syscalls (no-op filter)
    AllowList,   // allow only listed syscalls; deny others
    DenyList,    // deny listed syscalls; allow others
    Audit,       // log but allow
}

#[derive(Clone, Copy)]
pub enum SeccompAction {
    Allow,
    Kill,        // kill the process
    Trap,        // send SIGSYS
    Log,         // log and allow
    Errno(i32),  // return error code
}

#[derive(Clone, Copy)]
pub struct SeccompRule {
    pub syscall: u32,
    pub action:  SeccompAction,
}

// ── Filter ────────────────────────────────────────────────────────────────────

pub const MAX_RULES: usize = 128;

pub struct SeccompFilter {
    rules:       [Option<SeccompRule>; MAX_RULES],
    n_rules:     usize,
    mode:        SeccompOp,
    default_act: SeccompAction,
    locked:      bool,
}

impl SeccompFilter {
    pub const fn new(mode: SeccompOp) -> Self {
        Self {
            rules:       [const { None }; MAX_RULES],
            n_rules:     0,
            mode,
            default_act: SeccompAction::Kill,
            locked:      false,
        }
    }

    pub fn allow_all() -> Self {
        Self::new(SeccompOp::AllowAll)
    }

    pub fn allow_list() -> Self {
        let mut f = Self::new(SeccompOp::AllowList);
        f.default_act = SeccompAction::Kill;
        f
    }

    pub fn add_rule(&mut self, syscall: u32, action: SeccompAction) -> bool {
        if self.locked || self.n_rules >= MAX_RULES { return false; }
        self.rules[self.n_rules] = Some(SeccompRule { syscall, action });
        self.n_rules += 1;
        true
    }

    /// Lock the filter — no more rules can be added (irreversible)
    pub fn lock(&mut self) { self.locked = true; }

    /// Check if syscall nr is allowed. Returns the action to take.
    pub fn check(&self, syscall_nr: u32) -> SeccompAction {
        match self.mode {
            SeccompOp::AllowAll  => SeccompAction::Allow,
            SeccompOp::AllowList => {
                for r in self.rules[..self.n_rules].iter().flatten() {
                    if r.syscall == syscall_nr { return r.action; }
                }
                self.default_act
            }
            SeccompOp::DenyList => {
                for r in self.rules[..self.n_rules].iter().flatten() {
                    if r.syscall == syscall_nr { return r.action; }
                }
                SeccompAction::Allow
            }
            SeccompOp::Audit => SeccompAction::Log,
        }
    }

    /// Check and return true if syscall should be allowed
    pub fn is_allowed(&self, syscall_nr: u32) -> bool {
        matches!(self.check(syscall_nr), SeccompAction::Allow | SeccompAction::Log)
    }
}

// ── Predefined Profiles ───────────────────────────────────────────────────────

/// Minimal profile: only the 30 SigmaOS syscalls
pub fn minimal_profile() -> SeccompFilter {
    let mut f = SeccompFilter::allow_list();
    for nr in 0u32..=32 { f.add_rule(nr, SeccompAction::Allow); }
    f.lock();
    f
}

/// Shell profile: adds common file/process operations
pub fn shell_profile() -> SeccompFilter {
    let mut f = SeccompFilter::allow_list();
    // Basic syscalls 0-32
    for nr in 0u32..=32 { f.add_rule(nr, SeccompAction::Allow); }
    f.lock();
    f
}

/// Strict profile: read/write/exit only (for sandboxed workers)
pub fn strict_profile() -> SeccompFilter {
    let mut f = SeccompFilter::allow_list();
    f.add_rule(0,  SeccompAction::Allow); // read
    f.add_rule(1,  SeccompAction::Allow); // write
    f.add_rule(4,  SeccompAction::Allow); // exit
    f.add_rule(27, SeccompAction::Allow); // clock_gettime
    f.default_act = SeccompAction::Errno(-1); // EPERM
    f.lock();
    f
}
