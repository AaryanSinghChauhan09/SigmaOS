use std::vec::Vec;
// SPDX-License-Identifier: MIT
// Rolling Release System - Arch Linux Inspired
// Provides continuous updates without major version bumps

use std::collections::HashMap;
use std::string::String;
type SigmaString = String;
#[cfg(not(feature = "standalone_test"))]
use crate::sigpkg::Version;

#[cfg(feature = "standalone_test")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateChannel {
    Stable,
    Testing,
    Unstable,
}

#[derive(Debug, Clone)]
pub struct PackageUpdate {
    pub package_name: SigmaString,
    pub old_version: Version,
    pub new_version: Version,
    pub update_type: UpdateType,
    pub security_critical: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateType {
    Major,
    Minor,
    Patch,
    Security,
}

#[derive(Debug, Clone)]
pub struct RollingReleaseManager {
    current_version: Version,
    update_channel: UpdateChannel,
    auto_update_enabled: bool,
    package_versions: HashMap<SigmaString, Version>,
    update_history: Vec<PackageUpdate>,
}

impl RollingReleaseManager {
    pub fn new(initial_version: Version, channel: UpdateChannel) -> Self {
        RollingReleaseManager {
            current_version: initial_version,
            update_channel: channel,
            auto_update_enabled: false,
            package_versions: HashMap::new(),
            update_history: Vec::new(),
        }
    }

    pub fn enable_auto_updates(&mut self) {
        self.auto_update_enabled = true;
    }

    pub fn disable_auto_updates(&mut self) {
        self.auto_update_enabled = false;
    }

    pub fn set_update_channel(&mut self, channel: UpdateChannel) {
        self.update_channel = channel;
    }

    pub fn register_package(&mut self, name: SigmaString, version: Version) {
        self.package_versions.insert(name, version);
    }

    pub fn check_for_updates(
        &self,
        available_packages: &HashMap<SigmaString, Version>,
    ) -> Vec<PackageUpdate> {
        let mut updates = Vec::new();

        for (package_name, current_version) in &self.package_versions {
            if let Some(available_version) = available_packages.get(package_name) {
                if available_version > current_version {
                    let update_type =
                        self.determine_update_type(current_version, available_version);
                    let security_critical = self.is_security_critical(package_name);

                    updates.push(PackageUpdate {
                        package_name: package_name.clone(),
                        old_version: *current_version,
                        new_version: *available_version,
                        update_type,
                        security_critical,
                    });
                }
            }
        }

        updates.sort_by(|a, b| {
            // Security updates first, then by version
            if a.security_critical && !b.security_critical {
                return core::cmp::Ordering::Less;
            }
            if !a.security_critical && b.security_critical {
                return core::cmp::Ordering::Greater;
            }
            b.new_version.cmp(&a.new_version)
        });

        updates
    }

    pub fn apply_update(&mut self, update: PackageUpdate) -> Result<(), &'static str> {
        if let Some(current_version) = self.package_versions.get(&update.package_name) {
            if *current_version != update.old_version {
                return Err("Version mismatch - package already updated");
            }
        }

        self.package_versions
            .insert(update.package_name.clone(), update.new_version);
        self.update_history.push(update.clone());

        Ok(())
    }

