// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// kernel/security/sigma_landlock.rs — Landlock filesystem sandboxing (no_std)
// Language: Rust #![no_std] — OOP via LandlockRuleset + Rule

#![no_std]

pub const MAX_RULES:  usize = 64;
pub const MAX_PATH:   usize = 256;

// ── Allowed Access Rights ─────────────────────────────────────────────────────
pub struct Access(pub u64);
impl Access {
    pub const READ_FILE:        u64 = 1 << 0;
    pub const WRITE_FILE:       u64 = 1 << 1;
    pub const EXECUTE:          u64 = 1 << 2;
    pub const READ_DIR:         u64 = 1 << 3;
    pub const REMOVE_DIR:       u64 = 1 << 4;
    pub const REMOVE_FILE:      u64 = 1 << 5;
    pub const MAKE_CHAR:        u64 = 1 << 6;
    pub const MAKE_DIR:         u64 = 1 << 7;
    pub const MAKE_REG:         u64 = 1 << 8;
    pub const MAKE_SOCK:        u64 = 1 << 9;
    pub const MAKE_FIFO:        u64 = 1 << 10;
    pub const MAKE_BLOCK:       u64 = 1 << 11;
    pub const MAKE_SYM:         u64 = 1 << 12;
    pub const REFER:            u64 = 1 << 13;
    pub const TRUNCATE:         u64 = 1 << 14;
    pub const READ_ONLY: u64 = Self::READ_FILE | Self::READ_DIR;
    pub const READ_WRITE: u64 = Self::READ_FILE | Self::WRITE_FILE | Self::READ_DIR;
    pub const FULL:      u64 = u64::MAX;
}

// ── Path Rule ─────────────────────────────────────────────────────────────────
#[derive(Clone, Copy)]
pub struct PathRule {
    pub path:       [u8; MAX_PATH],
    pub path_len:   usize,
    pub access:     u64,
    pub recursive:  bool,
}

impl PathRule {
    pub fn new(path: &[u8], access: u64, recursive: bool) -> Self {
        let mut r = Self { path: [0u8;MAX_PATH], path_len: path.len().min(MAX_PATH),
                           access, recursive };
        r.path[..r.path_len].copy_from_slice(&path[..r.path_len]);
        r
    }
    pub fn path_matches(&self, target: &[u8]) -> bool {
        if self.recursive { target.starts_with(&self.path[..self.path_len]) }
        else { &self.path[..self.path_len] == target }
    }
    pub fn allows(&self, op: u64) -> bool { self.access & op == op }
}

// ── Ruleset ───────────────────────────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RulesetState { Building, Locked }

pub struct LandlockRuleset {
    rules:   [Option<PathRule>; MAX_RULES],
    n_rules: usize,
    state:   RulesetState,
    pub all_handled: u64,  // all access bits handled (deny-by-default for unlisted)
}

impl LandlockRuleset {
    pub const fn new() -> Self {
        Self {
            rules: [const { None }; MAX_RULES],
            n_rules: 0,
            state: RulesetState::Building,
            all_handled: Access::FULL,
        }
    }

    pub fn add_path_rule(&mut self, path: &[u8], access: u64, recursive: bool) -> bool {
        if self.state == RulesetState::Locked || self.n_rules >= MAX_RULES { return false; }
        for slot in &mut self.rules {
            if slot.is_none() {
                *slot = Some(PathRule::new(path, access, recursive));
                self.n_rules += 1;
                return true;
            }
        }
        false
    }

    /// Lock the ruleset — no more rules can be added
    pub fn lock(&mut self) { self.state = RulesetState::Locked; }

    /// Check if access `op` on `path` is permitted
    pub fn check(&self, path: &[u8], op: u64) -> bool {
        if self.state == RulesetState::Building { return true; } // not locked yet
        // Find most-specific matching rule (longest path prefix wins)
        let mut best_len = usize::MAX;
        let mut allowed  = false;
        let mut found    = false;
        for rule in self.rules[..self.n_rules].iter().flatten() {
            if rule.path_matches(path) {
                let rl = rule.path_len;
                if !found || rl > best_len {
                    best_len = rl;
                    allowed  = rule.allows(op);
                    found    = true;
                }
            }
        }
        if found { allowed } else { false } // deny by default once locked
    }

    /// Compose with sigma_unveil for defence-in-depth
    pub fn check_unveil_compat(&self, path: &[u8], op: u64) -> bool {
        self.check(path, op)
    }

    pub fn rule_count(&self) -> usize { self.n_rules }
    pub fn is_locked(&self) -> bool { self.state == RulesetState::Locked }
}

// ── Pre-built sandboxes ───────────────────────────────────────────────────────

/// Web browser sandbox: read /usr, write /tmp, deny /etc, /home/other
pub fn browser_sandbox() -> LandlockRuleset {
    let mut rs = LandlockRuleset::new();
    rs.add_path_rule(b"/usr",  Access::READ_ONLY, true);
    rs.add_path_rule(b"/lib",  Access::READ_ONLY, true);
    rs.add_path_rule(b"/tmp",  Access::READ_WRITE, true);
    rs.add_path_rule(b"/home/sovereign/Downloads", Access::READ_WRITE, true);
    rs.lock();
    rs
}

/// Text editor sandbox
pub fn editor_sandbox(home: &[u8]) -> LandlockRuleset {
    let mut rs = LandlockRuleset::new();
    rs.add_path_rule(b"/usr",  Access::READ_ONLY, true);
    rs.add_path_rule(home,     Access::READ_WRITE, true);
    rs.add_path_rule(b"/tmp",  Access::READ_WRITE, true);
    rs.lock();
    rs
}
