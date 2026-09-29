// Capability-based Security for SigmaOS
// Capability-based security per Wiki 07-Security.md
// Provides fine-grained resource permission management

use std::string::{String, ToString};
use std::vec::Vec;

/// Capability type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapabilityType {
    Read,
    Write,
    Execute,
    NetworkConnect,
    NetworkBind,
    ProcessCreate,
    ProcessKill,
    FileAccess(String),
}

impl CapabilityType {
    pub fn from_str(s: &str) -> Self {
        if s.starts_with("read:") {
            CapabilityType::FileAccess(s.to_string())
        } else if s.starts_with("write:") {
            CapabilityType::FileAccess(s.to_string())
        } else {
            match s.to_lowercase().as_str() {
                "read" => CapabilityType::Read,
                "write" => CapabilityType::Write,
                "execute" => CapabilityType::Execute,
                "network:connect" => CapabilityType::NetworkConnect,
                "network:bind" => CapabilityType::NetworkBind,
                "process:create" => CapabilityType::ProcessCreate,
                "process:kill" => CapabilityType::ProcessKill,
                _ => CapabilityType::Read,
            }
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            CapabilityType::Read => "read",
            CapabilityType::Write => "write",
            CapabilityType::Execute => "execute",
            CapabilityType::NetworkConnect => "network:connect",
            CapabilityType::NetworkBind => "network:bind",
            CapabilityType::ProcessCreate => "process:create",
            CapabilityType::ProcessKill => "process:kill",
            CapabilityType::FileAccess(s) => s,
        }
    }
}

/// Capability grant
#[derive(Debug, Clone)]
pub struct CapabilityGrant {
    pub process_name: String,
    pub capability: CapabilityType,
    pub granted: bool,
}

impl CapabilityGrant {
    pub fn new(process_name: String, capability: CapabilityType) -> Self {
        CapabilityGrant {
            process_name,
            capability,
            granted: true,
        }
    }

    pub fn revoke(&mut self) {
        self.granted = false;
    }

    pub fn is_granted(&self) -> bool {
        self.granted
    }
}

/// Capability manager
#[derive(Debug, Clone)]
pub struct CapabilityManager {
    pub grants: Vec<CapabilityGrant>,
}

impl CapabilityManager {
    pub fn new() -> Self {
        CapabilityManager {
            grants: Vec::new(),
        }
    }

    pub fn grant(&mut self, process_name: String, capability: CapabilityType) -> bool {
        // Remove existing grant for same capability
        self.grants.retain(|g| !(g.process_name == process_name && g.capability == capability));
        
        let grant = CapabilityGrant::new(process_name, capability);
        self.grants.push(grant);
        true
    }

    pub fn revoke(&mut self, process_name: String, capability: CapabilityType) -> bool {
        if let Some(grant) = self.grants.iter_mut().find(|g| g.process_name == process_name && g.capability == capability) {
            grant.revoke();
            true
        } else {
            false
        }
    }

    pub fn revoke_all(&mut self, process_name: String) -> usize {
        let original_len = self.grants.len();
        self.grants.retain(|g| g.process_name != process_name);
        original_len - self.grants.len()
    }

    pub fn check(&self, process_name: &str, capability: &CapabilityType) -> bool {
        self.grants.iter()
            .any(|g| g.process_name == process_name && g.capability == *capability && g.is_granted())
    }

    pub fn list_capabilities(&self, process_name: &str) -> Vec<String> {
        self.grants.iter()
            .filter(|g| g.process_name == process_name && g.is_granted())
            .map(|g| g.capability.as_str().to_string())
            .collect()
    }

    pub fn list_all(&self) -> Vec<String> {
        self.grants.iter()
            .filter(|g| g.is_granted())
            .map(|g| format!("{}: {}", g.process_name, g.capability.as_str()))
            .collect()
    }

    pub fn add_standard_capabilities(&mut self, process_name: String) {
        // Grant standard capabilities for a typical process
        self.grant(process_name.clone(), CapabilityType::Read);
        self.grant(process_name.clone(), CapabilityType::NetworkConnect);
        self.grant(process_name, CapabilityType::ProcessCreate);
    }
}

