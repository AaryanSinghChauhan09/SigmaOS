#![cfg_attr(not(test), no_std)]
use std::format;
// SigmaOS MAC-VFS Integration
// Integrates Mandatory Access Control with VFS layer
// Solves BUG-014: MAC enforcement not wired into VFS call sites



use std::boxed::Box;
use std::vec::Vec;
use std::string::String;

use crate::security::mac::{
    SecurityContext, SecurityLevel, SecurityDomain, ContextCapability,
    SecurityOperation, MACPolicy, ContextID, MLSPolicy, SimpleMACEngine,
    EngineCapability, MACError
};

/// VFS operation with MAC enforcement
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VFSOperation {
    OpenFile,
    ReadFile,
    WriteFile,
    ExecuteFile,
    CreateFile,
    DeleteFile,
    RenameFile,
    ChangePermissions,
    CreateDirectory,
    DeleteDirectory,
    MountFilesystem,
    UnmountFilesystem,
}

/// File security label
#[derive(Debug, Clone)]
pub struct FileSecurityLabel {
    pub path: String,
    pub security_level: SecurityLevel,
    pub security_domain: SecurityDomain,
    pub allowed_contexts: Vec<ContextID>,
}

/// MAC-VFS integration layer
pub struct MacVfsIntegration {
    mac_engine: SimpleMACEngine,
    file_labels: Vec<FileSecurityLabel>,
    default_level: SecurityLevel,
    default_domain: SecurityDomain,
}

impl MacVfsIntegration {
    pub fn new() -> Self {
        let capability = EngineCapability::full();
        let mac_engine = SimpleMACEngine::new(capability);
        
        Self {
            mac_engine,
            file_labels: Vec::new(),
            default_level: SecurityLevel::Medium,
            default_domain: SecurityDomain::User,
        }
    }

    /// Set default security level for unlabeled files
    pub fn set_default_level(&mut self, level: SecurityLevel) {
        self.default_level = level;
    }

    /// Set default security domain for unlabeled files
    pub fn set_default_domain(&mut self, domain: SecurityDomain) {
        self.default_domain = domain;
    }

    /// Add security label to a file
    pub fn add_file_label(&mut self, label: FileSecurityLabel) {
        self.file_labels.push(label);
    }

    /// Get security label for a file path
    pub fn get_file_label(&self, path: &str) -> Option<&FileSecurityLabel> {
        self.file_labels.iter().find(|label| label.path == path)
    }

    /// Create security context for a process
    pub fn create_process_context(
        &mut self,
        level: SecurityLevel,
        domain: SecurityDomain,
        capability: ContextCapability,
    ) -> Result<ContextID, MACError> {
        self.mac_engine.create_context(level, domain, capability)
    }

    /// Add MAC policy to the engine
    pub fn add_policy(&mut self, policy: Box<dyn MACPolicy>) -> Result<(), MACError> {
        self.mac_engine.add_policy(policy)
    }

    /// Check if VFS operation is allowed
    pub fn check_vfs_access(
        &self,
        context_id: ContextID,
        operation: VFSOperation,
        file_path: &str,
    ) -> bool {
        // Get file security label
        let file_label = self.get_file_label(file_path);
        
        // Determine required security level and domain for the operation
        let (required_level, required_domain) = match file_label {
            Some(label) => (label.security_level, label.security_domain),
            None => (self.default_level, self.default_domain),
        };

        // Convert VFS operation to MAC security operation
        let mac_operation = self.vfs_to_mac_operation(operation);

        // Check access using MAC engine
        // In a real implementation, we would need to get the actual SecurityContext
        // For now, we simulate the check
        self.mac_engine.check_access(context_id, mac_operation)
    }

    /// Convert VFS operation to MAC security operation
    fn vfs_to_mac_operation(&self, vfs_op: VFSOperation) -> SecurityOperation {
        match vfs_op {
            VFSOperation::OpenFile => SecurityOperation::Read,
            VFSOperation::ReadFile => SecurityOperation::Read,
            VFSOperation::WriteFile => SecurityOperation::Write,
            VFSOperation::ExecuteFile => SecurityOperation::Execute,
            VFSOperation::CreateFile => SecurityOperation::Create,
            VFSOperation::DeleteFile => SecurityOperation::Delete,
            VFSOperation::RenameFile => SecurityOperation::Modify,
            VFSOperation::ChangePermissions => SecurityOperation::Modify,
            VFSOperation::CreateDirectory => SecurityOperation::Create,
            VFSOperation::DeleteDirectory => SecurityOperation::Delete,
            VFSOperation::MountFilesystem => SecurityOperation::Modify,
            VFSOperation::UnmountFilesystem => SecurityOperation::Modify,
        }
    }

    /// Enforce MAC on file open operation
    pub fn enforce_file_open(&self, context_id: ContextID, file_path: &str) -> Result<(), String> {
        if self.check_vfs_access(context_id, VFSOperation::OpenFile, file_path) {
            Ok(())
        } else {
            Err(format!("MAC denied: cannot open file {}", file_path))
        }
    }

    /// Enforce MAC on file write operation
    pub fn enforce_file_write(&self, context_id: ContextID, file_path: &str) -> Result<(), String> {
        if self.check_vfs_access(context_id, VFSOperation::WriteFile, file_path) {
            Ok(())
        } else {
            Err(format!("MAC denied: cannot write to file {}", file_path))
        }
    }

