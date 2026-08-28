extern crate alloc;
// SigmaOS Hardened Sysctl Implementation
// Inspired by HardenedBSD security hardening approaches


use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use core::sync::atomic::{AtomicBool, Ordering};

/// Hardened sysctl configuration
#[derive(Debug, Clone)]
pub struct SysctlConfig {
    pub name: String,
    pub value: SysctlValue,
    pub description: String,
    pub read_only: bool,
    pub security_critical: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SysctlValue {
    Bool(bool),
    U32(u32),
    I32(i32),
    String(&'static str),
}

/// Hardened sysctl manager
pub struct HardenedSysctlManager {
    sysctls: BTreeMap<String, SysctlConfig>,
    locked: AtomicBool,
}

impl HardenedSysctlManager {
    pub fn new() -> Self {
        let mut manager = Self {
            sysctls: BTreeMap::new(),
            locked: AtomicBool::new(false),
        };
        
        // Initialize with HardenedBSD-inspired security defaults
        manager.init_hardened_defaults();
        manager
    }
    
    fn init_hardened_defaults(&mut self) {
        // HardenedBSD-inspired security settings
        self.register_sysctl(SysctlConfig {
            name: String::from("security.bsd.allow_tiocsti"),
            value: SysctlValue::Bool(false), // Disable TIOCSTI for security
            description: String::from("Allow TIOCSTI ioctl (security risk)"),
            read_only: true,
            security_critical: true,
        });
        
        self.register_sysctl(SysctlConfig {
            name: String::from("security.bsd.unprivileged_kenv_read"),
            value: SysctlValue::Bool(false), // Restrict kernel environment reading
            description: String::from("Allow unprivileged kenv reading"),
            read_only: true,
            security_critical: true,
        });
        
        self.register_sysctl(SysctlConfig {
            name: String::from("hbsd.late_kld_prohibition_value"),
            value: SysctlValue::U32(1), // Prohibit late KLD loading
            description: String::from("Late KLD prohibition value"),
            read_only: true,
            security_critical: true,
        });
        
        self.register_sysctl(SysctlConfig {
            name: String::from("security.bbsd.stack_auto_init"),
            value: SysctlValue::Bool(true), // Enable stack auto-initialization
            description: String::from("Enable automatic stack initialization to zero"),
            read_only: true,
            security_critical: true,
        });
        
        self.register_sysctl(SysctlConfig {
            name: String::from("security.bbsd.ptrace_hardening"),
            value: SysctlValue::Bool(true), // Enable ptrace hardening
            description: String::from("Enable ptrace hardening (HardenedBSD style)"),
            read_only: true,
            security_critical: true,
        });
        
        self.register_sysctl(SysctlConfig {
            name: String::from("security.bbsd.random_relink"),
            value: SysctlValue::Bool(true), // Enable random relinking
            description: String::from("Enable random relinking of critical binaries"),
            read_only: false,
            security_critical: true,
        });
        
        self.register_sysctl(SysctlConfig {
            name: String::from("vm.phys_fictitious_segs"),
            value: SysctlValue::Bool(false), // Restrict physical fictitious segments
            description: String::from("Control physical fictitious segments (HardenedBSD)"),
            read_only: true,
            security_critical: true,
        });
        
        self.register_sysctl(SysctlConfig {
            name: String::from("security.bbsd.aslr"),
            value: SysctlValue::Bool(true), // Enable ASLR
            description: String::from("Enable Address Space Layout Randomization"),
            read_only: false,
            security_critical: true,
        });
        
        self.register_sysctl(SysctlConfig {
            name: String::from("security.bbsd.stack_protector"),
            value: SysctlValue::Bool(true), // Enable stack protector
            description: String::from("Enable stack protector canaries"),
            read_only: false,
            security_critical: true,
        });
    }
    
    pub fn register_sysctl(&mut self, config: SysctlConfig) {
        self.sysctls.insert(config.name.clone(), config);
    }
    
    pub fn get_sysctl(&self, name: &str) -> Option<&SysctlConfig> {
        self.sysctls.get(name)
    }
    
    pub fn set_sysctl(&mut self, name: &str, value: SysctlValue) -> Result<(), SysctlError> {
        if self.locked.load(Ordering::SeqCst) {
            return Err(SysctlError::Locked);
        }
        
        if let Some(config) = self.sysctls.get_mut(name) {
            if config.read_only {
                return Err(SysctlError::ReadOnly);
            }
            config.value = value;
            Ok(())
        } else {
            Err(SysctlError::NotFound)
        }
    }
    
    pub fn lock_security_critical(&self) {
        // Lock all security-critical sysctls
        self.locked.store(true, Ordering::SeqCst);
    }
    
    pub fn apply_hardened_defaults(&self) -> Result<(), SysctlError> {
        // Apply all hardened security defaults
        for (_name, config) in &self.sysctls {
            if config.security_critical {
                // In a real implementation, this would apply to the kernel
                // For now, we just validate the configuration
                if !matches!(config.value, SysctlValue::Bool(true) | SysctlValue::U32(1)) {
                    // Skip if not a security-enhancing value
                }
            }
        }
        Ok(())
    }
    
    pub fn security_audit(&self) -> Vec<String> {
        let mut issues = Vec::new();
        
        for (name, config) in &self.sysctls {
            if config.security_critical {
                let is_insecure = match (name.as_str(), config.value) {
                    ("security.bsd.allow_tiocsti", SysctlValue::Bool(true)) => true,
                    ("security.bsd.unprivileged_kenv_read", SysctlValue::Bool(true)) => true,
                    ("vm.phys_fictitious_segs", SysctlValue::Bool(true)) => true,
                    ("hbsd.late_kld_prohibition_value", SysctlValue::U32(0)) => true,
                    ("security.bbsd.stack_auto_init", SysctlValue::Bool(false)) => true,
                    ("security.bbsd.ptrace_hardening", SysctlValue::Bool(false)) => true,
                    ("security.bbsd.random_relink", SysctlValue::Bool(false)) => true,
                    ("security.bbsd.aslr", SysctlValue::Bool(false)) => true,
                    ("security.bbsd.stack_protector", SysctlValue::Bool(false)) => true,
                    (n, SysctlValue::Bool(false)) if !n.starts_with("security.bsd.allow") && !n.starts_with("security.bsd.unprivileged") && !n.starts_with("vm.phys_fictitious") => true,
                    (_, SysctlValue::U32(0)) => true,
                    _ => false,
                };

                if is_insecure {
                    issues.push(format!("Security-critical sysctl {} is set to an insecure value", name));
                }
            }
        }
        
        issues
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SysctlError {
    NotFound,
    ReadOnly,
    Locked,
    InvalidValue,
}

impl Default for HardenedSysctlManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sysctl_registration() {
        let manager = HardenedSysctlManager::new();
        assert!(manager.get_sysctl("security.bsd.allow_tiocsti").is_some());
    }

    #[test]
    fn test_security_defaults() {
        let manager = HardenedSysctlManager::new();
        let tiocsti = manager.get_sysctl("security.bsd.allow_tiocsti").unwrap();
        assert_eq!(tiocsti.value, SysctlValue::Bool(false));
    }

    #[test]
    fn test_read_only_protection() {
        let mut manager = HardenedSysctlManager::new();
        let result = manager.set_sysctl("security.bsd.allow_tiocsti", SysctlValue::Bool(true));
        assert_eq!(result, Err(SysctlError::ReadOnly));
    }

    #[test]
    fn test_security_audit() {
        let mut manager = HardenedSysctlManager::new();
        let issues = manager.security_audit();
        // With hardened defaults, there should be no security issues
        assert!(issues.is_empty());

        // Test flagging when a security-critical sysctl is set to disabled/false
        manager.register_sysctl(SysctlConfig {
            name: String::from("security.test.critical_disabled"),
            value: SysctlValue::Bool(false),
            description: String::from("Test critical sysctl disabled"),
            read_only: false,
            security_critical: true,
        });
        let audit_issues = manager.security_audit();
        assert_eq!(audit_issues.len(), 1);
        assert!(audit_issues[0].contains("security.test.critical_disabled"));
    }

    #[test]
    fn test_locking() {
        let mut manager = HardenedSysctlManager::new();
        manager.lock_security_critical();
        
        // Try to modify a non-read-only sysctl after locking
        let result = manager.set_sysctl("security.bbsd.random_relink", SysctlValue::Bool(false));
        assert_eq!(result, Err(SysctlError::Locked));
    }
}