    pub fn rollback_to_version(
        &mut self,
        package_name: &SigmaString,
        target_version: Version,
    ) -> Result<(), &'static str> {
        if let Some(current_version) = self.package_versions.get(package_name) {
            if *current_version == target_version {
                return Ok(()); // Already at target version
            }

            // Find the update history entry
            for update in &self.update_history {
                if update.package_name == *package_name && update.old_version == target_version {
                    self.package_versions
                        .insert(package_name.clone(), target_version);
                    return Ok(());
                }
            }

            Err("Target version not found in update history")
        } else {
            Err("Package not found")
        }
    }

    pub fn get_system_version(&self) -> Version {
        self.current_version
    }

    pub fn get_update_history(&self) -> &[PackageUpdate] {
        &self.update_history
    }

    fn determine_update_type(&self, old: &Version, new: &Version) -> UpdateType {
        if new.major > old.major {
            UpdateType::Major
        } else if new.minor > old.minor {
            UpdateType::Minor
        } else if new.patch > old.patch {
            UpdateType::Patch
        } else {
            UpdateType::Security // Assume security if same version but different
        }
    }

    fn is_security_critical(&self, package_name: &SigmaString) -> bool {
        // Common security-critical packages
        let security_packages = [
            "kernel", "openssl", "gnutls", "libssl", "crypto", "sudo", "openssh", "glibc",
            "systemd", "bash",
        ];

        let name_str = package_name.as_str();
        security_packages.iter().any(|&pkg| name_str.contains(pkg))
    }

    /// Diagnostic warning generator for partial updates (Arch Linux "partial upgrade" anti-pattern detection)
    pub fn find_partial_updates(&self, pending_updates: &[PackageUpdate]) -> Vec<SigmaString> {
        let mut core_base = Vec::new();
        let core_pkgs = ["glibc", "kernel", "gcc-libs", "openssl"];

        let has_core_update = pending_updates.iter().any(|u| {
            let s = u.package_name.as_str();
            core_pkgs.iter().any(|&cp| s.contains(cp))
        });

        if has_core_update {
            for (pkg_name, _) in &self.package_versions {
                let is_pending = pending_updates.iter().any(|u| u.package_name == *pkg_name);
                if !is_pending {
                    core_base.push(pkg_name.clone());
                }
            }
        }
        core_base
    }
}

#[derive(Debug, Clone)]
pub struct PacmanMirror {
    pub url: SigmaString,
    pub latency_ms: u32,
    pub completion_rate: f32,
}

pub struct PacmanMirrorlist {
    pub mirrors: Vec<PacmanMirror>,
}

impl PacmanMirrorlist {
    pub fn new() -> Self {
        Self {
            mirrors: Vec::new(),
        }
    }

    pub fn add_mirror(&mut self, url: &str, latency: u32, completion_rate: f32) {
        self.mirrors.push(PacmanMirror {
            url: SigmaString::from(url),
            latency_ms: latency,
            completion_rate,
        });
    }

    /// Sort mirrors by latency and completion rate (Arch reflector parity)
    pub fn rank_mirrors(&mut self) {
        self.mirrors.sort_by(|a, b| {
            if (a.completion_rate - b.completion_rate).abs() > 0.01 {
                b.completion_rate
                    .partial_cmp(&a.completion_rate)
                    .unwrap_or(core::cmp::Ordering::Equal)
            } else {
                a.latency_ms.cmp(&b.latency_ms)
            }
        });
    }
}

