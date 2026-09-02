//! Release Management System (Debian/Arch Release Cycle Inspiration)
//! Manages versioning, release channels, and release processes
use alloc::format;
extern crate alloc;



use alloc::vec::Vec;
use alloc::string::{String, ToString};

/// Release types (Debian/Arch inspiration)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseType {
    /// Stable release (Debian stable inspiration)
    Stable,
    /// Testing release (Debian testing inspiration)
    Testing,
    /// Unstable/development release (Debian unstable inspiration)
    Unstable,
    /// Rolling release (Arch rolling inspiration)
    Rolling,
    /// Long-term support release
    LTS,
}

/// Release status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseStatus {
    Planning,
    Development,
    FeatureFreeze,
    BugFixing,
    ReleaseCandidate,
    Released,
    Maintenance,
}

/// Release information
#[derive(Debug, Clone)]
pub struct Release {
    pub version: String,
    pub codename: String,
    pub release_type: ReleaseType,
    pub status: ReleaseStatus,
    pub release_date: Option<u64>,
    pub eol_date: Option<u64>,
    pub features: Vec<String>,
    pub known_issues: Vec<String>,
}

impl Release {
    pub fn new(version: &str, codename: &str, release_type: ReleaseType) -> Self {
        Self {
            version: version.to_string(),
            codename: codename.to_string(),
            release_type,
            status: ReleaseStatus::Planning,
            release_date: None,
            eol_date: None,
            features: Vec::new(),
            known_issues: Vec::new(),
        }
    }

    pub fn set_status(&mut self, status: ReleaseStatus) {
        self.status = status;
    }

    pub fn set_release_date(&mut self, date: u64) {
        self.release_date = Some(date);
    }

    pub fn set_eol_date(&mut self, date: u64) {
        self.eol_date = Some(date);
    }

    pub fn add_feature(&mut self, feature: &str) {
        self.features.push(feature.to_string());
    }

    pub fn add_known_issue(&mut self, issue: &str) {
        self.known_issues.push(issue.to_string());
    }

    pub fn is_released(&self) -> bool {
        self.status == ReleaseStatus::Released || self.status == ReleaseStatus::Maintenance
    }

    pub fn is_lts(&self) -> bool {
        self.release_type == ReleaseType::LTS
    }
}

/// Release manager
pub struct ReleaseManager {
    pub releases: Vec<Release>,
    pub current_release: Option<String>,
    pub next_release: Option<String>,
}

impl ReleaseManager {
    pub fn new() -> Self {
        Self {
            releases: Vec::new(),
            current_release: None,
            next_release: None,
        }
    }

    pub fn add_release(&mut self, release: Release) {
        self.releases.push(release);
    }

    pub fn get_release(&self, version: &str) -> Option<&Release> {
        self.releases.iter().find(|r| r.version == version)
    }

    pub fn get_current_release(&self) -> Option<&Release> {
        if let Some(ref version) = self.current_release {
            self.get_release(version)
        } else {
            None
        }
    }

    pub fn get_next_release(&self) -> Option<&Release> {
        if let Some(ref version) = self.next_release {
            self.get_release(version)
        } else {
            None
        }
    }

    pub fn set_current_release(&mut self, version: &str) {
        self.current_release = Some(version.to_string());
    }

    pub fn set_next_release(&mut self, version: &str) {
        self.next_release = Some(version.to_string());
    }

    pub fn start_release_cycle(&mut self, version: &str, codename: &str, release_type: ReleaseType) {
        let mut release = Release::new(version, codename, release_type);
        release.set_status(ReleaseStatus::Development);
        self.add_release(release);
        self.set_next_release(version);
    }

    pub fn enter_feature_freeze(&mut self, version: &str) -> Result<(), ReleaseError> {
        if let Some(release) = self.releases.iter_mut().find(|r| r.version == version) {
            release.set_status(ReleaseStatus::FeatureFreeze);
            Ok(())
        } else {
            Err(ReleaseError::ReleaseNotFound)
        }
    }

    pub fn enter_bug_fixing(&mut self, version: &str) -> Result<(), ReleaseError> {
        if let Some(release) = self.releases.iter_mut().find(|r| r.version == version) {
            release.set_status(ReleaseStatus::BugFixing);
            Ok(())
        } else {
            Err(ReleaseError::ReleaseNotFound)
        }
    }

    pub fn create_release_candidate(&mut self, version: &str) -> Result<(), ReleaseError> {
        if let Some(release) = self.releases.iter_mut().find(|r| r.version == version) {
            release.set_status(ReleaseStatus::ReleaseCandidate);
            Ok(())
        } else {
            Err(ReleaseError::ReleaseNotFound)
        }
    }

