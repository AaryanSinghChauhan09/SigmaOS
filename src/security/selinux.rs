// SELinux-style Security Policy Framework
// Linux-style mandatory access control with policy enforcement

#![no_std]

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppArmorMode {
    Enforce,
    Complain,
    Audit,
    Disabled,
}

#[derive(Debug, Clone)]
pub struct AppArmorPathRule {
    pub path_pattern: String,
    pub permissions: String, // e.g. "rw", "rx", "r"
}

#[derive(Debug, Clone)]
pub struct AppArmorCapabilityRule {
    pub capability: String, // e.g. "sys_admin", "net_bind_service"
}

#[derive(Debug, Clone)]
pub struct AppArmorNetworkRule {
    pub domain: String, // e.g. "inet", "inet6"
    pub protocol: String, // e.g. "tcp", "udp"
}

#[derive(Debug, Clone)]
pub struct AppArmorProfile {
    pub name: String,
    pub path: String, // legacy backward-compatible path prefix
    pub permissions: Vec<SelinuxPermission>, // legacy backward-compatible permissions
    pub enabled: bool, // legacy backward-compatible status
    pub mode: AppArmorMode, // advanced profile mode (Ubuntu/Debian style)
    pub path_rules: Vec<AppArmorPathRule>, // advanced file path matching rules with globbing
    pub capability_rules: Vec<AppArmorCapabilityRule>, // advanced capabilities restrictions
    pub network_rules: Vec<AppArmorNetworkRule>, // advanced network socket creation controls
}

impl AppArmorProfile {
    /// Create new backward-compatible profile (legacy)
    pub fn new_legacy(name: String, path: String, permissions: Vec<SelinuxPermission>, enabled: bool) -> Self {
        Self {
            name,
            path,
            permissions,
            enabled,
            mode: if enabled { AppArmorMode::Enforce } else { AppArmorMode::Disabled },
            path_rules: Vec::new(),
            capability_rules: Vec::new(),
            network_rules: Vec::new(),
        }
    }

    /// Create new AppArmor profile with advanced rules (modern distros style)
    pub fn new(name: String, mode: AppArmorMode) -> Self {
        Self {
            name,
            path: String::new(),
            permissions: Vec::new(),
            enabled: mode != AppArmorMode::Disabled,
            mode,
            path_rules: Vec::new(),
            capability_rules: Vec::new(),
            network_rules: Vec::new(),
        }
    }

    pub fn with_path_rule(mut self, pattern: String, permissions: String) -> Self {
        self.path_rules.push(AppArmorPathRule {
            path_pattern: pattern,
            permissions,
        });
        self
    }

    pub fn with_capability_rule(mut self, capability: String) -> Self {
        self.capability_rules.push(AppArmorCapabilityRule { capability });
        self
    }

    pub fn with_network_rule(mut self, domain: String, protocol: String) -> Self {
        self.network_rules.push(AppArmorNetworkRule { domain, protocol });
        self
    }
}

/// Helper to match path patterns with simple globbing
pub fn match_path_pattern(pattern: &str, requested: &str) -> bool {
    if pattern == requested {
        return true;
    }

    fn glob_match(pattern_chars: &[char], req_chars: &[char]) -> bool {
        match (pattern_chars, req_chars) {
            ([], []) => true,
            (['*', '*', tail @ ..], _) => {
                for i in 0..=req_chars.len() {
                    if glob_match(tail, &req_chars[i..]) {
                        return true;
                    }
                }
                false
            }
            (['*', tail @ ..], _) => {
                if req_chars.is_empty() {
                    return glob_match(tail, req_chars);
                }
                for i in 0..=req_chars.len() {
                    if i > 0 && req_chars[i - 1] == '/' {
                        break;
                    }
                    if glob_match(tail, &req_chars[i..]) {
                        return true;
                    }
                }
                false
            }
            ([p, p_tail @ ..], [r, r_tail @ ..]) if *p == *r => {
                glob_match(p_tail, r_tail)
            }
            _ => false,
        }
    }

    let p_chars: Vec<char> = pattern.chars().collect();
    let r_chars: Vec<char> = requested.chars().collect();
    glob_match(&p_chars, &r_chars)
}

#[derive(Debug, Clone)]
pub struct AppArmorAuditLog {
    pub profile_name: String,
    pub mode: AppArmorMode,
    pub action: String,
    pub target: String,
    pub allowed: bool,
}

pub struct AppArmorManager {
    profiles: BTreeMap<String, AppArmorProfile>,
    enforcing_mode: bool,
    pub audit_logs: Vec<AppArmorAuditLog>,
}

impl AppArmorManager {
    pub fn new() -> Self {
        Self {
            profiles: BTreeMap::new(),
            enforcing_mode: true,
            audit_logs: Vec::new(),
        }
    }

