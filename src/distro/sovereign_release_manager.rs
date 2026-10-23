// SPDX-License-Identifier: MIT OR GPL-2.0
//! SigmaOS Sovereign Release Manager
//! (`src/distro/sovereign_release_manager.rs`)
//!
//! Manages the SigmaOS distribution release lifecycle, inspired by:
//! - Linux Mint release management (LTS-first, Ubuntu base)
//! - Ubuntu release cadence (LTS every 2 years, interim every 6 months)
//! - Arch Linux rolling release (no version, always current)
//! - Omarchy (opinionated, curated rolling release)
//! - Debian stable/testing/unstable channels
//! - FreeBSD CURRENT/STABLE/RELEASE model
//! - NixOS stable + unstable channels
//!
//! Principles:
//! ARCHITECTURE · BOY SCOUT RULE · CLEAR ABSTRACTION · COMMUNITY-ORIENTED ·
//! CONTINUOUS IMPROVEMENT · DISCOVERABILITY · DON'T REPEAT YOURSELF ·
//! GOVERNANCE AND DO-CRACY · IF IT AIN'T BROKE DON'T FIX IT · KEEP IT SIMPLE ·
//! MERITOCRACY · MODULARITY · OPEN STANDARD · PORTABILITY · PRISTINE SOURCES ·
//! PRACTICAL LICENSING AWARENESS · SEPARATION OF CONCERNS · SINGLE RESPONSIBILITY ·
//! SOFTWARE SUPPORT PROGRAM · SOFTWARE UPDATING PROGRAM · STABLE APIs ·
//! TRANSPARENCY & OPEN EXCHANGE · USER FREEDOM AND CONTROL · VERSION CONTROL PRACTICES

#![allow(dead_code)]

use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

// ============================================================
// Release Series — the top-level release concept
// ============================================================

/// SigmaOS release series designation
/// Follows Ubuntu-style versioning: YY.MM for dated releases,
/// plus rolling track (Arch/Omarchy style)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReleaseSeries {
    /// Long-Term Support: 5 years full + 5 years extended (Ubuntu LTS style)
    Lts {
        year: u16,
        month: u8,
        codename: String,
    },
    /// Standard release: 9 months support (Ubuntu interim style)
    Standard {
        year: u16,
        month: u8,
        codename: String,
    },
    /// Rolling release: always latest (Arch/Omarchy style)
    Rolling { codename: String },
    /// Development: pre-release (Debian testing style)
    Development { codename: String },
}

impl ReleaseSeries {
    pub fn version_string(&self) -> String {
        match self {
            ReleaseSeries::Lts { year, month, .. } => format!("{}.{:02} LTS", year, month),
            ReleaseSeries::Standard { year, month, .. } => format!("{}.{:02}", year, month),
            ReleaseSeries::Rolling { codename } => format!("Rolling ({})", codename),
            ReleaseSeries::Development { codename } => format!("Dev ({})", codename),
        }
    }

    pub fn codename(&self) -> &str {
        match self {
            ReleaseSeries::Lts { codename, .. } => codename,
            ReleaseSeries::Standard { codename, .. } => codename,
            ReleaseSeries::Rolling { codename } => codename,
            ReleaseSeries::Development { codename } => codename,
        }
    }

    pub fn is_lts(&self) -> bool {
        matches!(self, ReleaseSeries::Lts { .. })
    }

    pub fn is_rolling(&self) -> bool {
        matches!(self, ReleaseSeries::Rolling { .. })
    }

    /// Support duration in months (None = indefinite for rolling)
    pub fn support_months(&self) -> Option<u32> {
        match self {
            ReleaseSeries::Lts { .. } => Some(60), // 5 years
            ReleaseSeries::Standard { .. } => Some(9),
            ReleaseSeries::Rolling { .. } => None,
            ReleaseSeries::Development { .. } => None,
        }
    }

    /// Extended Security Maintenance availability (Ubuntu ESM style)
    pub fn has_esm(&self) -> bool {
        matches!(self, ReleaseSeries::Lts { .. })
    }
}

/// A specific point release (e.g. 26.04.1 LTS — first point release of 26.04 LTS)
#[derive(Debug, Clone)]
pub struct PointRelease {
    pub series: ReleaseSeries,
    pub point: u8,          // 0 = initial, 1 = first point release, etc.
    pub build_date: String, // ISO 8601 date
    pub iso_sha256: String, // SHA256 of the ISO image (PRISTINE SOURCES)
    pub iso_url: String,
    pub delta_url: Option<String>, // Optional delta from previous point release
    pub kernel_version: String,    // Bundled kernel version
    pub base_packages: Vec<String>, // Key packages and versions
    pub release_notes_url: String,
    pub is_current: bool,
    pub min_ram_mb: u32,
    pub min_storage_gb: u32,
}

