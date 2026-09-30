// Seccomp - Secure Computing Mode for Syscall Filtering
// Inspired by Linux seccomp for syscall filtering and sandboxing

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

/// Seccomp action
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeccompAction {
    Allow,
    KillProcess,
    KillThread,
    Trap,
    Errno(u32),
    Trace,
    Log,
}

/// Seccomp comparison operator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeccompCmpOp {
    NotEqual,
    LessThan,
    LessThanOrEqual,
    Equal,
    GreaterThanOrEqual,
    GreaterThan,
    MaskedEqual,
}

/// Seccomp argument filter
#[derive(Debug, Clone)]
pub struct SeccompArgFilter {
    pub arg_num: u8,
    pub op: SeccompCmpOp,
    pub value: u64,
    pub value_mask: u64,
}

/// Seccomp rule
#[derive(Debug, Clone)]
pub struct SeccompRule {
    pub syscall_number: u64,
    pub action: SeccompAction,
    pub arg_filters: Vec<SeccompArgFilter>,
}

/// Seccomp filter
pub struct SeccompFilter {
    pub name: String,
    pub rules: Vec<SeccompRule>,
    pub default_action: SeccompAction,
    pub architecture: String,
    pub enabled: bool,
}

impl SeccompFilter {
    pub fn new(name: String, default_action: SeccompAction, architecture: String) -> Self {
        Self {
            name,
            rules: Vec::new(),
            default_action,
            architecture,
            enabled: true,
        }
    }

    /// Add a rule to the filter
    pub fn add_rule(&mut self, rule: SeccompRule) {
        self.rules.push(rule);
    }

    /// Remove a rule by syscall number
    pub fn remove_rule(&mut self, syscall_number: u64) -> bool {
        if let Some(pos) = self
            .rules
            .iter()
            .position(|r| r.syscall_number == syscall_number)
        {
            self.rules.remove(pos);
            true
        } else {
            false
        }
    }

    /// Filter a syscall
    pub fn filter_syscall(&self, syscall_number: u64, args: &[u64]) -> SeccompAction {
        if !self.enabled {
            return SeccompAction::Allow;
        }

        // Check for matching rule
        for rule in &self.rules {
            if rule.syscall_number == syscall_number {
                // Check argument filters
                let matches = rule.arg_filters.iter().all(|filter| {
                    let arg_value = if (filter.arg_num as usize) < args.len() {
                        args[filter.arg_num as usize]
                    } else {
                        0
                    };

                    let masked_value = arg_value & filter.value_mask;
                    let masked_target = filter.value & filter.value_mask;

                    match filter.op {
                        SeccompCmpOp::Equal => masked_value == masked_target,
                        SeccompCmpOp::NotEqual => masked_value != masked_target,
                        SeccompCmpOp::LessThan => masked_value < masked_target,
                        SeccompCmpOp::LessThanOrEqual => masked_value <= masked_target,
                        SeccompCmpOp::GreaterThanOrEqual => masked_value >= masked_target,
                        SeccompCmpOp::GreaterThan => masked_value > masked_target,
                        SeccompCmpOp::MaskedEqual => {
                            (arg_value & filter.value_mask) == filter.value
                        }
                    }
                });

                if matches {
                    return rule.action;
                }
            }
        }

        // No matching rule - use default action
        self.default_action
    }

    /// Enable the filter
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable the filter
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Get rule count
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }
}

/// Seccomp manager
pub struct SeccompManager {
    filters: HashMap<String, SeccompFilter>,
    next_filter_id: AtomicU64,
    active_filter: Option<String>,
}

impl SeccompManager {
    pub fn new() -> Self {
        Self {
            filters: HashMap::new(),
            next_filter_id: AtomicU64::new(1),
            active_filter: None,
        }
    }

    /// Create a new filter
    pub fn create_filter(
        &mut self,
        name: String,
        default_action: SeccompAction,
        architecture: String,
    ) -> Result<(), &'static str> {
        if self.filters.contains_key(&name) {
            return Err("Filter already exists");
        }

        let filter = SeccompFilter::new(name.clone(), default_action, architecture);
        self.filters.insert(name, filter);

