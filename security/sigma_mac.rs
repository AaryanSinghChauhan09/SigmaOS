// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// security/sigma_mac.rs — Mandatory Access Control (MAC) Enforcement
//
// Fixes BUG-014: sigma_mac.rs label enforcement now wired into VFS call sites.
//
// Implements a Bell-LaPadula–style lattice MAC model (similar to SELinux MLS):
//   • Subjects (processes) and Objects (files, IPC) carry a MacLabel
//   • MacLabel has a sensitivity level (0=public … 255=top-secret) and
//     a category bitmask (u64 = up to 64 compartments)
//   • Dominance: A dominates B iff A.level >= B.level AND A.cats ⊇ B.cats
//   • Read policy  (ss-property): subject must dominate object
//   • Write policy (★-property):  object must dominate subject
//   • Execute:     subject must dominate object
//
// MacPolicy holds a fixed array of MacRule entries for domain type enforcement
// (DTE), analogous to SELinux type enforcement (TE) rules.
//
// Architecture (OOP, no_std, no alloc):
//   • MacLabel:   Sensitivity + category bitmask
//   • MacOp:      Bitmask of allowed operations (read/write/exec/append/create)
//   • MacRule:    (subject_label, object_label, allowed_ops)
//   • MacPolicy:  Fixed rule table + label lattice check
//   • MacContext: Per-process active label (stored in process descriptor)
//   • vfs_mac_check(): The missing VFS hook from BUG-014

#![no_std]
#![allow(dead_code)]

// ─────────────────────────────────────────────────────────────────────────────
// Constants
// ─────────────────────────────────────────────────────────────────────────────

pub const MAC_MAX_RULES:      usize = 256;
pub const MAC_MAX_LEVEL:      u8    = 255;
pub const MAC_SENSITIVITY_SECRET: u8 = 3;
pub const MAC_SENSITIVITY_TS:     u8 = 5;

// Operation bitmask (MacOp)
pub const MAC_OP_READ:    u32 = 1 << 0;
pub const MAC_OP_WRITE:   u32 = 1 << 1;
pub const MAC_OP_EXEC:    u32 = 1 << 2;
pub const MAC_OP_APPEND:  u32 = 1 << 3;
pub const MAC_OP_CREATE:  u32 = 1 << 4;
pub const MAC_OP_DELETE:  u32 = 1 << 5;
pub const MAC_OP_CHMOD:   u32 = 1 << 6;
pub const MAC_OP_CHOWN:   u32 = 1 << 7;
pub const MAC_OP_IOCTL:   u32 = 1 << 8;
pub const MAC_OP_CONNECT: u32 = 1 << 9;
pub const MAC_OP_ACCEPT:  u32 = 1 << 10;

// ─────────────────────────────────────────────────────────────────────────────
// MacLabel — sensitivity level + category bitmask
// ─────────────────────────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct MacLabel {
    /// Sensitivity level: 0 = unclassified, higher = more sensitive
    pub level:      u8,
    /// Category compartment bitmask (each bit = one compartment)
    pub categories: u64,
}

impl MacLabel {
    pub const fn new(level: u8, categories: u64) -> Self {
        Self { level, categories }
    }

    pub const fn public() -> Self         { Self::new(0, 0) }
    pub const fn secret() -> Self         { Self::new(MAC_SENSITIVITY_SECRET, 0) }
    pub const fn top_secret() -> Self     { Self::new(MAC_SENSITIVITY_TS, !0u64) }
    pub const fn system_low() -> Self     { Self::new(0, 0) }
    pub const fn system_high() -> Self    { Self::new(MAC_MAX_LEVEL, !0u64) }

    /// Label A dominates label B (A is at least as sensitive as B in all dimensions)
    pub fn dominates(&self, other: &MacLabel) -> bool {
        self.level >= other.level
            && (self.categories & other.categories) == other.categories
    }

    /// Label A is dominated by label B (B is at least as sensitive)
    pub fn dominated_by(&self, other: &MacLabel) -> bool {
        other.dominates(self)
    }

    /// Two labels are equivalent
    pub fn equivalent(&self, other: &MacLabel) -> bool {
        self.level == other.level && self.categories == other.categories
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// MacRule — type enforcement rule
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Copy, Clone)]
pub struct MacRule {
    pub subject:      MacLabel,
    pub object:       MacLabel,
    pub allowed_ops:  u32,
    pub enabled:      bool,
}

impl MacRule {
    pub const fn new(subject: MacLabel, object: MacLabel, ops: u32) -> Self {
        Self { subject, object, allowed_ops: ops, enabled: true }
    }

