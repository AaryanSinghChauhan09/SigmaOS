// SPDX-License-Identifier: MIT
// SigmaOS Seccomp (Secure Computing Mode) Subsystem
// System call filtering and security sandboxing inspired by Linux seccomp

#![allow(dead_code)]

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU32, Ordering};

/// System call number
pub type SyscallNumber = u64;

/// Seccomp action
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeccompAction {
    Allow,
    KillProcess,
    KillThread,
    Trap,
    Errno(u16),
    Trace,
    Log,
}

/// Seccomp comparison operator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeccompCompareOp {
    NotEqual,
    LessThan,
    LessThanOrEqual,
    Equal,
    GreaterThanOrEqual,
    GreaterThan,
    MaskedEqual(u64),
}

/// Seccomp rule
#[derive(Debug, Clone)]
pub struct SeccompRule {
    pub syscall: SyscallNumber,
    pub action: SeccompAction,
    pub args: Vec<(u32, SeccompCompareOp, u64)>, // (arg_index, op, value)
}

impl SeccompRule {
    pub fn new(syscall: SyscallNumber, action: SeccompAction) -> Self {
        SeccompRule {
            syscall,
            action,
            args: Vec::new(),
        }
    }

    pub fn add_arg(&mut self, arg_index: u32, op: SeccompCompareOp, value: u64) {
        self.args.push((arg_index, op, value));
    }

    pub fn matches(&self, args: &[u64]) -> bool {
        for (arg_index, op, value) in &self.args {
            if *arg_index as usize >= args.len() {
                return false;
            }

            let arg_value = args[*arg_index as usize];
            let matches = match op {
                SeccompCompareOp::NotEqual => arg_value != *value,
                SeccompCompareOp::LessThan => arg_value < *value,
                SeccompCompareOp::LessThanOrEqual => arg_value <= *value,
                SeccompCompareOp::Equal => arg_value == *value,
                SeccompCompareOp::GreaterThanOrEqual => arg_value >= *value,
                SeccompCompareOp::GreaterThan => arg_value > *value,
                SeccompCompareOp::MaskedEqual(mask) => (arg_value & mask) == (*value & mask),
            };

            if !matches {
                return false;
            }
        }

        true
    }
}

/// Seccomp filter
#[derive(Debug)]
pub struct SeccompFilter {
    pub id: u32,
    pub rules: Vec<SeccompRule>,
    pub default_action: SeccompAction,
    pub enabled: AtomicU32, // 0 = disabled, 1 = enabled
}

impl SeccompFilter {
    pub fn new(id: u32, default_action: SeccompAction) -> Self {
        SeccompFilter {
            id,
            rules: Vec::new(),
            default_action,
            enabled: AtomicU32::new(1),
        }
    }

    pub fn add_rule(&mut self, rule: SeccompRule) {
        self.rules.push(rule);
    }

    pub fn enable(&self) {
        self.enabled.store(1, Ordering::SeqCst);
    }

    pub fn disable(&self) {
        self.enabled.store(0, Ordering::SeqCst);
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::SeqCst) == 1
    }

    pub fn evaluate(&self, syscall: SyscallNumber, args: &[u64]) -> SeccompAction {
        if !self.is_enabled() {
            return SeccompAction::Allow;
        }

        for rule in &self.rules {
            if rule.syscall == syscall && rule.matches(args) {
                return rule.action;
            }
        }

        self.default_action
    }
}

/// Seccomp subsystem
#[derive(Debug)]
pub struct SeccompSubsystem {
    filters: BTreeMap<u32, SeccompFilter>,
    next_filter_id: AtomicU32,
}

impl SeccompSubsystem {
    pub fn new() -> Self {
        SeccompSubsystem {
            filters: BTreeMap::new(),
            next_filter_id: AtomicU32::new(1),
        }
    }

    /// Create a new filter
    pub fn create_filter(&mut self, default_action: SeccompAction) -> u32 {
        let id = self.next_filter_id.fetch_add(1, Ordering::SeqCst);
        let filter = SeccompFilter::new(id, default_action);
        self.filters.insert(id, filter);
        id
    }

    /// Get filter by ID
    pub fn get_filter(&self, id: u32) -> Option<&SeccompFilter> {
        self.filters.get(&id)
    }

    /// Get mutable filter by ID
    pub fn get_filter_mut(&mut self, id: u32) -> Option<&mut SeccompFilter> {
        self.filters.get_mut(&id)
    }

    /// Delete a filter
    pub fn delete_filter(&mut self, id: u32) -> Result<(), &'static str> {
        self.filters.remove(&id).ok_or("Filter not found")?;
        Ok(())
    }

    /// Evaluate syscall against all filters
    pub fn evaluate_syscall(&self, syscall: SyscallNumber, args: &[u64]) -> SeccompAction {
        for filter in self.filters.values() {
            let action = filter.evaluate(syscall, args);
            if action != SeccompAction::Allow {
                return action;
            }
        }
        SeccompAction::Allow
    }

    /// Get filter count
    pub fn filter_count(&self) -> usize {
        self.filters.len()
    }
}

impl Default for SeccompSubsystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seccomp_filter_creation() {
        let mut subsystem = SeccompSubsystem::new();
        
        let filter_id = subsystem.create_filter(SeccompAction::KillProcess);
        assert!(filter_id > 0);
        assert_eq!(subsystem.filter_count(), 1);
    }

    #[test]
    fn test_seccomp_rule_matching() {
        let rule = SeccompRule::new(1, SeccompAction::Allow);
        
        let args = vec![0x1000, 0x2000, 0x3000];
        assert!(rule.matches(&args));
    }

    #[test]
    fn test_seccomp_rule_with_args() {
        let mut rule = SeccompRule::new(1, SeccompAction::Allow);
        rule.add_arg(0, SeccompCompareOp::Equal, 0x1000);
        
        let args = vec![0x1000, 0x2000];
        assert!(rule.matches(&args));
        
        let args = vec![0x2000, 0x3000];
        assert!(!rule.matches(&args));
    }

    #[test]
    fn test_seccomp_filter_evaluation() {
        let mut subsystem = SeccompSubsystem::new();
        
        let filter_id = subsystem.create_filter(SeccompAction::KillProcess);
        let filter = subsystem.get_filter_mut(filter_id).unwrap();
        
        let rule = SeccompRule::new(1, SeccompAction::Allow);
        filter.add_rule(rule);
        
        let action = subsystem.evaluate_syscall(1, &[]);
        assert_eq!(action, SeccompAction::Allow);
    }

    #[test]
    fn test_seccomp_filter_enable_disable() {
        let mut subsystem = SeccompSubsystem::new();
        
        let filter_id = subsystem.create_filter(SeccompAction::KillProcess);
        let filter = subsystem.get_filter(filter_id).unwrap();
        
        filter.disable();
        assert!(!filter.is_enabled());
        
        filter.enable();
        assert!(filter.is_enabled());
    }

    #[test]
    fn test_seccomp_compare_ops() {
        let mut rule = SeccompRule::new(1, SeccompAction::Allow);
        rule.add_arg(0, SeccompCompareOp::LessThan, 100);
        
        let args = vec![50];
        assert!(rule.matches(&args));
        
        let args = vec![150];
        assert!(!rule.matches(&args));
    }
}
