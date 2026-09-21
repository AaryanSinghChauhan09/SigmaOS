// Landlock Security Sandbox (Linux-inspired)
// Provides filesystem access control with path-based rules

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Landlock access rights (subset of Linux Landlock ABI)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LandlockAccess(u64);

impl LandlockAccess {
    pub const EXECUTE: Self = Self(1 << 0);
    pub const WRITE_FILE: Self = Self(1 << 1);
    pub const READ_FILE: Self = Self(1 << 2);
    pub const READ_DIR: Self = Self(1 << 3);
    pub const REMOVE_DIR: Self = Self(1 << 4);
    pub const REMOVE_FILE: Self = Self(1 << 5);
    pub const MAKE_CHAR: Self = Self(1 << 6);
    pub const MAKE_DIR: Self = Self(1 << 7);
    pub const MAKE_REG: Self = Self(1 << 8);
    pub const MAKE_SOCK: Self = Self(1 << 9);
    pub const MAKE_FIFO: Self = Self(1 << 10);
    pub const MAKE_BLOCK: Self = Self(1 << 11);
    pub const MAKE_SYM: Self = Self(1 << 12);
    pub const REFER: Self = Self(1 << 13);
    pub const TRUNCATE: Self = Self(1 << 14);

    pub fn empty() -> Self {
        Self(0)
    }

    pub fn contains(&self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    pub fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    pub fn intersect(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}

/// Landlock rule for a specific path
#[derive(Debug, Clone)]
pub struct LandlockRule {
    pub path: String,
    pub access: LandlockAccess,
}

impl LandlockRule {
    pub fn new(path: String, access: LandlockAccess) -> Self {
        Self { path, access }
    }

    pub fn matches(&self, requested_path: &str) -> bool {
        requested_path.starts_with(&self.path)
    }

    pub fn allows(&self, access: LandlockAccess) -> bool {
        self.access.contains(access)
    }
}

/// Landlock ruleset containing multiple rules
#[derive(Debug, Clone)]
pub struct LandlockRuleset {
    pub rules: Vec<LandlockRule>,
    pub handled_access: LandlockAccess,
}

impl LandlockRuleset {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            handled_access: LandlockAccess::empty(),
        }
    }

    pub fn add_rule(&mut self, rule: LandlockRule) {
        self.handled_access = self.handled_access.union(rule.access);
        self.rules.push(rule);
    }

    pub fn allows(&self, path: &str, access: LandlockAccess) -> bool {
        // Find the most specific matching rule
        let mut best_match: Option<&LandlockRule> = None;

        for rule in &self.rules {
            if rule.matches(path) {
                match &best_match {
                    None => best_match = Some(rule),
                    Some(current) if rule.path.len() > current.path.len() => {
                        best_match = Some(rule);
                    }
                    _ => {}
                }
            }
        }

        match best_match {
            Some(rule) => rule.allows(access),
            None => false, // Default deny
        }
    }
}

impl Default for LandlockRuleset {
    fn default() -> Self {
        Self::new()
    }
}

/// Landlock domain for process isolation
#[derive(Debug, Clone)]
pub struct LandlockDomain {
    pub id: u64,
    pub ruleset: LandlockRuleset,
}

impl LandlockDomain {
    pub fn new(id: u64, ruleset: LandlockRuleset) -> Self {
        Self { id, ruleset }
    }

    pub fn allows(&self, path: &str, access: LandlockAccess) -> bool {
        self.ruleset.allows(path, access)
    }
}

/// Landlock manager for system-wide domain management
pub struct LandlockManager {
    domains: Arc<Mutex<HashMap<u64, LandlockDomain>>>,
    next_domain_id: Arc<Mutex<u64>>,
}

