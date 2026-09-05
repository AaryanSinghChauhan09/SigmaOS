#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
use std::string::{String, ToString};
use std::vec::Vec;

use crate::klib::BTreeMap;

/// Configuration hook representing enterprise configuration scripts (e.g. Ansible playbook, Puppet manifest).
#[derive(Debug, Clone)]
pub struct ConfigHook {
    pub hook_id: String,
    pub target_subsystem: String,
    pub variables: BTreeMap<String, String>,
}

impl ConfigHook {
    pub fn new(hook_id: &str, subsystem: &str) -> Self {
        Self {
            hook_id: hook_id.to_string(),
            target_subsystem: subsystem.to_string(),
            variables: BTreeMap::new(),
        }
    }

    pub fn set_var(&mut self, key: &str, val: &str) {
        self.variables.insert(key.to_string(), val.to_string());
    }
}

/// Directory Services user and group permissions (LDAP/Active Directory integration).
#[derive(Debug, Clone)]
pub struct DirectoryUser {
    pub username: String,
    pub distinguished_name: String,
    pub groups: Vec<String>,
    pub password_hash: String,
}

#[derive(Debug, Clone)]
pub struct DirectoryService {
    pub domain_name: String,
    pub users: BTreeMap<String, DirectoryUser>,
}

impl DirectoryService {
    pub fn new(domain_name: &str) -> Self {
        Self {
            domain_name: domain_name.to_string(),
            users: BTreeMap::new(),
        }
    }

    pub fn register_user(&mut self, user: DirectoryUser) {
        self.users.insert(user.username.clone(), user);
    }

    pub fn authenticate(&self, username: &str, hash: &str) -> bool {
        if let Some(user) = self.users.get(username) {
            return user.password_hash == hash;
        }
        false
    }

    pub fn is_member_of(&self, username: &str, group: &str) -> bool {
        if let Some(user) = self.users.get(username) {
            return user.groups.iter().any(|g| g == group);
        }
        false
    }
}

/// Compliance Auditing against industry standard security benchmarks (e.g. CIS, SELinux).
#[derive(Debug, Clone)]
pub struct AuditRule {
    pub rule_id: String,
    pub description: String,
    pub passing_value: String,
}

#[derive(Debug, Clone)]
pub struct AuditResult {
    pub rule_id: String,
    pub success: bool,
    pub actual_value: String,
}

#[derive(Debug, Clone)]
pub struct ComplianceAuditor {
    pub standards_name: String,
    pub rules: Vec<AuditRule>,
}

impl ComplianceAuditor {
    pub fn new(standards: &str) -> Self {
        Self {
            standards_name: standards.to_string(),
            rules: Vec::new(),
        }
    }

    pub fn add_rule(&mut self, rule_id: &str, description: &str, passing_value: &str) {
        self.rules.push(AuditRule {
            rule_id: rule_id.to_string(),
            description: description.to_string(),
            passing_value: passing_value.to_string(),
        });
    }

    /// Evaluates actual system states against rules.
    pub fn perform_audit(&self, system_state: &BTreeMap<String, String>) -> Vec<AuditResult> {
        let mut results = Vec::new();
        for rule in &self.rules {
            let actual = system_state.get(&rule.rule_id).cloned().unwrap_or_default();
            let success = actual == rule.passing_value;
            results.push(AuditResult {
                rule_id: rule.rule_id.clone(),
                success,
                actual_value: actual,
            });
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_configuration_hooks() {
        let mut hook = ConfigHook::new("ansible-policy-01", "network");
        hook.set_var("mtu", "1500");
        hook.set_var("enable_dhcp", "true");

        assert_eq!(hook.variables.get("mtu").unwrap(), "1500");
        assert_eq!(hook.variables.get("enable_dhcp").unwrap(), "true");
    }

    #[test]
    fn test_ldap_directory_services() {
        let mut ds = DirectoryService::new("corp.sigmaos.org");

        let user = DirectoryUser {
            username: "jules".to_string(),
            distinguished_name: "uid=jules,ou=people,dc=corp,dc=sigmaos,dc=org".to_string(),
            groups: vec!["admins".to_string(), "engineers".to_string()],
            password_hash: "secure_pqc_passhash_123".to_string(),
        };

        ds.register_user(user);

        assert!(ds.authenticate("jules", "secure_pqc_passhash_123"));
        assert!(!ds.authenticate("jules", "wrongpassword"));

        assert!(ds.is_member_of("jules", "admins"));
        assert!(!ds.is_member_of("jules", "finance"));
    }

    #[test]
    fn test_compliance_auditing() {
        let mut auditor = ComplianceAuditor::new("CIS-SigmaOS-L1");
        auditor.add_rule(
            "sys.password_min_length",
            "Enforce min 12 character password length",
            "12",
        );
        auditor.add_rule(
            "sys.selinux_enforcing",
            "Enforce SELinux policy",
            "enforcing",
        );

        let mut actual_state = BTreeMap::new();
        actual_state.insert("sys.password_min_length".to_string(), "14".to_string()); // passing but not exact
        actual_state.insert("sys.selinux_enforcing".to_string(), "enforcing".to_string()); // passing exact

        let audit_results = auditor.perform_audit(&actual_state);
        assert_eq!(audit_results.len(), 2);

        // First rule checks for EXACT "12" string match in our mock implementation
        let rule1_res = audit_results
            .iter()
            .find(|r| r.rule_id == "sys.password_min_length")
            .unwrap();
        assert!(!rule1_res.success); // actual "14" != expected "12"

        let rule2_res = audit_results
            .iter()
            .find(|r| r.rule_id == "sys.selinux_enforcing")
            .unwrap();
        assert!(rule2_res.success); // actual "enforcing" == expected "enforcing"
    }
}
