#![cfg_attr(not(test), no_std)]

extern crate alloc;

// Fedora-inspired SELinux (Security-Enhanced Linux) Mandatory Access Control Subsystem.
// Implements labeling security contexts (user:role:type:sensitivity), enforcement modes,
// an Access Vector Cache (AVC) for performance, policy rules, and detailed audit logging.

use std::collections::{HashMap, HashSet};

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
        let parts: Vec<&str> = context_str.splitn(4, ':').collect();
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
        format!("{}:{}:{}:{}", self.user, self.role, self.type_name, self.sensitivity)
    }

    /// Extracts the integer hierarchical sensitivity level from a level string (e.g., "s1" -> 1, "s0" -> 0)
    pub fn get_sensitivity_level(&self) -> u32 {
        let level_part = if let Some(idx) = self.sensitivity.find(':') {
            &self.sensitivity[..idx]
        } else {
            &self.sensitivity
        };

        if level_part.starts_with('s') {
            level_part[1..].parse::<u32>().unwrap_or(0)
        } else {
            0
        }
    }

    /// Extracts the categories set (e.g., "c0,c1" from "s1:c0,c1")
    pub fn get_categories(&self) -> HashSet<String> {
        let mut categories = HashSet::new();
        if let Some(idx) = self.sensitivity.find(':') {
            let cat_part = &self.sensitivity[idx + 1..];
            for cat in cat_part.split(',') {
                let trimmed = cat.trim();
                if !trimmed.is_empty() {
                    categories.insert(trimmed.to_string());
                }
            }
        }
        categories
    }

    /// Checks if this security context dominates another context (MLS/MCS Dominance Check)
    /// A dominates B if:
    /// 1. A's hierarchical sensitivity level is >= B's hierarchical sensitivity level.
    /// 2. B's category set is a subset of A's category set.
    pub fn dominates(&self, other: &SecurityContext) -> bool {
        let self_level = self.get_sensitivity_level();
        let other_level = other.get_sensitivity_level();
        if self_level < other_level {
            return false;
        }

        let self_cats = self.get_categories();
        let other_cats = other.get_categories();
        for cat in &other_cats {
            if !self_cats.contains(cat) {
                return false;
            }
        }
        true
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConditionalPolicyRule {
    pub key: AvcKey,
    pub boolean_name: String,
    pub expected_value: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeTransitionKey {
    pub source_type: String,
    pub target_type: String,
    pub class: String,
}

pub struct SelinuxEngine {
    pub mode: SeLinuxMode,
    pub policies: HashSet<AvcKey>,
    pub avc: AccessVectorCache,
    pub audit_logs: Vec<String>,
    pub booleans: HashMap<String, bool>,
    pub conditional_policies: Vec<ConditionalPolicyRule>,
    pub type_transitions: HashMap<TypeTransitionKey, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectType {
    File,
    Process,
    Socket,
    Capability,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelinuxPermission {
    Read,
    Write,
    Execute,
    Append,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityLabel {
    pub label: String,
}

#[derive(Debug, Clone)]
pub struct SecurityRule {
    pub source_type: String,
    pub target_type: String,
    pub object_type: ObjectType,
    pub permission: SelinuxPermission,
}

#[derive(Debug, Clone)]
pub struct SecurityPolicy {
    pub rules: Vec<SecurityRule>,
}

#[derive(Debug, Clone)]
pub struct AppArmorProfile {
    pub name: String,
    pub mode: String,
}

pub struct AppArmorManager {
    pub profiles: Vec<AppArmorProfile>,
}

impl AppArmorManager {
    pub fn new() -> Self {
        Self { profiles: Vec::new() }
    }
}

impl Default for AppArmorManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SelinuxEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            mode: SeLinuxMode::Enforcing,
            policies: HashSet::new(),
            avc: AccessVectorCache::new(),
            audit_logs: Vec::new(),
            booleans: HashMap::new(),
            conditional_policies: Vec::new(),
            type_transitions: HashMap::new(),
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
    }

    pub fn set_boolean(&mut self, name: &str, value: bool) {
        self.booleans.insert(name.to_string(), value);
        self.avc.clear(); // Flush cache when boolean changes
    }

    pub fn allow_conditional(
        &mut self,
        source: &str,
        target: &str,
        class: &str,
        permission: &str,
        boolean_name: &str,
        expected_value: bool,
    ) {
        let key = AvcKey {
            source_type: source.to_string(),
            target_type: target.to_string(),
            class: class.to_string(),
            permission: permission.to_string(),
        };
        self.conditional_policies.push(ConditionalPolicyRule {
            key,
            boolean_name: boolean_name.to_string(),
            expected_value,
        });
        self.avc.clear(); // Flush cache on policy update
    }

    pub fn add_type_transition(&mut self, source_type: &str, target_type: &str, class: &str, new_type: &str) {
        let key = TypeTransitionKey {
            source_type: source_type.to_string(),
            target_type: target_type.to_string(),
            class: class.to_string(),
        };
        self.type_transitions.insert(key, new_type.to_string());
    }

    pub fn compute_transition(&self, source: &str, target: &str, class: &str) -> Result<String, &'static str> {
        let src_context = SecurityContext::parse(source)?;
        let tgt_context = SecurityContext::parse(target)?;

        let key = TypeTransitionKey {
            source_type: src_context.type_name.clone(),
            target_type: tgt_context.type_name.clone(),
            class: class.to_string(),
        };

        if let Some(new_type) = self.type_transitions.get(&key) {
            Ok(SecurityContext {
                user: src_context.user.clone(),
                role: src_context.role.clone(),
                type_name: new_type.clone(),
                sensitivity: src_context.sensitivity.clone(),
            }.to_string())
        } else {
            Ok(source.to_string())
        }
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
            let mut decision = self.policies.contains(&avc_key);
            if !decision {
                for rule in &self.conditional_policies {
                    if rule.key == avc_key {
                        let active_val = self.booleans.get(&rule.boolean_name).copied().unwrap_or(false);
                        if active_val == rule.expected_value {
                            decision = true;
                            break;
                        }
                    }
                }
            }
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

#[cfg(test)]
mod tests {
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
    fn test_mls_mcs_dominance() {
        let low = SecurityContext::parse("system_u:object_r:tmp_t:s0").unwrap();
        let high = SecurityContext::parse("system_u:object_r:secret_t:s2").unwrap();

        assert!(high.dominates(&low));
        assert!(!low.dominates(&high));

        let s1_c0_c1 = SecurityContext::parse("system_u:object_r:topsecret_t:s1:c0,c1").unwrap();
        let s1_c0 = SecurityContext::parse("system_u:object_r:topsecret_t:s1:c0").unwrap();
        let s1_c2 = SecurityContext::parse("system_u:object_r:topsecret_t:s1:c2").unwrap();

        assert!(s1_c0_c1.dominates(&s1_c0));
        assert!(!s1_c0.dominates(&s1_c0_c1));
        assert!(!s1_c0_c1.dominates(&s1_c2));
    }

    #[test]
    fn test_selinux_booleans() {
        let mut engine = SelinuxEngine::new();
        let src = "system_u:system_r:httpd_t:s0";
        let tgt = "system_u:object_r:system_wlan_t:s0";

        // Initial check: not allowed
        let allowed_init = engine.has_permission(src, tgt, "network", "connect").unwrap();
        assert!(!allowed_init);

        // Add conditional policy rule
        engine.allow_conditional("httpd_t", "system_wlan_t", "network", "connect", "httpd_can_network_connect", true);

        // Still not allowed because the boolean is not set to true yet
        let allowed_cond_unset = engine.has_permission(src, tgt, "network", "connect").unwrap();
        assert!(!allowed_cond_unset);

        // Set boolean to true
        engine.set_boolean("httpd_can_network_connect", true);

        // Now it must be allowed!
        let allowed_cond_active = engine.has_permission(src, tgt, "network", "connect").unwrap();
        assert!(allowed_cond_active);

        // Set boolean back to false
        engine.set_boolean("httpd_can_network_connect", false);

        // Must be denied again
        let allowed_cond_disabled = engine.has_permission(src, tgt, "network", "connect").unwrap();
        assert!(!allowed_cond_disabled);
    }

    #[test]
    fn test_selinux_type_transitions() {
        let mut engine = SelinuxEngine::new();
        let src = "system_u:system_r:init_t:s0:c0";
        let target_file = "system_u:object_r:httpd_exec_t:s0";

        // No transition rule initially -> returns source
        let res_init = engine.compute_transition(src, target_file, "process").unwrap();
        assert_eq!(res_init, src);

        // Add type transition rule
        engine.add_type_transition("init_t", "httpd_exec_t", "process", "httpd_t");

        // Compute transition -> transitions to httpd_t, preserving user, role, and sensitivity
        let res_trans = engine.compute_transition(src, target_file, "process").unwrap();
        assert_eq!(res_trans, "system_u:system_r:httpd_t:s0:c0");
    }
}