    pub const fn empty() -> Self {
        Self {
            subject:     MacLabel::public(),
            object:      MacLabel::public(),
            allowed_ops: 0,
            enabled:     false,
        }
    }

    /// Returns true if this rule explicitly allows op for the given labels.
    pub fn matches(&self, sub: &MacLabel, obj: &MacLabel, op: u32) -> bool {
        self.enabled
            && self.subject.equivalent(sub)
            && self.object.equivalent(obj)
            && (self.allowed_ops & op) != 0
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// MacPolicy — full policy enforcement engine
// ─────────────────────────────────────────────────────────────────────────────

pub struct MacPolicy {
    rules:      [MacRule; MAC_MAX_RULES],
    rule_count: usize,
    /// If true, deny by default (whitelist mode); if false, allow by default (audit mode)
    enforcing:  bool,
}

impl MacPolicy {
    pub const fn new() -> Self {
        Self {
            rules:      [MacRule::empty(); MAC_MAX_RULES],
            rule_count: 0,
            enforcing:  true,
        }
    }

    /// Load default SigmaOS policy rules.
    pub fn load_defaults(&mut self) {
        // Public processes can read public objects
        self.add_rule(MacRule::new(
            MacLabel::public(), MacLabel::public(),
            MAC_OP_READ | MAC_OP_EXEC,
        ));

        // Public processes can write to public objects
        self.add_rule(MacRule::new(
            MacLabel::public(), MacLabel::public(),
            MAC_OP_WRITE | MAC_OP_APPEND | MAC_OP_CREATE,
        ));

        // Secret processes can read public + secret objects
        self.add_rule(MacRule::new(
            MacLabel::secret(), MacLabel::public(),
            MAC_OP_READ | MAC_OP_EXEC,
        ));
        self.add_rule(MacRule::new(
            MacLabel::secret(), MacLabel::secret(),
            MAC_OP_READ | MAC_OP_WRITE | MAC_OP_EXEC | MAC_OP_APPEND,
        ));

        // Public processes cannot read secret objects
        // (no rule added → enforcing mode denies it)

        // System can do anything to anything
        self.add_rule(MacRule::new(
            MacLabel::top_secret(), MacLabel::system_high(),
            MAC_OP_READ | MAC_OP_WRITE | MAC_OP_EXEC | MAC_OP_APPEND
            | MAC_OP_CREATE | MAC_OP_DELETE | MAC_OP_CHMOD | MAC_OP_CHOWN
            | MAC_OP_IOCTL | MAC_OP_CONNECT | MAC_OP_ACCEPT,
        ));
    }

    pub fn set_enforcing(&mut self, enforcing: bool) {
        self.enforcing = enforcing;
    }

    pub fn add_rule(&mut self, rule: MacRule) {
        if self.rule_count < MAC_MAX_RULES {
            self.rules[self.rule_count] = rule;
            self.rule_count += 1;
        }
    }

    /// Core access check: Bell-LaPadula lattice + explicit type enforcement.
    ///
    /// For READ:  subject must dominate object (no read up)
    /// For WRITE: object must dominate subject (no write down)
    /// For EXEC:  subject must dominate object
    /// Type enforcement rules can additionally restrict or expand access.
    pub fn check_access(&self, subject: &MacLabel, object: &MacLabel, op: u32) -> MacDecision {
        // ── Lattice checks (Bell-LaPadula) ──────────────────────────────────
        if op & MAC_OP_READ != 0 {
            // ss-property: no read up
            if !subject.dominates(object) {
                return MacDecision::Deny(MacDenyReason::ReadUp);
            }
        }
        if op & (MAC_OP_WRITE | MAC_OP_APPEND | MAC_OP_CREATE) != 0 {
            // ★-property: no write down
            if !object.dominates(subject) {
                return MacDecision::Deny(MacDenyReason::WriteDown);
            }
        }
        if op & MAC_OP_EXEC != 0 {
            if !subject.dominates(object) {
                return MacDecision::Deny(MacDenyReason::ExecDenied);
            }
        }

        // ── Type enforcement check ───────────────────────────────────────────
        for i in 0..self.rule_count {
            if self.rules[i].matches(subject, object, op) {
                return MacDecision::Allow;
            }
        }

        // No matching rule found
        if self.enforcing {
            MacDecision::Deny(MacDenyReason::NoRule)
        } else {
            MacDecision::Allow  // permissive/audit mode
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// MacDecision — result of an access check
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MacDenyReason {
    ReadUp,     // Subject tried to read object at higher sensitivity
    WriteDown,  // Subject tried to write object at lower sensitivity
    ExecDenied, // Subject cannot exec at higher sensitivity
    NoRule,     // No matching type enforcement rule
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MacDecision {
    Allow,
    Deny(MacDenyReason),
}

impl MacDecision {
    pub fn is_allowed(&self) -> bool { matches!(self, MacDecision::Allow) }
}

// ─────────────────────────────────────────────────────────────────────────────
// MacContext — per-process label (stored in process descriptor)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Copy, Clone)]
pub struct MacContext {
    pub label:        MacLabel,
    pub pid:          u32,
    pub uid:          u32,
    pub privileged:   bool,  // kernel/init bypass
}

impl MacContext {
    pub const fn new(pid: u32, uid: u32, label: MacLabel) -> Self {
        Self { label, pid, uid, privileged: false }
    }

    pub const fn kernel() -> Self {
        Self {
            label: MacLabel::top_secret(),
            pid:   0,
            uid:   0,
            privileged: true,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// BUG-014 FIX: VFS MAC hooks
// These functions are called at every VFS operation site to enforce MAC.
// ─────────────────────────────────────────────────────────────────────────────

static mut MAC_POLICY: MacPolicy = MacPolicy::new();

/// Initialize the MAC subsystem and load default policy.
#[no_mangle]
pub unsafe extern "C" fn sigma_mac_init() {
    MAC_POLICY.load_defaults();
}

/// VFS open hook — called before opening a file.
/// Returns 0 (allowed) or -13 (EACCES).
#[no_mangle]
pub unsafe extern "C" fn vfs_mac_check_open(
    subject_level:  u8,
    subject_cats:   u64,
    object_level:   u8,
    object_cats:    u64,
    flags:          u32,   // O_RDONLY=0, O_WRONLY=1, O_RDWR=2
) -> i32 {
    let sub = MacLabel::new(subject_level, subject_cats);
    let obj = MacLabel::new(object_level,  object_cats);

    let op = match flags & 3 {
        0 => MAC_OP_READ,
        1 => MAC_OP_WRITE,
        2 => MAC_OP_READ | MAC_OP_WRITE,
        _ => MAC_OP_READ,
    };

    if MAC_POLICY.check_access(&sub, &obj, op).is_allowed() { 0 } else { -13 }
}

/// VFS read hook.
#[no_mangle]
pub unsafe extern "C" fn vfs_mac_check_read(
    subject_level: u8, subject_cats: u64,
    object_level:  u8, object_cats:  u64,
) -> i32 {
    let sub = MacLabel::new(subject_level, subject_cats);
    let obj = MacLabel::new(object_level,  object_cats);
    if MAC_POLICY.check_access(&sub, &obj, MAC_OP_READ).is_allowed() { 0 } else { -13 }
}

/// VFS write hook.
#[no_mangle]
pub unsafe extern "C" fn vfs_mac_check_write(
    subject_level: u8, subject_cats: u64,
    object_level:  u8, object_cats:  u64,
) -> i32 {
    let sub = MacLabel::new(subject_level, subject_cats);
    let obj = MacLabel::new(object_level,  object_cats);
    if MAC_POLICY.check_access(&sub, &obj, MAC_OP_WRITE).is_allowed() { 0 } else { -13 }
}

/// VFS exec hook.
#[no_mangle]
pub unsafe extern "C" fn vfs_mac_check_exec(
    subject_level: u8, subject_cats: u64,
    object_level:  u8, object_cats:  u64,
) -> i32 {
    let sub = MacLabel::new(subject_level, subject_cats);
    let obj = MacLabel::new(object_level,  object_cats);
    if MAC_POLICY.check_access(&sub, &obj, MAC_OP_EXEC).is_allowed() { 0 } else { -13 }
}

/// Add a custom MAC rule at runtime.
#[no_mangle]
pub unsafe extern "C" fn sigma_mac_add_rule(
    sub_level: u8, sub_cats: u64,
    obj_level: u8, obj_cats: u64,
    ops:       u32,
) {
    MAC_POLICY.add_rule(MacRule::new(
        MacLabel::new(sub_level, sub_cats),
        MacLabel::new(obj_level, obj_cats),
        ops,
    ));
}

/// Switch enforcement mode (1=enforcing, 0=permissive).
#[no_mangle]
pub unsafe extern "C" fn sigma_mac_set_enforcing(enforcing: u8) {
    MAC_POLICY.set_enforcing(enforcing != 0);
}
