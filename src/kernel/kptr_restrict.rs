//! Kernel Security Mitigations (KPTR_RESTRICT, DMESG_RESTRICT)
//! Inspired by Linux kernel security hardening and BSD sysctl
//! Prevents kernel pointer exposure and restricts dmesg access

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::OnceLock;

/// Kernel pointer restriction level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KptrRestrictLevel {
    /// No restriction (default for development)
    None = 0,
    /// Restrict kernel pointer exposure in procfs
    Restricted = 1,
    /// Maximum restriction - no kernel pointers visible
    Strict = 2,
}

/// Dmesg restriction level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmesgRestrictLevel {
    /// No restriction
    None = 0,
    /// Restrict sensitive messages
    Restricted = 1,
    /// Maximum restriction - only critical messages
    Strict = 2,
}

/// Kernel security mitigation manager
pub struct KernelSecurityMitigations {
    kptr_restrict: AtomicU32,
    dmesg_restrict: AtomicU32,
    modules_disabled: AtomicU32,
}

impl KernelSecurityMitigations {
    pub fn new() -> Self {
        Self {
            kptr_restrict: AtomicU32::new(KptrRestrictLevel::None as u32),
            dmesg_restrict: AtomicU32::new(DmesgRestrictLevel::None as u32),
            modules_disabled: AtomicU32::new(0),
        }
    }

    /// Set kptr_restrict level
    pub fn set_kptr_restrict(&self, level: KptrRestrictLevel) {
        self.kptr_restrict.store(level as u32, Ordering::SeqCst);
    }

    /// Get current kptr_restrict level
    pub fn get_kptr_restrict(&self) -> KptrRestrictLevel {
        match self.kptr_restrict.load(Ordering::SeqCst) {
            0 => KptrRestrictLevel::None,
            1 => KptrRestrictLevel::Restricted,
            2 => KptrRestrictLevel::Strict,
            _ => KptrRestrictLevel::None,
        }
    }

    /// Set dmesg_restrict level
    pub fn set_dmesg_restrict(&self, level: DmesgRestrictLevel) {
        self.dmesg_restrict.store(level as u32, Ordering::SeqCst);
    }

    /// Get current dmesg_restrict level
    pub fn get_dmesg_restrict(&self) -> DmesgRestrictLevel {
        match self.dmesg_restrict.load(Ordering::SeqCst) {
            0 => DmesgRestrictLevel::None,
            1 => DmesgRestrictLevel::Restricted,
            2 => DmesgRestrictLevel::Strict,
            _ => DmesgRestrictLevel::None,
        }
    }

    /// Disable kernel module loading
    pub fn disable_modules(&self) {
        self.modules_disabled.store(1, Ordering::SeqCst);
    }

    /// Enable kernel module loading
    pub fn enable_modules(&self) {
        self.modules_disabled.store(0, Ordering::SeqCst);
    }

    /// Check if modules are disabled
    pub fn are_modules_disabled(&self) -> bool {
        self.modules_disabled.load(Ordering::SeqCst) == 1
    }

    /// Check if kernel pointer should be sanitized
    pub fn should_sanitize_pointer(&self) -> bool {
        matches!(self.get_kptr_restrict(), KptrRestrictLevel::Restricted | KptrRestrictLevel::Strict)
    }

    /// Sanitize kernel pointer for display
    pub fn sanitize_pointer(&self, ptr: usize) -> usize {
        if self.should_sanitize_pointer() {
            0 // Return null pointer
        } else {
            ptr
        }
    }

    /// Check if dmesg message should be displayed
    pub fn should_show_dmesg(&self, level: u32) -> bool {
        match self.get_dmesg_restrict() {
            DmesgRestrictLevel::None => true,
            DmesgRestrictLevel::Restricted => level >= 6, // Only show critical messages
            DmesgRestrictLevel::Strict => level >= 7, // Only show emergency messages
        }
    }
}

impl Default for KernelSecurityMitigations {
    fn default() -> Self {
        Self::new()
    }
}

/// Global kernel security mitigations instance
static GLOBAL_SECURITY_MITIGATIONS: std::sync::OnceLock<KernelSecurityMitigations> =
    std::sync::OnceLock::new();

/// Get global security mitigations instance
pub fn get_security_mitigations() -> &'static KernelSecurityMitigations {
    GLOBAL_SECURITY_MITIGATIONS.get_or_init(|| KernelSecurityMitigations::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kptr_restrict_levels() {
        let mitigations = KernelSecurityMitigations::new();
        assert_eq!(mitigations.get_kptr_restrict(), KptrRestrictLevel::None);
        
        mitigations.set_kptr_restrict(KptrRestrictLevel::Strict);
        assert_eq!(mitigations.get_kptr_restrict(), KptrRestrictLevel::Strict);
    }

    #[test]
    fn test_dmesg_restrict_levels() {
        let mitigations = KernelSecurityMitigations::new();
        assert_eq!(mitigations.get_dmesg_restrict(), DmesgRestrictLevel::None);
        
        mitigations.set_dmesg_restrict(DmesgRestrictLevel::Restricted);
        assert_eq!(mitigations.get_dmesg_restrict(), DmesgRestrictLevel::Restricted);
    }

    #[test]
    fn test_pointer_sanitization() {
        let mitigations = KernelSecurityMitigations::new();
        let ptr = 0xdeadbeefusize;
        
        // No restriction - pointer should not be sanitized
        assert_eq!(mitigations.sanitize_pointer(ptr), ptr);
        
        mitigations.set_kptr_restrict(KptrRestrictLevel::Strict);
        assert_eq!(mitigations.sanitize_pointer(ptr), 0);
    }

    #[test]
    fn test_module_loading_control() {
        let mitigations = KernelSecurityMitigations::new();
        assert!(!mitigations.are_modules_disabled());
        
        mitigations.disable_modules();
        assert!(mitigations.are_modules_disabled());
        
        mitigations.enable_modules();
        assert!(!mitigations.are_modules_disabled());
    }

    #[test]
    fn test_dmesg_filtering() {
        let mitigations = KernelSecurityMitigations::new();
        
        // No restriction - all messages shown
        assert!(mitigations.should_show_dmesg(1));
        assert!(mitigations.should_show_dmesg(7));
        
        mitigations.set_dmesg_restrict(DmesgRestrictLevel::Restricted);
        assert!(!mitigations.should_show_dmesg(1)); // Info level filtered
        assert!(mitigations.should_show_dmesg(6)); // Critical level shown
    }

    #[test]
    fn test_global_instance() {
        let global = get_security_mitigations();
        global.set_kptr_restrict(KptrRestrictLevel::Restricted);
        assert_eq!(global.get_kptr_restrict(), KptrRestrictLevel::Restricted);
    }
}