impl LandlockManager {
    pub fn new() -> Self {
        Self {
            domains: Arc::new(Mutex::new(HashMap::new())),
            next_domain_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Create a new Landlock domain with a ruleset
    pub fn create_domain(&self, ruleset: LandlockRuleset) -> u64 {
        let mut next_id = self.next_domain_id.lock().unwrap();
        let domain_id = *next_id;
        *next_id += 1;
        drop(next_id);

        let domain = LandlockDomain::new(domain_id, ruleset);
        let mut domains = self.domains.lock().unwrap();
        domains.insert(domain_id, domain);

        domain_id
    }

    /// Get a domain by ID
    pub fn get_domain(&self, domain_id: u64) -> Option<LandlockDomain> {
        let domains = self.domains.lock().unwrap();
        domains.get(&domain_id).cloned()
    }

    /// Remove a domain
    pub fn remove_domain(&self, domain_id: u64) -> Result<(), String> {
        let mut domains = self.domains.lock().unwrap();
        match domains.remove(&domain_id) {
            Some(_) => Ok(()),
            None => Err(format!("Domain {} not found", domain_id)),
        }
    }

    /// Check if a domain allows access to a path
    pub fn check_access(&self, domain_id: u64, path: &str, access: LandlockAccess) -> bool {
        match self.get_domain(domain_id) {
            Some(domain) => domain.allows(path, access),
            None => false, // No domain = deny
        }
    }

    /// Get number of active domains
    pub fn domain_count(&self) -> usize {
        let domains = self.domains.lock().unwrap();
        domains.len()
    }
}

impl Default for LandlockManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_landlock_access_flags() {
        let read = LandlockAccess::READ_FILE;
        let write = LandlockAccess::WRITE_FILE;
        let both = read.union(write);

        assert!(both.contains(read));
        assert!(both.contains(write));
        assert!(!read.contains(write));
    }

    #[test]
    fn test_landlock_rule_creation() {
        let rule = LandlockRule::new("/home".to_string(), LandlockAccess::READ_FILE);
        assert_eq!(rule.path, "/home");
        assert!(rule.access.contains(LandlockAccess::READ_FILE));
    }

    #[test]
    fn test_landlock_rule_matching() {
        let rule = LandlockRule::new("/home".to_string(), LandlockAccess::READ_FILE);
        assert!(rule.matches("/home/user"));
        assert!(rule.matches("/home/user/file.txt"));
        assert!(!rule.matches("/etc/passwd"));
    }

    #[test]
    fn test_landlock_ruleset() {
        let mut ruleset = LandlockRuleset::new();
        let rule1 = LandlockRule::new("/home".to_string(), LandlockAccess::READ_FILE);
        let rule2 = LandlockRule::new("/tmp".to_string(), LandlockAccess::WRITE_FILE);

        ruleset.add_rule(rule1);
        ruleset.add_rule(rule2);

        assert!(ruleset.allows("/home/user", LandlockAccess::READ_FILE));
        assert!(!ruleset.allows("/home/user", LandlockAccess::WRITE_FILE));
        assert!(ruleset.allows("/tmp/file", LandlockAccess::WRITE_FILE));
    }

    #[test]
    fn test_landlock_ruleset_specificity() {
        let mut ruleset = LandlockRuleset::new();
        ruleset.add_rule(LandlockRule::new("/".to_string(), LandlockAccess::READ_FILE));
        ruleset.add_rule(LandlockRule::new("/home".to_string(), LandlockAccess::WRITE_FILE));

        // More specific rule should take precedence
        assert!(ruleset.allows("/home/user", LandlockAccess::WRITE_FILE));
        assert!(!ruleset.allows("/etc", LandlockAccess::WRITE_FILE));
    }

    #[test]
    fn test_landlock_manager() {
        let manager = LandlockManager::new();

        let mut ruleset = LandlockRuleset::new();
        ruleset.add_rule(LandlockRule::new("/home".to_string(), LandlockAccess::READ_FILE));

        let domain_id = manager.create_domain(ruleset.clone());
        assert_eq!(domain_id, 1);

        assert!(manager.check_access(domain_id, "/home/user", LandlockAccess::READ_FILE));
        assert!(!manager.check_access(domain_id, "/home/user", LandlockAccess::WRITE_FILE));

        assert_eq!(manager.domain_count(), 1);

        manager.remove_domain(domain_id).unwrap();
        assert_eq!(manager.domain_count(), 0);
    }

    #[test]
    fn test_landlock_manager_multiple_domains() {
        let manager = LandlockManager::new();

        let mut ruleset1 = LandlockRuleset::new();
        ruleset1.add_rule(LandlockRule::new("/home".to_string(), LandlockAccess::READ_FILE));

        let mut ruleset2 = LandlockRuleset::new();
        ruleset2.add_rule(LandlockRule::new("/tmp".to_string(), LandlockAccess::WRITE_FILE));

        let domain_id1 = manager.create_domain(ruleset1);
        let domain_id2 = manager.create_domain(ruleset2);

        assert!(manager.check_access(domain_id1, "/home/user", LandlockAccess::READ_FILE));
        assert!(!manager.check_access(domain_id2, "/home/user", LandlockAccess::READ_FILE));

        assert!(manager.check_access(domain_id2, "/tmp/file", LandlockAccess::WRITE_FILE));
        assert!(!manager.check_access(domain_id1, "/tmp/file", LandlockAccess::WRITE_FILE));

        assert_eq!(manager.domain_count(), 2);
    }

    #[test]
    fn test_landlock_access_intersect() {
        let all = LandlockAccess::READ_FILE.union(LandlockAccess::WRITE_FILE);
        let read_only = LandlockAccess::READ_FILE;
        let intersection = all.intersect(read_only);

        assert_eq!(intersection, read_only);
    }

    #[test]
    fn test_landlock_default_deny() {
        let ruleset = LandlockRuleset::new();
        assert!(!ruleset.allows("/any/path", LandlockAccess::READ_FILE));
    }
}
