// SigmaOS Legal & Licensing Framework
// Clear licensing policies, patent/IP strategy, and compliance certification metrics

use std::collections::HashMap;

/// Supported open-source and proprietary license types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LicenseType {
    Gpl2,
    Gpl3,
    Lgpl,
    Mit,
    Bsd,
    Apache2,
    Proprietary,
    Unknown,
}

impl LicenseType {
    pub fn is_permissive(&self) -> bool {
        matches!(
            self,
            LicenseType::Mit | LicenseType::Bsd | LicenseType::Apache2
        )
    }

    pub fn is_copyleft(&self) -> bool {
        matches!(
            self,
            LicenseType::Gpl2 | LicenseType::Gpl3 | LicenseType::Lgpl
        )
    }
}

/// A software component with license details
#[derive(Debug, Clone)]
pub struct ComponentLicense {
    pub name: String,
    pub version: String,
    pub license_type: LicenseType,
    pub spdx_id: String,
}

/// Security & Legal patent strategy records
#[derive(Debug, Clone)]
pub struct PatentRecord {
    pub id: String,
    pub title: String,
    pub country: String,
    pub registration_number: String,
    pub protection_status: String, // e.g. "Shielded", "OpenAccess", "Pending"
}

/// Industry compliance certifications (FIPS, Common Criteria, etc.)
#[derive(Debug, Clone)]
pub struct ComplianceCert {
    pub name: String,
    pub standard_id: String,
    pub status: String, // e.g. "Targeted", "InReview", "Certified"
    pub audit_date: Option<String>,
}

/// Risk level classification for Contract Audits
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Identified contract risk details
#[derive(Debug, Clone)]
pub struct ContractRisk {
    pub description: String,
    pub level: RiskLevel,
    pub recommendation: String,
}

/// Generated audit report for legal contracts (NDAs, SLAs, Terms)
#[derive(Debug, Clone)]
pub struct ContractAuditReport {
    pub title: String,
    pub risks: Vec<ContractRisk>,
    pub overall_score: u32, // Risk score out of 100 (higher is riskier)
    pub recommendation: String,
}

/// Dynamic checklist for Regulatory Privacy Compliance (GDPR, CCPA, HIPAA)
#[derive(Debug, Clone)]
pub struct PrivacyComplianceChecklist {
    pub user_consent_enabled: bool,
    pub right_to_be_forgotten_functional: bool,
    pub data_minimization_enforced: bool,
    pub data_encryption_at_rest: bool,
    pub activity_audit_logging: bool,
}

impl PrivacyComplianceChecklist {
    /// Evaluates if the system passes regulatory privacy gates
    pub fn is_compliant(&self) -> bool {
        self.user_consent_enabled
            && self.right_to_be_forgotten_functional
            && self.data_minimization_enforced
            && self.data_encryption_at_rest
            && self.activity_audit_logging
    }

    /// Evaluates the missing compliance dimensions
    pub fn get_missing_standards(&self) -> Vec<&'static str> {
        let mut missing = Vec::new();
        if !self.user_consent_enabled {
            missing.push("Consent Gate (GDPR Art. 7)");
        }
        if !self.right_to_be_forgotten_functional {
            missing.push("Right to Erasure (GDPR Art. 17)");
        }
        if !self.data_minimization_enforced {
            missing.push("Data Minimization (GDPR Art. 5)");
        }
        if !self.data_encryption_at_rest {
            missing.push("AES Encryption-at-Rest (HIPAA Security Rule)");
        }
        if !self.activity_audit_logging {
            missing.push("Traceable Activity Audit Logging (ISO 27001)");
        }
        missing
    }
}

/// Legal Compliance Registry Manager
pub struct LegalComplianceRegistry {
    pub licensed_components: HashMap<String, ComponentLicense>,
    pub patent_pool: HashMap<String, PatentRecord>,
    pub compliance_certs: HashMap<String, ComplianceCert>,
    pub default_strict_copyleft_allow: bool,
}

impl LegalComplianceRegistry {
    pub fn new() -> Self {
        Self {
            licensed_components: HashMap::new(),
            patent_pool: HashMap::new(),
            compliance_certs: HashMap::new(),
            default_strict_copyleft_allow: true,
        }
    }

    pub fn register_component(
        &mut self,
        name: String,
        version: String,
        license: LicenseType,
        spdx: String,
    ) {
        let comp = ComponentLicense {
            name: name.clone(),
            version,
            license_type: license,
            spdx_id: spdx,
        };
        self.licensed_components.insert(name, comp);
    }

