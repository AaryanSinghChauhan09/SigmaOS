//! Release Management System (Debian/Arch Release Cycle Inspiration)
//! Manages versioning, release channels, and release processes

#![no_std]

extern crate alloc;

use crate::klib::{Vec};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::string::String;

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

    #[test]
    fn test_version_parsing() {
        let version = VersionManager::parse("1.2.3-alpha+build.123").unwrap();
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, 3);
        assert_eq!(version.pre_release, Some("alpha".to_string()));
        assert_eq!(version.build_metadata, Some("build.123".to_string()));
    }
}