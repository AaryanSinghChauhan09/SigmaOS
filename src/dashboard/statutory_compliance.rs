// Statutory Governance & Compliance Overlay Dashboard for SigmaOS
// Integrates global statutory frameworks (GDPR, ISO 27001, Indian DPDP Act 2023, HIPAA, PCI-DSS).

use std::collections::HashMap;

/// Statutory regulatory frameworks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StatutoryFramework {
    Gdpr,                // EU General Data Protection Regulation
    IndianDpdpAct2023,   // Indian Digital Personal Data Protection Act 2023
    Iso27001,            // ISO/IEC 27001 Information Security
    Hipaa,               // Health Insurance Portability and Accountability Act
    PciDss,              // Payment Card Industry Data Security Standard
}

impl StatutoryFramework {
    pub fn as_str(&self) -> &'static str {
        match self {
            StatutoryFramework::Gdpr => "EU GDPR",
            StatutoryFramework::IndianDpdpAct2023 => "Indian DPDP Act 2023",
            StatutoryFramework::Iso27001 => "ISO/IEC 27001",
            StatutoryFramework::Hipaa => "US HIPAA",
            StatutoryFramework::PciDss => "PCI-DSS v4.0",
        }
    }
}

/// Statutory compliance rule status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplianceRuleStatus {
    Compliant,
    Warning,
    Breached,
    NotApplicable,
}

/// Statutory Governance Rule entry
#[derive(Debug, Clone)]
pub struct StatutoryGovernanceRule {
    pub rule_id: String,
    pub framework: StatutoryFramework,
    pub description: String,
    pub status: ComplianceRuleStatus,
    pub max_penalty_amount_usd: u64,
}

/// Breach Alert notification
#[derive(Debug, Clone)]
pub struct StatutoryBreachAlert {
    pub rule_id: String,
    pub framework: StatutoryFramework,
    pub breach_details: String,
    pub potential_fine_usd: u64,
    pub timestamp: u64,
}

/// Penalty Breach Notifier Subsystem
pub struct PenaltyBreachNotifier {
    pub alerts: Vec<StatutoryBreachAlert>,
}

impl PenaltyBreachNotifier {
    pub fn new() -> Self {
        Self { alerts: Vec::new() }
    }

    pub fn notify_breach(&mut self, rule: &StatutoryGovernanceRule, breach_details: &str, timestamp: u64) {
        self.alerts.push(StatutoryBreachAlert {
            rule_id: rule.rule_id.clone(),
            framework: rule.framework,
            breach_details: breach_details.to_string(),
            potential_fine_usd: rule.max_penalty_amount_usd,
            timestamp,
        });
    }
}

impl Default for PenaltyBreachNotifier {
    fn default() -> Self {
        Self::new()
    }
}

/// Dispute Audit Rollback Engine for regulatory compliance rollbacks
pub struct DisputeAuditRollbackEngine {
    pub audit_checkpoint_history: Vec<(usize, String)>,
}

impl DisputeAuditRollbackEngine {
    pub fn new() -> Self {
        Self {
            audit_checkpoint_history: Vec::new(),
        }
    }

    pub fn create_audit_checkpoint(&mut self, checkpoint_id: usize, state_hash: &str) {
        self.audit_checkpoint_history.push((checkpoint_id, state_hash.to_string()));
    }

    pub fn rollback_dispute_checkpoint(&mut self, target_checkpoint_id: usize) -> Result<String, &'static str> {
        if let Some(pos) = self.audit_checkpoint_history.iter().position(|(id, _)| *id == target_checkpoint_id) {
            let hash = self.audit_checkpoint_history[pos].1.clone();
            self.audit_checkpoint_history.truncate(pos + 1);
            Ok(hash)
        } else {
            Err("Audit checkpoint not found for rollback")
        }
    }
}

impl Default for DisputeAuditRollbackEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Main Statutory Governance & Compliance Overlay Layer
pub struct StatutoryGovernanceLayer {
    pub rules: HashMap<String, StatutoryGovernanceRule>,
    pub notifier: PenaltyBreachNotifier,
    pub rollback_engine: DisputeAuditRollbackEngine,
}

impl StatutoryGovernanceLayer {
    pub fn new() -> Self {
        let mut layer = Self {
            rules: HashMap::new(),
            notifier: PenaltyBreachNotifier::new(),
            rollback_engine: DisputeAuditRollbackEngine::new(),
        };
        layer.seed_statutory_rules();
        layer
    }

    pub fn seed_statutory_rules(&mut self) {
        self.register_rule(StatutoryGovernanceRule {
            rule_id: "DPDP-SEC-01".to_string(),
            framework: StatutoryFramework::IndianDpdpAct2023,
            description: "Personal Data Breach Prevention & Consent Management".to_string(),
            status: ComplianceRuleStatus::Compliant,
            max_penalty_amount_usd: 30_000_000, // INR 250 Crore
        });

        self.register_rule(StatutoryGovernanceRule {
            rule_id: "GDPR-ART-32".to_string(),
            framework: StatutoryFramework::Gdpr,
            description: "Security of Data Processing & Cryptographic Erasure".to_string(),
            status: ComplianceRuleStatus::Compliant,
            max_penalty_amount_usd: 20_000_000,
        });

        self.register_rule(StatutoryGovernanceRule {
            rule_id: "ISO-27001-A12".to_string(),
            framework: StatutoryFramework::Iso27001,
            description: "Operations Security & Logging Audit Traces".to_string(),
            status: ComplianceRuleStatus::Compliant,
            max_penalty_amount_usd: 5_000_000,
        });
    }

    pub fn register_rule(&mut self, rule: StatutoryGovernanceRule) {
        self.rules.insert(rule.rule_id.clone(), rule);
    }

    pub fn evaluate_compliance_posture(&mut self, current_timestamp: u64) -> u32 {
        let total_rules = self.rules.len();
        if total_rules == 0 {
            return 100;
        }

        let mut compliant_count = 0;
        for rule in self.rules.values_mut() {
            if rule.status == ComplianceRuleStatus::Breached {
                self.notifier.notify_breach(rule, "Automated compliance audit detected rule breach", current_timestamp);
            } else if rule.status == ComplianceRuleStatus::Compliant {
                compliant_count += 1;
            }
        }

        ((compliant_count as f32 / total_rules as f32) * 100.0) as u32
    }
}

impl Default for StatutoryGovernanceLayer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statutory_compliance_layer_and_breach_notifications() {
        let mut layer = StatutoryGovernanceLayer::new();

        // 1. Initial posture should be 100% compliant
        let score = layer.evaluate_compliance_posture(1000);
        assert_eq!(score, 100);

        // 2. Mark DPDP rule as breached
        if let Some(rule) = layer.rules.get_mut("DPDP-SEC-01") {
            rule.status = ComplianceRuleStatus::Breached;
        }

        let score_after_breach = layer.evaluate_compliance_posture(1005);
        assert!(score_after_breach < 100);
        assert_eq!(layer.notifier.alerts.len(), 1);
        assert_eq!(layer.notifier.alerts[0].framework, StatutoryFramework::IndianDpdpAct2023);

        // 3. Test Dispute Audit Rollback Engine
        layer.rollback_engine.create_audit_checkpoint(1, "hash_state_001");
        layer.rollback_engine.create_audit_checkpoint(2, "hash_state_002");

        let hash = layer.rollback_engine.rollback_dispute_checkpoint(1).unwrap();
        assert_eq!(hash, "hash_state_001");
        assert_eq!(layer.rollback_engine.audit_checkpoint_history.len(), 1);
    }
}