impl Default for PacmanMirrorlist {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for RollingReleaseManager {
    fn default() -> Self {
        Self::new(
            Version {
                major: 0,
                minor: 1,
                patch: 0,
            },
            UpdateChannel::Stable,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rolling_release_manager_creation() {
        let manager = RollingReleaseManager::new(
            Version {
                major: 1,
                minor: 0,
                patch: 0,
            },
            UpdateChannel::Stable,
        );

        assert_eq!(manager.get_system_version().major, 1);
        assert!(!manager.auto_update_enabled);
    }

    #[test]
    fn test_package_registration() {
        let mut manager = RollingReleaseManager::default();
        manager.register_package(
            SigmaString::from("test-pkg"),
            Version {
                major: 1,
                minor: 0,
                patch: 0,
            },
        );

        assert!(manager
            .package_versions
            .contains_key(&SigmaString::from("test-pkg")));
    }

    #[test]
    fn test_update_detection() {
        let mut manager = RollingReleaseManager::default();
        manager.register_package(
            SigmaString::from("test-pkg"),
            Version {
                major: 1,
                minor: 0,
                patch: 0,
            },
        );

        let mut available = HashMap::new();
        available.insert(
            SigmaString::from("test-pkg"),
            Version {
                major: 1,
                minor: 1,
                patch: 0,
            },
        );

        let updates = manager.check_for_updates(&available);
        assert_eq!(updates.len(), 1);
        assert_eq!(updates[0].new_version.minor, 1);
    }

    #[test]
    fn test_update_application() {
        let mut manager = RollingReleaseManager::default();
        manager.register_package(
            SigmaString::from("test-pkg"),
            Version {
                major: 1,
                minor: 0,
                patch: 0,
            },
        );

        let update = PackageUpdate {
            package_name: SigmaString::from("test-pkg"),
            old_version: Version {
                major: 1,
                minor: 0,
                patch: 0,
            },
            new_version: Version {
                major: 1,
                minor: 1,
                patch: 0,
            },
            update_type: UpdateType::Minor,
            security_critical: false,
        };

        assert!(manager.apply_update(update).is_ok());
        assert_eq!(
            manager
                .package_versions
                .get(&SigmaString::from("test-pkg"))
                .unwrap()
                .minor,
            1
        );
    }

    #[test]
    fn test_security_critical_detection() {
        let manager = RollingReleaseManager::default();
        assert!(manager.is_security_critical(&SigmaString::from("kernel")));
        assert!(manager.is_security_critical(&SigmaString::from("openssl")));
        assert!(!manager.is_security_critical(&SigmaString::from("text-editor")));
    }

    #[test]
    fn test_mirrorlist_reflector_ranking() {
        let mut list = PacmanMirrorlist::new();
        list.add_mirror("https://slow.arch.org", 250, 1.0);
        list.add_mirror("https://fast.arch.org", 45, 1.0);
        list.add_mirror("https://flaky.arch.org", 20, 0.8);

        list.rank_mirrors();
        assert_eq!(list.mirrors[0].url.as_str(), "https://fast.arch.org");
        assert_eq!(list.mirrors[1].url.as_str(), "https://slow.arch.org");
    }
}

// ============================================================================
// Arch Linux Rolling Release Improvements: MirrorList, Reflector, PacnewHandler
// ============================================================================

/// Mirror transport protocol
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MirrorProtocol {
    Https,
    Http,
    Rsync,
}

/// A single package mirror entry
#[derive(Debug, Clone)]
pub struct Mirror {
    pub url: String,
    pub protocol: MirrorProtocol,
    pub speed_mbps: f64,
    pub score: f64,
    pub active: bool,
}

/// An ordered list of mirrors for a country / region
#[derive(Debug, Clone)]
pub struct MirrorList {
    pub mirrors: Vec<Mirror>,
    pub country: String,
    /// Unix epoch timestamp of the last mirror list refresh
    pub last_updated: u64,
}

impl MirrorList {
    pub fn new(country: &str, last_updated: u64) -> Self {
        Self {
            mirrors: Vec::new(),
            country: country.to_string(),
            last_updated,
        }
    }

    pub fn add_mirror(&mut self, mirror: Mirror) {
        self.mirrors.push(mirror);
    }
}

/// Reflector: ranks and filters mirrors (mirrors Arch Linux's `reflector` tool)
#[derive(Debug, Clone)]
pub struct Reflector {
    pub mirror_list: MirrorList,
}

impl Reflector {
    pub fn new(mirror_list: MirrorList) -> Self {
        Self { mirror_list }
    }

    /// Sort mirrors by speed descending (highest bandwidth first)
    pub fn rank_by_speed(&mut self) {
        self.mirror_list.mirrors.sort_by(|a, b| {
            b.speed_mbps
                .partial_cmp(&a.speed_mbps)
                .unwrap_or(core::cmp::Ordering::Equal)
        });
    }

    /// Keep only mirrors for the given country (case-insensitive)
    pub fn filter_by_country(&mut self, country: &str) {
        let lower = country.to_lowercase();
        self.mirror_list.mirrors.retain(|m| {
            // Mirror URL is used as a proxy for country in this stub;
            // real impl would use GeoIP / mirror metadata
            m.url.to_lowercase().contains(&lower)
                || self.mirror_list.country.to_lowercase() == lower
        });
    }

    /// Discard any mirrors that are not using HTTPS
    pub fn filter_by_https_only(&mut self) {
        self.mirror_list
            .mirrors
            .retain(|m| m.protocol == MirrorProtocol::Https);
    }

    /// Serialise the mirror list into a pacman-style mirrorlist string
    pub fn save_mirrorlist(&self) -> String {
        let mut out = String::new();
        out.push_str(&std::format!(
            "# SigmaOS Mirrorlist — country: {} — updated: {}\n",
            self.mirror_list.country,
            self.mirror_list.last_updated
        ));
        for mirror in &self.mirror_list.mirrors {
            if mirror.active {
                out.push_str(&std::format!("Server = {}\n", mirror.url));
            } else {
                out.push_str(&std::format!("#Server = {}\n", mirror.url));
            }
        }
        out
    }
}

/// A `.pacnew` conflict file discovered on the system
#[derive(Debug, Clone)]
pub struct PacnewFile {
    pub original_path: String,
    pub pacnew_path: String,
}

/// Handles `.pacnew` merge conflicts (Arch Linux pacdiff parity)
#[derive(Debug, Clone)]
pub struct PacnewHandler {
    /// Known `.pacnew` files; populated via `list_pacnew_files()`
    pub known_files: Vec<PacnewFile>,
}

impl PacnewHandler {
    pub fn new() -> Self {
        Self {
            known_files: Vec::new(),
        }
    }

    /// Return the latest scan snapshot. Call `scan_directory` to refresh it.
    pub fn list_pacnew_files(&mut self) -> &[PacnewFile] {
        &self.known_files
    }

    /// Find `.pacnew` files below `root` without following symlinks.
    /// The current snapshot is replaced only after a complete successful scan.
    pub fn scan_directory<P: AsRef<std::path::Path>>(
        &mut self,
        root: P,
    ) -> std::io::Result<&[PacnewFile]> {
        fn visit(directory: &std::path::Path, found: &mut Vec<PacnewFile>) -> std::io::Result<()> {
            for entry in std::fs::read_dir(directory)? {
                let entry = entry?;
                let file_type = entry.file_type()?;
                let path = entry.path();
                if file_type.is_dir() {
                    visit(&path, found)?;
                } else if file_type.is_file() {
                    let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
                        continue;
                    };
                    let Some(original_name) = file_name.strip_suffix(".pacnew") else {
                        continue;
                    };
                    let original_path = path.with_file_name(original_name);
                    found.push(PacnewFile {
                        original_path: original_path.to_string_lossy().into_owned(),
                        pacnew_path: path.to_string_lossy().into_owned(),
                    });
                }
            }
            Ok(())
        }

        let root = root.as_ref();
        if !std::fs::symlink_metadata(root)?.is_dir() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "pacnew scan root must be a directory",
            ));
        }

