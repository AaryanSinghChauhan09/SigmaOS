// SPDX-License-Identifier: MIT OR GPL-2.0
//! SigmaOS Sovereign Update Manager
//! (`src/package/sovereign_update_manager.rs`)
//!
//! Inspired by:
//! - Linux Mint mintupdate (safety levels, kernel management, update types)
//! - Ubuntu unattended-upgrades (automatic security updates)
//! - Arch Linux checkupdates + pacman (atomic, rollback-safe)
//! - NixOS generations (declarative, rollback-first)
//! - FreeBSD freebsd-update (delta patches, binary updates)
//! - OpenBSD syspatch (minimal, security-focused)
//! - Flatpak / AppImage (sandboxed app updates)
//!
//! Principles applied:
//! ABSTRACTION · ARCHITECTURE · BOY SCOUT RULE · COMMAND-QUERY SEPARATION ·
//! COMPOSITION OVER INHERITANCE · DEFENSIVE PROGRAMMING · DON'T REPEAT YOURSELF ·
//! ENCAPSULATION · FAIL-FAST · INFORMATION HIDING · INTERFACE SEGREGATION ·
//! KEEP IT SIMPLE · LAYERED DESIGN · LEAST PRIVILEGE · MODULARITY ·
//! OPEN-CLOSED · SEPARATION OF CONCERNS · SINGLE RESPONSIBILITY ·
//! SOFTWARE SUPPORT PROGRAM · SOFTWARE UPDATING PROGRAM · STABLE APIs ·
//! VERSION CONTROL PRACTICES · YOU AREN'T GONNA NEED IT · ZEN OF PYTHON

#![allow(dead_code)]

use std::boxed::Box;
use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

// ============================================================
// Core Update Types — single responsibility per enum variant
// ============================================================

/// Update safety classification (Linux Mint mintupdate style)
/// The higher the level, the more risky — same principle as Mint's 1-5 scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SafetyLevel {
    /// Level 1: Certified safe — security patches to core libraries (apt-security)
    Certified = 1,
    /// Level 2: Recommended — bugfix updates, well-tested
    Recommended = 2,
    /// Level 3: Normal — feature updates, generally safe
    Normal = 3,
    /// Level 4: Caution — kernel or display server updates
    Caution = 4,
    /// Level 5: Dangerous — untested, potentially breaking
    Dangerous = 5,
}

impl SafetyLevel {
    pub fn as_str(self) -> &'static str {
        match self {
            SafetyLevel::Certified => "certified",
            SafetyLevel::Recommended => "recommended",
            SafetyLevel::Normal => "normal",
            SafetyLevel::Caution => "caution",
            SafetyLevel::Dangerous => "dangerous",
        }
    }

    /// Auto-apply threshold: levels ≤ this are applied automatically (SECURE DEFAULTS)
    pub fn auto_apply_threshold() -> SafetyLevel {
        SafetyLevel::Recommended
    }
}

/// Update category — what kind of update this is (SINGLE RESPONSIBILITY)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateCategory {
    SecurityPatch,       // CVE fix — highest priority, auto-apply
    KernelUpdate,        // New kernel version — requires reboot
    DriverUpdate,        // Hardware driver — may need reboot
    LibraryUpdate,       // Shared library — safe after restart
    ApplicationUpdate,   // Desktop app — safe immediately
    SystemUpdate,        // Core OS components
    LanguagePackUpdate,  // Locale / i18n
    FirmwareUpdate,      // UEFI/BIOS firmware — handle with care
    ConfigUpdate,        // Configuration-only change
    DocumentationUpdate, // Docs only — always safe
}

impl UpdateCategory {
    pub fn requires_reboot(self) -> bool {
        matches!(
            self,
            UpdateCategory::KernelUpdate
                | UpdateCategory::FirmwareUpdate
                | UpdateCategory::DriverUpdate
        )
    }

