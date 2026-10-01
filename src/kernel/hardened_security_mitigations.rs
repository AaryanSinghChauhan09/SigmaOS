// SPDX-License-Identifier: MIT
// SigmaOS Sovereign Kernel Subsystem (`src/kernel/hardened_security_mitigations.rs`)
// Linux kptr_restrict / dmesg_restrict, BSD security sysctl, and Control Flow Integrity (CFI) Engine

use std::collections::BTreeMap;
use std::format;
use std::string::String;
use std::vec::Vec;

/// Kernel Pointer Restriction Mode (`/proc/sys/kernel/kptr_restrict`)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KptrRestrictLevel {
    ExposeRaw = 0,         // %px exposes raw kernel addresses
    ZeroNonRoot = 1,       // Kernel pointers zeroed out for unprivileged callers
    ZeroAll = 2,           // Kernel pointers zeroed out for all callers (hardened)
}

/// Control Flow Integrity (CFI) indirect call signature
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CfiFunctionSignature {
    pub function_name: String,
    pub expected_hash: u64,
    pub target_address: usize,
}

/// Sovereign Kernel Hardened Security Mitigations Engine
pub struct SovereignHardenedSecurityMitigationsEngine {
    kptr_restrict: KptrRestrictLevel,
    dmesg_restrict: bool,
    unprivileged_proc_debug_allowed: bool,
    hardlink_check_enabled: bool,
    cfi_signatures: BTreeMap<usize, CfiFunctionSignature>,
}

impl SovereignHardenedSecurityMitigationsEngine {
    pub fn new() -> Self {
        Self {
            kptr_restrict: KptrRestrictLevel::ZeroAll,
            dmesg_restrict: true,
            unprivileged_proc_debug_allowed: false,
            hardlink_check_enabled: true,
            cfi_signatures: BTreeMap::new(),
        }
    }

    /// Set kernel pointer restriction level (`kptr_restrict`)
    pub fn set_kptr_restrict(&mut self, level: KptrRestrictLevel) {
        self.kptr_restrict = level;
    }

    /// Format a kernel address for userland display based on active `kptr_restrict` setting
    pub fn format_kptr(&self, address: usize, is_root: bool) -> String {
        match self.kptr_restrict {
            KptrRestrictLevel::ExposeRaw => format!("{:#018x}", address),
            KptrRestrictLevel::ZeroNonRoot => {
                if is_root {
                    format!("{:#018x}", address)
                } else {
                    String::from("0x0000000000000000")
                }
            }
            KptrRestrictLevel::ZeroAll => String::from("0x0000000000000000"),
        }
    }

    /// Set kernel dmesg ring buffer restriction (`dmesg_restrict`)
    pub fn set_dmesg_restrict(&mut self, restrict: bool) {
        self.dmesg_restrict = restrict;
    }

    /// Verify whether a process can access kernel dmesg logs
    pub fn can_access_dmesg(&self, is_root: bool) -> bool {
        if self.dmesg_restrict {
            is_root
        } else {
            true
        }
    }

    /// Register a CFI indirect call signature target
    pub fn register_cfi_target(&mut self, target_address: usize, name: &str, signature_hash: u64) {
        self.cfi_signatures.insert(
            target_address,
            CfiFunctionSignature {
                function_name: String::from(name),
                expected_hash: signature_hash,
                target_address,
            },
        );
    }

    /// Validate a forward-edge indirect function call against registered CFI signatures
    pub fn validate_indirect_call(&self, target_address: usize, actual_hash: u64) -> Result<String, &'static str> {
        let sig = self
            .cfi_signatures
            .get(&target_address)
            .ok_or("EPROT: Unregistered CFI indirect call target")?;

        if sig.expected_hash != actual_hash {
            return Err("EPERM: Forward-edge CFI signature mismatch (control flow hijack attempt)");
        }

        Ok(format!(
            "CFI verified indirect call to '{}' at {:#018x}",
            sig.function_name, target_address
        ))
    }
}

impl Default for SovereignHardenedSecurityMitigationsEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hardened_security_mitigations_flow() {
        let mut engine = SovereignHardenedSecurityMitigationsEngine::new();

        // 1. Verify default kptr_restrict = ZeroAll
        let formatted = engine.format_kptr(0xffffffff81000000, true);
        assert_eq!(formatted, "0x0000000000000000");

        // 2. Change to ZeroNonRoot
        engine.set_kptr_restrict(KptrRestrictLevel::ZeroNonRoot);
        assert_eq!(engine.format_kptr(0xffffffff81000000, false), "0x0000000000000000");
        assert_eq!(engine.format_kptr(0xffffffff81000000, true), "0xffffffff81000000");

        // 3. Verify dmesg restriction
        assert!(engine.can_access_dmesg(true));
        assert!(!engine.can_access_dmesg(false));

        // 4. Register CFI target & validate
        let target = 0xffffffff82001000;
        let hash = 0x1A2B3C4D5E6F7890;
        engine.register_cfi_target(target, "sys_read", hash);

        let cfi_res = engine.validate_indirect_call(target, hash).unwrap();
        assert!(cfi_res.contains("sys_read"));

        // Invalid hash should fail
        assert!(engine.validate_indirect_call(target, 0xBAD).is_err());
    }
}
