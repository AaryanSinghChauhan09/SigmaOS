// SigmaOS Support & Services Framework (Red Hat Insights, Ubuntu Pro, and SUSE Manager Parity)
// Professional support tiers, LTS maintenance guarantees, Expanded Security Maintenance (ESM),
// FIPS/CIS Compliance Scanners, Automated Remediation Playbooks, and System Drift Detectors.


#[cfg(test_disabled)]
extern crate std;


use std::string::String;
use std::vec::Vec;
use std::string::ToString;
use core::sync::atomic::{AtomicBool, Ordering};

use crate::klib::HashMap;

/// Professional support levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportTier {
    Basic,
    Developer,
    Business,
    Enterprise,
}

/// Professional SLA / Support contract
#[derive(Debug, Clone)]
pub struct SupportContract {
    pub client_name: String,
    pub tier: SupportTier,
    pub sla_resolution_hours: u32,
    pub active_tickets: u32,
}

/// Long-Term Maintenance (LTS) release lifecycle
#[derive(Debug, Clone)]
pub struct LtsRelease {
    pub version: String,
    pub release_codename: String,
    pub release_date: String,
    pub supported_until: String,
    pub kernel_version: String,
    pub is_esm_eligible: bool,
}

/// Disaster recovery tool mapping (such as Rescue ISO configuration)
#[derive(Debug, Clone)]
pub struct RecoveryConfig {
    pub rescue_iso_name: String,
    pub diagnostic_tools_included: Vec<String>,
    pub automount_system_drives: bool,
}

/// Ubuntu Pro-style Expanded Security Maintenance (ESM) Subscription
#[derive(Debug, Clone)]
pub struct EsmSubscription {
    pub token: String,
    pub active: bool,
    pub enabled_repositories: Vec<String>, // e.g. "fips", "livepatch", "cis-audit"
}

/// Compliance profile types (RHEL / Ubuntu Pro parity)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplianceStandard {
    CisBenchmark,
    Hipaa,
    PciDss,
    Fips140_3,
}

/// FIPS / CIS Compliance Auditor and Benchmark Scanner
pub struct ComplianceScanner {
    pub standard: ComplianceStandard,
    pub enforce_fips_cryptography: bool,
}

impl ComplianceScanner {
    pub fn new(standard: ComplianceStandard) -> Self {
        Self {
            standard,
            enforce_fips_cryptography: standard == ComplianceStandard::Fips140_3,
        }
    }

    /// Performs audit scanning on security profiles, returning compliance status and score
    pub fn execute_audit(&self, active_pledges: &[String]) -> (bool, u32) {
        let mut score = 100;
        // Compliance Rules
        if self.enforce_fips_cryptography && !active_pledges.contains(&"fips-crypto".to_string()) {
            score -= 40; // Cryptographic module non-compliance penalty
        }
        if self.standard == ComplianceStandard::CisBenchmark && active_pledges.contains(&"unveiled-root".to_string()) {
            score -= 30; // Unrestricted filesystem unveil violation
        }
        (score >= 80, score)
    }
}

/// Red Hat Insights-style Vulnerability Remediation Playbook
#[derive(Debug, Clone)]
pub struct RemediationPlaybook {
    pub play_id: String,
    pub description: String,
    pub target_cve: String,
    pub remediation_steps: Vec<String>,
}

/// SUSE Manager-style Configuration Drift Detector
pub struct DriftDetector {
    pub baseline_hashes: HashMap<String, String>, // filepath -> md5/sha256 baseline
}

impl DriftDetector {
    pub fn new() -> Self {
        Self {
            baseline_hashes: HashMap::new(),
        }
    }

    pub fn register_baseline(&mut self, filepath: &str, hash: &str) {
        self.baseline_hashes.insert(filepath.to_string(), hash.to_string());
    }