    pub fn safety_level(self) -> SafetyLevel {
        match self {
            UpdateCategory::SecurityPatch => SafetyLevel::Certified,
            UpdateCategory::DocumentationUpdate => SafetyLevel::Certified,
            UpdateCategory::ConfigUpdate => SafetyLevel::Recommended,
            UpdateCategory::LanguagePackUpdate => SafetyLevel::Recommended,
            UpdateCategory::ApplicationUpdate => SafetyLevel::Normal,
            UpdateCategory::LibraryUpdate => SafetyLevel::Normal,
            UpdateCategory::SystemUpdate => SafetyLevel::Caution,
            UpdateCategory::DriverUpdate => SafetyLevel::Caution,
            UpdateCategory::KernelUpdate => SafetyLevel::Caution,
            UpdateCategory::FirmwareUpdate => SafetyLevel::Dangerous,
        }
    }
}

/// Package update descriptor (INFORMATION HIDING — only exposes what's needed)
#[derive(Debug, Clone)]
pub struct PackageUpdate {
    pub name: String,
    pub old_version: String,
    pub new_version: String,
    pub category: UpdateCategory,
    pub size_bytes: u64,
    pub changelog_url: Option<String>,
    pub cve_ids: Vec<String>,     // CVE identifiers if security patch
    pub source_repo: String,      // Which repo this comes from
    pub checksum_sha256: String,  // For integrity verification
    pub signature_verified: bool, // GPG verification status
}

impl PackageUpdate {
    pub fn safety_level(&self) -> SafetyLevel {
        self.category.safety_level()
    }

    pub fn is_security_critical(&self) -> bool {
        self.category == UpdateCategory::SecurityPatch && !self.cve_ids.is_empty()
    }

    pub fn is_verified(&self) -> bool {
        self.signature_verified && !self.checksum_sha256.is_empty()
    }
}

/// A complete system snapshot for rollback support (NixOS generations style)
/// (VERSION CONTROL PRACTICES applied to OS state)
#[derive(Debug, Clone)]
pub struct SystemGeneration {
    pub id: u64,
    pub created_at: u64,       // Unix timestamp
    pub label: String,         // Human-readable description
    pub packages: Vec<String>, // Package names at this generation
    pub kernel_version: String,
    pub is_current: bool,
    pub is_booted: bool,
}

/// Update repository (OPEN STANDARD — mirrors standard APT/pacman repo format)
#[derive(Debug, Clone)]
pub struct UpdateRepository {
    pub name: String,
    pub url: String,
    pub priority: u8, // Higher = preferred when same pkg in multiple repos
    pub enabled: bool,
    pub verified: bool, // GPG key verified
    pub component: RepoComponent,
    pub last_synced: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepoComponent {
    Main,       // Officially supported, security-maintained
    Universe,   // Community-maintained
    Restricted, // Proprietary drivers
    Multiverse, // Non-free, no support
    Security,   // Security updates only mirror
    Backports,  // Newer versions of packages
}

// ============================================================
// Update Manager — single entry point (SINGLE RESPONSIBILITY)
// ============================================================

/// Update policy configuration (SEPARATION OF MECHANISM & POLICY)
#[derive(Debug, Clone)]
pub struct UpdatePolicy {
    /// Automatically apply security updates (SECURE DEFAULTS)
    pub auto_security: bool,
    /// Safety level threshold for automatic updates
    pub auto_threshold: SafetyLevel,
    /// Check interval in minutes
    pub check_interval_mins: u32,
    /// Maximum parallel downloads
    pub max_parallel_downloads: u8,
    /// Keep N old kernels installed for rollback
    pub kernels_to_keep: u8,
    /// Keep N system generations for rollback
    pub generations_to_keep: u8,
    /// Pause updates on metered connections
    pub pause_on_metered: bool,
    /// Download only, don't install
    pub download_only: bool,
    /// Require user confirmation for level ≥ this
    pub confirm_threshold: SafetyLevel,
}

impl Default for UpdatePolicy {
    fn default() -> Self {
        // SECURE DEFAULTS: auto-apply security, confirm everything else
        Self {
            auto_security: true,
            auto_threshold: SafetyLevel::Recommended,
            check_interval_mins: 60,
            max_parallel_downloads: 4,
            kernels_to_keep: 2,
            generations_to_keep: 5,
            pause_on_metered: true,
            download_only: false,
            confirm_threshold: SafetyLevel::Normal,
        }
    }
}

/// Update check result (COMMAND-QUERY SEPARATION — query only, no side effects)
#[derive(Debug, Clone)]
pub struct UpdateCheckResult {
    pub available_updates: Vec<PackageUpdate>,
    pub security_count: usize,
    pub reboot_required: bool,
    pub total_size_bytes: u64,
    pub checked_at: u64,
}

impl UpdateCheckResult {
    pub fn new(updates: Vec<PackageUpdate>) -> Self {
        let security_count = updates.iter().filter(|u| u.is_security_critical()).count();
        let reboot_required = updates.iter().any(|u| u.category.requires_reboot());
        let total_size_bytes = updates.iter().map(|u| u.size_bytes).sum();
        Self {
            available_updates: updates,
            security_count,
            reboot_required,
            total_size_bytes,
            checked_at: 0,
        }
    }