        Ok(())
    }

    /// Delete a filter
    pub fn delete_filter(&mut self, name: &str) -> Result<(), &'static str> {
        if self.active_filter.as_ref() == Some(&name.to_string()) {
            return Err("Cannot delete active filter");
        }

        if self.filters.remove(name).is_some() {
            Ok(())
        } else {
            Err("Filter not found")
        }
    }

    /// Get a filter
    pub fn get_filter(&self, name: &str) -> Option<&SeccompFilter> {
        self.filters.get(name)
    }

    /// Get mutable filter
    pub fn get_filter_mut(&mut self, name: &str) -> Option<&mut SeccompFilter> {
        self.filters.get_mut(name)
    }

    /// Set active filter
    pub fn set_active_filter(&mut self, name: String) -> Result<(), &'static str> {
        if !self.filters.contains_key(&name) {
            return Err("Filter not found");
        }

        self.active_filter = Some(name);
        Ok(())
    }

    /// Filter a syscall using active filter
    pub fn filter_syscall(&self, syscall_number: u64, args: &[u64]) -> SeccompAction {
        if let Some(ref active_name) = self.active_filter {
            if let Some(filter) = self.filters.get(active_name) {
                return filter.filter_syscall(syscall_number, args);
            }
        }

        // No active filter - allow by default
        SeccompAction::Allow
    }

    /// Get active filter name
    pub fn active_filter(&self) -> Option<&str> {
        self.active_filter.as_deref()
    }

    /// Get filter count
    pub fn filter_count(&self) -> usize {
        self.filters.len()
    }

    /// List all filters
    pub fn list_filters(&self) -> Vec<String> {
        self.filters.keys().cloned().collect()
    }

    /// Create a strict filter (deny all except allowed)
    pub fn create_strict_filter(
        &mut self,
        name: String,
        allowed_syscalls: Vec<u64>,
    ) -> Result<(), &'static str> {
        self.create_filter(
            name.clone(),
            SeccompAction::KillProcess,
            "x86_64".to_string(),
        )?;

        let filter = self.get_filter_mut(&name).unwrap();

        // Add allow rules for allowed syscalls
        for syscall_num in allowed_syscalls {
            let rule = SeccompRule {
                syscall_number: syscall_num,
                action: SeccompAction::Allow,
                arg_filters: Vec::new(),
            };
            filter.add_rule(rule);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_filter() {
        let mut manager = SeccompManager::new();

        assert!(manager
            .create_filter(
                "test".to_string(),
                SeccompAction::KillProcess,
                "x86_64".to_string()
            )
            .is_ok());
        assert_eq!(manager.filter_count(), 1);
    }

    #[test]
    fn test_add_rule() {
        let mut manager = SeccompManager::new();

        manager
            .create_filter(
                "test".to_string(),
                SeccompAction::KillProcess,
                "x86_64".to_string(),
            )
            .unwrap();

        let rule = SeccompRule {
            syscall_number: 1,
            action: SeccompAction::Allow,
            arg_filters: Vec::new(),
        };

        let filter = manager.get_filter_mut("test").unwrap();
        filter.add_rule(rule);

        assert_eq!(filter.rule_count(), 1);
    }

    #[test]
    fn test_filter_syscall() {
        let mut manager = SeccompManager::new();

        manager
            .create_filter(
                "test".to_string(),
                SeccompAction::KillProcess,
                "x86_64".to_string(),
            )
            .unwrap();

        let rule = SeccompRule {
            syscall_number: 1,
            action: SeccompAction::Allow,
            arg_filters: Vec::new(),
        };

        let filter = manager.get_filter_mut("test").unwrap();
        filter.add_rule(rule);

        let action = filter.filter_syscall(1, &[]);
        assert_eq!(action, SeccompAction::Allow);

        let action = filter.filter_syscall(2, &[]);
        assert_eq!(action, SeccompAction::KillProcess);
    }

    #[test]
    fn test_arg_filter() {
        let mut manager = SeccompManager::new();

        manager
            .create_filter(
                "test".to_string(),
                SeccompAction::KillProcess,
                "x86_64".to_string(),
            )
            .unwrap();

        let arg_filter = SeccompArgFilter {
            arg_num: 0,
            op: SeccompCmpOp::Equal,
            value: 100,
            value_mask: u64::MAX,
        };

        let rule = SeccompRule {
            syscall_number: 1,
            action: SeccompAction::Allow,
            arg_filters: vec![arg_filter],
        };

        let filter = manager.get_filter_mut("test").unwrap();
        filter.add_rule(rule);

        let action = filter.filter_syscall(1, &[100]);
        assert_eq!(action, SeccompAction::Allow);

        let action = filter.filter_syscall(1, &[200]);
        assert_eq!(action, SeccompAction::KillProcess);
    }

    #[test]
    fn test_set_active_filter() {
        let mut manager = SeccompManager::new();

        manager
            .create_filter(
                "test".to_string(),
                SeccompAction::KillProcess,
                "x86_64".to_string(),
            )
            .unwrap();
        assert!(manager.set_active_filter("test".to_string()).is_ok());

        assert_eq!(manager.active_filter(), Some("test"));
    }

    #[test]
    fn test_strict_filter() {
        let mut manager = SeccompManager::new();

        assert!(manager
            .create_strict_filter("strict".to_string(), vec![1, 2, 3])
            .is_ok());

        let filter = manager.get_filter("strict").unwrap();
        assert_eq!(filter.default_action, SeccompAction::KillProcess);
        assert_eq!(filter.rule_count(), 3);
    }

    #[test]
    fn test_enable_disable() {
        let mut manager = SeccompManager::new();

        manager
            .create_filter(
                "test".to_string(),
                SeccompAction::KillProcess,
                "x86_64".to_string(),
            )
            .unwrap();

        let filter = manager.get_filter_mut("test").unwrap();
        filter.disable();
        assert!(!filter.enabled);

        filter.enable();
        assert!(filter.enabled);
    }
}
