// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Aaryan Singh Chauhan

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Security policy type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityPolicyType {
    Authentication,
    Authorization,
    Encryption,
    Network,
    FileAccess,
}

/// Security policy
#[derive(Debug, Clone)]
pub struct SecurityPolicy {
    pub id: String,
    pub name: String,
    pub policy_type: SecurityPolicyType,
    pub enabled: bool,
    pub severity: String,
}

/// Security manager for managing security policies
/// Inspired by Linux security frameworks and Omarchy security setup
#[derive(Debug, Clone)]
pub struct SigmaSecurityManager2 {
    policies: Vec<SecurityPolicy>,
}

impl SigmaSecurityManager2 {
    pub fn new() -> Self {
        Self {
            policies: Vec::new(),
        }
    }

    /// Add a security policy
    pub fn add_policy(&mut self, policy: SecurityPolicy) {
        self.policies.push(policy);
    }

    /// Get policy by ID
    pub fn get_policy(&self, id: &str) -> Option<&SecurityPolicy> {
        self.policies.iter().find(|p| p.id == id)
    }

    /// Get all policies
    pub fn get_policies(&self) -> &[SecurityPolicy] {
        &self.policies
    }

    /// Enable a policy
    pub fn enable_policy(&mut self, id: &str) -> Result<String, String> {
        if let Some(policy) = self.policies.iter_mut().find(|p| p.id == id) {
            policy.enabled = true;
            Ok(format!("Enabled policy: {}", policy.name))
        } else {
            Err(format!("Policy not found: {}", id))
        }
    }

    /// Disable a policy
    pub fn disable_policy(&mut self, id: &str) -> Result<String, String> {
        if let Some(policy) = self.policies.iter_mut().find(|p| p.id == id) {
            policy.enabled = false;
            Ok(format!("Disabled policy: {}", policy.name))
        } else {
            Err(format!("Policy not found: {}", id))
        }
    }

    /// Get enabled policies
    pub fn get_enabled_policies(&self) -> Vec<&SecurityPolicy> {
        self.policies.iter().filter(|p| p.enabled).collect()
    }

    /// Get policies by type
    pub fn get_policies_by_type(&self, policy_type: SecurityPolicyType) -> Vec<&SecurityPolicy> {
        self.policies
            .iter()
            .filter(|p| p.policy_type == policy_type)
            .collect()
    }

    /// Get high severity policies
    pub fn get_high_severity_policies(&self) -> Vec<&SecurityPolicy> {
        self.policies
            .iter()
            .filter(|p| p.severity == "high" || p.severity == "critical")
            .collect()
    }

    /// Check if policy is enabled
    pub fn is_policy_enabled(&self, id: &str) -> bool {
        self.policies
            .iter()
            .any(|p| p.id == id && p.enabled)
    }
}

impl Default for SigmaSecurityManager2 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_policy() {
        let mut manager = SigmaSecurityManager2::new();
        let policy = SecurityPolicy {
            id: "auth-1".to_string(),
            name: "Password Policy".to_string(),
            policy_type: SecurityPolicyType::Authentication,
            enabled: true,
            severity: "high".to_string(),
        };
        manager.add_policy(policy);
        assert_eq!(manager.get_policies().len(), 1);
    }

    #[test]
    fn test_enable_policy() {
        let mut manager = SigmaSecurityManager2::new();
        let policy = SecurityPolicy {
            id: "auth-1".to_string(),
            name: "Password Policy".to_string(),
            policy_type: SecurityPolicyType::Authentication,
            enabled: false,
            severity: "high".to_string(),
        };
        manager.add_policy(policy);
        manager.enable_policy("auth-1").unwrap();
        assert!(manager.get_policy("auth-1").unwrap().enabled);
    }

    #[test]
    fn test_disable_policy() {
        let mut manager = SigmaSecurityManager2::new();
        let policy = SecurityPolicy {
            id: "auth-1".to_string(),
            name: "Password Policy".to_string(),
            policy_type: SecurityPolicyType::Authentication,
            enabled: true,
            severity: "high".to_string(),
        };
        manager.add_policy(policy);
        manager.disable_policy("auth-1").unwrap();
        assert!(!manager.get_policy("auth-1").unwrap().enabled);
    }

    #[test]
    fn test_get_enabled_policies() {
        let mut manager = SigmaSecurityManager2::new();
        let policy1 = SecurityPolicy {
            id: "auth-1".to_string(),
            name: "Password Policy".to_string(),
            policy_type: SecurityPolicyType::Authentication,
            enabled: true,
            severity: "high".to_string(),
        };
        let policy2 = SecurityPolicy {
            id: "auth-2".to_string(),
            name: "2FA Policy".to_string(),
            policy_type: SecurityPolicyType::Authentication,
            enabled: false,
            severity: "high".to_string(),
        };
        manager.add_policy(policy1);
        manager.add_policy(policy2);
        assert_eq!(manager.get_enabled_policies().len(), 1);
    }

    #[test]
    fn test_get_policies_by_type() {
        let mut manager = SigmaSecurityManager2::new();
        let policy1 = SecurityPolicy {
            id: "auth-1".to_string(),
            name: "Password Policy".to_string(),
            policy_type: SecurityPolicyType::Authentication,
            enabled: true,
            severity: "high".to_string(),
        };
        let policy2 = SecurityPolicy {
            id: "net-1".to_string(),
            name: "Firewall Policy".to_string(),
            policy_type: SecurityPolicyType::Network,
            enabled: true,
            severity: "high".to_string(),
        };
        manager.add_policy(policy1);
        manager.add_policy(policy2);
        assert_eq!(manager.get_policies_by_type(SecurityPolicyType::Authentication).len(), 1);
    }
}
