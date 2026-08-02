#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SELinux-style Security Policy Framework
// Linux-style mandatory access control with policy enforcement

// (no_std only applicable at crate root - removed)

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityContext {
    Unconfined,
    System,
    User,
    Guest,
    Container,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelinuxPermission {
    Read,
    Write,
    Execute,
    Create,
    Delete,
    Connect,
    Bind,
    Accept,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectType {
    File,
    Directory,
    Socket,
    Process,
    Network,
    Device,
}

#[derive(Debug, Clone)]
pub struct SecurityLabel {
    pub user: String,
    pub role: String,
    pub type_: String,
    pub level: String,
}

#[derive(Debug, Clone)]
pub struct SecurityRule {
    pub source: SecurityLabel,
    pub target: SecurityLabel,
    pub object_type: ObjectType,
    pub permissions: Vec<SelinuxPermission>,
    pub enabled: bool,
}

pub struct SecurityPolicy {
    rules: Vec<SecurityRule>,
    default_context: SecurityContext,
    enforcing_mode: bool,
}

impl SecurityPolicy {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            default_context: SecurityContext::Unconfined,
            enforcing_mode: true,
        }
    }

    /// Add a security rule
    pub fn add_rule(&mut self, rule: SecurityRule) -> Result<(), &'static str> {
        self.rules.push(rule);
        Ok(())
    }

    /// Check if a permission is allowed
    pub fn check_permission(
        &self,
        source: &SecurityLabel,
        target: &SecurityLabel,
        object_type: ObjectType,
        permission: SelinuxPermission,
    ) -> bool {
        if !self.enforcing_mode {
            return true; // Permissive mode
        }

        for rule in &self.rules {
            if !rule.enabled {
                continue;
            }

            if self.labels_match(&rule.source, source)
                && self.labels_match(&rule.target, target)
                && rule.object_type == object_type
                && rule.permissions.contains(&permission)
            {
                return true;
            }
        }

        false
    }

    /// Check if security labels match
    fn labels_match(&self, rule_label: &SecurityLabel, check_label: &SecurityLabel) -> bool {
        // Wildcard matching - empty strings match anything
        let user_match = rule_label.user.is_empty() || rule_label.user == check_label.user;
        let role_match = rule_label.role.is_empty() || rule_label.role == check_label.role;
        let type_match = rule_label.type_.is_empty() || rule_label.type_ == check_label.type_;
        let level_match = rule_label.level.is_empty() || rule_label.level == check_label.level;

        user_match && role_match && type_match && level_match
    }

    /// Enable or disable enforcing mode
    pub fn set_enforcing(&mut self, enforcing: bool) {
        self.enforcing_mode = enforcing;
    }

    /// Get enforcing mode
    pub fn is_enforcing(&self) -> bool {
        self.enforcing_mode
    }

    /// Set default security context
    pub fn set_default_context(&mut self, context: SecurityContext) {
        self.default_context = context;
    }

    /// Get default security context
    pub fn default_context(&self) -> SecurityContext {
        self.default_context
    }

    /// Get rule count
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    /// Delete a rule by index
    pub fn delete_rule(&mut self, index: usize) -> Result<(), &'static str> {
        if index >= self.rules.len() {
            return Err("Rule index out of bounds");
        }
        self.rules.remove(index);
        Ok(())
    }

    /// Get all rules
    pub fn get_rules(&self) -> &[SecurityRule] {
        &self.rules
    }
}

impl Default for SecurityPolicy {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct AppArmorProfile {
    pub name: String,
    pub path: String,
    pub permissions: Vec<SelinuxPermission>,
    pub enabled: bool,
}

pub struct AppArmorManager {
    profiles: BTreeMap<String, AppArmorProfile>,
    enforcing_mode: bool,
}

impl AppArmorManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            profiles: BTreeMap::new(),
            enforcing_mode: true,
        }
    }

    /// Add an AppArmor profile
    pub fn add_profile(&mut self, profile: AppArmorProfile) -> Result<(), &'static str> {
        self.profiles.insert(profile.name.clone(), profile);
        Ok(())
    }

    /// Check if a path is allowed by its profile
    pub fn check_path(&self, path: &str, permission: SelinuxPermission) -> bool {
        if !self.enforcing_mode {
            return true;
        }

        for profile in self.profiles.values() {
            if path.starts_with(&profile.path) && profile.enabled && profile.permissions.contains(&permission) {
                return true;
            }
        }

        false
    }

    /// Enable or disable enforcing mode
    pub fn set_enforcing(&mut self, enforcing: bool) {
        self.enforcing_mode = enforcing;
    }

    /// Get enforcing mode
    pub fn is_enforcing(&self) -> bool {
        self.enforcing_mode
    }

    /// Get profile count
    pub fn profile_count(&self) -> usize {
        self.profiles.len()
    }

    /// Delete a profile
    pub fn delete_profile(&mut self, name: &str) -> Result<(), &'static str> {
        self.profiles.remove(name).ok_or("Profile not found")?;
        Ok(())
    }

    /// Get all profiles
    pub fn get_profiles(&self) -> Vec<&AppArmorProfile> {
        self.profiles.values().collect()
    }
}