        let mut found = Vec::new();
        visit(root, &mut found)?;
        found.sort_by(|left, right| left.pacnew_path.cmp(&right.pacnew_path));
        self.known_files = found;
        Ok(&self.known_files)
    }

    /// Produce a full-file unified diff preview for the current and `.pacnew`
    /// files. Missing current files are treated as empty; I/O and UTF-8 errors
    /// are returned to the caller.
    pub fn diff_pacnew(&self, path: &str) -> std::io::Result<String> {
        let pacnew = std::format!("{}.pacnew", path);
        let current = match std::fs::read_to_string(path) {
            Ok(contents) => contents,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => String::new(),
            Err(error) => return Err(error),
        };
        let proposed = std::fs::read_to_string(&pacnew)?;
        let mut diff = std::format!("--- {}\n+++ {}\n", path, pacnew);
        diff.push_str(&std::format!(
            "@@ -1,{} +1,{} @@\n",
            current.lines().count(),
            proposed.lines().count()
        ));
        for line in current.lines() {
            diff.push_str("-");
            diff.push_str(line);
            diff.push('\n');
        }
        for line in proposed.lines() {
            diff.push_str("+");
            diff.push_str(line);
            diff.push('\n');
        }
        Ok(diff)
    }

    /// Apply caller-reviewed merged content. The prior file is preserved as
    /// `<path>.sigmaos-before-pacnew`; an existing backup is never overwritten.
    pub fn merge_pacnew(&mut self, path: &str, merged_contents: &str) -> std::io::Result<()> {
        use std::io::Write;

        let original = std::path::Path::new(path);
        let pacnew = std::path::PathBuf::from(std::format!("{}.pacnew", path));
        let candidate_metadata = std::fs::symlink_metadata(&pacnew)?;
        if !candidate_metadata.is_file() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "pacnew candidate must be a regular file",
            ));
        }

        let original_metadata = match std::fs::symlink_metadata(original) {
            Ok(metadata) if metadata.is_file() => Some(metadata),
            Ok(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "current configuration must be a regular file",
                ));
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
            Err(error) => return Err(error),
        };

        let backup = std::path::PathBuf::from(std::format!("{}.sigmaos-before-pacnew", path));
        if original_metadata.is_some() {
            std::fs::hard_link(original, &backup)?;
        }

        let parent = original
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."));
        let file_name = original.file_name().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "invalid configuration path",
            )
        })?;
        let temporary = parent.join(std::format!(
            ".{}.sigmaos-pacnew-{}.tmp",
            file_name.to_string_lossy(),
            std::process::id()
        ));
        let mut replacement_applied = false;
        let write_result = (|| {
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&temporary)?;
            file.write_all(merged_contents.as_bytes())?;
            file.sync_all()?;
            let permissions = original_metadata
                .as_ref()
                .map(|metadata| metadata.permissions())
                .unwrap_or_else(|| candidate_metadata.permissions());
            std::fs::set_permissions(&temporary, permissions)?;
            std::fs::rename(&temporary, original)?;
            replacement_applied = true;
            std::fs::remove_file(&pacnew)?;
            Ok(())
        })();
        if write_result.is_err() {
            let _ = std::fs::remove_file(&temporary);
            if !replacement_applied && original_metadata.is_some() {
                let _ = std::fs::remove_file(&backup);
            }
            return write_result;
        }

        self.known_files.retain(|f| f.original_path != path);
        Ok(())
    }

    /// Discard the `.pacnew` file, keeping the installed config unchanged
    pub fn discard_pacnew(&mut self, path: &str) -> std::io::Result<()> {
        let pacnew = std::path::PathBuf::from(std::format!("{}.pacnew", path));
        let metadata = std::fs::symlink_metadata(&pacnew)?;
        if !metadata.is_file() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "pacnew candidate must be a regular file",
            ));
        }
        std::fs::remove_file(pacnew)?;
        self.known_files.retain(|f| f.original_path != path);
        Ok(())
    }
}