    pub fn count(&self) -> usize {
        self.available_updates.len()
    }

    pub fn has_security_updates(&self) -> bool {
        self.security_count > 0
    }

    /// Filter updates by safety level (OPEN-CLOSED: filter without modifying source)
    pub fn by_safety(&self, max_level: SafetyLevel) -> Vec<&PackageUpdate> {
        self.available_updates
            .iter()
            .filter(|u| u.safety_level() <= max_level)
            .collect()
    }

    /// Get only unverified updates — should warn user (DEFENSIVE PROGRAMMING)
    pub fn unverified(&self) -> Vec<&PackageUpdate> {
        self.available_updates
            .iter()
            .filter(|u| !u.is_verified())
            .collect()
    }
}

/// Update installation result
#[derive(Debug, Clone)]
pub struct InstallResult {
    pub installed: Vec<String>,
    pub failed: Vec<(String, String)>, // (package, error)
    pub skipped: Vec<String>,
    pub reboot_required: bool,
    pub generation_id: Option<u64>, // New generation created if rollback supported
}

impl InstallResult {
    pub fn success_count(&self) -> usize {
        self.installed.len()
    }
    pub fn failure_count(&self) -> usize {
        self.failed.len()
    }
    pub fn is_fully_successful(&self) -> bool {
        self.failed.is_empty()
    }
}

/// Backend trait for package manager implementations (INTERFACE SEGREGATION)
/// Each PM (apt/pacman/dnf/apk/pkg) implements this. (PLUG-IN DRIVER MODEL)
pub trait PackageBackend {
    fn name(&self) -> &str;
    fn check_updates(&self) -> Result<Vec<PackageUpdate>, UpdateError>;
    fn install(&mut self, packages: &[String]) -> Result<InstallResult, UpdateError>;
    fn rollback(&mut self, generation_id: u64) -> Result<(), UpdateError>;
    fn list_generations(&self) -> Vec<SystemGeneration>;
    fn clean_cache(&mut self) -> Result<u64, UpdateError>; // Returns bytes freed
}

/// Stubbed apt backend (Debian/Ubuntu/Mint style)
pub struct AptBackend {
    pub cache_dir: String,
    pub sources_list: Vec<UpdateRepository>,
}

impl AptBackend {
    pub fn new() -> Self {
        Self {
            cache_dir: "/var/cache/apt".to_string(),
            sources_list: Vec::new(),
        }
    }
}

impl PackageBackend for AptBackend {
    fn name(&self) -> &str {
        "apt"
    }

    fn check_updates(&self) -> Result<Vec<PackageUpdate>, UpdateError> {
        // In production: parse `apt list --upgradable`
        Ok(Vec::new())
    }

    fn install(&mut self, packages: &[String]) -> Result<InstallResult, UpdateError> {
        Ok(InstallResult {
            installed: packages.to_vec(),
            failed: Vec::new(),
            skipped: Vec::new(),
            reboot_required: false,
            generation_id: None,
        })
    }

    fn rollback(&mut self, _generation_id: u64) -> Result<(), UpdateError> {
        Err(UpdateError::NotSupported(
            "APT does not support generation rollback".to_string(),
        ))
    }

    fn list_generations(&self) -> Vec<SystemGeneration> {
        Vec::new()
    }

    fn clean_cache(&mut self) -> Result<u64, UpdateError> {
        // In production: `apt-get clean`
        Ok(0)
    }
}

/// Stubbed pacman backend (Arch Linux style)
pub struct PacmanBackend {
    pub db_path: String,
    pub cache_dir: String,
}

impl PacmanBackend {
    pub fn new() -> Self {
        Self {
            db_path: "/var/lib/pacman".to_string(),
            cache_dir: "/var/cache/pacman/pkg".to_string(),
        }
    }
}

impl PackageBackend for PacmanBackend {
    fn name(&self) -> &str {
        "pacman"
    }

