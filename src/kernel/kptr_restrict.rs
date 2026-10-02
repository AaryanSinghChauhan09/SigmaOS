//! Kernel Pointer Restriction (kptr_restrict)
//! 
//! Inspired by Linux kernel's kptr_restrict security feature that prevents
//! kernel pointer leaks to userspace, mitigating kernel address space layout
//! information disclosure attacks.
//!
//! # Linux kptr_restrict Levels
//! - 0: No restrictions (kernel pointers visible)
//! - 1: Restricted to CAP_SYSLOG capability
//! - 2: Always hidden, even for privileged users
//!
//! This prevents attackers from learning kernel memory layout via /proc,
//! /sys, dmesg, and other information disclosure vectors.

#![cfg_attr(not(any(feature = "standalone_test", test)), no_std)]

use core::sync::atomic::{AtomicU8, Ordering};

/// Kernel pointer restriction level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum KptrRestrictLevel {
    /// No restrictions - kernel pointers visible (unsafe, debugging only)
    Unrestricted = 0,
    /// Restricted to privileged users with CAP_SYSLOG
    Privileged = 1,
    /// Always hidden, even for root (maximum security)
    AlwaysHidden = 2,
}

impl KptrRestrictLevel {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Unrestricted),
            1 => Some(Self::Privileged),
            2 => Some(Self::AlwaysHidden),
            _ => None,
        }
    }

    pub fn as_u8(self) -> u8 {
        self as u8
    }
}

/// Global kernel pointer restriction setting
static KPTR_RESTRICT: AtomicU8 = AtomicU8::new(KptrRestrictLevel::Privileged as u8);

/// Security mitigation flags
#[derive(Debug, Clone, Copy)]
pub struct SecurityMitigations {
    pub kptr_restrict: bool,
    pub dmesg_restrict: bool,
    pub perf_event_paranoid: bool,
    pub unprivileged_bpf_disabled: bool,
}

impl SecurityMitigations {
    pub fn current() -> Self {
        Self {
            kptr_restrict: get_kptr_restrict() != KptrRestrictLevel::Unrestricted,
            dmesg_restrict: true,  // Assume enabled
            perf_event_paranoid: true,
            unprivileged_bpf_disabled: true,
        }
    }
}

/// Get current kptr_restrict level
pub fn get_kptr_restrict() -> KptrRestrictLevel {
    let value = KPTR_RESTRICT.load(Ordering::Relaxed);
    KptrRestrictLevel::from_u8(value).unwrap_or(KptrRestrictLevel::Privileged)
}

/// Set kptr_restrict level
/// 
/// # Safety
/// Should only be called during early boot or by privileged kernel code
pub fn set_kptr_restrict(level: KptrRestrictLevel) {
    KPTR_RESTRICT.store(level.as_u8(), Ordering::Relaxed);
}

/// Check if kernel pointer should be hidden
/// 
/// Returns true if the pointer should be hidden based on current
/// restriction level and caller privileges
pub fn should_hide_kptr(has_cap_syslog: bool) -> bool {
    match get_kptr_restrict() {
        KptrRestrictLevel::Unrestricted => false,
        KptrRestrictLevel::Privileged => !has_cap_syslog,
        KptrRestrictLevel::AlwaysHidden => true,
    }
}

/// Format kernel pointer for display
/// 
/// Returns sanitized pointer representation based on security policy:
/// - If visible: actual pointer value
/// - If hidden: "0x0000000000000000" or hashed value
pub fn format_kptr(ptr: usize, has_cap_syslog: bool) -> u64 {
    if should_hide_kptr(has_cap_syslog) {
        // Return zeroed or hashed pointer
        hash_ptr(ptr)
    } else {
        ptr as u64
    }
}

/// Hash kernel pointer for display (one-way function)
/// 
/// Provides a consistent identifier without revealing actual address.
/// Inspired by Linux %pK format specifier implementation.
fn hash_ptr(ptr: usize) -> u64 {
    // Simple hash - in production, use cryptographic hash with secret seed
    const HASH_MIX: u64 = 0x9e3779b97f4a7c15; // Golden ratio
    let val = ptr as u64;
    val.wrapping_mul(HASH_MIX) & 0xFFFF // Only show lower 16 bits
}