impl Default for CapabilityManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_type_from_str() {
        assert_eq!(CapabilityType::from_str("read"), CapabilityType::Read);
        assert_eq!(CapabilityType::from_str("write"), CapabilityType::Write);
        assert_eq!(CapabilityType::from_str("network:connect"), CapabilityType::NetworkConnect);
    }

    #[test]
    fn test_capability_grant_creation() {
        let grant = CapabilityGrant::new(
            String::from("test_process"),
            CapabilityType::Read,
        );
        assert_eq!(grant.process_name, "test_process");
        assert!(grant.is_granted());
    }

    #[test]
    fn test_capability_grant_revoke() {
        let mut grant = CapabilityGrant::new(
            String::from("test_process"),
            CapabilityType::Read,
        );
        grant.revoke();
        assert!(!grant.is_granted());
    }

    #[test]
    fn test_capability_manager_creation() {
        let manager = CapabilityManager::new();
        assert_eq!(manager.grants.len(), 0);
    }

    #[test]
    fn test_capability_manager_grant() {
        let mut manager = CapabilityManager::new();
        assert!(manager.grant(
            String::from("test_process"),
            CapabilityType::Read,
        ));
        assert_eq!(manager.grants.len(), 1);
    }

    #[test]
    fn test_capability_manager_revoke() {
        let mut manager = CapabilityManager::new();
        manager.grant(String::from("test_process"), CapabilityType::Read);
        assert!(manager.revoke(
            String::from("test_process"),
            CapabilityType::Read,
        ));
        
        let grant = manager.grants.first().unwrap();
        assert!(!grant.is_granted());
    }

    #[test]
    fn test_capability_manager_revoke_all() {
        let mut manager = CapabilityManager::new();
        manager.grant(String::from("test_process"), CapabilityType::Read);
        manager.grant(String::from("test_process"), CapabilityType::Write);
        
        let count = manager.revoke_all(String::from("test_process"));
        assert_eq!(count, 2);
        assert_eq!(manager.grants.len(), 0);
    }

    #[test]
    fn test_capability_manager_check() {
        let mut manager = CapabilityManager::new();
        manager.grant(String::from("test_process"), CapabilityType::Read);
        
        assert!(manager.check("test_process", &CapabilityType::Read));
        assert!(!manager.check("test_process", &CapabilityType::Write));
    }

    #[test]
    fn test_capability_manager_list_capabilities() {
        let mut manager = CapabilityManager::new();
        manager.grant(String::from("test_process"), CapabilityType::Read);
        manager.grant(String::from("test_process"), CapabilityType::Write);
        
        let caps = manager.list_capabilities("test_process");
        assert_eq!(caps.len(), 2);
    }

    #[test]
    fn test_capability_manager_list_all() {
        let mut manager = CapabilityManager::new();
        manager.grant(String::from("test_process"), CapabilityType::Read);
        
        let all = manager.list_all();
        assert!(!all.is_empty());
    }

    #[test]
    fn test_add_standard_capabilities() {
        let mut manager = CapabilityManager::new();
        manager.add_standard_capabilities(String::from("test_process"));
        
        let caps = manager.list_capabilities("test_process");
        assert!(caps.contains(&String::from("read")));
        assert!(caps.contains(&String::from("network:connect")));
        assert!(caps.contains(&String::from("process:create")));
    }

    #[test]
    fn test_file_access_capability() {
        let cap = CapabilityType::FileAccess(String::from("read:/etc/config"));
        assert_eq!(cap.as_str(), "read:/etc/config");
    }

    #[test]
    fn test_capability_duplication() {
        let mut manager = CapabilityManager::new();
        manager.grant(String::from("test_process"), CapabilityType::Read);
        manager.grant(String::from("test_process"), CapabilityType::Read);
        
        // Should only have one grant (duplicates removed)
        assert_eq!(manager.grants.len(), 1);
    }
}