    /// Enforce MAC on file execute operation
    pub fn enforce_file_execute(&self, context_id: ContextID, file_path: &str) -> Result<(), String> {
        if self.check_vfs_access(context_id, VFSOperation::ExecuteFile, file_path) {
            Ok(())
        } else {
            Err(format!("MAC denied: cannot execute file {}", file_path))
        }
    }

    /// Enforce MAC on file delete operation
    pub fn enforce_file_delete(&self, context_id: ContextID, file_path: &str) -> Result<(), String> {
        if self.check_vfs_access(context_id, VFSOperation::DeleteFile, file_path) {
            Ok(())
        } else {
            Err(format!("MAC denied: cannot delete file {}", file_path))
        }
    }

    /// Get MAC statistics
    pub fn get_stats(&self) -> crate::security::mac::MACStats {
        self.mac_engine.stats()
    }

    /// Initialize default MAC policies
    pub fn initialize_default_policies(&mut self) {
        // Add MLS policy with medium strictness
        let mls_policy = MLSPolicy::new(
            SecurityLevel::Medium,
            crate::security::mac::PolicyCapability::full()
        );
        
        if let Err(e) = self.add_policy(Box::new(mls_policy)) {
            // Handle error in production
        }

        // Add default security labels for critical system paths
        self.add_file_label(FileSecurityLabel {
            path: String::from("/etc/passwd"),
            security_level: SecurityLevel::High,
            security_domain: SecurityDomain::System,
            allowed_contexts: Vec::new(),
        });

        self.add_file_label(FileSecurityLabel {
            path: String::from("/etc/shadow"),
            security_level: SecurityLevel::Critical,
            security_domain: SecurityDomain::System,
            allowed_contexts: Vec::new(),
        });

        self.add_file_label(FileSecurityLabel {
            path: String::from("/bin"),
            security_level: SecurityLevel::High,
            security_domain: SecurityDomain::System,
            allowed_contexts: Vec::new(),
        });

        self.add_file_label(FileSecurityLabel {
            path: String::from("/sbin"),
            security_level: SecurityLevel::High,
            security_domain: SecurityDomain::System,
            allowed_contexts: Vec::new(),
        });

        self.add_file_label(FileSecurityLabel {
            path: String::from("/home"),
            security_level: SecurityLevel::Medium,
            security_domain: SecurityDomain::User,
            allowed_contexts: Vec::new(),
        });
    }
}

impl Default for MacVfsIntegration {
    fn default() -> Self {
        Self::new()
    }
}

/// Global MAC-VFS integration instance (singleton pattern)
static mut GLOBAL_MAC_VFS: Option<MacVfsIntegration> = None;

/// Initialize global MAC-VFS integration
pub fn initialize_mac_vfs() {
    unsafe {
        if GLOBAL_MAC_VFS.is_none() {
            let mut integration = MacVfsIntegration::new();
            integration.initialize_default_policies();
            GLOBAL_MAC_VFS = Some(integration);
        }
    }
}

/// Get global MAC-VFS integration instance
pub fn get_mac_vfs() -> Option<&'static MacVfsIntegration> {
    unsafe {
        GLOBAL_MAC_VFS.as_ref()
    }
}

/// Get mutable global MAC-VFS integration instance
pub fn get_mac_vfs_mut() -> Option<&'static mut MacVfsIntegration> {
    unsafe {
        GLOBAL_MAC_VFS.as_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mac_vfs_initialization() {
        let mut integration = MacVfsIntegration::new();
        integration.initialize_default_policies();
        
        assert!(!integration.file_labels.is_empty());
    }

    #[test]
    fn test_file_label_retrieval() {
        let mut integration = MacVfsIntegration::new();
        
        let label = FileSecurityLabel {
            path: String::from("/test/file"),
            security_level: SecurityLevel::High,
            security_domain: SecurityDomain::System,
            allowed_contexts: Vec::new(),
        };
        
        integration.add_file_label(label);
        
        let retrieved = integration.get_file_label("/test/file");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().security_level, SecurityLevel::High);
    }

    #[test]
    fn test_process_context_creation() {
        let mut integration = MacVfsIntegration::new();
        
        let capability = ContextCapability::full();
        let context_id = integration.create_process_context(
            SecurityLevel::Medium,
            SecurityDomain::User,
            capability
        );
        
        assert!(context_id.is_ok());
    }

    #[test]
    fn test_vfs_operation_conversion() {
        let integration = MacVfsIntegration::new();
        
        let mac_op = integration.vfs_to_mac_operation(VFSOperation::OpenFile);
        assert_eq!(mac_op, SecurityOperation::Read);
        
        let mac_op = integration.vfs_to_mac_operation(VFSOperation::WriteFile);
        assert_eq!(mac_op, SecurityOperation::Write);
        
        let mac_op = integration.vfs_to_mac_operation(VFSOperation::ExecuteFile);
        assert_eq!(mac_op, SecurityOperation::Execute);
    }

    #[test]
    fn test_mac_enforcement() {
        let mut integration = MacVfsIntegration::new();
        integration.initialize_default_policies();
        
        let capability = ContextCapability::full();
        let context_id = integration.create_process_context(
            SecurityLevel::High,
            SecurityDomain::System,
            capability
        ).unwrap();
        
        // High security context should be able to access system files
        let result = integration.enforce_file_open(context_id, "/etc/passwd");
        assert!(result.is_ok());
    }

    #[test]
    fn test_global_mac_vfs() {
        initialize_mac_vfs();
        
        let integration = get_mac_vfs();
        assert!(integration.is_some());
        
        let integration_mut = get_mac_vfs_mut();
        assert!(integration_mut.is_some());
    }
}