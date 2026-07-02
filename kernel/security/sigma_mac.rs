// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// kernel/security/sigma_mac.rs — Mandatory Access Control (SELinux-inspired)
// Language: Rust #![no_std] — OOP via MacPolicy struct + label system

#![no_std]

pub const MAX_LABELS:   usize = 64;
pub const MAX_RULES:    usize = 256;
pub const LABEL_LEN:    usize = 32;

// ── Security Labels ───────────────────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct Label(pub u32);

impl Label {
    pub const UNLABELED: Label = Label(0);
    pub const KERNEL:    Label = Label(1);
    pub const SYSTEM:    Label = Label(2);
    pub const USER:      Label = Label(3);
}

// ── Object Classes ────────────────────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ObjClass {
    File      = 0,
    Dir       = 1,
    Process   = 2,
    Socket    = 3,
    Ipc       = 4,
    Shard     = 5,
    Capability = 6,
}

// ── Permission vector (64-bit bitmask) ───────────────────────────────────────
#[derive(Clone, Copy, Default)]
pub struct Perms(pub u64);

impl Perms {
    pub const READ:    u64 = 1 << 0;
    pub const WRITE:   u64 = 1 << 1;
    pub const EXEC:    u64 = 1 << 2;
    pub const CREATE:  u64 = 1 << 3;
    pub const UNLINK:  u64 = 1 << 4;
    pub const CONNECT: u64 = 1 << 5;
    pub const SEND:    u64 = 1 << 6;
    pub const RECV:    u64 = 1 << 7;
    pub const SIGNAL:  u64 = 1 << 8;
    pub const FORK:    u64 = 1 << 9;
    pub const MOUNT:   u64 = 1 << 10;
    pub const LOAD:    u64 = 1 << 11;
    pub const ALL:     u64 = u64::MAX;

    pub fn allows(&self, p: u64) -> bool { self.0 & p == p }
    pub fn add(&mut self, p: u64) { self.0 |= p; }
}

// ── Policy Rule ───────────────────────────────────────────────────────────────
#[derive(Clone, Copy)]
pub struct MacRule {
    pub src:     Label,
    pub dst:     Label,
    pub class:   ObjClass,
    pub allow:   Perms,
    pub audit:   bool,
    pub enabled: bool,
}

// ── Label Registry ────────────────────────────────────────────────────────────
#[derive(Clone, Copy)]
struct LabelEntry {
    label: Label,
    name:  [u8; LABEL_LEN],
    len:   usize,
}

// ── MAC Policy Engine ─────────────────────────────────────────────────────────
pub struct MacPolicy {
    labels:    [Option<LabelEntry>; MAX_LABELS],
    n_labels:  usize,
    rules:     [Option<MacRule>; MAX_RULES],
    n_rules:   usize,
    enforcing: bool,
    default_deny: bool,
}

impl MacPolicy {
    pub const fn new() -> Self {
        Self {
            labels:    [const { None }; MAX_LABELS],
            n_labels:  0,
            rules:     [const { None }; MAX_RULES],
            n_rules:   0,
            enforcing: true,
            default_deny: true,
        }
    }

    pub fn set_enforcing(&mut self, v: bool) { self.enforcing = v; }

    /// Load a permissive default policy (all-allow) for initial boot
    pub fn load_permissive(&mut self) {
        self.enforcing    = false;
        self.default_deny = false;
    }

    /// Register a security label by name
    pub fn register_label(&mut self, name: &[u8]) -> Label {
        let id = self.n_labels as u32 + 4; // start at 4 (0-3 reserved)
        let label = Label(id);
        let mut entry = LabelEntry { label, name: [0u8; LABEL_LEN], len: name.len().min(LABEL_LEN) };
        entry.name[..entry.len].copy_from_slice(&name[..entry.len]);
        for slot in &mut self.labels {
            if slot.is_none() { *slot = Some(entry); self.n_labels += 1; return label; }
        }
        Label::UNLABELED
    }

    /// Add a policy rule: allow src→dst class permissions
    pub fn allow(&mut self, src: Label, dst: Label, class: ObjClass, perms: u64) -> bool {
        if self.n_rules >= MAX_RULES { return false; }
        for slot in &mut self.rules {
            if slot.is_none() {
                *slot = Some(MacRule {
                    src, dst, class,
                    allow: Perms(perms), audit: false, enabled: true,
                });
                self.n_rules += 1;
                return true;
            }
        }
        false
    }

    /// Check if (src_label, dst_label, class, perms) is allowed
    pub fn check(&self, src: Label, dst: Label, class: ObjClass, perms: u64) -> MacDecision {
        if !self.enforcing { return MacDecision::Allow; }

        for rule in self.rules[..self.n_rules].iter().flatten() {
            if !rule.enabled { continue; }
            if rule.src == src && rule.dst == dst && rule.class == class {
                if rule.allow.allows(perms) {
                    if rule.audit { return MacDecision::AllowAudit; }
                    return MacDecision::Allow;
                } else {
                    return MacDecision::Deny;
                }
            }
        }
        // No matching rule → default
        if self.default_deny { MacDecision::Deny } else { MacDecision::Allow }
    }

    /// Bootstrap default policy for SigmaOS
    pub fn load_default(&mut self) {
        // Kernel can do everything
        self.allow(Label::KERNEL, Label::KERNEL, ObjClass::File,    Perms::ALL);
        self.allow(Label::KERNEL, Label::KERNEL, ObjClass::Process, Perms::ALL);
        // System services: r/w files, connect sockets
        self.allow(Label::SYSTEM, Label::SYSTEM, ObjClass::File,
                   Perms::READ | Perms::WRITE | Perms::CREATE);
        self.allow(Label::SYSTEM, Label::SYSTEM, ObjClass::Socket,
                   Perms::CONNECT | Perms::SEND | Perms::RECV);
        // User processes: read/exec files, no kernel objects
        self.allow(Label::USER, Label::USER, ObjClass::File,
                   Perms::READ | Perms::EXEC | Perms::CREATE);
        self.allow(Label::USER, Label::USER, ObjClass::Socket,
                   Perms::CONNECT | Perms::SEND | Perms::RECV);
        // User → system: read only
        self.allow(Label::USER, Label::SYSTEM, ObjClass::File, Perms::READ);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MacDecision { Allow, AllowAudit, Deny }