    pub fn register_patent(
        &mut self,
        id: String,
        title: String,
        country: String,
        reg_num: String,
        status: String,
    ) {
        let patent = PatentRecord {
            id: id.clone(),
            title,
            country,
            registration_number: reg_num,
            protection_status: status,
        };
        self.patent_pool.insert(id, patent);
    }

    pub fn register_cert(&mut self, name: String, standard_id: String, status: String) {
        let cert = ComplianceCert {
            name: name.clone(),
            standard_id,
            status,
            audit_date: None,
        };
        self.compliance_certs.insert(name, cert);
    }

    /// Verifies if a component complies with distribution policies (e.g., no Proprietary in open-source builds)
    pub fn verify_license_compliance(&self, component_name: &str, allow_proprietary: bool) -> bool {
        if let Some(comp) = self.licensed_components.get(component_name) {
            match comp.license_type {
                LicenseType::Proprietary => allow_proprietary,
                LicenseType::Unknown => false,
                _ => true,
            }
        } else {
            false
        }
    }

    /// Analyzes the license compatibility between two software components to prevent licensing violations
    pub fn verify_license_compatibility(&self, comp_a: &str, comp_b: &str) -> Result<bool, &'static str> {
        let license_a = self.licensed_components.get(comp_a)
            .ok_or("Component A not found in legal registry")?.license_type;
        let license_b = self.licensed_components.get(comp_b)
            .ok_or("Component B not found in legal registry")?.license_type;

        // GPL-3.0 and Proprietary are strictly incompatible
        if (license_a == LicenseType::Gpl3 && license_b == LicenseType::Proprietary) ||
           (license_b == LicenseType::Gpl3 && license_a == LicenseType::Proprietary) {
            return Ok(false);
        }

        // GPL-2.0 and Apache-2.0 are incompatible according to Free Software Foundation guidelines
        if (license_a == LicenseType::Gpl2 && license_b == LicenseType::Apache2) ||
           (license_b == LicenseType::Gpl2 && license_a == LicenseType::Apache2) {
            return Ok(false);
        }

        Ok(true)
    }

    /// Scans legal contract texts (such as NDAs, Terms of Service, or SLAs) to identify high-risk legal clauses
    pub fn audit_contract_text(&self, title: &str, text: &str) -> ContractAuditReport {
        let mut risks = Vec::new();
        let mut score = 0;
        let lowercase_text = text.to_lowercase();

        // 1. Unilateral Modification
        if lowercase_text.contains("unilateral") || lowercase_text.contains("reserve the right to modify") {
            risks.push(ContractRisk {
                description: "Unilateral Contract Modification detected.".to_string(),
                level: RiskLevel::High,
                recommendation: "Negotiate bilateral consent requirements for any terms or service changes.".to_string(),
            });
            score += 35;
        }

        // 2. Unlimited Liability / Cap Exclusion
        if lowercase_text.contains("unlimited liability") || lowercase_text.contains("no liability limits") {
            risks.push(ContractRisk {
                description: "Lack of liability limitation detected.".to_string(),
                level: RiskLevel::Critical,
                recommendation: "Ensure liability caps are strictly defined and proportional to commercial contract value.".to_string(),
            });
            score += 45;
        }

        // 3. Indemnification Clauses
        if lowercase_text.contains("indemnify and hold harmless") || lowercase_text.contains("shall indemnify") {
            risks.push(ContractRisk {
                description: "Broad, unsanitized indemnification terms detected.".to_string(),
                level: RiskLevel::Medium,
                recommendation: "Incorporate standard negligence exclusions and carve-outs to balance indemnity exposure.".to_string(),
            });
            score += 15;
        }

        // 4. Intellectual Property Broad Assignment
        if lowercase_text.contains("assigns all rights") || lowercase_text.contains("shall assign intellectual property") {
            risks.push(ContractRisk {
                description: "Broad, non-specific Intellectual Property assignment detected.".to_string(),
                level: RiskLevel::High,
                recommendation: "Refactor assignment to restrict transfer to custom deliverables only, retaining core IP.".to_string(),
            });
            score += 25;
        }

        // Bound final score to 100
        if score > 100 {
            score = 100;
        }

        let recommendation = if score >= 60 {
            "CRITICAL LEGAL ACTION REQUIRED: Broad high-risk liability and IP transfers detected. Do not sign without amendment.".to_string()
        } else if score >= 30 {
            "MEDIUM LEGAL ATTENTION ADVISED: Certain unilateral or indemnity clauses warrant renegotiation or carve-outs.".to_string()
        } else {
            "LOW RISK: Contract text represents standard permissive and bilateral operating guidelines.".to_string()
        };

        ContractAuditReport {
            title: title.to_string(),
            risks,
            overall_score: score,
            recommendation,
        }
    }

    pub fn check_overall_compliance_percentage(&self) -> f64 {
        if self.compliance_certs.is_empty() {
            return 100.0;
        }
        let certified = self
            .compliance_certs
            .values()
            .filter(|c| c.status == "Certified")
            .count();
        (certified as f64 / self.compliance_certs.len() as f64) * 100.0
    }
}