impl Default for AppArmorManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_policy() {
        let mut policy = SecurityPolicy::new();

        let source = SecurityLabel {
            user: "system_u".to_string(),
            role: "system_r".to_string(),
            type_: "system_t".to_string(),
            level: "s0".to_string(),
        };

        let target = SecurityLabel {
            user: "system_u".to_string(),
            role: "object_r".to_string(),
            type_: "etc_t".to_string(),
            level: "s0".to_string(),
        };

        let rule = SecurityRule {
            source: source.clone(),
            target: target.clone(),
            object_type: ObjectType::File,
            permissions: vec![SelinuxPermission::Read, SelinuxPermission::Write],
            enabled: true,
        };

        policy.add_rule(rule).unwrap();
        assert_eq!(policy.rule_count(), 1);

        let allowed =
            policy.check_permission(&source, &target, ObjectType::File, SelinuxPermission::Read);
        assert!(allowed);
    }

    #[test]
    fn test_enforcing_mode() {
        let mut policy = SecurityPolicy::new();

        policy.set_enforcing(false);
        assert!(!policy.is_enforcing());

        policy.set_enforcing(true);
        assert!(policy.is_enforcing());
    }

    #[test]
    fn test_wildcard_matching() {
        let mut policy = SecurityPolicy::new();

        let source = SecurityLabel {
            user: "".to_string(), // Wildcard
            role: "".to_string(),
            type_: "".to_string(),
            level: "".to_string(),
        };

        let target = SecurityLabel {
            user: "system_u".to_string(),
            role: "object_r".to_string(),
            type_: "etc_t".to_string(),
            level: "s0".to_string(),
        };

        let rule = SecurityRule {
            source: source.clone(),
            target: target.clone(),
            object_type: ObjectType::File,
            permissions: vec![SelinuxPermission::Read],
            enabled: true,
        };

        policy.add_rule(rule).unwrap();

        let check_source = SecurityLabel {
            user: "any_user".to_string(),
            role: "any_role".to_string(),
            type_: "any_type".to_string(),
            level: "s0".to_string(),
        };

        let allowed = policy.check_permission(
            &check_source,
            &target,
            ObjectType::File,
            SelinuxPermission::Read,
        );
        assert!(allowed);
    }

    #[test]
    fn test_apparmor_manager() {
        let mut manager = AppArmorManager::new();

        let profile = AppArmorProfile {
            name: "test_profile".to_string(),
            path: "/etc/".to_string(),
            permissions: vec![SelinuxPermission::Read, SelinuxPermission::Write],
            enabled: true,
        };

        manager.add_profile(profile).unwrap();
        assert_eq!(manager.profile_count(), 1);

        let allowed = manager.check_path("/etc/passwd", SelinuxPermission::Read);
        assert!(allowed);
    }

    #[test]
    fn test_apparmor_enforcing() {
        let mut manager = AppArmorManager::new();

        manager.set_enforcing(false);
        assert!(!manager.is_enforcing());

        manager.set_enforcing(true);
        assert!(manager.is_enforcing());
    }

    #[test]
    fn test_delete_rule() {
        let mut policy = SecurityPolicy::new();

        let source = SecurityLabel {
            user: "system_u".to_string(),
            role: "system_r".to_string(),
            type_: "system_t".to_string(),
            level: "s0".to_string(),
        };

        let target = SecurityLabel {
            user: "system_u".to_string(),
            role: "object_r".to_string(),
            type_: "etc_t".to_string(),
            level: "s0".to_string(),
        };

        let rule = SecurityRule {
            source,
            target,
            object_type: ObjectType::File,
            permissions: vec![SelinuxPermission::Read],
            enabled: true,
        };

        policy.add_rule(rule).unwrap();
        policy.delete_rule(0).unwrap();

        assert_eq!(policy.rule_count(), 0);
    }
}