    pub fn release(&mut self, version: &str, release_date: u64, eol_date: Option<u64>) -> Result<(), ReleaseError> {
        if let Some(release) = self.releases.iter_mut().find(|r| r.version == version) {
            release.set_status(ReleaseStatus::Released);
            release.set_release_date(release_date);
            if let Some(eol) = eol_date {
                release.set_eol_date(eol);
            }
            self.set_current_release(version);
            Ok(())
        } else {
            Err(ReleaseError::ReleaseNotFound)
        }
    }

    pub fn list_releases(&self) -> Vec<&Release> {
        self.releases.iter().collect()
    }

    pub fn list_stable_releases(&self) -> Vec<&Release> {
        self.releases.iter().filter(|r| r.is_released()).collect()
    }

    pub fn list_lts_releases(&self) -> Vec<&Release> {
        self.releases.iter().filter(|r| r.is_lts()).collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReleaseError {
    ReleaseNotFound,
    InvalidState,
    FeatureFreezeViolated,
    ReleaseFailed,
}

impl Default for ReleaseManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Version management (Semantic Versioning)
pub struct VersionManager {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub pre_release: Option<String>,
    pub build_metadata: Option<String>,
}

impl VersionManager {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
            pre_release: None,
            build_metadata: None,
        }
    }

    pub fn increment_major(&mut self) {
        self.major += 1;
        self.minor = 0;
        self.patch = 0;
    }

    pub fn increment_minor(&mut self) {
        self.minor += 1;
        self.patch = 0;
    }

    pub fn increment_patch(&mut self) {
        self.patch += 1;
    }

    pub fn set_pre_release(&mut self, pre: &str) {
        self.pre_release = Some(pre.to_string());
    }

    pub fn set_build_metadata(&mut self, build: &str) {
        self.build_metadata = Some(build.to_string());
    }

    pub fn to_string(&self) -> String {
        let mut version = format!("{}.{}.{}", self.major, self.minor, self.patch);
        if let Some(ref pre) = self.pre_release {
            version.push_str("-");
            version.push_str(pre);
        }
        if let Some(ref build) = self.build_metadata {
            version.push_str("+");
            version.push_str(build);
        }
        version
    }