    /// Scans modified files, returning list of paths that drifted from configurations
    pub fn detect_drift(&self, current_hashes: &HashMap<String, String>) -> Vec<String> {
        let mut drifted = Vec::new();
        for (path, base_hash) in &self.baseline_hashes {
            if let Some(curr_hash) = current_hashes.get(path) {
                if curr_hash != base_hash {
                    drifted.push(path.clone());
                }
            } else {
                drifted.push(path.clone()); // File missing from live system is considered drift
            }
        }
        drifted
    }
}

impl Default for DriftDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Support & Services Manager
pub struct SupportServicesManager {
    pub active_contracts: HashMap<String, SupportContract>,
    pub lts_releases: HashMap<String, LtsRelease>,
    pub recovery_tools: Vec<RecoveryConfig>,

    // Enterprise Competitor Services
    pub esm_sub: Option<EsmSubscription>,
    pub registered_playbooks: HashMap<String, RemediationPlaybook>,
    pub drift_detector: DriftDetector,
}

impl SupportServicesManager {
    pub fn new() -> Self {
        Self {
            active_contracts: HashMap::new(),
            lts_releases: HashMap::new(),
            recovery_tools: Vec::new(),
            esm_sub: None,
            registered_playbooks: HashMap::new(),
            drift_detector: DriftDetector::new(),
        }
    }

    pub fn register_contract(&mut self, client: String, tier: SupportTier, sla: u32) {
        let contract = SupportContract {
            client_name: client.clone(),
            tier,
            sla_resolution_hours: sla,
            active_tickets: 0,
        };
        self.active_contracts.insert(client, contract);
    }

    pub fn open_support_ticket(&mut self, client: &str) -> bool {
        if let Some(contract) = self.active_contracts.get_mut(client) {
            contract.active_tickets += 1;
            true
        } else {
            false
        }
    }

    pub fn register_lts_release(
        &mut self,
        version: String,
        codename: String,
        release_date: String,
        supported_until: String,
        kernel: String,
        is_esm: bool,
    ) {
        let release = LtsRelease {
            version: version.clone(),
            release_codename: codename,
            release_date,
            supported_until,
            kernel_version: kernel,
            is_esm_eligible: is_esm,
        };
        self.lts_releases.insert(version, release);
    }

    /// Retrieve Vivid LTS release manifest metadata
    pub fn vivid_lts_manifest(&self) -> Option<&LtsRelease> {
        self.lts_releases
            .values()
            .find(|r| r.release_codename.eq_ignore_ascii_case("vivid") || r.version.contains("Vivid"))
    }

    pub fn add_recovery_tool(&mut self, config: RecoveryConfig) {
        self.recovery_tools.push(config);
    }

    pub fn get_sla_limit(&self, client: &str) -> Option<u32> {
        self.active_contracts
            .get(client)
            .map(|c| c.sla_resolution_hours)
    }

    /// Register/activate Ubuntu Pro Expanded Security Maintenance
    pub fn activate_esm_subscription(&mut self, token: &str, repos: Vec<String>) {
        self.esm_sub = Some(EsmSubscription {
            token: token.to_string(),
            active: true,
            enabled_repositories: repos,
        });
    }

    /// Register proactive CVE playbooks (Red Hat Insights parity)
    pub fn register_remediation_playbook(&mut self, play: RemediationPlaybook) {
        self.registered_playbooks.insert(play.play_id.clone(), play);
    }
}

impl Default for SupportServicesManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_support_contracts_and_tickets() {
        let mut manager = SupportServicesManager::new();
        manager.register_contract("SovereignCloudCorp".to_string(), SupportTier::Enterprise, 4);

        assert_eq!(manager.get_sla_limit("SovereignCloudCorp"), Some(4));
        assert_eq!(manager.get_sla_limit("Nonexistent"), None);

