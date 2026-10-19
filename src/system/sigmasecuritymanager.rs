//! SigmaSecurityManager - Security Management System
//!
//! Provides comprehensive security management inspired by Arch, Debian, and CachyOS.
//! Supports firewall rules, SELinux/AppArmor, security policies, and access control.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Firewall action
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FirewallAction {
    Allow,
    Deny,
    Reject,
    Log,
}

/// Firewall protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FirewallProtocol {
    TCP,
    UDP,
    ICMP,
    All,
}

/// Firewall rule
#[derive(Debug, Clone)]
pub struct FirewallRule {
    pub id: String,
    pub name: String,
    pub action: FirewallAction,
    pub protocol: FirewallProtocol,
    pub source_address: String,
    pub source_port: Option<u16>,
    pub destination_address: String,
    pub destination_port: Option<u16>,
    pub enabled: bool,
}

impl FirewallRule {
    pub fn new(
        id: String,
        name: String,
        action: FirewallAction,
        protocol: FirewallProtocol,
    ) -> Self {
        Self {
            id,
            name,
            action,
            protocol,
            source_address: "0.0.0.0/0".to_string(),
            source_port: None,
            destination_address: "0.0.0.0/0".to_string(),
            destination_port: None,
            enabled: true,
        }
    }

    pub fn set_source(&mut self, address: String, port: Option<u16>) {
        self.source_address = address;
        self.source_port = port;
    }