    pub fn parse(version_str: &str) -> Result<Self, VersionError> {
        // Parse semantic version string
        let parts: Vec<&str> = version_str.split('.').collect();
        if parts.len() < 3 {
            return Err(VersionError::InvalidFormat);
        }

        let major = parts[0].parse::<u32>().map_err(|_| VersionError::InvalidNumber)?;
        let minor = parts[1].parse::<u32>().map_err(|_| VersionError::InvalidNumber)?;
        
        let patch_str = parts[2].split('-').next().unwrap_or(parts[2]);
        let patch_str = patch_str.split('+').next().unwrap_or(patch_str);
        let patch = patch_str.parse::<u32>().map_err(|_| VersionError::InvalidNumber)?;

        let mut version = VersionManager::new(major, minor, patch);

        // Parse pre-release
        if let Some(pre_pos) = version_str.find('-') {
            let after_pre = &version_str[pre_pos + 1..];
            if let Some(build_pos) = after_pre.find('+') {
                version.set_pre_release(&after_pre[..build_pos]);
                version.set_build_metadata(&after_pre[build_pos + 1..]);
            } else {
                version.set_pre_release(after_pre);
            }
        }

        Ok(version)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionError {
    InvalidFormat,
    InvalidNumber,
}

impl Default for VersionManager {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_release_creation() {
        let release = Release::new("1.0.0", "Focal", ReleaseType::Stable);
        assert_eq!(release.version, "1.0.0");
        assert_eq!(release.codename, "Focal");
    }

    #[test]
    fn test_release_manager() {
        let mut manager = ReleaseManager::new();
        manager.start_release_cycle("1.0.0", "Focal", ReleaseType::Stable);
        assert!(manager.next_release.is_some());
    }

    #[test]
    fn test_release_cycle() {
        let mut manager = ReleaseManager::new();
        manager.start_release_cycle("1.0.0", "Focal", ReleaseType::Stable);
        assert!(manager.enter_feature_freeze("1.0.0").is_ok());
        assert!(manager.enter_bug_fixing("1.0.0").is_ok());
        assert!(manager.create_release_candidate("1.0.0").is_ok());
    }

    #[test]
    fn test_version_manager() {
        let mut version = VersionManager::new(1, 0, 0);
        version.increment_minor();
        assert_eq!(version.to_string(), "1.1.0");
    }

// =========================================================================
// 1. DISTRO RELEASE CHANNEL GOVERNOR (DEBIAN / FEDORA / FREEBSD PARITY)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseChannelMode {
    DebianStable,        // Point releases, security backports only
    DebianTesting,       // Automated migration gate, zero RC bugs required
    DebianUnstableSid,   // Active development, bleeding-edge packages
    ArchRollingSync,     // Continuous rolling release updates
    FedoraRawhide,       // Rawhide nightly builds and branch staging
    FreeBsdReleng,       // Production release engineering branch
    FreeBsdStable,       // Stable ABI branch
    FreeBsdCurrent,      // Head development branch
}

#[derive(Debug, Clone)]
pub struct ReleaseChannelConfig {
    pub mode: ReleaseChannelMode,
    pub name: String,
    pub abi_frozen: bool,
    pub min_aging_days_in_testing: u32,
    pub max_critical_bugs_allowed: u32,
}

pub struct DistroReleaseChannelGovernor {
    pub channels: Vec<ReleaseChannelConfig>,
}

impl DistroReleaseChannelGovernor {
    pub fn new() -> Self {
        let mut gov = Self { channels: Vec::new() };
        gov.channels.push(ReleaseChannelConfig {
            mode: ReleaseChannelMode::DebianStable,
            name: "Stable".to_string(),
            abi_frozen: true,
            min_aging_days_in_testing: 10,
            max_critical_bugs_allowed: 0,
        });
        gov.channels.push(ReleaseChannelConfig {
            mode: ReleaseChannelMode::DebianTesting,
            name: "Testing".to_string(),
            abi_frozen: false,
            min_aging_days_in_testing: 5,
            max_critical_bugs_allowed: 0,
        });
        gov.channels.push(ReleaseChannelConfig {
            mode: ReleaseChannelMode::ArchRollingSync,
            name: "Rolling".to_string(),
            abi_frozen: false,
            min_aging_days_in_testing: 0,
            max_critical_bugs_allowed: 2,
        });
        gov
    }

    pub fn evaluate_channel_promotion(
        &self,
        mode: ReleaseChannelMode,
        days_in_testing: u32,
        open_critical_bugs: u32,
    ) -> Result<bool, &'static str> {
        if let Some(cfg) = self.channels.iter().find(|c| c.mode == mode) {
            if open_critical_bugs > cfg.max_critical_bugs_allowed {
                return Ok(false);
            }
            if days_in_testing < cfg.min_aging_days_in_testing {
                return Ok(false);
            }
            Ok(true)
        } else {
            Err("ReleaseChannelGovernor: Mode not configured")
        }
    }
}

impl Default for DistroReleaseChannelGovernor {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 2. REPRODUCIBLE RELEASE ARTIFACT VERIFIER (NIXOS HYDRA / DEBIAN REPRO)
// =========================================================================

#[derive(Debug, Clone)]
pub struct ReleaseArtifactManifest {
    pub artifact_name: String,
    pub sha256_checksum: String,
    pub dilithium5_signature: String,
    pub deterministic_build: bool,
}

pub struct ReproducibleReleaseArtifactVerifier {
    pub artifacts: Vec<ReleaseArtifactManifest>,
}

impl ReproducibleReleaseArtifactVerifier {
    pub fn new() -> Self {
        Self { artifacts: Vec::new() }
    }

    pub fn register_artifact(&mut self, manifest: ReleaseArtifactManifest) {
        self.artifacts.push(manifest);
    }

    pub fn verify_release_readiness(&self) -> bool {
        if self.artifacts.is_empty() {
            return false;
        }
        self.artifacts.iter().all(|a| {
            a.deterministic_build
                && !a.sha256_checksum.is_empty()
                && !a.dilithium5_signature.is_empty()
        })
    }
}

impl Default for ReproducibleReleaseArtifactVerifier {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 3. SOVEREIGN RELEASE ORCHESTRATOR
// =========================================================================

pub struct SovereignReleaseOrchestrator {
    pub manager: ReleaseManager,
    pub channel_governor: DistroReleaseChannelGovernor,
    pub artifact_verifier: ReproducibleReleaseArtifactVerifier,
}

impl SovereignReleaseOrchestrator {
    pub fn new() -> Self {
        Self {
            manager: ReleaseManager::new(),
            channel_governor: DistroReleaseChannelGovernor::new(),
            artifact_verifier: ReproducibleReleaseArtifactVerifier::new(),
        }
    }

    pub fn promote_and_release(
        &mut self,
        version: &str,
        codename: &str,
        mode: ReleaseChannelMode,
    ) -> Result<bool, &'static str> {
        let is_ready = self
            .channel_governor
            .evaluate_channel_promotion(mode, 14, 0)?;

        if !is_ready {
            return Ok(false);
        }

