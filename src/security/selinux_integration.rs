#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
#![cfg_attr(not(test), no_std)]
// SigmaOS SELinux-System Call Integration
// Bridges SELinux policy engine with syscall dispatcher for real MAC enforcement
// Solves the gap: SELinux engine exists but not integrated with actual syscalls



use std::string::String;
use std::string::ToString;
use std::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use crate::klib::HashMap;

use crate::security::selinux::{
    SeLinuxMode, SelinuxEngine, SecurityContext
};

/// System call types that require SELinux permission checks
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyscallSecurityClass {
    File,
    Process,
    Network,
    IPC,
    System,
}

/// SELinux-syscall integration manager
pub struct SelinuxSyscallIntegration {
    pub selinux_engine: SelinuxEngine,
    pub integration_enabled: AtomicBool,
    pub syscall_checks: AtomicUsize,
    pub denied_syscalls: AtomicUsize,
    pub permissive_denials: AtomicUsize,
    pub process_contexts: HashMap<usize, String>,
    pub file_contexts: HashMap<String, String>,
}

impl SelinuxSyscallIntegration {
    pub fn new() -> Self {
        Self {
            selinux_engine: SelinuxEngine::new(),
            integration_enabled: AtomicBool::new(true),
            syscall_checks: AtomicUsize::new(0),
            denied_syscalls: AtomicUsize::new(0),
            permissive_denials: AtomicUsize::new(0),
            process_contexts: HashMap::new(),
            file_contexts: HashMap::new(),
        }
    }

    pub fn initialize(&mut self) {
        self.load_default_contexts();
        self.integration_enabled.store(true, Ordering::SeqCst);
    }

    fn load_default_contexts(&mut self) {
        self.process_contexts.insert(1, "system_u:system_r:init_t:s0".to_string());
        self.process_contexts.insert(2, "system_u:system_r:kernel_t:s0".to_string());
        self.process_contexts.insert(100, "system_u:system_r:httpd_t:s0".to_string());
        self.process_contexts.insert(101, "system_u:system_r:unconfined_t:s0".to_string());

        self.file_contexts.insert("/etc/passwd".to_string(), "system_u:object_r:etc_t:s0".to_string());
        self.file_contexts.insert("/etc/shadow".to_string(), "system_u:object_r:shadow_t:s0".to_string());
        self.file_contexts.insert("/var/www/html".to_string(), "system_u:object_r:httpd_sys_content_t:s0".to_string());
        self.file_contexts.insert("/home".to_string(), "system_u:object_r:home_root_t:s0".to_string());
        self.file_contexts.insert("/bin".to_string(), "system_u:object_r:bin_t:s0".to_string());
        self.file_contexts.insert("/sbin".to_string(), "system_u:object_r:sbin_t:s0".to_string());
    }

    pub fn check_syscall_permission(
        &mut self,
        process_id: usize,
        syscall_number: usize,
        resource_path: Option<&str>,
    ) -> Result<bool, SelinuxError> {
        if !self.integration_enabled.load(Ordering::SeqCst) {
            return Ok(true);
        }

        self.syscall_checks.fetch_add(1, Ordering::SeqCst);

        let (security_class, permission) = self.map_syscall_to_permission(syscall_number);
        let source_context = self.get_process_context(process_id)?;
        let target_context = self.get_target_context(resource_path)?;

        let src_type = SecurityContext::parse(&source_context).map(|c| c.type_name).unwrap_or_default();
        let tgt_type = SecurityContext::parse(&target_context).map(|c| c.type_name).unwrap_or_default();

        let is_rule_allowed = self.selinux_engine.policies.iter().any(|p| {
            p.source_type == src_type && p.target_type == tgt_type && p.class == security_class.as_str() && p.permission == permission
        });

        let mode = self.selinux_engine.mode;
        if !is_rule_allowed {
            self.selinux_engine.has_permission(
                &source_context,
                &target_context,
                security_class.as_str(),
                permission,
            ).ok();

            if mode == SeLinuxMode::Enforcing {
                self.denied_syscalls.fetch_add(1, Ordering::SeqCst);
                return Err(SelinuxError::PermissionDenied);
            } else if mode == SeLinuxMode::Permissive {
                self.permissive_denials.fetch_add(1, Ordering::SeqCst);
            }
        }

        Ok(true)
    }

