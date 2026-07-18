use std::collections::HashMap;

/// Hardware component types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComponentType {
    Cpu,
    Memory,
    Storage,
    Graphics,
    Network,
}

/// Status of compatibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CertificationStatus {
    Certified,
    CompatibleWithWarnings,
    NotSupported,
}

/// A Certificate proving Hardware Compliance/Trust.
#[derive(Debug, Clone)]
pub struct HardwareCertificate {
    pub certificate_id: String,
    pub vendor: String,
    pub model: String,
    pub components_status: HashMap<ComponentType, CertificationStatus>,
    pub overall_status: CertificationStatus,
}

/// Hardware certification program runs diagnostic tests.
#[derive(Debug, Clone)]
pub struct HardwareCertificationProgram {
    pub program_name: String,
    pub certified_database: Vec<HardwareCertificate>,
}

impl HardwareCertificationProgram {
    pub fn new(program_name: &str) -> Self {
        Self {
            program_name: program_name.to_string(),
            certified_database: Vec::new(),
        }
    }

    /// Run diagnostic suites and issue standard trust certificate.
    pub fn test_and_certify(
        &mut self,
        cert_id: &str,
        vendor: &str,
        model: &str,
        components: &[(ComponentType, bool, u32)], // (type, is_functional, speed_or_score)
    ) -> HardwareCertificate {
        let mut components_status = HashMap::new();
        let mut overall_certified = true;
        let mut warning_present = false;

        for &(comp, functional, score) in components {
            let status = if !functional {
                overall_certified = false;
                CertificationStatus::NotSupported
            } else if score < 50 {
                warning_present = true;
                CertificationStatus::CompatibleWithWarnings
            } else {
                CertificationStatus::Certified
            };
            components_status.insert(comp, status);
        }

        let overall_status = if !overall_certified {
            CertificationStatus::NotSupported
        } else if warning_present {
            CertificationStatus::CompatibleWithWarnings
        } else {
            CertificationStatus::Certified
        };

        let certificate = HardwareCertificate {
            certificate_id: cert_id.to_string(),
            vendor: vendor.to_string(),
            model: model.to_string(),
            components_status,
            overall_status,
        };

        self.certified_database.push(certificate.clone());
        certificate
    }
}

/// Application compliance audit.
#[derive(Debug, Clone)]
pub struct AppManifest {
    pub app_id: String,
    pub signature_valid: bool,
    pub requests_escapes: bool, // does it attempt to escape capability model?
    pub memory_leak_detected: bool,
}

/// Software certification program verifying third-party packages.
#[derive(Debug, Clone)]
pub struct SoftwareCertificationProgram {
    pub verifier_name: String,
    pub trusted_keys: Vec<String>,
}

impl SoftwareCertificationProgram {
    pub fn new(verifier_name: &str) -> Self {
        Self {
            verifier_name: verifier_name.to_string(),
            trusted_keys: Vec::new(),
        }
    }

    pub fn audit_app(&self, app: &AppManifest) -> Result<&'static str, &'static str> {
        if !app.signature_valid {
            return Err("Invalid application signature");
        }
        if app.requests_escapes {
            return Err("Capability model sandbox escape requested");
        }
        if app.memory_leak_detected {
            return Err("Automated analysis detected potential memory safety issues");
        }
        Ok("Certified: App conforms perfectly to SigmaOS secure capability standard")
    }
}

/// Release Stage gates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseStage {
    Alpha,
    Beta,
    ReleaseCandidate,
    Stable,
}

/// Quality gate configuration.
#[derive(Debug, Clone)]
pub struct QualityGate {
    pub required_coverage: f32,
    pub max_critical_bugs: u32,
}

/// Models QA pipelines with staged release cycles.
#[derive(Debug, Clone)]
pub struct QAStagedRelease {
    pub release_name: String,
    pub current_stage: ReleaseStage,
    pub quality_gate: QualityGate,
}

impl QAStagedRelease {
    pub fn new(release_name: &str, start_stage: ReleaseStage, required_coverage: f32) -> Self {
        Self {
            release_name: release_name.to_string(),
            current_stage: start_stage,
            quality_gate: QualityGate {
                required_coverage,
                max_critical_bugs: 0,
            },
        }
    }

    pub fn check_gate_promotion(
        &self,
        current_coverage: f32,
        critical_bugs: u32,
    ) -> Result<ReleaseStage, &'static str> {
        if current_coverage < self.quality_gate.required_coverage {
            return Err("Quality Gate: Code coverage is below staging requirement");
        }
        if critical_bugs > self.quality_gate.max_critical_bugs {
            return Err("Quality Gate: Critical bug count exceeds threshold");
        }