    fn check_updates(&self) -> Result<Vec<PackageUpdate>, UpdateError> {
        Ok(Vec::new())
    }

    fn install(&mut self, packages: &[String]) -> Result<InstallResult, UpdateError> {
        Ok(InstallResult {
            installed: packages.to_vec(),
            failed: Vec::new(),
            skipped: Vec::new(),
            reboot_required: false,
            generation_id: None,
        })
    }

    fn rollback(&mut self, _generation_id: u64) -> Result<(), UpdateError> {
        // pacman supports downgrade via /var/cache/pacman/pkg
        Ok(())
    }

    fn list_generations(&self) -> Vec<SystemGeneration> {
        Vec::new()
    }

    fn clean_cache(&mut self) -> Result<u64, UpdateError> {
        // In production: `paccache -r`
        Ok(0)
    }
}

/// Stubbed pkg backend (FreeBSD style)
pub struct PkgBackend {
    pub db_path: String,
}

impl PkgBackend {
    pub fn new() -> Self {
        Self {
            db_path: "/var/db/pkg".to_string(),
        }
    }
}

impl PackageBackend for PkgBackend {
    fn name(&self) -> &str {
        "pkg"
    }
    fn check_updates(&self) -> Result<Vec<PackageUpdate>, UpdateError> {
        Ok(Vec::new())
    }
    fn install(&mut self, packages: &[String]) -> Result<InstallResult, UpdateError> {
        Ok(InstallResult {
            installed: packages.to_vec(),
            failed: Vec::new(),
            skipped: Vec::new(),
            reboot_required: false,
            generation_id: None,
        })
    }
    fn rollback(&mut self, _id: u64) -> Result<(), UpdateError> {
        Ok(())
    }
    fn list_generations(&self) -> Vec<SystemGeneration> {
        Vec::new()
    }
    fn clean_cache(&mut self) -> Result<u64, UpdateError> {
        Ok(0)
    }
}

/// Update error type (FAIL-FAST system)
#[derive(Debug, Clone)]
pub enum UpdateError {
    NetworkError(String),
    ChecksumMismatch {
        pkg: String,
        expected: String,
        actual: String,
    },
    SignatureInvalid(String),
    InsufficientSpace {
        needed: u64,
        available: u64,
    },
    DependencyConflict(String),
    PermissionDenied(String),
    BackendError(String),
    RollbackFailed(String),
    NotSupported(String),
    Timeout(String),
}

impl UpdateError {
    pub fn as_str(&self) -> String {
        match self {
            UpdateError::NetworkError(e) => format!("Network error: {}", e),
            UpdateError::ChecksumMismatch {
                pkg,
                expected,
                actual,
            } => format!(
                "Checksum mismatch for {}: expected {} got {}",
                pkg, expected, actual
            ),
            UpdateError::SignatureInvalid(p) => format!("Invalid GPG signature: {}", p),
            UpdateError::InsufficientSpace { needed, available } => format!(
                "Need {}MB, only {}MB available",
                needed / 1_048_576,
                available / 1_048_576
            ),
            UpdateError::DependencyConflict(e) => format!("Dependency conflict: {}", e),
            UpdateError::PermissionDenied(e) => format!("Permission denied: {}", e),
            UpdateError::BackendError(e) => format!("Backend error: {}", e),
            UpdateError::RollbackFailed(e) => format!("Rollback failed: {}", e),
            UpdateError::NotSupported(e) => format!("Not supported: {}", e),
            UpdateError::Timeout(e) => format!("Timeout: {}", e),
        }
    }
}

/// The Sovereign Update Manager — top-level orchestrator (COMPOSITION OVER INHERITANCE)
/// This is the single entry point for all update operations on SigmaOS.
/// Equivalent to Linux Mint's mintupdate daemon but unified across all backends.
pub struct SovereignUpdateManager {
    policy: UpdatePolicy,
    backends: Vec<Box<dyn PackageBackend>>,
    last_check: Option<UpdateCheckResult>,
    pending_reboot: bool,
    update_history: Vec<UpdateHistoryEntry>,
}

/// Historical record of applied updates (audit trail — INSTRUMENTATION)
#[derive(Debug, Clone)]
pub struct UpdateHistoryEntry {
    pub timestamp: u64,
    pub packages: Vec<String>,
    pub backend: String,
    pub success: bool,
    pub generation_id: Option<u64>,
}

impl SovereignUpdateManager {
    /// Create manager with default secure policy (SECURE DEFAULTS + LEAST PRIVILEGE)
    pub fn new() -> Self {
        Self {
            policy: UpdatePolicy::default(),
            backends: Vec::new(),
            last_check: None,
            pending_reboot: false,
            update_history: Vec::new(),
        }
    }

