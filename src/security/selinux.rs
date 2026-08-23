#![cfg_attr(not(test), no_std)]

extern crate alloc;

// Fedora-inspired SELinux (Security-Enhanced Linux) Mandatory Access Control Subsystem.
// Implements labeling security contexts (user:role:type:sensitivity), enforcement modes,
// an Access Vector Cache (AVC) for performance, policy rules, and detailed audit logging.

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectType {
    File,
    Process,
    Socket,
    Ipc,
    Capability,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SelinuxPermission {
    Read,
    Write,
    Execute,
    Append,
    Transition,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SecurityLabel {
    pub label: String,
}

impl SecurityLabel {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityRule {
    pub source_type: String,
    pub target_type: String,
    pub object_type: ObjectType,
    pub permission: SelinuxPermission,
    pub allow: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityPolicy {
    pub rules: Vec<SecurityRule>,
}

impl SecurityPolicy {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, rule: SecurityRule) {
        self.rules.push(rule);
    }

    pub fn check_permission(
        &self,
        source_type: &str,
        target_type: &str,
        obj_type: ObjectType,
        perm: SelinuxPermission,
    ) -> bool {
        for rule in &self.rules {
            if rule.source_type == source_type
                && rule.target_type == target_type
                && rule.object_type == obj_type
                && rule.permission == perm
            {
                return rule.allow;
            }
        }
        false
    }
}

#[derive(Debug, Clone)]
pub struct AppArmorProfile {
    pub name: String,
    pub attachments: Vec<String>, // path attachments
    pub allow_rules: HashSet<String>,
}

impl AppArmorProfile {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            attachments: Vec::new(),
            allow_rules: HashSet::new(),
        }
    }

    pub fn add_allow_rule(&mut self, rule: &str) {
        self.allow_rules.insert(rule.to_string());
    }

    pub fn is_allowed(&self, path: &str) -> bool {
        self.allow_rules.contains(path)
    }
}

pub struct AppArmorManager {
    pub profiles: HashMap<String, AppArmorProfile>,
}

impl AppArmorManager {
    pub fn new() -> Self {
        Self {
            profiles: HashMap::new(),
        }
    }

    pub fn load_profile(&mut self, profile: AppArmorProfile) {
        self.profiles.insert(profile.name.clone(), profile);
    }

    pub fn check_access(&self, profile_name: &str, path: &str) -> bool {
        if let Some(profile) = self.profiles.get(profile_name) {
            profile.is_allowed(path)
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeLinuxMode {
    Enforcing,
    Permissive,
    Disabled,
}

impl SeLinuxMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            SeLinuxMode::Enforcing => "enforcing",
            SeLinuxMode::Permissive => "permissive",
            SeLinuxMode::Disabled => "disabled",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SecurityContext {
    pub user: String,
    pub role: String,
    pub type_name: String,
    pub sensitivity: String,
}

impl SecurityContext {
    pub fn parse(context_str: &str) -> Result<Self, &'static str> {
        let parts: Vec<&str> = context_str.split(':').collect();
        if parts.len() != 4 {
            return Err("Invalid SELinux context format! Must be user:role:type:sensitivity");
        }
        Ok(Self {
            user: parts[0].to_string(),
            role: parts[1].to_string(),
            type_name: parts[2].to_string(),
            sensitivity: parts[3].to_string(),
        })
    }

    pub fn to_string(&self) -> String {
        format!(
            "{}:{}:{}:{}",
            self.user, self.role, self.type_name, self.sensitivity
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AvcKey {
    pub source_type: String,
    pub target_type: String,
    pub class: String,
    pub permission: String,
}

pub struct AccessVectorCache {
    pub cache: HashMap<AvcKey, bool>,
    pub hits: usize,
    pub misses: usize,
}

impl AccessVectorCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            hits: 0,
            misses: 0,
        }
    }

    pub fn query(&mut self, key: &AvcKey) -> Option<bool> {
        if let Some(&decision) = self.cache.get(key) {
            self.hits += 1;
            Some(decision)
        } else {
            self.misses += 1;
            None
        }
    }

    pub fn insert(&mut self, key: AvcKey, decision: bool) {
        self.cache.insert(key, decision);
    }

    pub fn clear(&mut self) {
        self.cache.clear();
        self.hits = 0;
        self.misses = 0;
    }
}

pub struct PolicyRule {
    pub source_type: String,
    pub target_type: String,
    pub class: String,
    pub permission: String,
}

pub struct AppArmorManager;
pub struct AppArmorProfile;
pub enum ObjectType {
    File,
    Directory,
}
pub struct SecurityLabel;
pub struct SecurityPolicy;
pub struct SecurityRule;
pub enum SelinuxPermission {
    Read,
    Write,
    Execute,
}
pub type Permission = SelinuxPermission;

pub struct SelinuxEngine {
    pub mode: SeLinuxMode,
    pub policies: HashSet<AvcKey>,
    pub avc: AccessVectorCache,
    pub audit_logs: Vec<String>,
}

impl SelinuxEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            mode: SeLinuxMode::Enforcing,
            policies: HashSet::new(),
            avc: AccessVectorCache::new(),
            audit_logs: Vec::new(),
        };
        engine.load_default_policies();
        engine
    }

    fn load_default_policies(&mut self) {
        // httpd_t can read httpd_sys_content_t files
        self.allow("httpd_t", "httpd_sys_content_t", "file", "read");

        // unconfined_t can do everything
        self.allow("unconfined_t", "admin_home_t", "file", "read");
        self.allow("unconfined_t", "admin_home_t", "file", "write");
        self.allow("unconfined_t", "unconfined_t", "process", "transition");
    }

    pub fn allow(&mut self, source: &str, target: &str, class: &str, permission: &str) {
        let key = AvcKey {
            source_type: source.to_string(),
            target_type: target.to_string(),
            class: class.to_string(),
            permission: permission.to_string(),
        };
        self.policies.insert(key);
        self.avc.clear(); // Flush cache on policy update
    }

    pub fn set_mode(&mut self, mode: SeLinuxMode) {
        self.mode = mode;
        self.avc.clear();
    }

    /// Verifies access between a source context and target context
    pub fn has_permission(
        &mut self,
        source: &str,
        target: &str,
        class: &str,
        permission: &str,
    ) -> Result<bool, &'static str> {
        if self.mode == SeLinuxMode::Disabled {
            return Ok(true);
        }

        let src_context = SecurityContext::parse(source)?;
        let tgt_context = SecurityContext::parse(target)?;

        let avc_key = AvcKey {
            source_type: src_context.type_name.clone(),
            target_type: tgt_context.type_name.clone(),
            class: class.to_string(),
            permission: permission.to_string(),
        };

        // Query AVC cache
        let allowed = if let Some(decision) = self.avc.query(&avc_key) {
            decision
        } else {
            let decision = self.policies.contains(&avc_key);
            self.avc.insert(avc_key, decision);
            decision
        };

        if !allowed {
            // Log audit failure to memory buffer in standard auditd format
            let audit_entry = format!(
                "type=AVC msg=audit(1700000000.123:456): avc:  denied  {{ {} }} for  pid=1234 comm=\"service\" \
                 scontext={} tcontext={} tclass={}",
                permission, source, target, class
            );
            self.audit_logs.push(audit_entry);

            if self.mode == SeLinuxMode::Enforcing {
                return Ok(false); // Gated!
            } else {
                // Permissive mode allows but audits the alert
                return Ok(true);
            }
        }

        Ok(true)
    }
}

