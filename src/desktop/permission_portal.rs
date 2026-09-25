use std::collections::HashMap;
use std::time::{Duration, SystemTime};

/// Represents different permission types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Permission {
    Filesystem,
    Network,
    Bluetooth,
    Camera,
    Microphone,
    DisplayServer,
    DBus,
    USB,
    GPU,
    Notifications,
    Geolocation,
    Printing,
    ScreenCapture,
}

/// Represents a permission policy
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PermissionPolicy {
    Allow,
    Deny,
    AskOnce,
    AskAlways,
    AllowTemporary(Duration),
}

/// Preset permission profiles
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PermissionProfile {
    Strict,
    Standard,
    Permissive,
    Custom,
}

/// Struct for runtime permission prompts
#[derive(Debug, Clone)]
pub struct PortalRequest {
    pub app_id: String,
    pub permission: Permission,
    pub timestamp: SystemTime,
    pub justification: String,
}

/// XDGPortal protocol representation
#[derive(Debug, Clone)]
pub enum XDGPortal {
    FileChooser,
    ScreenCast,
    Camera,
    RemoteDesktop,
    Notification,
    Settings,
}

/// A log entry for a permission grant or denial
#[derive(Debug, Clone)]
pub struct PermissionLogEntry {
    pub app_id: String,
    pub permission: Permission,
    pub granted: bool,
    pub timestamp: SystemTime,
}

/// The PermissionPortal coordinating all app permissions
pub struct PermissionPortal {
    profile: PermissionProfile,
    app_policies: HashMap<String, HashMap<Permission, PermissionPolicy>>,
    logs: Vec<PermissionLogEntry>,
}

impl PermissionPortal {
    pub fn new(profile: PermissionProfile) -> Self {
        PermissionPortal {
            profile,
            app_policies: HashMap::new(),
            logs: Vec::new(),
        }
    }

    pub fn set_policy(&mut self, app_id: &str, permission: Permission, policy: PermissionPolicy) {
        let app_map = self.app_policies.entry(app_id.to_string()).or_insert_with(HashMap::new);
        app_map.insert(permission, policy);
    }

    pub fn get_policy(&self, app_id: &str, permission: &Permission) -> PermissionPolicy {
        if let Some(app_map) = self.app_policies.get(app_id) {
            if let Some(policy) = app_map.get(permission) {
                return policy.clone();
            }
        }
        
        match self.profile {
            PermissionProfile::Strict => PermissionPolicy::AskAlways,
            PermissionProfile::Standard => PermissionPolicy::AskOnce,
            PermissionProfile::Permissive => PermissionPolicy::Allow,
            PermissionProfile::Custom => PermissionPolicy::AskAlways,
        }
    }

    pub fn request_permission(&mut self, request: &PortalRequest) -> bool {
        let policy = self.get_policy(&request.app_id, &request.permission);
        
        let granted = match policy {
            PermissionPolicy::Allow => true,
            PermissionPolicy::Deny => false,
            PermissionPolicy::AskOnce | PermissionPolicy::AskAlways | PermissionPolicy::AllowTemporary(_) => {
                // In a real system, this would trigger a UI prompt.
                // For logic purposes, let's say it's true if the justification is provided.
                !request.justification.is_empty()
            }
        };

        self.logs.push(PermissionLogEntry {
            app_id: request.app_id.clone(),
            permission: request.permission.clone(),
            granted,
            timestamp: request.timestamp,
        });

        granted
    }

    pub fn get_logs(&self) -> &[PermissionLogEntry] {
        &self.logs
    }
}

pub struct FlatpakPermissionAuditor;

impl FlatpakPermissionAuditor {
    pub fn scan_app_permissions(app_id: &str) -> Vec<Permission> {
        // Stub implementation simulating flatpak overrides or info
        if app_id == "org.mozilla.firefox" {
            vec![Permission::Network, Permission::DisplayServer, Permission::Camera]
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strict_profile_default() {
        let portal = PermissionPortal::new(PermissionProfile::Strict);
        assert_eq!(portal.get_policy("test.app", &Permission::Camera), PermissionPolicy::AskAlways);
    }

    #[test]
    fn test_set_custom_policy() {
        let mut portal = PermissionPortal::new(PermissionProfile::Standard);
        portal.set_policy("org.mozilla.firefox", Permission::Network, PermissionPolicy::Allow);
        assert_eq!(
            portal.get_policy("org.mozilla.firefox", &Permission::Network),
            PermissionPolicy::Allow
        );
    }

    #[test]
    fn test_request_permission_allow() {
        let mut portal = PermissionPortal::new(PermissionProfile::Standard);
        portal.set_policy("test.app", Permission::Microphone, PermissionPolicy::Allow);
        
        let req = PortalRequest {
            app_id: "test.app".to_string(),
            permission: Permission::Microphone,
            timestamp: SystemTime::now(),
            justification: String::new(),
        };
        
        assert!(portal.request_permission(&req));
        assert_eq!(portal.get_logs().len(), 1);
        assert!(portal.get_logs()[0].granted);
    }

    #[test]
    fn test_request_permission_deny() {
        let mut portal = PermissionPortal::new(PermissionProfile::Standard);
        portal.set_policy("test.app", Permission::Filesystem, PermissionPolicy::Deny);
        
        let req = PortalRequest {
            app_id: "test.app".to_string(),
            permission: Permission::Filesystem,
            timestamp: SystemTime::now(),
            justification: "Need access".to_string(),
        };
        
        assert!(!portal.request_permission(&req));
    }

    #[test]
    fn test_request_permission_ask_with_justification() {
        let mut portal = PermissionPortal::new(PermissionProfile::Strict);
        
        let req = PortalRequest {
            app_id: "test.app".to_string(),
            permission: Permission::Geolocation,
            timestamp: SystemTime::now(),
            justification: "To show maps".to_string(), // In tests, non-empty gives true
        };
        // wait, Location isn't a variant, Geolocation is. Let's fix that.
    }
}

#[cfg(test)]
mod more_tests {
    use super::*;
    
    #[test]
    fn test_geolocation() {
        let mut portal = PermissionPortal::new(PermissionProfile::Strict);
        let req = PortalRequest {
            app_id: "test.app".to_string(),
            permission: Permission::Geolocation,
            timestamp: SystemTime::now(),
            justification: "maps".to_string(),
        };
        assert!(portal.request_permission(&req));
    }

    #[test]
    fn test_flatpak_auditor() {
        let perms = FlatpakPermissionAuditor::scan_app_permissions("org.mozilla.firefox");
        assert!(perms.contains(&Permission::Network));
    }
}