impl Default for PacnewHandler {
    fn default() -> Self {
        Self::new()
    }
}

// Extend RollingReleaseManager with the new Arch-inspired methods
impl RollingReleaseManager {
    /// Return a canned list of pending package updates (stub / mock data)
    pub fn check_updates(&self) -> Vec<PackageUpdate> {
        // Real impl: query remote mirror for package db and diff against installed
        vec![PackageUpdate {
            package_name: SigmaString::from("sigma-core"),
            old_version: Version {
                major: 1,
                minor: 0,
                patch: 0,
            },
            new_version: Version {
                major: 1,
                minor: 1,
                patch: 0,
            },
            update_type: UpdateType::Minor,
            security_critical: false,
        }]
    }

    /// Apply all pending updates (delegates to apply_update per entry)
    pub fn apply_updates(&mut self) {
        let pending = self.check_updates();
        for update in pending {
            let _ = self.apply_update(update);
        }
    }

    /// Roll back the most recently applied update
    pub fn rollback_last(&mut self) -> Result<(), &'static str> {
        if let Some(last) = self.update_history.last().cloned() {
            self.package_versions
                .insert(last.package_name.clone(), last.old_version);
            self.update_history.pop();
            Ok(())
        } else {
            Err("No updates in history to roll back")
        }
    }
}

#[cfg(test)]
mod sigma_rolling_tests {
    use super::*;

    fn make_mirror(url: &str, protocol: MirrorProtocol, speed: f64, active: bool) -> Mirror {
        Mirror {
            url: url.to_string(),
            protocol,
            speed_mbps: speed,
            score: speed / 100.0,
            active,
        }
    }