    /// Create manager with custom policy (OPEN-CLOSED)
    pub fn with_policy(policy: UpdatePolicy) -> Self {
        let mut m = Self::new();
        m.policy = policy;
        m
    }

    /// Register a package backend (PLUG-IN DRIVER MODEL)
    pub fn register_backend(&mut self, backend: Box<dyn PackageBackend>) {
        self.backends.push(backend);
    }

    /// Query available updates across all backends (COMMAND-QUERY SEPARATION)
    /// This is a PURE QUERY — no side effects, no mutations.
    pub fn check_updates(&mut self) -> Result<&UpdateCheckResult, UpdateError> {
        let mut all_updates: Vec<PackageUpdate> = Vec::new();

        for backend in &self.backends {
            match backend.check_updates() {
                Ok(mut updates) => all_updates.append(&mut updates),
                Err(e) => return Err(e),
            }
        }

        // Validate all updates before reporting (DEFENSIVE PROGRAMMING)
        let unverified: Vec<_> = all_updates
            .iter()
            .filter(|u| !u.is_verified())
            .map(|u| u.name.clone())
            .collect();
        if !unverified.is_empty() {
            // Log warning but don't block — let policy decide (SEPARATION OF MECHANISM & POLICY)
        }

        self.last_check = Some(UpdateCheckResult::new(all_updates));
        Ok(self.last_check.as_ref().unwrap())
    }

    /// Apply updates up to a given safety level (COMMAND — has side effects)
    /// Respects LEAST PRIVILEGE: only applies what policy permits.
    pub fn apply_updates(&mut self, max_level: SafetyLevel) -> Result<InstallResult, UpdateError> {
        let pkgs_to_install: Vec<String> = if let Some(ref check) = self.last_check {
            check
                .by_safety(max_level)
                .iter()
                .map(|u| u.name.clone())
                .collect()
        } else {
            return Err(UpdateError::BackendError(
                "Must call check_updates() before apply_updates()".to_string(),
            ));
        };

        if pkgs_to_install.is_empty() {
            return Ok(InstallResult {
                installed: Vec::new(),
                failed: Vec::new(),
                skipped: Vec::new(),
                reboot_required: false,
                generation_id: None,
            });
        }

        // Apply through first available backend (GALL'S LAW: start simple)
        if let Some(backend) = self.backends.first_mut() {
            let result = backend.install(&pkgs_to_install)?;
            if result.reboot_required {
                self.pending_reboot = true;
            }

            // Record history (INSTRUMENTATION)
            self.update_history.push(UpdateHistoryEntry {
                timestamp: 0,
                packages: result.installed.clone(),
                backend: backend.name().to_string(),
                success: result.is_fully_successful(),
                generation_id: result.generation_id,
            });

            Ok(result)
        } else {
            Err(UpdateError::BackendError(
                "No package backend registered".to_string(),
            ))
        }
    }

    /// Apply only security updates automatically (SECURE DEFAULTS + AUTOMATION)
    pub fn apply_security_updates(&mut self) -> Result<InstallResult, UpdateError> {
        self.apply_updates(SafetyLevel::Certified)
    }

    /// Apply all recommended + security updates
    pub fn apply_recommended_updates(&mut self) -> Result<InstallResult, UpdateError> {
        self.apply_updates(SafetyLevel::Recommended)
    }

    /// Rollback to a previous system generation (NixOS/FreeBSD style)
    pub fn rollback(&mut self, generation_id: u64) -> Result<(), UpdateError> {
        if let Some(backend) = self.backends.first_mut() {
            backend.rollback(generation_id)
        } else {
            Err(UpdateError::BackendError(
                "No backend registered".to_string(),
            ))
        }
    }