impl Default for LegalComplianceRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_license_properties() {
        assert!(LicenseType::Mit.is_permissive());
        assert!(!LicenseType::Mit.is_copyleft());
        assert!(LicenseType::Gpl3.is_copyleft());
    }

    #[test]
    fn test_license_compliance() {
        let mut registry = LegalComplianceRegistry::new();
        registry.register_component(
            "KernelCore".to_string(),
            "0.1.0".to_string(),
            LicenseType::Gpl2,
            "GPL-2.0-only".to_string(),
        );
        registry.register_component(
            "ProprietaryBlob".to_string(),
            "1.0.0".to_string(),
            LicenseType::Proprietary,
            "LicenseRef-Proprietary".to_string(),
        );

        assert!(registry.verify_license_compliance("KernelCore", false));
        assert!(!registry.verify_license_compliance("ProprietaryBlob", false));
        assert!(registry.verify_license_compliance("ProprietaryBlob", true));
    }

    #[test]
    fn test_compliance_and_patents() {
        let mut registry = LegalComplianceRegistry::new();
        registry.register_patent(
            "PAT-01".to_string(),
            "Zero-Latency Sovereign IPC".to_string(),
            "India".to_string(),
            "IN123456".to_string(),
            "Shielded".to_string(),
        );
        assert_eq!(registry.patent_pool.len(), 1);

        registry.register_cert(
            "Common Criteria EAL6+".to_string(),
            "ISO-15408".to_string(),
            "InReview".to_string(),
        );
        registry.register_cert(
            "FIPS 140-3 Level 4".to_string(),
            "FIPS-140-3".to_string(),
            "Certified".to_string(),
        );

        assert_eq!(registry.check_overall_compliance_percentage(), 50.0);
    }

    #[test]
    fn test_contract_audit_risk_detection() {
        let registry = LegalComplianceRegistry::new();

        let contract_text = "This agreement is subject to unilateral changes. The developer assigns all rights to the client and accepts unlimited liability, agreeing to indemnify and hold harmless the client.";
        let report = registry.audit_contract_text("Enterprise SLA NDA", contract_text);

        assert_eq!(report.title, "Enterprise SLA NDA");
        assert!(report.risks.len() >= 3);
        assert!(report.overall_score >= 80);
        assert!(report.recommendation.contains("CRITICAL LEGAL ACTION REQUIRED"));
    }

    #[test]
    fn test_license_compatibility_checker() {
        let mut registry = LegalComplianceRegistry::new();
        registry.register_component(
            "LibGpl3".to_string(),
            "3.0.0".to_string(),
            LicenseType::Gpl3,
            "GPL-3.0-or-later".to_string(),
        );
        registry.register_component(
            "ClosedEngine".to_string(),
            "1.2.0".to_string(),
            LicenseType::Proprietary,
            "Proprietary".to_string(),
        );
        registry.register_component(
            "PermissiveBsd".to_string(),
            "2.0.0".to_string(),
            LicenseType::Bsd,
            "BSD-3-Clause".to_string(),
        );

        let gpl_prop_compatible = registry.verify_license_compatibility("LibGpl3", "ClosedEngine").unwrap();
        assert!(!gpl_prop_compatible);

        let gpl_bsd_compatible = registry.verify_license_compatibility("LibGpl3", "PermissiveBsd").unwrap();
        assert!(gpl_bsd_compatible);
    }

    #[test]
    fn test_privacy_compliance_checklists() {
        let non_compliant = PrivacyComplianceChecklist {
            user_consent_enabled: false,
            right_to_be_forgotten_functional: true,
            data_minimization_enforced: false,
            data_encryption_at_rest: true,
            activity_audit_logging: false,
        };

        assert!(!non_compliant.is_compliant());
        let missing = non_compliant.get_missing_standards();
        assert_eq!(missing.len(), 3);
        assert!(missing.contains(&"Consent Gate (GDPR Art. 7)"));

        let compliant = PrivacyComplianceChecklist {
            user_consent_enabled: true,
            right_to_be_forgotten_functional: true,
            data_minimization_enforced: true,
            data_encryption_at_rest: true,
            activity_audit_logging: true,
        };
        assert!(compliant.is_compliant());
        assert_eq!(compliant.get_missing_standards().len(), 0);
    }
}
