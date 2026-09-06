#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
use std::string::{String, ToString};
use std::vec::Vec;
use std::format;
// SigmaOS Legal & Licensing Framework
// Clear licensing policies, patent/IP strategy, and compliance certification metrics

use crate::klib::HashMap;

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

#[cfg(test_disabled)]
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
}