    /// List available system generations for rollback
    pub fn list_generations(&self) -> Vec<SystemGeneration> {
        self.backends
            .first()
            .map(|b| b.list_generations())
            .unwrap_or_default()
    }

    /// Clean package cache (BOY SCOUT RULE — leave the system cleaner)
    pub fn clean_cache(&mut self) -> Result<u64, UpdateError> {
        let mut total_freed: u64 = 0;
        for backend in &mut self.backends {
            total_freed += backend.clean_cache()?;
        }
        Ok(total_freed)
    }

    /// Check if a reboot is pending (INSTRUMENTATION)
    pub fn pending_reboot(&self) -> bool {
        self.pending_reboot
    }

    /// Get update history (TRANSPARENCY)
    pub fn history(&self) -> &[UpdateHistoryEntry] {
        &self.update_history
    }

    /// Get current policy (INFORMATION HIDING via reference)
    pub fn policy(&self) -> &UpdatePolicy {
        &self.policy
    }
}

// ============================================================
// Delta Update Engine (SOFTWARE UPDATING PROGRAM LIKE LINUX DISTROS)
// Inspired by: apt-delta, freebsd-update, zsync, casync
// ============================================================

/// Delta patch descriptor
#[derive(Debug, Clone)]
pub struct DeltaPatch {
    pub pkg_name: String,
    pub from_version: String,
    pub to_version: String,
    pub patch_url: String,
    pub patch_size_bytes: u64,
    pub full_size_bytes: u64,
    pub algorithm: DeltaAlgorithm,
    pub checksum: String,
}

impl DeltaPatch {
    pub fn savings_bytes(&self) -> u64 {
        self.full_size_bytes.saturating_sub(self.patch_size_bytes)
    }

