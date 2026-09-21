// Linux-inspired seccomp (secure computing mode) filter
// BPF-based syscall filtering for SigmaOS

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

/// Seccomp operation (Linux seccomp.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeccompOperation {
    Allow,
    KillProcess,
    KillThread,
    Trap,
    Errno(u32),
    Trace(u32),
    Log,
}

/// Seccomp comparison operator (Linux seccomp.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeccompCompare {
    NotEqual,
    LessThan,
    LessOrEqual,
    Equal,
    GreaterOrEqual,
    GreaterThan,
    MaskedEqual(u64),
}

/// Seccomp argument filter
#[derive(Debug, Clone)]
pub struct SeccompArgFilter {
    index: u32,
    value: u64,
    mask: u64,
    op: SeccompCompare,
}

impl SeccompArgFilter {
    pub fn new(index: u32, value: u64, mask: u64, op: SeccompCompare) -> Self {
        SeccompArgFilter {
            index,
            value,
            mask,
            op,
        }
    }

    /// Check if argument matches filter
    pub fn matches(&self, arg: u64) -> bool {
        let masked_arg = arg & self.mask;
        let masked_value = self.value & self.mask;

        match self.op {
            SeccompCompare::NotEqual => masked_arg != masked_value,
            SeccompCompare::LessThan => masked_arg < masked_value,
            SeccompCompare::LessOrEqual => masked_arg <= masked_value,
            SeccompCompare::Equal => masked_arg == masked_value,
            SeccompCompare::GreaterOrEqual => masked_arg >= masked_value,
            SeccompCompare::GreaterThan => masked_arg > masked_value,
            SeccompCompare::MaskedEqual(_) => masked_arg == masked_value,
        }
    }
}

/// Seccomp rule for a syscall
#[derive(Debug, Clone)]
pub struct SeccompRule {
    syscall: i32,
    args: Vec<SeccompArgFilter>,
    action: SeccompOperation,
}

impl SeccompRule {
    pub fn new(syscall: i32, action: SeccompOperation) -> Self {
        SeccompRule {
            syscall,
            args: Vec::new(),
            action,
        }
    }

    /// Add argument filter
    pub fn add_arg_filter(&mut self, filter: SeccompArgFilter) {
        self.args.push(filter);
    }

    /// Check if syscall matches rule
    pub fn matches(&self, syscall: i32, args: &[u64]) -> bool {
        if self.syscall != syscall {
            return false;
        }

        for arg_filter in &self.args {
            let arg_index = arg_filter.index as usize;
            if arg_index >= args.len() {
                return false;
            }
            if !arg_filter.matches(args[arg_index]) {
                return false;
            }
        }

        true
    }

    /// Get action
    pub fn action(&self) -> SeccompOperation {
        self.action
    }
}

/// Seccomp filter for a process
#[derive(Debug, Clone)]
pub struct SeccompFilter {
    rules: Vec<SeccompRule>,
    default_action: SeccompOperation,
}

impl SeccompFilter {
    pub fn new(default_action: SeccompOperation) -> Self {
        SeccompFilter {
            rules: Vec::new(),
            default_action,
        }
    }

    /// Add rule
    pub fn add_rule(&mut self, rule: SeccompRule) {
        self.rules.push(rule);
    }

    /// Check syscall against filter
    pub fn check_syscall(&self, syscall: i32, args: &[u64]) -> SeccompOperation {
        for rule in &self.rules {
            if rule.matches(syscall, args) {
                return rule.action();
            }
        }
        self.default_action
    }

    /// Get rule count
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }
}

impl Default for SeccompFilter {
    fn default() -> Self {
        Self::new(SeccompOperation::KillProcess)
    }
}

/// Seccomp manager for the system
pub struct SeccompManager {
    process_filters: BTreeMap<u32, Arc<Mutex<SeccompFilter>>>,
    next_pid: u32,
}

impl SeccompManager {
    pub fn new() -> Self {
        SeccompManager {
            process_filters: BTreeMap::new(),
            next_pid: 1,
        }
    }

    /// Create a new process with default filter
    pub fn create_process(&mut self) -> u32 {
        let pid = self.next_pid;
        self.next_pid += 1;

        let filter = Arc::new(Mutex::new(SeccompFilter::default()));
        self.process_filters.insert(pid, filter);

        pid
    }

    /// Get process filter
    pub fn get_filter(&self, pid: u32) -> Option<Arc<Mutex<SeccompFilter>>> {
        self.process_filters.get(&pid).cloned()
    }