/// Multi-Level Security (MLS) sensitivity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SensitivityLevel {
    Unclassified = 0,
    Confidential = 1,
    Secret = 2,
    TopSecret = 3,
}

/// Dynamic Mandatory Access Control (MAC) MLS/MCS Enforcer (RHEL Parity)
pub struct DynamicMacEnforcer {
    pub process_levels: HashMap<String, SensitivityLevel>,
    pub object_levels: HashMap<String, SensitivityLevel>,
    pub process_categories: HashMap<String, HashSet<u32>>,
    pub object_categories: HashMap<String, HashSet<u32>>,
}

impl DynamicMacEnforcer {
    pub fn new() -> Self {
        Self {
            process_levels: HashMap::new(),
            object_levels: HashMap::new(),
            process_categories: HashMap::new(),
            object_categories: HashMap::new(),
        }
    }

    pub fn set_process_level(
        &mut self,
        process_id: &str,
        level: SensitivityLevel,
        categories: HashSet<u32>,
    ) {
        self.process_levels.insert(process_id.to_string(), level);
        self.process_categories
            .insert(process_id.to_string(), categories);
    }

    pub fn set_object_level(
        &mut self,
        object_id: &str,
        level: SensitivityLevel,
        categories: HashSet<u32>,
    ) {
        self.object_levels.insert(object_id.to_string(), level);
        self.object_categories
            .insert(object_id.to_string(), categories);
    }