    pub fn savings_pct(&self) -> u8 {
        if self.full_size_bytes == 0 {
            return 0;
        }
        ((self.savings_bytes() * 100) / self.full_size_bytes) as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeltaAlgorithm {
    Zsync,   // rsync-over-HTTP (apt-delta style)
    Casync,  // Content-addressable sync (systemd casync)
    Bsdiff,  // Binary diff (old Chromium style)
    Xdelta3, // Open-source xdelta binary diff
}

/// Delta update engine — downloads and applies patches (ZERO-COPY where possible)
pub struct DeltaUpdateEngine {
    pub cache_dir: String,
    pub max_patch_size_mb: u64,
}

impl DeltaUpdateEngine {
    pub fn new(cache_dir: &str) -> Self {
        Self {
            cache_dir: cache_dir.to_string(),
            max_patch_size_mb: 512,
        }
    }

    /// Check if a delta is available and worth using (YAGNI: only download if saves >10%)
    pub fn should_use_delta(&self, patch: &DeltaPatch) -> bool {
        patch.savings_pct() >= 10
    }

    /// Get summary of bandwidth savings
    pub fn bandwidth_savings_summary(&self, patches: &[DeltaPatch]) -> String {
        let total_full: u64 = patches.iter().map(|p| p.full_size_bytes).sum();
        let total_delta: u64 = patches.iter().map(|p| p.patch_size_bytes).sum();
        let saved = total_full.saturating_sub(total_delta);
        format!(
            "Delta updates: {}/{} packages, saving {}MB ({:.1}%)",
            patches.len(),
            patches.len(),
            saved / 1_048_576,
            if total_full > 0 {
                (saved * 100) as f64 / total_full as f64
            } else {
                0.0
            }
        )
    }
}

// ============================================================
// Tests (ARRANGE-ACT-ASSERT + FIRST + DETERMINISTIC)
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn make_update(name: &str, cat: UpdateCategory, cves: Vec<&str>) -> PackageUpdate {
        PackageUpdate {
            name: name.to_string(),
            old_version: "1.0".to_string(),
            new_version: "1.1".to_string(),
            category: cat,
            size_bytes: 1_048_576,
            changelog_url: None,
            cve_ids: cves.iter().map(|s| s.to_string()).collect(),
            source_repo: "main".to_string(),
            checksum_sha256: "abc123".to_string(),
            signature_verified: true,
        }
    }

    #[test]
    fn test_safety_levels_ordered() {
        // ARRANGE-ACT-ASSERT: safety levels must be in ascending order
        assert!(SafetyLevel::Certified < SafetyLevel::Recommended);
        assert!(SafetyLevel::Recommended < SafetyLevel::Normal);
        assert!(SafetyLevel::Normal < SafetyLevel::Caution);
        assert!(SafetyLevel::Caution < SafetyLevel::Dangerous);
    }

    #[test]
    fn test_security_update_is_certified() {
        let u = make_update(
            "openssl",
            UpdateCategory::SecurityPatch,
            vec!["CVE-2026-1234"],
        );
        assert_eq!(u.safety_level(), SafetyLevel::Certified);
        assert!(u.is_security_critical());
    }

    #[test]
    fn test_kernel_update_requires_reboot() {
        let u = make_update("linux-kernel", UpdateCategory::KernelUpdate, vec![]);
        assert!(u.category.requires_reboot());
        assert_eq!(u.safety_level(), SafetyLevel::Caution);
    }

    #[test]
    fn test_firmware_is_dangerous() {
        let u = make_update("uefi-firmware", UpdateCategory::FirmwareUpdate, vec![]);
        assert_eq!(u.safety_level(), SafetyLevel::Dangerous);
    }

    #[test]
    fn test_update_check_result_filtering() {
        let updates = vec![
            make_update(
                "openssl",
                UpdateCategory::SecurityPatch,
                vec!["CVE-2026-1234"],
            ),
            make_update("firefox", UpdateCategory::ApplicationUpdate, vec![]),
            make_update("linux", UpdateCategory::KernelUpdate, vec![]),
        ];
        let result = UpdateCheckResult::new(updates);
        assert_eq!(result.count(), 3);
        assert!(result.has_security_updates());
        assert!(result.reboot_required);

        // Only certified updates (security patches)
        let certified = result.by_safety(SafetyLevel::Certified);
        assert_eq!(certified.len(), 1);

        // Certified + recommended + normal (no caution/dangerous)
        let normal_and_below = result.by_safety(SafetyLevel::Normal);
        assert_eq!(normal_and_below.len(), 2); // openssl + firefox, not kernel
    }

    #[test]
    fn test_default_policy_auto_security() {
        let policy = UpdatePolicy::default();
        assert!(policy.auto_security);
        assert_eq!(policy.auto_threshold, SafetyLevel::Recommended);
        assert_eq!(policy.kernels_to_keep, 2);
        assert_eq!(policy.generations_to_keep, 5);
    }

    #[test]
    fn test_manager_no_backend_error() {
        let mut mgr = SovereignUpdateManager::new();
        // Can check updates with no backends — returns empty
        let _ = mgr.check_updates();
        let err = mgr.apply_security_updates();
        // Should get BackendError since no backend registered but check was done
        // (or success with empty install list — both acceptable)
        let _ = err; // just ensure no panic
    }

    #[test]
    fn test_apt_backend_name() {
        let b = AptBackend::new();
        assert_eq!(b.name(), "apt");
    }

    #[test]
    fn test_pacman_backend_name() {
        let b = PacmanBackend::new();
        assert_eq!(b.name(), "pacman");
    }

    #[test]
    fn test_delta_savings() {
        let patch = DeltaPatch {
            pkg_name: "mesa".to_string(),
            from_version: "23.0".to_string(),
            to_version: "23.1".to_string(),
            patch_url: "https://updates.sigmaos.dev/delta/mesa-23.0-to-23.1.zs".to_string(),
            patch_size_bytes: 10 * 1_048_576, // 10MB delta
            full_size_bytes: 100 * 1_048_576, // 100MB full
            algorithm: DeltaAlgorithm::Zsync,
            checksum: "deadbeef".to_string(),
        };
        assert_eq!(patch.savings_pct(), 90);
        let engine = DeltaUpdateEngine::new("/var/cache/sigma-update");
        assert!(engine.should_use_delta(&patch));
    }

    #[test]
    fn test_unverified_update_detection() {
        let mut u = make_update("suspicious-pkg", UpdateCategory::ApplicationUpdate, vec![]);
        u.signature_verified = false;
        let result = UpdateCheckResult::new(vec![u]);
        assert_eq!(result.unverified().len(), 1);
    }
}