    pub fn set_destination(&mut self, address: String, port: Option<u16>) {
        self.destination_address = address;
        self.destination_port = port;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

/// Security policy
#[derive(Debug, Clone)]
pub struct SecurityPolicy {
    pub id: String,
    pub name: String,
    pub policy_type: String, // SELinux, AppArmor
    pub mode: String,        // Enforcing, Permissive, Disabled
    pub enabled: bool,
}

impl SecurityPolicy {
    pub fn new(id: String, name: String, policy_type: String, mode: String) -> Self {
        Self {
            id,
            name,
            policy_type,
            mode,
            enabled: true,
        }
    }

    pub fn set_mode(&mut self, mode: String) {
        self.mode = mode;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

/// Access control entry
#[derive(Debug, Clone)]
pub struct AccessControlEntry {
    pub id: String,
    pub resource: String,
    pub user: String,
    pub permissions: String, // rwx, etc.
}

impl AccessControlEntry {
    pub fn new(id: String, resource: String, user: String, permissions: String) -> Self {
        Self {
            id,
            resource,
            user,
            permissions,
        }
    }
}

/// SigmaSecurityManager - Security management
#[derive(Debug)]
pub struct SigmaSecurityManager {
    firewall_rules: BTreeMap<String, FirewallRule>,
    security_policies: BTreeMap<String, SecurityPolicy>,
    access_controls: BTreeMap<String, AccessControlEntry>,
}

impl SigmaSecurityManager {
    pub fn new() -> Self {
        Self {
            firewall_rules: BTreeMap::new(),
            security_policies: BTreeMap::new(),
            access_controls: BTreeMap::new(),
        }
    }

    pub fn add_firewall_rule(&mut self, rule: FirewallRule) {
        self.firewall_rules.insert(rule.id.clone(), rule);
    }

    pub fn remove_firewall_rule(&mut self, id: &str) -> bool {
        self.firewall_rules.remove(id).is_some()
    }

    pub fn get_firewall_rule(&self, id: &str) -> Option<&FirewallRule> {
        self.firewall_rules.get(id)
    }

    pub fn add_security_policy(&mut self, policy: SecurityPolicy) {
        self.security_policies.insert(policy.id.clone(), policy);
    }

    pub fn remove_security_policy(&mut self, id: &str) -> bool {
        self.security_policies.remove(id).is_some()
    }

    pub fn get_security_policy(&self, id: &str) -> Option<&SecurityPolicy> {
        self.security_policies.get(id)
    }

    pub fn add_access_control(&mut self, ace: AccessControlEntry) {
        self.access_controls.insert(ace.id.clone(), ace);
    }

    pub fn remove_access_control(&mut self, id: &str) -> bool {
        self.access_controls.remove(id).is_some()
    }

    pub fn get_access_control(&self, id: &str) -> Option<&AccessControlEntry> {
        self.access_controls.get(id)
    }

    pub fn list_enabled_firewall_rules(&self) -> Vec<&FirewallRule> {
        self.firewall_rules
            .values()
            .filter(|rule| rule.enabled)
            .collect()
    }

    pub fn list_firewall_rules_by_action(&self, action: FirewallAction) -> Vec<&FirewallRule> {
        self.firewall_rules
            .values()
            .filter(|rule| rule.action == action)
            .collect()
    }

    pub fn list_enforcing_policies(&self) -> Vec<&SecurityPolicy> {
        self.security_policies
            .values()
            .filter(|policy| policy.enabled && policy.mode == "Enforcing")
            .collect()
    }

    pub fn list_access_by_resource(&self, resource: &str) -> Vec<&AccessControlEntry> {
        self.access_controls
            .values()
            .filter(|ace| ace.resource == resource)
            .collect()
    }

    pub fn firewall_rule_count(&self) -> usize {
        self.firewall_rules.len()
    }

    pub fn security_policy_count(&self) -> usize {
        self.security_policies.len()
    }

    pub fn access_control_count(&self) -> usize {
        self.access_controls.len()
    }
}

impl Default for SigmaSecurityManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaSecurityManager instance
static mut SIGMASECURITYMANAGER_INSTANCE: Option<SigmaSecurityManager> = None;

/// Initialize the global SigmaSecurityManager instance
pub fn init_sigmasecuritymanager() {
    unsafe {
        SIGMASECURITYMANAGER_INSTANCE = Some(SigmaSecurityManager::new());
    }
}

/// Get the global SigmaSecurityManager instance
pub fn get_sigmasecuritymanager() -> Option<&'static SigmaSecurityManager> {
    unsafe { SIGMASECURITYMANAGER_INSTANCE.as_ref() }
}

/// Get the global SigmaSecurityManager instance (mutable)
pub fn get_sigmasecuritymanager_mut() -> Option<&'static mut SigmaSecurityManager> {
    unsafe { SIGMASECURITYMANAGER_INSTANCE.as_mut() }
}

/// Convenience function to add a firewall rule
pub fn add_firewall_rule(rule: FirewallRule) -> bool {
    if let Some(manager) = get_sigmasecuritymanager_mut() {
        manager.add_firewall_rule(rule);
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_firewall_rule_creation() {
        let rule = FirewallRule::new(
            "rule1".to_string(),
            "Allow SSH".to_string(),
            FirewallAction::Allow,
            FirewallProtocol::TCP,
        );
        assert_eq!(rule.id, "rule1");
        assert_eq!(rule.action, FirewallAction::Allow);
    }

    #[test]
    fn test_firewall_rule_modification() {
        let mut rule = FirewallRule::new(
            "rule1".to_string(),
            "Test".to_string(),
            FirewallAction::Deny,
            FirewallProtocol::UDP,
        );
        rule.set_source("192.168.1.0/24".to_string(), Some(53));
        rule.set_destination("0.0.0.0/0".to_string(), None);
        rule.set_enabled(false);
        assert_eq!(rule.source_port, Some(53));
        assert!(!rule.enabled);
    }

    #[test]
    fn test_security_policy_creation() {
        let policy = SecurityPolicy::new(
            "policy1".to_string(),
            "SELinux Policy".to_string(),
            "SELinux".to_string(),
            "Enforcing".to_string(),
        );
        assert_eq!(policy.id, "policy1");
        assert_eq!(policy.mode, "Enforcing");
    }

    #[test]
    fn test_access_control_creation() {
        let ace = AccessControlEntry::new(
            "ace1".to_string(),
            "/etc/shadow".to_string(),
            "root".to_string(),
            "rw".to_string(),
        );
        assert_eq!(ace.id, "ace1");
        assert_eq!(ace.resource, "/etc/shadow");
    }

    #[test]
    fn test_sigmasecuritymanager_creation() {
        let manager = SigmaSecurityManager::new();
        assert_eq!(manager.firewall_rule_count(), 0);
        assert_eq!(manager.security_policy_count(), 0);
    }

    #[test]
    fn test_sigmasecuritymanager_management() {
        let mut manager = SigmaSecurityManager::new();
        let rule = FirewallRule::new(
            "rule1".to_string(),
            "Test".to_string(),
            FirewallAction::Allow,
            FirewallProtocol::TCP,
        );
        manager.add_firewall_rule(rule);
        assert_eq!(manager.firewall_rule_count(), 1);
    }

    #[test]
    fn test_action_filtering() {
        let mut manager = SigmaSecurityManager::new();
        let rule = FirewallRule::new(
            "rule1".to_string(),
            "Deny".to_string(),
            FirewallAction::Deny,
            FirewallProtocol::TCP,
        );
        manager.add_firewall_rule(rule);
        let deny_rules = manager.list_firewall_rules_by_action(FirewallAction::Deny);
        assert_eq!(deny_rules.len(), 1);
    }
}