    fn map_syscall_to_permission(&self, syscall_number: usize) -> (SyscallSecurityClass, &'static str) {
        match syscall_number {
            2 => (SyscallSecurityClass::File, "read"),
            3 => (SyscallSecurityClass::File, "write"),
            4 => (SyscallSecurityClass::File, "read"),
            5 => (SyscallSecurityClass::File, "unlink"),
            6 => (SyscallSecurityClass::File, "getattr"),
            7 => (SyscallSecurityClass::Process, "fork"),
            8 => (SyscallSecurityClass::Process, "exec"),
            9 => (SyscallSecurityClass::Process, "kill"),
            10 => (SyscallSecurityClass::Process, "signal"),
            11 => (SyscallSecurityClass::Network, "bind"),
            12 => (SyscallSecurityClass::Network, "connect"),
            13 => (SyscallSecurityClass::Network, "listen"),
            14 => (SyscallSecurityClass::IPC, "read"),
            15 => (SyscallSecurityClass::IPC, "write"),
            16 => (SyscallSecurityClass::System, "admin"),
            17 => (SyscallSecurityClass::System, "admin"),
            _ => (SyscallSecurityClass::System, "generic"),
        }
    }

    fn get_process_context(&self, process_id: usize) -> Result<String, SelinuxError> {
        self.process_contexts
            .get(&process_id)
            .cloned()
            .ok_or(SelinuxError::ContextNotFound)
    }

    fn get_target_context(&self, resource_path: Option<&str>) -> Result<String, SelinuxError> {
        match resource_path {
            Some(path) => self.file_contexts
                .get(path)
                .cloned()
                .ok_or(SelinuxError::ContextNotFound),
            None => Ok("system_u:object_r:default_t:s0".to_string()),
        }
    }

    pub fn set_process_context(&mut self, process_id: usize, context: String) {
        self.process_contexts.insert(process_id, context);
    }

    pub fn set_file_context(&mut self, file_path: String, context: String) {
        self.file_contexts.insert(file_path, context);
    }

    pub fn set_enforcement_mode(&mut self, mode: SeLinuxMode) {
        self.selinux_engine.set_mode(mode);
    }

    pub fn set_integration_enabled(&self, enabled: bool) {
        self.integration_enabled.store(enabled, Ordering::SeqCst);
    }

    pub fn get_stats(&self) -> SelinuxStats {
        SelinuxStats {
            syscall_checks: self.syscall_checks.load(Ordering::SeqCst),
            denied_syscalls: self.denied_syscalls.load(Ordering::SeqCst),
            permissive_denials: self.permissive_denials.load(Ordering::SeqCst),
            integration_enabled: self.integration_enabled.load(Ordering::SeqCst),
            enforcement_mode: self.selinux_engine.mode,
        }
    }

    pub fn add_policy_rule(&mut self, source: &str, target: &str, class: &str, permission: &str) {
        self.selinux_engine.allow(source, target, class, permission);
    }

    pub fn load_policy_string(&mut self, policy_string: &str) -> Result<(), SelinuxError> {
        for line in policy_string.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() != 4 {
                return Err(SelinuxError::InvalidPolicy);
            }

            self.add_policy_rule(parts[0], parts[1], parts[2], parts[3]);
        }

        Ok(())
    }
}

impl SyscallSecurityClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            SyscallSecurityClass::File => "file",
            SyscallSecurityClass::Process => "process",
            SyscallSecurityClass::Network => "network",
            SyscallSecurityClass::IPC => "ipc",
            SyscallSecurityClass::System => "system",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelinuxError {
    PermissionDenied,
    ContextNotFound,
    InvalidPolicy,
    IntegrationDisabled,
}

#[derive(Debug, Clone)]
pub struct SelinuxStats {
    pub syscall_checks: usize,
    pub denied_syscalls: usize,
    pub permissive_denials: usize,
    pub integration_enabled: bool,
    pub enforcement_mode: SeLinuxMode,
}

