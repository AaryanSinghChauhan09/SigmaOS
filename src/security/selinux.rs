// SigmaOS SELinux-inspired Security Framework
// Implements Fedora-style mandatory access control adapted for capability-based security
// Inspired by Fedora's SELinux for enhanced security architecture

use std::collections::BTreeMap;
use std::string::String;
use std::vec::Vec;

/// Security context
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityContext {
    pub user: String,
    pub role: String,
    pub type_: String,
    pub level: String,
}

impl SecurityContext {
    pub fn new(user: String, role: String, type_: String, level: String) -> Self {
        Self {
            user,
            role,
            type_,
            level,
        }
    }

    pub fn from_string(s: &str) -> Result<Self, String> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() == 4 {
            Ok(Self {
                user: parts[0].to_string(),
                role: parts[1].to_string(),
                type_: parts[2].to_string(),
                level: parts[3].to_string(),
            })
        } else {
            Err("Invalid security context format".to_string())
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}:{}:{}:{}", self.user, self.role, self.type_, self.level)
    }
}

/// Policy rule
#[derive(Debug, Clone)]
pub enum PolicyRule {
    Allow {
        source_type: String,
        target_type: String,
        target_class: String,
        permissions: Vec<String>,
    },
    TypeTransition {
        source_type: String,
        target_type: String,
        target_class: String,
        default_type: String,
    },
    TypeChange {
        source_type: String,
        target_type: String,
        target_class: String,
        default_type: String,
    },
}

/// SELinux policy
#[derive(Debug, Clone)]
pub struct SELinuxPolicy {
    pub rules: Vec<PolicyRule>,
    pub types: BTreeMap<String, Type>,
    pub attributes: BTreeMap<String, Attribute>,
    pub roles: BTreeMap<String, Role>,
    pub users: BTreeMap<String, SELinuxUser>,
}

#[derive(Debug, Clone)]
pub struct Type {
    pub name: String,
    pub attributes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: String,
    pub types: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Role {
    pub name: String,
    pub types: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SELinuxUser {
    pub name: String,
    pub roles: Vec<String>,
    pub mls_range: String,
    pub mls_level: String,
}

impl SELinuxPolicy {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            types: BTreeMap::new(),
            attributes: BTreeMap::new(),
            roles: BTreeMap::new(),
            users: BTreeMap::new(),
        }
    }

    /// Add policy rule
    pub fn add_rule(&mut self, rule: PolicyRule) {
        self.rules.push(rule);
    }

    /// Add type
    pub fn add_type(&mut self, type_: Type) {
        self.types.insert(type_.name.clone(), type_);
    }

    /// Add attribute
    pub fn add_attribute(&mut self, attribute: Attribute) {
        self.attributes.insert(attribute.name.clone(), attribute);
    }

    /// Add role
    pub fn add_role(&mut self, role: Role) {
        self.roles.insert(role.name.clone(), role);
    }

    /// Add user
    pub fn add_user(&mut self, user: SELinuxUser) {
        self.users.insert(user.name.clone(), user);
    }

    /// Check if operation is allowed
    pub fn check_permission(
        &self,
        source_type: &str,
        target_type: &str,
        target_class: &str,
        permission: &str,
    ) -> bool {
        for rule in &self.rules {
            if let PolicyRule::Allow {
                source_type: src,
                target_type: tgt,
                target_class: cls,
                permissions,
            } = rule
            {
                if src == source_type
                    && tgt == target_type
                    && cls == target_class
                    && permissions.contains(&permission.to_string())
                {
                    return true;
                }
            }
        }
        false
    }
}

impl Default for SELinuxPolicy {
    fn default() -> Self {
        Self::new()
    }
}

/// SELinux security manager
pub struct SigmaSELinux {
    pub policy: SELinuxPolicy,
    pub contexts: BTreeMap<String, SecurityContext>,
    pub booleans: BTreeMap<String, bool>,
    pub enforcement: bool,
}

impl SigmaSELinux {
    pub fn new() -> Self {
        Self {
            policy: SELinuxPolicy::new(),
            contexts: BTreeMap::new(),
            booleans: BTreeMap::new(),
            enforcement: true,
        }
    }

    /// Set enforcement mode
    pub fn set_enforcement(&mut self, enforced: bool) {
        self.enforcement = enforced;
    }

    /// Get enforcement mode
    pub fn is_enforcing(&self) -> bool {
        self.enforcement
    }

    /// Set security context for path
    pub fn set_context(&mut self, path: String, context: SecurityContext) {
        self.contexts.insert(path, context);
    }

    /// Get security context for path
    pub fn get_context(&self, path: &str) -> Option<&SecurityContext> {
        self.contexts.get(path)
    }

    /// Set boolean
    pub fn set_boolean(&mut self, name: String, value: bool) {
        self.booleans.insert(name, value);
    }

    /// Get boolean
    pub fn get_boolean(&self, name: &str) -> Option<bool> {
        self.booleans.get(name).copied()
    }

    /// Check file access
    pub fn check_file_access(&self, path: &str, source_type: &str, permission: &str) -> bool {
        if !self.enforcement {
            return true;
        }

        if let Some(context) = self.get_context(path) {
            self.policy
                .check_permission(source_type, &context.type_, "file", permission)
        } else {
            false
        }
    }

    /// Check process transition
    pub fn check_process_transition(&self, source_type: &str, target_type: &str) -> bool {
        if !self.enforcement {
            return true;
        }

        self.policy
            .check_permission(source_type, target_type, "process", "transition")
    }

    /// Get status
    pub fn get_status(&self) -> String {
        let mode = if self.enforcement {
            "Enforcing"
        } else {
            "Permissive"
        };
        format!(
            "SELinux status: {}\nBooleans: {}\nContexts: {}",
            mode,
            self.booleans.len(),
            self.contexts.len()
        )
    }
}

impl Default for SigmaSELinux {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_context() {
        let ctx = SecurityContext::new(
            "user_u".to_string(),
            "user_r".to_string(),
            "user_t".to_string(),
            "s0".to_string(),
        );
        assert_eq!(ctx.to_string(), "user_u:user_r:user_t:s0");
    }

    #[test]
    fn test_security_context_parse() {
        let ctx = SecurityContext::from_string("user_u:user_r:user_t:s0").unwrap();
        assert_eq!(ctx.user, "user_u");
        assert_eq!(ctx.role, "user_r");
        assert_eq!(ctx.type_, "user_t");
        assert_eq!(ctx.level, "s0");
    }

    #[test]
    fn test_selinux_policy() {
        let mut policy = SELinuxPolicy::new();
        policy.add_rule(PolicyRule::Allow {
            source_type: "user_t".to_string(),
            target_type: "user_home_t".to_string(),
            target_class: "file".to_string(),
            permissions: vec!["read".to_string(), "write".to_string()],
        });

        assert!(policy.check_permission("user_t", "user_home_t", "file", "read"));
        assert!(!policy.check_permission("user_t", "user_home_t", "file", "execute"));
    }

    #[test]
    fn test_selinux_manager() {
        let mut selinux = SigmaSELinux::new();
        selinux.set_boolean("httpd_enable_cgi".to_string(), true);

        assert_eq!(selinux.get_boolean("httpd_enable_cgi"), Some(true));
        assert!(selinux.is_enforcing());
    }
}