impl PointRelease {
    pub fn full_version(&self) -> String {
        let base = self.series.version_string();
        if self.point == 0 {
            base
        } else {
            format!("{}.{}", base.replace(" LTS", ""), self.point)
                + if self.series.is_lts() { " LTS" } else { "" }
        }
    }
}

// ============================================================
// Upgrade Path — how users move between releases
// ============================================================

/// Upgrade transition descriptor (USER FREEDOM AND CONTROL)
#[derive(Debug, Clone)]
pub struct UpgradePath {
    pub from: String,    // Source release codename
    pub to: String,      // Target release codename
    pub supported: bool, // Whether this transition is officially supported
    pub method: UpgradeMethod,
    pub estimated_duration_mins: u32,
    pub risk_level: UpgradeRisk,
    pub pre_checks: Vec<String>, // Required preconditions
    pub post_steps: Vec<String>, // Steps to take after upgrade
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpgradeMethod {
    /// In-place package upgrade (apt dist-upgrade / pacman -Syu style)
    InPlace,
    /// Fresh reinstall preserving /home (Linux Mint recommended approach)
    FreshInstallPreserveHome,
    /// Full reinstall (blank slate)
    FreshInstall,
    /// Declarative config re-apply (NixOS / Omarchy style)
    DeclarativeReapply,
    /// Delta ISO upgrade (freebsd-update style binary patches)
    DeltaIso,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpgradeRisk {
    Low,      // Routine point release
    Medium,   // Minor version bump
    High,     // Major version, kernel change
    Critical, // Crosses EOL, major architecture change
}

// ============================================================
// EOL Tracker — End of Life management (SOFTWARE SUPPORT PROGRAM)
// ============================================================

/// Tracks when releases reach end-of-life and what to do
#[derive(Debug, Clone)]
pub struct EolTracker {
    /// Map from codename → EOL date (ISO 8601 string)
    eol_dates: BTreeMap<String, String>,
    /// Map from codename → recommended upgrade target
    upgrade_targets: BTreeMap<String, String>,
}

impl EolTracker {
    pub fn new() -> Self {
        let mut tracker = Self {
            eol_dates: BTreeMap::new(),
            upgrade_targets: BTreeMap::new(),
        };
        // Initialize with known SigmaOS release EOL dates
        tracker.register("apex", "2027-04-30", Some("vanguard"));
        tracker.register("vanguard", "2029-04-30", None);
        tracker.register("sovereign", "2031-04-30", None);
        tracker
    }

    pub fn register(&mut self, codename: &str, eol_date: &str, upgrade_to: Option<&str>) {
        self.eol_dates
            .insert(codename.to_string(), eol_date.to_string());
        if let Some(target) = upgrade_to {
            self.upgrade_targets
                .insert(codename.to_string(), target.to_string());
        }
    }

    pub fn is_eol(&self, codename: &str, current_date: &str) -> bool {
        if let Some(eol) = self.eol_dates.get(codename) {
            current_date >= eol.as_str()
        } else {
            false // Unknown release — assume not EOL (IF IT AIN'T BROKE DON'T FIX IT)
        }
    }

    pub fn recommended_upgrade(&self, codename: &str) -> Option<&str> {
        self.upgrade_targets.get(codename).map(|s| s.as_str())
    }

    pub fn eol_date(&self, codename: &str) -> Option<&str> {
        self.eol_dates.get(codename).map(|s| s.as_str())
    }
}

// ============================================================
// Release Announcement Generator (COMMUNITY-ORIENTED + TRANSPARENCY)
// ============================================================

/// Generates structured release notes / announcements
pub struct ReleaseAnnouncement {
    pub release: PointRelease,
    pub highlights: Vec<ReleaseHighlight>,
    pub breaking_changes: Vec<String>,
    pub known_issues: Vec<KnownIssue>,
    pub contributors: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ReleaseHighlight {
    pub category: HighlightCategory,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightCategory {
    Security,
    Performance,
    NewFeature,
    Removed,
    BugFix,
    DriverSupport,
    DesktopExperience,
}

impl HighlightCategory {
    pub fn emoji(self) -> &'static str {
        match self {
            HighlightCategory::Security => "🔐",
            HighlightCategory::Performance => "⚡",
            HighlightCategory::NewFeature => "✨",
            HighlightCategory::Removed => "🗑️",
            HighlightCategory::BugFix => "🐛",
            HighlightCategory::DriverSupport => "🔌",
            HighlightCategory::DesktopExperience => "🖥️",
        }
    }
}

#[derive(Debug, Clone)]
pub struct KnownIssue {
    pub title: String,
    pub description: String,
    pub workaround: Option<String>,
    pub tracking_url: Option<String>,
}

impl ReleaseAnnouncement {
    /// Generate markdown release notes (DISCOVERABILITY + TRANSPARENCY)
    pub fn to_markdown(&self) -> String {
        let mut out = String::new();

        out.push_str(&format!(
            "# SigmaOS {} Release Notes\n\n",
            self.release.full_version()
        ));
        out.push_str(&format!("**Release Date:** {}\n", self.release.build_date));
        out.push_str(&format!("**Kernel:** {}\n", self.release.kernel_version));
        out.push_str(&format!("**Download:** [ISO]({})\n", self.release.iso_url));
        out.push_str(&format!("**SHA256:** `{}`\n\n", self.release.iso_sha256));
        out.push_str(&format!(
            "**Minimum Requirements:** {}MB RAM, {}GB storage\n\n",
            self.release.min_ram_mb, self.release.min_storage_gb
        ));

        // Highlights by category
        out.push_str("## Highlights\n\n");
        for h in &self.highlights {
            out.push_str(&format!(
                "{} **{}**: {}\n\n",
                h.category.emoji(),
                h.title,
                h.description
            ));
        }

        // Breaking changes
        if !self.breaking_changes.is_empty() {
            out.push_str("## ⚠️ Breaking Changes\n\n");
            for bc in &self.breaking_changes {
                out.push_str(&format!("- {}\n", bc));
            }
            out.push('\n');
        }

        // Known issues
        if !self.known_issues.is_empty() {
            out.push_str("## 🔍 Known Issues\n\n");
            for ki in &self.known_issues {
                out.push_str(&format!("### {}\n{}\n", ki.title, ki.description));
                if let Some(ref w) = ki.workaround {
                    out.push_str(&format!("**Workaround:** {}\n", w));
                }
                out.push('\n');
            }
        }

        // Contributors (COMMUNITY-ORIENTED + MERITOCRACY)
        if !self.contributors.is_empty() {
            out.push_str("## 🙏 Contributors\n\n");
            out.push_str("Thank you to all contributors:\n");
            for c in &self.contributors {
                out.push_str(&format!("- @{}\n", c));
            }
        }

        out
    }
}

// ============================================================
// Release Manager — orchestrates everything
// ============================================================

/// The Sovereign Release Manager
pub struct SovereignReleaseManager {
    pub current_release: Option<PointRelease>,
    pub available_releases: Vec<PointRelease>,
    pub eol_tracker: EolTracker,
    pub upgrade_paths: Vec<UpgradePath>,
}

impl SovereignReleaseManager {
    pub fn new() -> Self {
        Self {
            current_release: None,
            available_releases: Vec::new(),
            eol_tracker: EolTracker::new(),
            upgrade_paths: Vec::new(),
        }
    }

    /// Check if the current release is approaching EOL (warn 90 days before)
    pub fn eol_warning(&self, current_date: &str) -> Option<String> {
        let release = self.current_release.as_ref()?;
        let codename = release.series.codename();

        if let Some(eol_date) = self.eol_tracker.eol_date(codename) {
            // Simple string comparison works for ISO 8601 dates
            if current_date >= eol_date {
                let upgrade_to = self
                    .eol_tracker
                    .recommended_upgrade(codename)
                    .unwrap_or("the latest release");
                return Some(format!(
                    "⚠️ SigmaOS {} ({}) has reached End of Life ({}). Please upgrade to {}.",
                    release.full_version(),
                    codename,
                    eol_date,
                    upgrade_to
                ));
            }
        }
        None
    }

    /// Find the best upgrade path from current release
    pub fn recommended_upgrade_path(&self) -> Option<&UpgradePath> {
        let current = self.current_release.as_ref()?;
        let codename = current.series.codename();
        self.upgrade_paths
            .iter()
            .find(|p| p.from == codename && p.supported)
    }

    /// Check if a new point release is available
    pub fn has_new_point_release(&self) -> bool {
        if let Some(ref current) = self.current_release {
            self.available_releases.iter().any(|r| {
                r.series.codename() == current.series.codename() && r.point > current.point
            })
        } else {
            !self.available_releases.is_empty()
        }
    }

    /// Get release notes URL for current release
    pub fn release_notes_url(&self) -> Option<&str> {
        self.current_release
            .as_ref()
            .map(|r| r.release_notes_url.as_str())
    }
}

// ============================================================
// Tests (ARRANGE-ACT-ASSERT + DETERMINISTIC + FAST-ISOLATED)
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn make_lts_release() -> PointRelease {
        PointRelease {
            series: ReleaseSeries::Lts {
                year: 2026,
                month: 4,
                codename: "apex".to_string(),
            },
            point: 0,
            build_date: "2026-04-20".to_string(),
            iso_sha256: "abc123def456".to_string(),
            iso_url: "https://releases.sigmaos.dev/26.04/sigmaos-26.04-desktop-amd64.iso"
                .to_string(),
            delta_url: None,
            kernel_version: "6.9.0-sigmaos".to_string(),
            base_packages: vec!["systemd-256".to_string(), "hyprland-0.41".to_string()],
            release_notes_url: "https://releases.sigmaos.dev/26.04/notes".to_string(),
            is_current: true,
            min_ram_mb: 2048,
            min_storage_gb: 20,
        }
    }

    #[test]
    fn test_lts_version_string() {
        let r = ReleaseSeries::Lts {
            year: 2026,
            month: 4,
            codename: "apex".to_string(),
        };
        assert_eq!(r.version_string(), "2026.04 LTS");
        assert!(r.is_lts());
        assert!(!r.is_rolling());
        assert_eq!(r.support_months(), Some(60));
        assert!(r.has_esm());
    }

    #[test]
    fn test_standard_version_string() {
        let r = ReleaseSeries::Standard {
            year: 2026,
            month: 10,
            codename: "vanguard".to_string(),
        };
        assert_eq!(r.version_string(), "2026.10");
        assert!(!r.is_lts());
        assert_eq!(r.support_months(), Some(9));
    }

    #[test]
    fn test_rolling_no_eol() {
        let r = ReleaseSeries::Rolling {
            codename: "sovereign".to_string(),
        };
        assert!(r.is_rolling());
        assert_eq!(r.support_months(), None);
        assert!(!r.has_esm());
    }

    #[test]
    fn test_point_release_full_version() {
        let mut r = make_lts_release();
        assert_eq!(r.full_version(), "2026.04 LTS");
        r.point = 1;
        assert_eq!(r.full_version(), "2026.04.1 LTS");
        r.point = 2;
        assert_eq!(r.full_version(), "2026.04.2 LTS");
    }

    #[test]
    fn test_eol_tracker() {
        let tracker = EolTracker::new();
        // apex EOL is 2027-04-30
        assert!(!tracker.is_eol("apex", "2026-10-23"));
        assert!(tracker.is_eol("apex", "2027-05-01"));
        assert_eq!(tracker.recommended_upgrade("apex"), Some("vanguard"));
        assert_eq!(tracker.eol_date("apex"), Some("2027-04-30"));
    }

    #[test]
    fn test_eol_warning_not_triggered() {
        let mut mgr = SovereignReleaseManager::new();
        mgr.current_release = Some(make_lts_release());
        // Before EOL — no warning
        let warning = mgr.eol_warning("2026-10-23");
        assert!(warning.is_none());
    }

    #[test]
    fn test_eol_warning_triggered() {
        let mut mgr = SovereignReleaseManager::new();
        mgr.current_release = Some(make_lts_release());
        // After EOL date
        let warning = mgr.eol_warning("2027-05-01");
        assert!(warning.is_some());
        let msg = warning.unwrap();
        assert!(msg.contains("End of Life"));
        assert!(msg.contains("vanguard")); // upgrade target
    }

    #[test]
    fn test_has_new_point_release() {
        let mut mgr = SovereignReleaseManager::new();
        mgr.current_release = Some(make_lts_release()); // point = 0
        let mut newer = make_lts_release();
        newer.point = 1;
        mgr.available_releases.push(newer);
        assert!(mgr.has_new_point_release());
    }

    #[test]
    fn test_no_new_point_release() {
        let mut mgr = SovereignReleaseManager::new();
        mgr.current_release = Some(make_lts_release()); // point = 0
                                                        // No available releases
        assert!(!mgr.has_new_point_release());
    }

    #[test]
    fn test_release_announcement_markdown() {
        let release = make_lts_release();
        let ann = ReleaseAnnouncement {
            highlights: vec![
                ReleaseHighlight {
                    category: HighlightCategory::Security,
                    title: "Hardened kernel".to_string(),
                    description: "CFI and KASLR improvements".to_string(),
                },
                ReleaseHighlight {
                    category: HighlightCategory::Performance,
                    title: "Faster boot".to_string(),
                    description: "Boot time reduced by 30%".to_string(),
                },
            ],
            breaking_changes: vec!["Removed Python 2 support".to_string()],
            known_issues: vec![],
            contributors: vec!["aaryansinghchauhan09".to_string()],
            release,
        };
        let md = ann.to_markdown();
        assert!(md.contains("SigmaOS 2026.04 LTS Release Notes"));
        assert!(md.contains("🔐"));
        assert!(md.contains("⚡"));
        assert!(md.contains("Breaking Changes"));
        assert!(md.contains("@aaryansinghchauhan09"));
    }

    #[test]
    fn test_upgrade_risk_levels() {
        // Just ensure enum coverage
        let levels = [
            UpgradeRisk::Low,
            UpgradeRisk::Medium,
            UpgradeRisk::High,
            UpgradeRisk::Critical,
        ];
        assert_eq!(levels.len(), 4);
    }
}