    /// Add an AppArmor profile
    pub fn add_profile(&mut self, profile: AppArmorProfile) -> Result<(), &'static str> {
        self.profiles.insert(profile.name.clone(), profile);
        Ok(())
    }

    /// Check if a path is allowed by its profile (backward-compatible legacy fallback)
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

    /// Check file path access under a specific profile with globbing support (Ubuntu style)
    pub fn check_file_access(&mut self, profile_name: &str, requested_path: &str, requested_perm: char) -> bool {
        let profile = match self.profiles.get(profile_name) {
            Some(p) => p,
            None => return !self.enforcing_mode,
        };

        if profile.mode == AppArmorMode::Disabled || !profile.enabled {
            return true;
        }

        let mut allowed = false;
        for rule in &profile.path_rules {
            if match_path_pattern(&rule.path_pattern, requested_path) {
                if rule.permissions.contains(requested_perm) {
                    allowed = true;
                    break;
                }
            }
        }

        // Log to audit if Complain, Audit, or Enforce fails
        if profile.mode == AppArmorMode::Audit || !allowed || profile.mode == AppArmorMode::Complain {
            let log = AppArmorAuditLog {
                profile_name: profile_name.to_string(),
                mode: profile.mode,
                action: alloc::format!("file_access:{}", requested_perm),
                target: requested_path.to_string(),
                allowed: allowed || profile.mode == AppArmorMode::Complain,
            };
            self.audit_logs.push(log);
        }

        if profile.mode == AppArmorMode::Complain {
            return true; // Complain mode always allows but logs warning
        }

        allowed
    }

    /// Check capability access (restrict process privileges)
    pub fn check_capability(&mut self, profile_name: &str, capability: &str) -> bool {
        let profile = match self.profiles.get(profile_name) {
            Some(p) => p,
            None => return !self.enforcing_mode,
        };

        if profile.mode == AppArmorMode::Disabled || !profile.enabled {
            return true;
        }

        let allowed = profile.capability_rules.iter().any(|r| r.capability == capability);

        if profile.mode == AppArmorMode::Audit || !allowed || profile.mode == AppArmorMode::Complain {
            let log = AppArmorAuditLog {
                profile_name: profile_name.to_string(),
                mode: profile.mode,
                action: "capability".to_string(),
                target: capability.to_string(),
                allowed: allowed || profile.mode == AppArmorMode::Complain,
            };
            self.audit_logs.push(log);
        }

        if profile.mode == AppArmorMode::Complain {
            return true;
        }

        allowed
    }

    /// Check network socket creation (restrict socket domain/protocols)
    pub fn check_network(&mut self, profile_name: &str, domain: &str, protocol: &str) -> bool {
        let profile = match self.profiles.get(profile_name) {
            Some(p) => p,
            None => return !self.enforcing_mode,
        };

        if profile.mode == AppArmorMode::Disabled || !profile.enabled {
            return true;
        }

        let allowed = profile.network_rules.iter().any(|r| r.domain == domain && r.protocol == protocol);

        if profile.mode == AppArmorMode::Audit || !allowed || profile.mode == AppArmorMode::Complain {
            let log = AppArmorAuditLog {
                profile_name: profile_name.to_string(),
                mode: profile.mode,
                action: "network".to_string(),
                target: alloc::format!("{}:{}", domain, protocol),
                allowed: allowed || profile.mode == AppArmorMode::Complain,
            };
            self.audit_logs.push(log);
        }

        if profile.mode == AppArmorMode::Complain {
            return true;
        }

        allowed
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
            mode: AppArmorMode::Enforce,
            path_rules: Vec::new(),
            capability_rules: Vec::new(),
            network_rules: Vec::new(),
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

    #[test]
    fn test_apparmor_glob_matching() {
        // Test single asterisk '*' wildcard (matches within directories)
        assert!(match_path_pattern("/var/log/*", "/var/log/syslog"));
        assert!(match_path_pattern("/var/log/*", "/var/log/auth.log"));
        assert!(!match_path_pattern("/var/log/*", "/var/log/nginx/access.log")); // should fail (recursive path)

        // Test double asterisk '**' wildcard (matches recursively)
        assert!(match_path_pattern("/home/**/*.txt", "/home/ubuntu/notes.txt"));
        assert!(match_path_pattern("/home/**/*.txt", "/home/guest/documents/secret.txt"));
        assert!(!match_path_pattern("/home/**/*.txt", "/home/ubuntu/notes.pdf")); // wrong extension
    }

    #[test]
    fn test_apparmor_modes_and_auditing() {
        let mut manager = AppArmorManager::new();

        // 1. Enforce mode - strictly denies access if no rule matches
        let profile_enforce = AppArmorProfile::new("restricted_bin".to_string(), AppArmorMode::Enforce)
            .with_path_rule("/var/log/*.log".to_string(), "r".to_string());
        manager.add_profile(profile_enforce).unwrap();

        assert!(manager.check_file_access("restricted_bin", "/var/log/syslog.log", 'r'));
        assert!(!manager.check_file_access("restricted_bin", "/etc/shadow", 'r')); // denied

        // 2. Complain mode - allows access but logs a warning
        let profile_complain = AppArmorProfile::new("complain_bin".to_string(), AppArmorMode::Complain)
            .with_path_rule("/var/log/*.log".to_string(), "r".to_string());
        manager.add_profile(profile_complain).unwrap();

        // Should be allowed and audited
        assert!(manager.check_file_access("complain_bin", "/etc/shadow", 'r'));
        assert_eq!(manager.audit_logs.len(), 2); // 1 from restricted_bin deny, 1 from complain_bin bypass
        assert_eq!(manager.audit_logs[1].profile_name, "complain_bin");
        assert_eq!(manager.audit_logs[1].mode, AppArmorMode::Complain);
        assert!(manager.audit_logs[1].allowed); // Complain allows
    }

    #[test]
    fn test_apparmor_capabilities_and_networks() {
        let mut manager = AppArmorManager::new();

        let profile = AppArmorProfile::new("networking_daemon".to_string(), AppArmorMode::Enforce)
            .with_capability_rule("net_bind_service".to_string())
            .with_network_rule("inet".to_string(), "tcp".to_string());
        manager.add_profile(profile).unwrap();

        // Verify capability restriction
        assert!(manager.check_capability("networking_daemon", "net_bind_service"));
        assert!(!manager.check_capability("networking_daemon", "sys_admin")); // denied

        // Verify network restriction
        assert!(manager.check_network("networking_daemon", "inet", "tcp"));
        assert!(!manager.check_network("networking_daemon", "inet", "udp")); // denied
    }
}