    /// Read access check: No Read Up (Simple Security Property - Bell-LaPadula)
    pub fn can_read(&self, process_id: &str, object_id: &str) -> bool {
        let p_level = match self.process_levels.get(process_id) {
            Some(lvl) => *lvl,
            None => return false,
        };
        let o_level = match self.object_levels.get(object_id) {
            Some(lvl) => *lvl,
            None => return false,
        };

        if p_level < o_level {
            return false; // Read Up prohibited
        }

        // Category containment check (MCS)
        if let Some(o_cats) = self.object_categories.get(object_id) {
            if !o_cats.is_empty() {
                let p_cats = match self.process_categories.get(process_id) {
                    Some(cats) => cats,
                    None => return false,
                };
                if !o_cats.is_subset(p_cats) {
                    return false;
                }
            }
        }

        true
    }

    /// Write access check: No Write Down (*-Property - Bell-LaPadula)
    pub fn can_write(&self, process_id: &str, object_id: &str) -> bool {
        let p_level = match self.process_levels.get(process_id) {
            Some(lvl) => *lvl,
            None => return false,
        };
        let o_level = match self.object_levels.get(object_id) {
            Some(lvl) => *lvl,
            None => return false,
        };

        p_level <= o_level
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_dynamic_mac_enforcer() {
        let mut mac = DynamicMacEnforcer::new();
        let mut cats_secret = HashSet::new();
        cats_secret.insert(1);
        cats_secret.insert(2);

        mac.set_process_level("proc_app", SensitivityLevel::Secret, cats_secret.clone());
        mac.set_object_level(
            "file_top_secret",
            SensitivityLevel::TopSecret,
            cats_secret.clone(),
        );
        mac.set_object_level("file_secret", SensitivityLevel::Secret, cats_secret.clone());

        // Cannot read TopSecret file from Secret process (No Read Up)
        assert!(!mac.can_read("proc_app", "file_top_secret"));

        // Can read Secret file from Secret process
        assert!(mac.can_read("proc_app", "file_secret"));

        // Can write TopSecret file from Secret process (Write Up)
        assert!(mac.can_write("proc_app", "file_top_secret"));
    }
    use super::*;

    #[test]
    fn test_context_parsing() {
        let raw = "system_u:object_r:httpd_sys_content_t:s0";
        let context = SecurityContext::parse(raw).unwrap();
        assert_eq!(context.user, "system_u");
        assert_eq!(context.role, "object_r");
        assert_eq!(context.type_name, "httpd_sys_content_t");
        assert_eq!(context.sensitivity, "s0");
        assert_eq!(context.to_string(), raw);
    }

    #[test]
    fn test_selinux_avc_caching() {
        let mut engine = SelinuxEngine::new();
        let src = "system_u:system_r:httpd_t:s0";
        let tgt = "system_u:object_r:httpd_sys_content_t:s0";

        // Query 1 (Miss)
        let allowed1 = engine.has_permission(src, tgt, "file", "read").unwrap();
        assert!(allowed1);
        assert_eq!(engine.avc.misses, 1);
        assert_eq!(engine.avc.hits, 0);

        // Query 2 (Hit)
        let allowed2 = engine.has_permission(src, tgt, "file", "read").unwrap();
        assert!(allowed2);
        assert_eq!(engine.avc.misses, 1);
        assert_eq!(engine.avc.hits, 1);
    }

    #[test]
    fn test_selinux_modes() {
        let mut engine = SelinuxEngine::new();
        let src = "system_u:system_r:httpd_t:s0";
        let tgt = "system_u:object_r:admin_home_t:s0";

        // 1. Enforcing (Denied + Blocked)
        engine.set_mode(SeLinuxMode::Enforcing);
        let allowed = engine.has_permission(src, tgt, "file", "write").unwrap();
        assert!(!allowed);
        assert_eq!(engine.audit_logs.len(), 1);
        assert!(engine.audit_logs[0].contains("denied"));

        // 2. Permissive (Allowed + Audited)
        engine.set_mode(SeLinuxMode::Permissive);
        let allowed_p = engine.has_permission(src, tgt, "file", "write").unwrap();
        assert!(allowed_p); // Permissive lets it through
        assert_eq!(engine.audit_logs.len(), 2);

        // 3. Disabled (Allowed + Not Audited)
        engine.set_mode(SeLinuxMode::Disabled);
        let allowed_d = engine.has_permission(src, tgt, "file", "write").unwrap();
        assert!(allowed_d);
        assert_eq!(engine.audit_logs.len(), 2); // No new audit log added
    }

    #[test]
    fn test_app_armor_and_policy() {
        // AppArmor Manager check
        let mut am = AppArmorManager::new();
        let mut profile = AppArmorProfile::new("firefox");
        profile.add_allow_rule("/usr/bin/firefox");
        profile.add_allow_rule("/etc/resolv.conf");
        am.load_profile(profile);

        assert!(am.check_access("firefox", "/usr/bin/firefox"));
        assert!(!am.check_access("firefox", "/etc/shadow"));

        // SecurityPolicy rule check
        let mut policy = SecurityPolicy::new();
        policy.add_rule(SecurityRule {
            source_type: "unconfined_t".to_string(),
            target_type: "etc_t".to_string(),
            object_type: ObjectType::File,
            permission: SelinuxPermission::Read,
            allow: true,
        });

        assert!(policy.check_permission(
            "unconfined_t",
            "etc_t",
            ObjectType::File,
            SelinuxPermission::Read
        ));
        assert!(!policy.check_permission(
            "unconfined_t",
            "shadow_t",
            ObjectType::File,
            SelinuxPermission::Read
        ));
    }

    #[test]
    fn test_selinux_dynamic_policy_updates() {
        let mut engine = SelinuxEngine::new();
        let src = "system_u:system_r:container_t:s0";
        let tgt = "system_u:object_r:container_file_t:s0";

        // Initially no policy rule allows container_t to write container_file_t
        let initial_access = engine.has_permission(src, tgt, "file", "write").unwrap();
        assert!(!initial_access);

        // Dynamically allow container_t container_file_t file write
        engine.allow("container_t", "container_file_t", "file", "write");

        // Now access is granted and AVC cache is invalidated/updated
        let updated_access = engine.has_permission(src, tgt, "file", "write").unwrap();
        assert!(updated_access);
    }

    #[test]
    fn test_invalid_context_rejection() {
        let mut engine = SelinuxEngine::new();
        let invalid_src = "malformed_context_string";
        let valid_tgt = "system_u:object_r:etc_t:s0";

        let result = engine.has_permission(invalid_src, valid_tgt, "file", "read");
        assert!(result.is_err());
    }
}