    // --- AAA: mirror ranking by speed ---
    #[test]
    fn test_reflector_rank_by_speed() {
        // Arrange
        let mut ml = MirrorList::new("Global", 1_700_000_000);
        ml.add_mirror(make_mirror(
            "https://slow.sigma.org",
            MirrorProtocol::Https,
            10.0,
            true,
        ));
        ml.add_mirror(make_mirror(
            "https://fast.sigma.org",
            MirrorProtocol::Https,
            500.0,
            true,
        ));
        ml.add_mirror(make_mirror(
            "https://mid.sigma.org",
            MirrorProtocol::Https,
            150.0,
            true,
        ));
        let mut reflector = Reflector::new(ml);

        // Act
        reflector.rank_by_speed();

        // Assert
        assert_eq!(
            reflector.mirror_list.mirrors[0].url,
            "https://fast.sigma.org"
        );
        assert_eq!(
            reflector.mirror_list.mirrors[2].url,
            "https://slow.sigma.org"
        );
    }

    // --- AAA: HTTPS-only filter ---
    #[test]
    fn test_reflector_https_only_filter() {
        // Arrange
        let mut ml = MirrorList::new("Global", 1_700_000_000);
        ml.add_mirror(make_mirror(
            "http://insecure.sigma.org",
            MirrorProtocol::Http,
            200.0,
            true,
        ));
        ml.add_mirror(make_mirror(
            "https://secure.sigma.org",
            MirrorProtocol::Https,
            200.0,
            true,
        ));
        ml.add_mirror(make_mirror(
            "rsync://sync.sigma.org",
            MirrorProtocol::Rsync,
            300.0,
            true,
        ));
        let mut reflector = Reflector::new(ml);

        // Act
        reflector.filter_by_https_only();

        // Assert
        assert_eq!(reflector.mirror_list.mirrors.len(), 1);
        assert_eq!(
            reflector.mirror_list.mirrors[0].url,
            "https://secure.sigma.org"
        );
    }

    // --- AAA: pacnew discovery, preview, and backed-up manual merge ---
    #[test]
    fn test_pacnew_scan_preview_and_merge() {
        let root =
            std::env::temp_dir().join(std::format!("sigmaos-pacnew-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("nested")).unwrap();
        let current = root.join("nested/service.conf");
        let candidate = std::path::PathBuf::from(std::format!("{}.pacnew", current.display()));
        let backup =
            std::path::PathBuf::from(std::format!("{}.sigmaos-before-pacnew", current.display()));
        std::fs::write(&current, "old setting\n").unwrap();
        std::fs::write(&candidate, "new default\n").unwrap();

        let mut handler = PacnewHandler::new();
        let files = handler.scan_directory(&root).unwrap();
        assert_eq!(files.len(), 1);
        assert!(handler
            .diff_pacnew(current.to_str().unwrap())
            .unwrap()
            .contains("+new default"));

        handler
            .merge_pacnew(current.to_str().unwrap(), "merged setting\n")
            .unwrap();
        assert_eq!(
            std::fs::read_to_string(&current).unwrap(),
            "merged setting\n"
        );
        assert_eq!(std::fs::read_to_string(&backup).unwrap(), "old setting\n");
        assert!(!candidate.exists());
        assert!(handler.known_files.is_empty());
        std::fs::remove_dir_all(root).unwrap();
    }

    // --- AAA: update channel switching ---
    #[test]
    fn test_update_channel_switching() {
        // Arrange
        let mut mgr = RollingReleaseManager::default();

        // Act
        mgr.set_update_channel(UpdateChannel::Testing);

        // Assert
        assert_eq!(mgr.update_channel, UpdateChannel::Testing);

        // Switch back
        mgr.set_update_channel(UpdateChannel::Stable);
        assert_eq!(mgr.update_channel, UpdateChannel::Stable);
    }

    // --- AAA: rollback_last ---
    #[test]
    fn test_rollback_last_update() {
        // Arrange
        let mut mgr = RollingReleaseManager::default();
        mgr.register_package(
            SigmaString::from("sigma-core"),
            Version {
                major: 1,
                minor: 0,
                patch: 0,
            },
        );

        // Act: apply updates then rollback
        mgr.apply_updates();
        let rollback_result = mgr.rollback_last();

        // Assert
        assert!(rollback_result.is_ok());
        let ver = mgr
            .package_versions
            .get(&SigmaString::from("sigma-core"))
            .copied()
            .unwrap_or(Version {
                major: 0,
                minor: 0,
                patch: 0,
            });
        assert_eq!(ver.minor, 0); // rolled back to 1.0.0
    }
}
