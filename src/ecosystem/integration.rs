// SigmaOS Ecosystem Integration Framework
// Mobile/embedded presence matrices, enterprise partnerships, and hardware/software certification pipelines

use std::collections::HashMap;

/// Hardware architectures supported by SigmaOS
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchTier {
    Tier1, // Fully supported, automated CI
    Tier2, // Compiles, partially tested
    Tier3, // Planned or community-maintained
}

/// Target market ecosystem classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EcosystemPlatform {
    Mobile,
    EmbeddedIoT,
    EnterpriseServer,
    SovereignCloud,
}

/// An architecture port details
#[derive(Debug, Clone)]
pub struct ArchitecturePort {
    pub name: String,
    pub platform: EcosystemPlatform,
    pub tier: ArchTier,
    pub is_bootable: bool,
}

/// Enterprise relationship / partner details (SAP, Oracle, IBM, etc.)
#[derive(Debug, Clone)]
pub struct EnterprisePartner {
    pub partner_name: String,
    pub service_scope: String, // e.g., "ERP Database Integration", "AI-Native Cloud Compute"
    pub contract_level: String, // e.g., "Strategic", "Standard"
    pub verified_and_integrated: bool,
}

/// Certification status for physical hardware or third-party enterprise software packages
#[derive(Debug, Clone)]
pub struct EcosystemCertification {
    pub product_id: String,
    pub product_name: String,
    pub hardware_compatible: bool,
    pub certification_status: String, // e.g., "Passed", "Failed", "Pending"
    pub compliance_stamp: Option<String>,
}

/// Ecosystem Integration Manager
pub struct EcosystemManager {
    pub architecture_matrix: HashMap<String, ArchitecturePort>,
    pub enterprise_partners: HashMap<String, EnterprisePartner>,
    pub cert_pipeline: HashMap<String, EcosystemCertification>,
}

impl EcosystemManager {
    pub fn new() -> Self {
        Self {
            architecture_matrix: HashMap::new(),
            enterprise_partners: HashMap::new(),
            cert_pipeline: HashMap::new(),
        }
    }

    pub fn register_architecture(
        &mut self,
        name: String,
        platform: EcosystemPlatform,
        tier: ArchTier,
        is_bootable: bool,
    ) {
        let port = ArchitecturePort {
            name: name.clone(),
            platform,
            tier,
            is_bootable,
        };
        self.architecture_matrix.insert(name, port);
    }

    pub fn register_partner(&mut self, name: String, scope: String, contract: String) {
        let partner = EnterprisePartner {
            partner_name: name.clone(),
            service_scope: scope,
            contract_level: contract,
            verified_and_integrated: false,
        };
        self.enterprise_partners.insert(name, partner);
    }

    pub fn verify_partner_integration(&mut self, name: &str) -> bool {
        if let Some(partner) = self.enterprise_partners.get_mut(name) {
            partner.verified_and_integrated = true;
            true
        } else {
            false
        }
    }

    pub fn submit_certification(&mut self, cert: EcosystemCertification) {
        self.cert_pipeline.insert(cert.product_id.clone(), cert);
    }

    pub fn is_hardware_certified(&self, product_id: &str) -> bool {
        self.cert_pipeline
            .get(product_id)
            .map(|c| c.hardware_compatible && c.certification_status == "Passed")
            .unwrap_or(false)
    }
}

impl Default for EcosystemManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_architecture_presence_matrix() {
        let mut manager = EcosystemManager::new();
        manager.register_architecture(
            "ARM64_Mobile_Sovereign".to_string(),
            EcosystemPlatform::Mobile,
            ArchTier::Tier1,
            true,
        );
        manager.register_architecture(
            "RISCV64_IoT_Embedded".to_string(),
            EcosystemPlatform::EmbeddedIoT,
            ArchTier::Tier2,
            false,
        );

        let arm64 = manager
            .architecture_matrix
            .get("ARM64_Mobile_Sovereign")
            .unwrap();
        assert_eq!(arm64.platform, EcosystemPlatform::Mobile);
        assert_eq!(arm64.tier, ArchTier::Tier1);
        assert!(arm64.is_bootable);

        let riscv = manager
            .architecture_matrix
            .get("RISCV64_IoT_Embedded")
            .unwrap();
        assert_eq!(riscv.tier, ArchTier::Tier2);
        assert!(!riscv.is_bootable);
    }

    #[test]
    fn test_enterprise_partnerships() {
        let mut manager = EcosystemManager::new();
        manager.register_partner(
            "IBM India".to_string(),
            "Mainframe Security Cloud Integration".to_string(),
            "Strategic".to_string(),
        );

        assert!(
            !manager
                .enterprise_partners
                .get("IBM India")
                .unwrap()
                .verified_and_integrated
        );
        assert!(manager.verify_partner_integration("IBM India"));
        assert!(
            manager
                .enterprise_partners
                .get("IBM India")
                .unwrap()
                .verified_and_integrated
        );
        assert!(!manager.verify_partner_integration("SAP Nonexistent"));
    }

    #[test]
    fn test_hardware_certifications() {
        let mut manager = EcosystemManager::new();
        let cert = EcosystemCertification {
            product_id: "HW-THINKPAD-T14".to_string(),
            product_name: "Lenovo ThinkPad T14 Gen 4".to_string(),
            hardware_compatible: true,
            certification_status: "Passed".to_string(),
            compliance_stamp: Some("STQC-INDIAN-GOVT-2025".to_string()),
        };

        manager.submit_certification(cert);
        assert!(manager.is_hardware_certified("HW-THINKPAD-T14"));
        assert!(!manager.is_hardware_certified("HW-DELL-NONEXISTENT"));
    }
}
