// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SigmaOS: Governance Rules & Quota Manager (Rust, no_std)
//! Replaces: agents/policy/GovernanceRules.cpp
//!           agents/quota/QuotaManager.cpp
//! =========================================================================

const MAX_RULES: usize = 128;
const MAX_QUOTAS: usize = 256;

// ─── Governance Rules ──────────────────────────────────────────────────────

#[derive(Copy, Clone, PartialEq)]
pub enum RuleAction {
    Allow,
    Deny,
    Audit,
    Throttle,
}

#[derive(Copy, Clone)]
pub struct GovernanceRule {
    pub id: u32,
    pub action: RuleAction,
    pub priority: u8,
    pub enabled: bool,
}

impl GovernanceRule {
    pub const fn new(id: u32, action: RuleAction, priority: u8) -> Self {
        Self { id, action, priority, enabled: true }
    }

    pub fn class_name(&self) -> &'static str {
        "GovernanceRule"
    }
}

pub struct GovernanceEngine {
    rules: [Option<GovernanceRule>; MAX_RULES],
    count: usize,
}

impl GovernanceEngine {
    pub const fn new() -> Self {
        Self { rules: [None; MAX_RULES], count: 0 }
    }

    pub fn add_rule(&mut self, rule: GovernanceRule) -> bool {
        if self.count >= MAX_RULES {
            return false;
        }
        self.rules[self.count] = Some(rule);
        self.count += 1;
        true
    }

    pub fn evaluate(&self, rule_id: u32) -> RuleAction {
        for i in 0..self.count {
            if let Some(ref r) = self.rules[i] {
                if r.id == rule_id && r.enabled {
                    return r.action;
                }
            }
        }
        RuleAction::Deny
    }

    pub fn class_name(&self) -> &'static str {
        "GovernanceEngine"
    }
}

// ─── Quota Manager ─────────────────────────────────────────────────────────

#[derive(Copy, Clone)]
pub struct Quota {
    pub entity_id: u32,
    pub cpu_limit_pct: u8,
    pub mem_limit_mb: u32,
    pub io_limit_mbps: u32,
    pub current_cpu: u8,
    pub current_mem: u32,
}

impl Quota {
    pub const fn new(entity_id: u32, cpu: u8, mem: u32, io: u32) -> Self {
        Self {
            entity_id,
            cpu_limit_pct: cpu,
            mem_limit_mb: mem,
            io_limit_mbps: io,
            current_cpu: 0,
            current_mem: 0,
        }
    }

    pub fn is_cpu_exceeded(&self) -> bool {
        self.current_cpu > self.cpu_limit_pct
    }

    pub fn is_mem_exceeded(&self) -> bool {
        self.current_mem > self.mem_limit_mb
    }

    pub fn class_name(&self) -> &'static str {
        "Quota"
    }
}

pub struct QuotaManager {
    quotas: [Option<Quota>; MAX_QUOTAS],
    count: usize,
}

impl QuotaManager {
    pub const fn new() -> Self {
        Self { quotas: [None; MAX_QUOTAS], count: 0 }
    }

    pub fn add_quota(&mut self, q: Quota) -> bool {
        if self.count >= MAX_QUOTAS {
            return false;
        }
        self.quotas[self.count] = Some(q);
        self.count += 1;
        true
    }

    pub fn check_quota(&self, entity_id: u32) -> Option<&Quota> {
        for i in 0..self.count {
            if let Some(ref q) = self.quotas[i] {
                if q.entity_id == entity_id {
                    return Some(q);
                }
            }
        }
        None
    }

    pub fn class_name(&self) -> &'static str {
        "QuotaManager"
    }
}