static mut GLOBAL_SELINUX_INTEGRATION: Option<SelinuxSyscallIntegration> = None;

pub fn initialize_selinux_integration() -> Result<(), SelinuxError> {
    unsafe {
        if GLOBAL_SELINUX_INTEGRATION.is_none() {
            let mut integration = SelinuxSyscallIntegration::new();
            integration.initialize();
            GLOBAL_SELINUX_INTEGRATION = Some(integration);
        }
        Ok(())
    }
}

pub fn get_selinux_integration() -> Option<&'static mut SelinuxSyscallIntegration> {
    unsafe {
        GLOBAL_SELINUX_INTEGRATION.as_mut()
    }
}

pub fn check_syscall_selinux(
    process_id: usize,
    syscall_number: usize,
    resource_path: Option<&str>,
) -> Result<bool, SelinuxError> {
    if let Some(integration) = get_selinux_integration() {
        integration.check_syscall_permission(process_id, syscall_number, resource_path)
    } else {
        Ok(true)
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_selinux_integration_initialization() {
        let mut integration = SelinuxSyscallIntegration::new();
        integration.initialize();
        
        assert!(integration.integration_enabled.load(Ordering::SeqCst));
        assert!(integration.process_contexts.len() > 0);
        assert!(integration.file_contexts.len() > 0);
    }

    #[test]
    fn test_syscall_permission_check() {
        let mut integration = SelinuxSyscallIntegration::new();
        integration.initialize();
        
        let result = integration.check_syscall_permission(100, 2, Some("/var/www/html"));
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_permission_denial() {
        let mut integration = SelinuxSyscallIntegration::new();
        integration.initialize();
        integration.set_enforcement_mode(SeLinuxMode::Enforcing);
        
        let result = integration.check_syscall_permission(100, 3, Some("/etc/passwd"));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), SelinuxError::PermissionDenied);
    }

    #[test]
    fn test_permissive_mode() {
        let mut integration = SelinuxSyscallIntegration::new();
        integration.initialize();
        integration.set_enforcement_mode(SeLinuxMode::Permissive);
        
        let result = integration.check_syscall_permission(100, 3, Some("/etc/passwd"));
        assert!(result.is_ok());
        assert!(result.unwrap());
        assert!(integration.permissive_denials.load(Ordering::SeqCst) > 0);
    }

    #[test]
    fn test_custom_policy_loading() {
        let mut integration = SelinuxSyscallIntegration::new();
        integration.initialize();
        
        let policy = "httpd_t etc_t file write";
        assert!(integration.load_policy_string(policy).is_ok());
        
        let result = integration.check_syscall_permission(100, 3, Some("/etc/passwd"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_context_management() {
        let mut integration = SelinuxSyscallIntegration::new();
        integration.initialize();
        
        integration.set_process_context(200, "system_u:system_r:custom_t:s0".to_string());
        integration.set_file_context("/custom/path".to_string(), "system_u:object_r:custom_t:s0".to_string());
        
        let context = integration.get_process_context(200);
        assert!(context.is_ok());
        assert!(context.unwrap().contains("custom_t"));
    }

    #[test]
    fn test_integration_statistics() {
        let mut integration = SelinuxSyscallIntegration::new();
        integration.initialize();
        integration.add_policy_rule("httpd_t", "etc_t", "file", "write");
        
        integration.check_syscall_permission(100, 2, Some("/var/www/html")).unwrap();
        integration.check_syscall_permission(100, 3, Some("/etc/passwd")).unwrap();
        
        let stats = integration.get_stats();
        assert_eq!(stats.syscall_checks, 2);
        assert!(stats.integration_enabled);
    }

    #[test]
    fn test_integration_disable() {
        let mut integration = SelinuxSyscallIntegration::new();
        integration.initialize();
        integration.set_integration_enabled(false);
        
        let result = integration.check_syscall_permission(100, 3, Some("/etc/passwd"));
        assert!(result.is_ok());
        assert!(result.unwrap());
    }
}