/// Check if kernel symbol should be visible
/// 
/// Used by /proc/kallsyms and similar interfaces
pub fn can_view_kernel_symbols(has_cap_syslog: bool) -> bool {
    !should_hide_kptr(has_cap_syslog)
}

/// Initialize kptr_restrict subsystem
/// 
/// Sets up kernel pointer restriction to secure default (level 1)
pub fn init_kptr_restrict() {
    // Default to privileged mode for security
    set_kptr_restrict(KptrRestrictLevel::Privileged);
}

/// Set maximum security mode (level 2)
/// 
/// Called for high-security environments where kernel ASLR must be
/// maximally protected even from privileged users
pub fn enable_maximum_kptr_security() {
    set_kptr_restrict(KptrRestrictLevel::AlwaysHidden);
}

/// Get all active security mitigations
pub fn get_security_mitigations() -> SecurityMitigations {
    SecurityMitigations::current()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kptr_levels() {
        assert_eq!(KptrRestrictLevel::Unrestricted.as_u8(), 0);
        assert_eq!(KptrRestrictLevel::Privileged.as_u8(), 1);
        assert_eq!(KptrRestrictLevel::AlwaysHidden.as_u8(), 2);
    }

    #[test]
    fn test_level_conversion() {
        assert_eq!(
            KptrRestrictLevel::from_u8(0),
            Some(KptrRestrictLevel::Unrestricted)
        );
        assert_eq!(
            KptrRestrictLevel::from_u8(1),
            Some(KptrRestrictLevel::Privileged)
        );
        assert_eq!(
            KptrRestrictLevel::from_u8(2),
            Some(KptrRestrictLevel::AlwaysHidden)
        );
        assert_eq!(KptrRestrictLevel::from_u8(3), None);
    }

    #[test]
    fn test_should_hide_unrestricted() {
        set_kptr_restrict(KptrRestrictLevel::Unrestricted);
        assert!(!should_hide_kptr(false));
        assert!(!should_hide_kptr(true));
    }

    #[test]
    fn test_should_hide_privileged() {
        set_kptr_restrict(KptrRestrictLevel::Privileged);
        assert!(should_hide_kptr(false)); // Non-privileged user
        assert!(!should_hide_kptr(true)); // Has CAP_SYSLOG
    }

    #[test]
    fn test_should_hide_always() {
        set_kptr_restrict(KptrRestrictLevel::AlwaysHidden);
        assert!(should_hide_kptr(false));
        assert!(should_hide_kptr(true)); // Even privileged users can't see
    }

    #[test]
    fn test_format_kptr_visible() {
        set_kptr_restrict(KptrRestrictLevel::Unrestricted);
        let ptr: usize = 0x12345678;
        assert_eq!(format_kptr(ptr, false), ptr as u64);
    }

    #[test]
    fn test_format_kptr_hidden() {
        set_kptr_restrict(KptrRestrictLevel::AlwaysHidden);
        let ptr: usize = 0x12345678;
        let formatted = format_kptr(ptr, true);
        // Should not equal actual pointer
        assert_ne!(formatted, ptr as u64);
    }

    #[test]
    fn test_hash_ptr_consistency() {
        let ptr: usize = 0xDEADBEEF;
        let hash1 = hash_ptr(ptr);
        let hash2 = hash_ptr(ptr);
        assert_eq!(hash1, hash2); // Same pointer should hash to same value
    }

    #[test]
    fn test_can_view_symbols() {
        set_kptr_restrict(KptrRestrictLevel::Privileged);
        assert!(!can_view_kernel_symbols(false));
        assert!(can_view_kernel_symbols(true));

        set_kptr_restrict(KptrRestrictLevel::AlwaysHidden);
        assert!(!can_view_kernel_symbols(false));
        assert!(!can_view_kernel_symbols(true));
    }

    #[test]
    fn test_init_sets_default() {
        init_kptr_restrict();
        assert_eq!(get_kptr_restrict(), KptrRestrictLevel::Privileged);
    }

    #[test]
    fn test_maximum_security() {
        enable_maximum_kptr_security();
        assert_eq!(get_kptr_restrict(), KptrRestrictLevel::AlwaysHidden);
    }

    #[test]
    fn test_security_mitigations() {
        set_kptr_restrict(KptrRestrictLevel::Privileged);
        let mitigations = get_security_mitigations();
        assert!(mitigations.kptr_restrict);
    }
}
