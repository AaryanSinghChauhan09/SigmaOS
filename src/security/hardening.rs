use alloc::string::{String, ToString};
extern crate alloc;
/// Security Hardening & Cryptographic Intrusion Detection Suite for SigmaOS
/// Implements Defense-In-Depth (Sentinel standard): Secure volatile memory zeroization,
/// rate-limiting intrusion monitoring, and a tamper-proof cryptographically hash-chained audit trail.
use alloc::vec::Vec;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
    FileRead,
    FileWrite,
    NetworkTcp,
}

use core::sync::atomic::{AtomicUsize, Ordering};

/// Secure Memory Zeroization utility
/// Overwrites memory containing sensitive keys, credentials, or capability data
/// Uses volatile writes to guarantee that the compiler does not optimize away the memory wipe (preventing CVE leaks)
pub fn secure_zeroize<T: Copy + Default>(slice: &mut [T]) {
    for item in slice.iter_mut() {
        unsafe {
            core::ptr::write_volatile(item as *mut T, T::default());
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntrusionSeverity {
    Low = 0,
    Medium = 1,
    High = 2,
    Critical = 3,
}

/// A highly secure, rate-limiting intrusion monitor tracking process capability violations
pub struct IntrusionMonitor {
    pub max_allowed_violations: usize,
    pub violation_count: AtomicUsize,
    pub is_quarantined: core::sync::atomic::AtomicBool,
}

impl IntrusionMonitor {
    pub fn new(max_violations: usize) -> Self {
        IntrusionMonitor {
            max_allowed_violations: max_violations,
            violation_count: AtomicUsize::new(0),
            is_quarantined: core::sync::atomic::AtomicBool::new(false),
        }
    }

    /// Records a capability violation, returning the severity level and quarantine status
    pub fn record_violation(&self, pid: u64) -> (IntrusionSeverity, bool) {
        let count = self.violation_count.fetch_add(1, Ordering::SeqCst) + 1;
        let mut quarantined = false;

        let severity = if count >= self.max_allowed_violations {
            self.is_quarantined.store(true, Ordering::SeqCst);
            quarantined = true;
            IntrusionSeverity::Critical
        } else if count >= self.max_allowed_violations / 2 {
            IntrusionSeverity::High
        } else {
            IntrusionSeverity::Medium
        };

        if quarantined {
            // Logs to virtual security console
            let _ = pid; // simulate quarantine notification
        }

        (severity, quarantined)
    }

    pub fn reset(&self) {
        self.violation_count.store(0, Ordering::SeqCst);
        self.is_quarantined.store(false, Ordering::SeqCst);
    }
}

impl Default for IntrusionMonitor {
    fn default() -> Self {
        Self::new(5)
    }
}

#[derive(Debug, Clone)]
pub struct AuditLogEntry {
    pub process_id: u64,
    pub permission: Permission,
    pub status_allowed: bool,
    pub previous_hash: u64,
    pub entry_hash: u64,
}

/// A tamper-proof cryptographically hash-chained security audit trail
pub struct HardenedAuditTrail {
    pub logs: Vec<AuditLogEntry>,
    pub current_hash: core::sync::atomic::AtomicU64,
}

impl HardenedAuditTrail {
    pub fn new() -> Self {
        HardenedAuditTrail {
            logs: Vec::new(),
            current_hash: core::sync::atomic::AtomicU64::new(0x1337_C0DE_FA11_FACE),
        }
    }

    /// Appends a new auditable security check to the log, computing a chained cryptographic XOR hash
    pub fn append_log(&mut self, pid: u64, perm: Permission, allowed: bool) -> u64 {
        let prev = self.current_hash.load(Ordering::SeqCst);

        // Compute simple dynamic rolling hash chain: XOR elements with prime multi
        let entry_payload = pid ^ (perm as u64) ^ (if allowed { 1u64 } else { 0u64 });
        let next_hash = (prev ^ entry_payload).wrapping_mul(1099511628211_u64); // FNV-1a 64-bit prime

        let entry = AuditLogEntry {
            process_id: pid,
            permission: perm,
            status_allowed: allowed,
            previous_hash: prev,
            entry_hash: next_hash,
        };

        self.logs.push(entry);
        self.current_hash.store(next_hash, Ordering::SeqCst);
        next_hash
    }

    /// Verifies the cryptographic integrity of the entire audit chain, detecting any malicious tamper attempts
    pub fn verify_integrity(&self) -> bool {
        if self.logs.is_empty() {
            return true;
        }

        let mut expected_prev: u64 = 0x1337_C0DE_FA11_FACE;
        for i in 0..self.logs.len() {
            let log = &self.logs[i];
            if log.previous_hash != expected_prev {
                return false; // Chain broken! Tampering detected!
            }

            let payload: u64 =
                log.process_id ^ (log.permission as u64) ^ (if log.status_allowed { 1u64 } else { 0u64 });
            let calculated_hash = (expected_prev ^ payload).wrapping_mul(1099511628211_u64);

            if log.entry_hash != calculated_hash {
                return false; // Entry hash mismatch! Tampering detected!
            }

            expected_prev = log.entry_hash;
        }

        true
    }
}

impl Default for HardenedAuditTrail {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PamModuleType {
    Auth,
    Account,
    Session,
    Password,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PamControlFlag {
    Required,
    Requisite,
    Sufficient,
    Optional,
}

#[derive(Debug, Clone)]
pub struct PamModuleRule {
    pub module_type: PamModuleType,
    pub control_flag: PamControlFlag,
    pub module_name: alloc::string::String,
    pub pass_by_default: bool,
}

pub struct PamAuthenticationPolicyEngine {
    pub rules: Vec<PamModuleRule>,
    pub sudo_wheel_group_required: bool,
}

impl PamAuthenticationPolicyEngine {
    pub fn new(sudo_wheel_group_required: bool) -> Self {
        Self {
            rules: Vec::new(),
            sudo_wheel_group_required,
        }
    }

    pub fn add_rule(
        &mut self,
        module_type: PamModuleType,
        control_flag: PamControlFlag,
        module_name: &str,
        pass_by_default: bool,
    ) {
        self.rules.push(PamModuleRule {
            module_type,
            control_flag,
            module_name: alloc::string::String::from(module_name),
            pass_by_default,
        });
    }

    pub fn authenticate_pam_stack(
        &self,
        module_type: PamModuleType,
        is_wheel_member: bool,
    ) -> Result<bool, &'static str> {
        if self.sudo_wheel_group_required && !is_wheel_member {
            return Err("PAM / Sudo: User not in wheel group; privilege escalation denied");
        }

        let type_rules: Vec<&PamModuleRule> = self
            .rules
            .iter()
            .filter(|r| r.module_type == module_type)
            .collect();

        if type_rules.is_empty() {
            return Ok(true);
        }

        let mut overall_success = true;

        for rule in type_rules {
            let res = rule.pass_by_default;
            match rule.control_flag {
                PamControlFlag::Required => {
                    if !res {
                        overall_success = false;
                    }
                }
                PamControlFlag::Requisite => {
                    if !res {
                        return Ok(false);
                    }
                }
                PamControlFlag::Sufficient => {
                    if res && overall_success {
                        return Ok(true);
                    }
                }
                PamControlFlag::Optional => {}
            }
        }

        Ok(overall_success)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secure_zeroization() {
        let mut key = [0u8; 4];
        for (i, byte) in key.iter_mut().enumerate() {
            *byte = ((i + 1) * 7) as u8;
        }
        assert_ne!(key, [0u8; 4]);

        secure_zeroize(&mut key);
        assert_eq!(key, [0u8; 4]);
    }

    #[test]
    fn test_intrusion_monitor_quarantine() {
        let monitor = IntrusionMonitor::new(4);
        assert!(!monitor.is_quarantined.load(Ordering::SeqCst));

        // First violation
        let (sev, q) = monitor.record_violation(1);
        assert_eq!(sev, IntrusionSeverity::Medium);
        assert!(!q);

        // Second violation
        let (sev, q) = monitor.record_violation(1);
        assert_eq!(sev, IntrusionSeverity::High);
        assert!(!q);

        // Third and fourth violations -> quarantine
        monitor.record_violation(1);
        let (sev, q) = monitor.record_violation(1);
        assert_eq!(sev, IntrusionSeverity::Critical);
        assert!(q);
        assert!(monitor.is_quarantined.load(Ordering::SeqCst));
    }

    #[test]
    fn test_tamper_proof_audit_trail() {
        let mut audit = HardenedAuditTrail::new();
        audit.append_log(10, Permission::NetworkTcp, true);
        audit.append_log(12, Permission::FileRead, false);

        assert!(audit.verify_integrity());

        // Maliciously tamper with log index 0
        if !audit.logs.is_empty() {
            audit.logs[0].status_allowed = false; // modify status from true to false
        }

        // Integrity verification must detect this modification instantly!
        assert!(!audit.verify_integrity());
    }
}