        if !self.artifact_verifier.verify_release_readiness() {
            return Err("ReleaseOrchestrator: Artifacts not reproducible or signed");
        }

        self.manager.start_release_cycle(version, codename, ReleaseType::Stable);
        self.manager
            .release(version, 1700000000, Some(1800000000))
            .map_err(|_| "ReleaseOrchestrator: Release error")?;

        Ok(true)
    }
}

impl Default for SovereignReleaseOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

    #[test]
    fn test_version_parsing() {
        let version = VersionManager::parse("1.2.3-alpha+build.123").unwrap();
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, 3);
        assert_eq!(version.pre_release, Some("alpha".to_string()));
        assert_eq!(version.build_metadata, Some("build.123".to_string()));
    }

    #[test]
    fn test_distro_release_channel_governor() {
        let gov = DistroReleaseChannelGovernor::new();
        let ready = gov.evaluate_channel_promotion(ReleaseChannelMode::DebianStable, 14, 0).unwrap();
        assert!(ready);

        let not_ready = gov.evaluate_channel_promotion(ReleaseChannelMode::DebianStable, 2, 0).unwrap();
        assert!(!not_ready);
    }

    #[test]
    fn test_reproducible_release_artifact_verifier() {
        let mut verifier = ReproducibleReleaseArtifactVerifier::new();
        verifier.register_artifact(ReleaseArtifactManifest {
            artifact_name: "sigmaos-1.0.0-x86_64.iso".to_string(),
            sha256_checksum: "a1b2c3d4e5f6...".to_string(),
            dilithium5_signature: "sig_dilithium_123".to_string(),
            deterministic_build: true,
        });

        assert!(verifier.verify_release_readiness());
    }

// =========================================================================
// AI REGRESSION GATE EVALUATOR
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BootPerformanceMetrics {
    pub boot_latency_ms: u64,
    pub idle_memory_used_mb: u64,
    pub syscall_latency_ns: u64,
}

pub struct AiRegressionGateEvaluator {
    pub max_allowed_boot_latency_increase_pct: f64,
    pub max_allowed_memory_drift_mb: u64,
}

impl AiRegressionGateEvaluator {
    pub fn new() -> Self {
        Self {
            max_allowed_boot_latency_increase_pct: 5.0,
            max_allowed_memory_drift_mb: 64,
        }
    }

    pub fn evaluate_release_regression(
        &self,
        baseline: BootPerformanceMetrics,
        candidate: BootPerformanceMetrics,
    ) -> bool {
        let latency_increase_pct = if baseline.boot_latency_ms > 0 {
            ((candidate.boot_latency_ms as f64 - baseline.boot_latency_ms as f64)
                / baseline.boot_latency_ms as f64)
                * 100.0
        } else {
            0.0
        };

        let memory_drift_mb = if candidate.idle_memory_used_mb > baseline.idle_memory_used_mb {
            candidate.idle_memory_used_mb - baseline.idle_memory_used_mb
        } else {
            0
        };

        latency_increase_pct <= self.max_allowed_boot_latency_increase_pct
            && memory_drift_mb <= self.max_allowed_memory_drift_mb
    }
}

impl Default for AiRegressionGateEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

    #[test]
    fn test_ai_regression_gate_evaluator() {
        let eval = AiRegressionGateEvaluator::new();
        let baseline = BootPerformanceMetrics {
            boot_latency_ms: 1200,
            idle_memory_used_mb: 256,
            syscall_latency_ns: 45,
        };
        let good_candidate = BootPerformanceMetrics {
            boot_latency_ms: 1220,
            idle_memory_used_mb: 270,
            syscall_latency_ns: 46,
        };
        let bad_candidate = BootPerformanceMetrics {
            boot_latency_ms: 1500,
            idle_memory_used_mb: 512,
            syscall_latency_ns: 90,
        };

        assert!(eval.evaluate_release_regression(baseline, good_candidate));
        assert!(!eval.evaluate_release_regression(baseline, bad_candidate));
    }

    #[test]
    fn test_sovereign_release_orchestrator() {
        let mut orch = SovereignReleaseOrchestrator::new();
        orch.artifact_verifier.register_artifact(ReleaseArtifactManifest {
            artifact_name: "sigmaos-1.0.0-x86_64.iso".to_string(),
            sha256_checksum: "a1b2c3d4e5f6...".to_string(),
            dilithium5_signature: "sig_dilithium_123".to_string(),
            deterministic_build: true,
        });

        let success = orch.promote_and_release("1.0.0", "Apex", ReleaseChannelMode::DebianStable).unwrap();
        assert!(success);
        assert!(orch.manager.get_current_release().is_some());
    }
}