    /// Set process filter
    pub fn set_filter(&mut self, pid: u32, filter: SeccompFilter) -> Result<(), String> {
        if !self.process_filters.contains_key(&pid) {
            return Err(format!("Process not found: {}", pid));
        }

        let filter = Arc::new(Mutex::new(filter));
        self.process_filters.insert(pid, filter);
        Ok(())
    }

    /// Check syscall for process
    pub fn check_syscall(&self, pid: u32, syscall: i32, args: &[u64]) -> Result<SeccompOperation, String> {
        let filter = self.process_filters.get(&pid)
            .ok_or_else(|| format!("Process not found: {}", pid))?;
        
        let filter_guard = filter.lock().unwrap();
        Ok(filter_guard.check_syscall(syscall, args))
    }

    /// Remove process
    pub fn remove_process(&mut self, pid: u32) -> Result<(), String> {
        self.process_filters.remove(&pid)
            .ok_or_else(|| format!("Process not found: {}", pid))?;
        Ok(())
    }

    /// Get process count
    pub fn process_count(&self) -> usize {
        self.process_filters.len()
    }
}

impl Default for SeccompManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seccomp_arg_filter_equal() {
        let filter = SeccompArgFilter::new(0, 42, 0xFFFFFFFFFFFFFFFF, SeccompCompare::Equal);
        assert!(filter.matches(42));
        assert!(!filter.matches(43));
    }

    #[test]
    fn test_seccomp_arg_filter_less_than() {
        let filter = SeccompArgFilter::new(0, 100, 0xFFFFFFFFFFFFFFFF, SeccompCompare::LessThan);
        assert!(filter.matches(99));
        assert!(!filter.matches(100));
        assert!(!filter.matches(101));
    }

    #[test]
    fn test_seccomp_arg_filter_masked() {
        let filter = SeccompArgFilter::new(0, 0x1234, 0xFFFF, SeccompCompare::MaskedEqual(0xFFFF));
        assert!(filter.matches(0x1234));
        assert!(filter.matches(0x51234)); // upper bits masked
        assert!(!filter.matches(0x5678));
    }

    #[test]
    fn test_seccomp_rule_creation() {
        let rule = SeccompRule::new(1, SeccompOperation::Allow);
        assert_eq!(rule.action(), SeccompOperation::Allow);
    }

    #[test]
    fn test_seccomp_rule_with_arg_filter() {
        let mut rule = SeccompRule::new(1, SeccompOperation::Allow);
        let filter = SeccompArgFilter::new(0, 42, 0xFFFFFFFFFFFFFFFF, SeccompCompare::Equal);
        rule.add_arg_filter(filter);

        assert!(rule.matches(1, &[42]));
        assert!(!rule.matches(1, &[43]));
        assert!(!rule.matches(2, &[42]));
    }

    #[test]
    fn test_seccomp_filter_creation() {
        let filter = SeccompFilter::new(SeccompOperation::KillProcess);
        assert_eq!(filter.rule_count(), 0);
    }

    #[test]
    fn test_seccomp_filter_add_rule() {
        let mut filter = SeccompFilter::new(SeccompOperation::KillProcess);
        let rule = SeccompRule::new(1, SeccompOperation::Allow);
        filter.add_rule(rule);

        assert_eq!(filter.rule_count(), 1);
    }

    #[test]
    fn test_seccomp_filter_check_syscall() {
        let mut filter = SeccompFilter::new(SeccompOperation::KillProcess);
        let rule = SeccompRule::new(1, SeccompOperation::Allow);
        filter.add_rule(rule);

        assert_eq!(filter.check_syscall(1, &[]), SeccompOperation::Allow);
        assert_eq!(filter.check_syscall(2, &[]), SeccompOperation::KillProcess);
    }

    #[test]
    fn test_seccomp_manager_creation() {
        let manager = SeccompManager::new();
        assert_eq!(manager.process_count(), 0);
    }

    #[test]
    fn test_seccomp_manager_create_process() {
        let mut manager = SeccompManager::new();
        let pid = manager.create_process();

        assert_eq!(pid, 1);
        assert_eq!(manager.process_count(), 1);
    }

    #[test]
    fn test_seccomp_manager_set_filter() {
        let mut manager = SeccompManager::new();
        let pid = manager.create_process();

        let filter = SeccompFilter::new(SeccompOperation::Allow);
        assert!(manager.set_filter(pid, filter).is_ok());
    }

    #[test]
    fn test_seccomp_manager_check_syscall() {
        let mut manager = SeccompManager::new();
        let pid = manager.create_process();

        let result = manager.check_syscall(pid, 1, &[]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_seccomp_manager_remove() {
        let mut manager = SeccompManager::new();
        let pid = manager.create_process();

        manager.remove_process(pid).unwrap();
        assert_eq!(manager.process_count(), 0);
    }
}