        assert!(manager.open_support_ticket("SovereignCloudCorp"));
        assert_eq!(
            manager
                .active_contracts
                .get("SovereignCloudCorp")
                .unwrap()
                .active_tickets,
            1
        );
    }

    #[test]
    fn test_lts_release_management() {
        let mut manager = SupportServicesManager::new();
        manager.register_lts_release(
            "v1.0-LTS".to_string(),
            "Vivid".to_string(),
            "2025-01-15".to_string(),
            "2030-01-15".to_string(),
            "sigma-6.1-hardened".to_string(),
            true,
        );

        assert_eq!(manager.lts_releases.len(), 1);
        let release = manager.lts_releases.get("v1.0-LTS").unwrap();
        assert_eq!(release.kernel_version, "sigma-6.1-hardened");
        assert_eq!(release.release_codename, "Vivid");
        assert!(release.is_esm_eligible);

        let vivid = manager.vivid_lts_manifest().unwrap();
        assert_eq!(vivid.version, "v1.0-LTS");
    }

    #[test]
    fn test_recovery_tools() {
        let mut manager = SupportServicesManager::new();
        let config = RecoveryConfig {
            rescue_iso_name: "SigmaOS-Rescue-v1.0.iso".to_string(),
            diagnostic_tools_included: {
                let mut v = Vec::new();
                v.push("fsck.sigmafs".to_string());
                v.push("memtester".to_string());
                v
            },
            automount_system_drives: true,
        };

        manager.add_recovery_tool(config);
        assert_eq!(manager.recovery_tools.len(), 1);
        assert_eq!(
            manager.recovery_tools[0].rescue_iso_name,
            "SigmaOS-Rescue-v1.0.iso"
        );
        assert!(manager.recovery_tools[0].automount_system_drives);
    }

    #[test]
    fn test_esm_and_playbooks_proactive_scans() {
        let mut manager = SupportServicesManager::new();

        // 1. Expanded Security Maintenance Subscription
        let mut repos = Vec::new();
        repos.push("fips-crypto".to_string());
        repos.push("livepatch".to_string());
        manager.activate_esm_subscription("UbuntuPro-token-9999", repos);

        assert!(manager.esm_sub.is_some());
        let sub = manager.esm_sub.as_ref().unwrap();
        assert!(sub.active);
        assert_eq!(sub.enabled_repositories[0], "fips-crypto");

        // 2. FIPS Compliance Audit Scanning
        let scanner = ComplianceScanner::new(ComplianceStandard::Fips140_3);

        // Audit fails without FIPS cryptography pledges enabled
        let (passed_1, score_1) = scanner.execute_audit(&[]);
        assert!(!passed_1);
        assert_eq!(score_1, 60);

        // Audit succeeds once FIPS cryptography active pledges are present
        let (passed_2, score_2) = scanner.execute_audit(&["fips-crypto".to_string()]);
        assert!(passed_2);
        assert_eq!(score_2, 100);

        // 3. Proactive Insights Playbooks
        let mut play_steps = Vec::new();
        play_steps.push("sysctl -w net.ipv4.ip_forward=0".to_string());
        let playbook = RemediationPlaybook {
            play_id: "insights-play-01".to_string(),
            description: "Remediate IP forward vulnerability".to_string(),
            target_cve: "CVE-2025-0012".to_string(),
            remediation_steps: play_steps,
        };
        manager.register_remediation_playbook(playbook);
        assert!(manager.registered_playbooks.contains_key("insights-play-01"));

        // 4. Configuration Drift Detection
        let mut live_hashes = HashMap::new();
        live_hashes.insert("/etc/sysctl.conf".to_string(), "hash-001".to_string());
        live_hashes.insert("/etc/hosts".to_string(), "hash-002".to_string());

        manager.drift_detector.register_baseline("/etc/sysctl.conf", "hash-001");
        manager.drift_detector.register_baseline("/etc/hosts", "hash-002");

        // No drift detected under matching baseline hashes
        let drifted_files = manager.drift_detector.detect_drift(&live_hashes);
        assert_eq!(drifted_files.len(), 0);

        // Modify a configuration file hash -> triggers drift detection
        live_hashes.insert("/etc/sysctl.conf".to_string(), "modified-hash".to_string());
        let drifted_files_after = manager.drift_detector.detect_drift(&live_hashes);
        assert_eq!(drifted_files_after.len(), 1);
        assert_eq!(drifted_files_after[0], "/etc/sysctl.conf");
    }
}
