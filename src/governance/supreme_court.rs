// SPDX-License-Identifier: MIT
// SigmaOS Supreme Court Governance & Judicial Framework
// Implements Article I-V Judicial Chambers, Audit Inspection,
// Rollback Safety Verification, Sandbox Policy Enforcement, and Remedy Dispatch.


use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

/// Article II — Supreme Court Judicial Chambers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupremeCourtChamber {
    /// Oversees disputes related to kernel resilience, schedulers, and IPC performance
    KernelChamber,
    /// Arbitrates rollback failures, journaling disputes, and distributed VFS conflicts
    FilesystemChamber,
    /// Rules on sandbox bypasses, capability violations, firewall rules, and compliance breaches
    SecurityChamber,
    /// Reviews community module contributions and verifies compliance pipelines
    CommunityChamber,
}

/// Article I — Jurisdiction & Dispute Categories
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisputeCategory {
    ComplianceViolation,
    ModuleConflict,
    UserRightsBypass,
    RollbackFailure,
    SandboxBypass,
}

/// Judicial Dispute Record
#[derive(Debug, Clone)]
pub struct ComplianceDispute {
    pub dispute_id: u64,
    pub title: String,
    pub category: DisputeCategory,
    pub chamber: SupremeCourtChamber,
    pub complainant: String,
    pub target_module: String,
    pub audit_log_evidence: Vec<String>,
    pub rollback_success_rate: f32, // Must be >= 0.99 for appeals
    pub is_resolved: bool,
}

/// Article IV — Enforcement & Judicial Remedies
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JudicialRemedy {
    EnforceRollback { snapshot_id: String },
    ModuleQuarantine { module_name: String },
    GrantTransparencyReport { user_id: String },
    EnforceSandboxRule { rule_spec: String },
    DismissDispute { reason: String },
}

/// Article III & V — Master SigmaOS Supreme Court Engine
#[derive(Debug, Clone)]
pub struct SigmaSupremeCourtEngine {
    pub disputes: BTreeMap<u64, ComplianceDispute>,
    pub remedies: BTreeMap<u64, JudicialRemedy>,
    pub next_dispute_id: u64,
}

impl SigmaSupremeCourtEngine {
    pub fn new() -> Self {
        Self {
            disputes: BTreeMap::new(),
            remedies: BTreeMap::new(),
            next_dispute_id: 1,
        }
    }

    /// Article III — File a new compliance or rights dispute
    pub fn file_dispute(
        &mut self,
        title: &str,
        category: DisputeCategory,
        complainant: &str,
        target_module: &str,
        evidence: &[&str],
        rollback_rate: f32,
    ) -> u64 {
        let dispute_id = self.next_dispute_id;
        self.next_dispute_id += 1;

        let chamber = match category {
            DisputeCategory::ComplianceViolation | DisputeCategory::SandboxBypass => SupremeCourtChamber::SecurityChamber,
            DisputeCategory::RollbackFailure => SupremeCourtChamber::FilesystemChamber,
            DisputeCategory::ModuleConflict => SupremeCourtChamber::CommunityChamber,
            DisputeCategory::UserRightsBypass => SupremeCourtChamber::KernelChamber,
        };

        let dispute = ComplianceDispute {
            dispute_id,
            title: title.to_string(),
            category,
            chamber,
            complainant: complainant.to_string(),
            target_module: target_module.to_string(),
            audit_log_evidence: evidence.iter().map(|&s| s.to_string()).collect(),
            rollback_success_rate: rollback_rate,
            is_resolved: false,
        };

        self.disputes.insert(dispute_id, dispute);
        dispute_id
    }

    /// Article III & IV — Adjudicate dispute, run rollback/sandbox checks, and issue final binding remedy
    pub fn adjudicate_dispute(&mut self, dispute_id: u64) -> Result<JudicialRemedy, &'static str> {
        let dispute = self.disputes.get_mut(&dispute_id).ok_or("Dispute ID not found")?;

        // Article III Procedural Check: Rollback Test (>= 99% success required)
        if dispute.rollback_success_rate < 0.99 && dispute.category == DisputeCategory::RollbackFailure {
            let remedy = JudicialRemedy::EnforceRollback {
                snapshot_id: format!("auto-recovery-snap-{}", dispute_id),
            };
            dispute.is_resolved = true;
            self.remedies.insert(dispute_id, remedy.clone());
            return Ok(remedy);
        }

        let remedy = match dispute.category {
            DisputeCategory::ComplianceViolation | DisputeCategory::SandboxBypass => {
                JudicialRemedy::ModuleQuarantine {
                    module_name: dispute.target_module.clone(),
                }
            }
            DisputeCategory::UserRightsBypass => {
                JudicialRemedy::GrantTransparencyReport {
                    user_id: dispute.complainant.clone(),
                }
            }
            DisputeCategory::ModuleConflict => {
                JudicialRemedy::EnforceSandboxRule {
                    rule_spec: format!("isolate_module({})", dispute.target_module),
                }
            }
            DisputeCategory::RollbackFailure => {
                JudicialRemedy::EnforceRollback {
                    snapshot_id: "genesis-rollback-point".to_string(),
                }
            }
        };

        dispute.is_resolved = true;
        self.remedies.insert(dispute_id, remedy.clone());
        Ok(remedy)
    }

    pub fn get_dispute_summary(&self, dispute_id: u64) -> Option<String> {
        self.disputes.get(&dispute_id).map(|d| {
            format!(
                "Supreme Court [Chamber: {:?}] Dispute #{}: '{}' | Target: {} | Resolved: {}",
                d.chamber, d.dispute_id, d.title, d.target_module, d.is_resolved
            )
        })
    }
}

impl Default for SigmaSupremeCourtEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_supreme_court_jurisdiction_and_adjudication() {
        let mut court = SigmaSupremeCourtEngine::new();

        // File sandbox bypass dispute
        let id1 = court.file_dispute(
            "Untrusted driver sandbox bypass",
            DisputeCategory::SandboxBypass,
            "user_alice",
            "untrusted_net_driver",
            &["[AUDIT_LOG] Syscall #42 invoked outside pledge rules"],
            1.0,
        );

        assert_eq!(id1, 1);
        let summary1 = court.get_dispute_summary(id1).unwrap();
        assert!(summary1.contains("SecurityChamber"));

        // Adjudicate and assert quarantine remedy
        let remedy1 = court.adjudicate_dispute(id1).unwrap();
        assert_eq!(
            remedy1,
            JudicialRemedy::ModuleQuarantine {
                module_name: "untrusted_net_driver".to_string()
            }
        );

        // File rollback failure dispute
        let id2 = court.file_dispute(
            "Rollback verification failure below 99% threshold",
            DisputeCategory::RollbackFailure,
            "system_auditor",
            "vfs_journal_module",
            &["[VFS_LOG] CoW checksum mismatch"],
            0.95, // 95% < 99%
        );

        let remedy2 = court.adjudicate_dispute(id2).unwrap();
        assert_eq!(
            remedy2,
            JudicialRemedy::EnforceRollback {
                snapshot_id: "auto-recovery-snap-2".to_string()
            }
        );
    }
}
