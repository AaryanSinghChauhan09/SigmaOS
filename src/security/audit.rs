/// SigmaOS Security Audit Module (Phase 5/Section 8)
/// Implements security checks identified in the stabilization plan.

use std::string::String;
use std::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub enum AuditSeverity { Critical, High, Medium, Low, Info }

#[derive(Debug, Clone)]
pub struct AuditFinding {
    pub id: String,
    pub severity: AuditSeverity,
    pub component: String,
    pub description: String,
    pub remediation: String,
    pub resolved: bool,
}

pub struct SecurityAuditor {
    pub findings: Vec<AuditFinding>,
}

impl SecurityAuditor {
    pub fn new() -> Self { Self { findings: Vec::new() } }

    pub fn audit_installer_defaults(&mut self) {
        self.findings.push(AuditFinding {
            id: "SEC-001".into(), severity: AuditSeverity::Critical, component: "installer".into(),
            description: "Installer must not default to /dev/sda or any disk".into(),
            remediation: "Require explicit DiskTarget::Explicit() with user confirmation".into(),
            resolved: true, // Fixed in safe_installer.rs
        });
        self.findings.push(AuditFinding {
            id: "SEC-002".into(), severity: AuditSeverity::Critical, component: "installer".into(),
            description: "Passwords must never be stored in plaintext".into(),
            remediation: "Use Argon2id hashing before storage".into(),
            resolved: true, // Fixed in safe_installer.rs
        });
    }

    pub fn audit_package_signatures(&mut self) {
        self.findings.push(AuditFinding {
            id: "SEC-003".into(), severity: AuditSeverity::Critical, component: "package".into(),
            description: "Package verification must reject expired and revoked keys".into(),
            remediation: "Implemented in package/signing.rs with key expiry and revocation checks".into(),
            resolved: true,
        });
        self.findings.push(AuditFinding {
            id: "SEC-004".into(), severity: AuditSeverity::High, component: "package".into(),
            description: "Package install hooks must run sandboxed".into(),
            remediation: "Use pledge/unveil-style restrictions on hook execution".into(),
            resolved: false,
        });
    }

    pub fn audit_kernel_pointers(&mut self) {
        self.findings.push(AuditFinding {
            id: "SEC-005".into(), severity: AuditSeverity::Critical, component: "syscall".into(),
            description: "User-provided pointers must be validated before kernel access".into(),
            remediation: "Add copy_from_user/copy_to_user helpers with bounds checking".into(),
            resolved: false,
        });
    }

    pub fn run_full_audit(&mut self) {
        self.audit_installer_defaults();
        self.audit_package_signatures();
        self.audit_kernel_pointers();
    }

    pub fn unresolved_critical(&self) -> Vec<&AuditFinding> {
        self.findings.iter().filter(|f| !f.resolved && f.severity == AuditSeverity::Critical).collect()
    }

    pub fn report(&self) -> String {
        let resolved = self.findings.iter().filter(|f| f.resolved).count();
        let total = self.findings.len();
        let critical_open = self.unresolved_critical().len();
        format!("Security Audit: {}/{} resolved, {} critical open", resolved, total, critical_open)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_audit() {
        let mut auditor = SecurityAuditor::new();
        auditor.run_full_audit();
        assert!(auditor.findings.len() >= 5);
        let report = auditor.report();
        assert!(report.contains("resolved"));
    }
}