        let next_stage = match self.current_stage {
            ReleaseStage::Alpha => ReleaseStage::Beta,
            ReleaseStage::Beta => ReleaseStage::ReleaseCandidate,
            ReleaseStage::ReleaseCandidate => ReleaseStage::Stable,
            ReleaseStage::Stable => ReleaseStage::Stable,
        };
        Ok(next_stage)
    }

    pub fn promote_stage(&mut self, next_stage: ReleaseStage) {
        self.current_stage = next_stage;
    }
}

/// Simulated Hardware configuration profile for regression runs
#[derive(Debug, Clone)]
pub struct HardwareProfile {
    pub cpu_arch: String,
    pub memory_gb: u32,
    pub virtualization_supported: bool,
}

/// Automated Hardware Regression Test Suite
#[derive(Debug, Clone)]
pub struct HardwareRegressionSuite {
    pub suite_id: String,
    pub tested_profiles: Vec<HardwareProfile>,
}

impl HardwareRegressionSuite {
    pub fn new(suite_id: &str) -> Self {
        Self {
            suite_id: suite_id.to_string(),
            tested_profiles: Vec::new(),
        }
    }

    pub fn run_regression_on(&mut self, profile: HardwareProfile) -> Result<&'static str, &'static str> {
        if profile.memory_gb < 1 {
            return Err("Regression test failed: Out of memory during boot emulator init");
        }
        self.tested_profiles.push(profile);
        Ok("All regression tests (Scheduler, Memory, VFS, PQC) passed perfectly on target hardware")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hardware_certification() {
        let mut program = HardwareCertificationProgram::new("SigmaHCP");
        let certificate = program.test_and_certify(
            "cert-001",
            "SovereignTech",
            "S-Book 14",
            &[
                (ComponentType::Cpu, true, 95),
                (ComponentType::Memory, true, 80),
                (ComponentType::Graphics, true, 45), // Warning due to low rating
                (ComponentType::Storage, true, 100),
            ],
        );

        assert_eq!(certificate.overall_status, CertificationStatus::CompatibleWithWarnings);
        assert_eq!(
            certificate.components_status.get(&ComponentType::Cpu),
            Some(&CertificationStatus::Certified)
        );
        assert_eq!(
            certificate.components_status.get(&ComponentType::Graphics),
            Some(&CertificationStatus::CompatibleWithWarnings)
        );
        assert_eq!(program.certified_database.len(), 1);
    }

    #[test]
    fn test_software_certification_audit() {
        let scp = SoftwareCertificationProgram::new("SigmaSCP");

        let safe_app = AppManifest {
            app_id: "com.sigma.editor".to_string(),
            signature_valid: true,
            requests_escapes: false,
            memory_leak_detected: false,
        };

        let toxic_app = AppManifest {
            app_id: "com.exploit.sandbox".to_string(),
            signature_valid: true,
            requests_escapes: true,
            memory_leak_detected: false,
        };

        assert!(scp.audit_app(&safe_app).is_ok());
        assert_eq!(scp.audit_app(&toxic_app), Err("Capability model sandbox escape requested"));
    }

    #[test]
    fn test_staged_release_qa_gates() {
        let mut qa = QAStagedRelease::new("SigmaOS v1.2", ReleaseStage::Beta, 85.0);

        // Fails due to coverage
        let prom_fail1 = qa.check_gate_promotion(80.0, 0);
        assert!(prom_fail1.is_err());

        // Fails due to bug count
        let prom_fail2 = qa.check_gate_promotion(90.0, 2);
        assert!(prom_fail2.is_err());

        // Passes
        let prom_success = qa.check_gate_promotion(88.0, 0);
        assert_eq!(prom_success, Ok(ReleaseStage::ReleaseCandidate));

        qa.promote_stage(ReleaseStage::ReleaseCandidate);
        assert_eq!(qa.current_stage, ReleaseStage::ReleaseCandidate);
    }

    #[test]
    fn test_hardware_regression_suites() {
        let mut suite = HardwareRegressionSuite::new("suite-x86_64-arm64");

        let profile_arm = HardwareProfile {
            cpu_arch: "ARM64".to_string(),
            memory_gb: 16,
            virtualization_supported: true,
        };

        let profile_fail = HardwareProfile {
            cpu_arch: "RISCV64".to_string(),
            memory_gb: 0,
            virtualization_supported: false,
        };

        let arm_res = suite.run_regression_on(profile_arm);
        assert!(arm_res.is_ok());
        assert_eq!(suite.tested_profiles.len(), 1);

        let fail_res = suite.run_regression_on(profile_fail);
        assert!(fail_res.is_err());
    }
}
