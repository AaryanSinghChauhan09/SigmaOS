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
        format!("{}:{}:{}:{}", self.user, self.role, self.type_name, self.sensitivity)
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
pub enum ObjectType { File, Directory }
pub struct SecurityLabel;
pub struct SecurityPolicy;
pub struct SecurityRule;
pub enum SelinuxPermission { Read, Write, Execute }

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
}
