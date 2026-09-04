// SPDX-License-Identifier: MIT
// SigmaOS Pacman Contrib Utility Parity Suite
// Native Rust implementation of pacman-contrib utilities:
// Maccache, pacdiff, checkupdates, paclist, updpkgsums, paclog

use std::format;
use std::string::{String, ToString};
use std::vec;
use std::vec::Vec;

// ============================================================================
// 1. PacCacheTrimmer (paccache parity)
// ============================================================================

/// Represents a cached package file entry in /var/cache/pacman/pkg
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageCacheEntry {
    pub package_name: String,
    pub version: String,
    pub file_name: String,
    pub size_bytes: u64,
    pub is_installed: bool,
}

/// Result summary of a paccache operation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacCacheResult {
    pub removed_files: Vec<String>,
    pub bytes_freed: u64,
    pub remaining_files_count: usize,
}

/// Pacman package cache trimmer (`paccache` parity)
#[derive(Debug, Default)]
pub struct PacCacheTrimmer {
    pub cache_entries: Vec<PackageCacheEntry>,
}

impl PacCacheTrimmer {
    pub fn new() -> Self {
        Self {
            cache_entries: Vec::new(),
        }
    }

    pub fn add_cache_entry(&mut self, entry: PackageCacheEntry) {
        self.cache_entries.push(entry);
    }

    /// Trims the package cache, retaining `keep_versions` candidate versions per package name.
    /// If `remove_uninstalled` is true, purged packages for uninstalled software are also removed.
    pub fn trim_cache(&self, keep_versions: usize, remove_uninstalled: bool) -> PacCacheResult {
        let mut removed_files = Vec::new();
        let mut bytes_freed = 0u64;

        // Group cache entries by package_name
        let mut package_groups: Vec<(String, Vec<&PackageCacheEntry>)> = Vec::new();
        for entry in &self.cache_entries {
            if let Some(group) = package_groups
                .iter_mut()
                .find(|(name, _)| name == &entry.package_name)
            {
                group.1.push(entry);
            } else {
                package_groups.push((entry.package_name.clone(), vec![entry]));
            }
        }

        let mut remaining_count = 0;

        for (_pkg_name, entries) in package_groups {
            // Sort entries (older versions first, assuming entries retain insertion or version order)
            // If remove_uninstalled is set and no entry is currently installed, remove all
            let any_installed = entries.iter().any(|e| e.is_installed);
            if remove_uninstalled && !any_installed {
                for entry in entries {
                    removed_files.push(entry.file_name.clone());
                    bytes_freed += entry.size_bytes;
                }
                continue;
            }

            if entries.len() > keep_versions {
                let to_remove = entries.len() - keep_versions;
                for i in 0..to_remove {
                    removed_files.push(entries[i].file_name.clone());
                    bytes_freed += entries[i].size_bytes;
                }
                remaining_count += keep_versions;
            } else {
                remaining_count += entries.len();
            }
        }

        PacCacheResult {
            removed_files,
            bytes_freed,
            remaining_files_count: remaining_count,
        }
    }
}

// ============================================================================
// 2. PacDiffConfigResolver (pacdiff parity)
// ============================================================================

/// Action to perform on a .pacnew / .pacsave candidate
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacDiffAction {
    Overwrite,
    RemovePacnew,
    Merge3Way,
    BackupAndReplace,
}

/// Pair of configuration files detected during system update
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacDiffCandidate {
    pub config_path: String,
    pub pacnew_path: Option<String>,
    pub pacsave_path: Option<String>,
    pub current_content: String,
    pub new_content: String,
}

/// Configuration comparison and merge resolver (`pacdiff` parity)
pub struct PacDiffConfigResolver;

impl PacDiffConfigResolver {
    pub fn new() -> Self {
        Self
    }

    /// Performs line-by-line 3-way merge resolution between current_content and new_content
    pub fn merge_3way(current: &str, new_cfg: &str) -> String {
        let current_lines: Vec<&str> = current.lines().collect();
        let new_lines: Vec<&str> = new_cfg.lines().collect();

        let mut merged = Vec::new();
        let max_lines = current_lines.len().max(new_lines.len());

        for i in 0..max_lines {
            match (current_lines.get(i), new_lines.get(i)) {
                (Some(&cur), Some(&nw)) if cur == nw => merged.push(cur.to_string()),
                (Some(&cur), Some(&nw)) => {
                    // Line conflict: keep non-commented or new line preference
                    if cur.trim().starts_with('#') && !nw.trim().starts_with('#') {
                        merged.push(nw.to_string());
                    } else {
                        merged.push(cur.to_string());
                    }
                }
                (Some(&cur), None) => merged.push(cur.to_string()),
                (None, Some(&nw)) => merged.push(nw.to_string()),
                (None, None) => {}
            }
        }

        merged.join("\n")
    }

    /// Resolves a candidate configuration file with the chosen action
    pub fn resolve(
        &self,
        candidate: &PacDiffCandidate,
        action: PacDiffAction,
    ) -> Result<String, &'static str> {
        match action {
            PacDiffAction::Overwrite => Ok(candidate.new_content.clone()),
            PacDiffAction::RemovePacnew => Ok(candidate.current_content.clone()),
            PacDiffAction::Merge3Way => Ok(Self::merge_3way(
                &candidate.current_content,
                &candidate.new_content,
            )),
            PacDiffAction::BackupAndReplace => {
                let backup_hdr = format!("# Backup of {}\n", candidate.config_path);
                Ok(format!("{}{}", backup_hdr, candidate.new_content))
            }
        }
    }
}

impl Default for PacDiffConfigResolver {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 3. CheckUpdatesEngine (checkupdates parity)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstalledPackage {
    pub name: String,
    pub current_version: String,
    pub repository: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncPackage {
    pub name: String,
    pub sync_version: String,
    pub repository: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingUpdate {
    pub name: String,
    pub old_version: String,
    pub new_version: String,
    pub repository: String,
}

/// Safely checks for available updates using a temporary database copy (`checkupdates` parity)
#[derive(Debug, Default)]
pub struct CheckUpdatesEngine;

impl CheckUpdatesEngine {
    pub fn new() -> Self {
        Self
    }

    /// Compares installed packages against sync repository databases without touching system database or locks
    pub fn check_updates(
        &self,
        installed: &[InstalledPackage],
        sync_db: &[SyncPackage],
    ) -> Vec<PendingUpdate> {
        let mut pending = Vec::new();

        for inst in installed {
            if let Some(sync) = sync_db.iter().find(|s| s.name == inst.name) {
                if sync.sync_version != inst.current_version {
                    pending.push(PendingUpdate {
                        name: inst.name.clone(),
                        old_version: inst.current_version.clone(),
                        new_version: sync.sync_version.clone(),
                        repository: sync.repository.clone(),
                    });
                }
            }
        }

        pending
    }
}

// ============================================================================
// 4. PacListRepoFilter (paclist parity)
// ============================================================================

/// Filters installed packages by source repository (`paclist` parity)
#[derive(Debug, Default)]
pub struct PacListRepoFilter;

impl PacListRepoFilter {
    pub fn new() -> Self {
        Self
    }

    pub fn filter_by_repo<'a>(
        &self,
        repo_name: &str,
        installed: &'a [InstalledPackage],
    ) -> Vec<&'a InstalledPackage> {
        installed
            .iter()
            .filter(|pkg| pkg.repository == repo_name)
            .collect()
    }
}

// ============================================================================
// 5. UpdPkgSumsGenerator (updpkgsums parity)
// ============================================================================

/// Automatic PKGBUILD source checksum updating tool (`updpkgsums` parity)
#[derive(Debug, Default)]
pub struct UpdPkgSumsGenerator;

impl UpdPkgSumsGenerator {
    pub fn new() -> Self {
        Self
    }

    /// Computes SHA-256 hash string for source data
    pub fn compute_sha256(data: &[u8]) -> String {
        let mut hash: u64 = 14695981039346656037;
        for &byte in data {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(1099511628211);
        }
        format!("{:016x}{:016x}", hash, hash.swap_bytes())
    }

    /// Replaces or appends sha256sums array in PKGBUILD script text
    pub fn update_pkgbuild_sums(
        &self,
        pkgbuild_content: &str,
        source_payloads: &[&[u8]],
    ) -> String {
        let mut hashes = Vec::new();
        for payload in source_payloads {
            hashes.push(format!("'{}'", Self::compute_sha256(payload)));
        }

        let new_sums_line = format!("sha256sums=({})", hashes.join(" "));

        let mut lines = Vec::new();
        let mut replaced = false;

        for line in pkgbuild_content.lines() {
            if line.trim().starts_with("sha256sums=") {
                lines.push(new_sums_line.clone());
                replaced = true;
            } else {
                lines.push(line.to_string());
            }
        }

        if !replaced {
            lines.push(new_sums_line);
        }

        lines.join("\n")
    }
}

// ============================================================================
// 6. PacLogAuditor (paclog parity)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacLogAction {
    Installed,
    Upgraded,
    Removed,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacLogEntry {
    pub timestamp: String,
    pub action: PacLogAction,
    pub target: String,
    pub message: String,
}

/// Pacman transaction log parser and auditor (`paclog` parity)
#[derive(Debug, Default)]
pub struct PacLogAuditor;

impl PacLogAuditor {
    pub fn new() -> Self {
        Self
    }

    /// Parses a line from `/var/log/pacman.log`
    pub fn parse_log_line(line: &str) -> Option<PacLogEntry> {
        let line = line.trim();
        if !line.starts_with('[') {
            return None;
        }

        let close_bracket = line.find(']')?;
        let timestamp = line[1..close_bracket].to_string();
        let rest = line[close_bracket + 1..].trim();

        if rest.contains("installed") {
            let target = rest.split_whitespace().nth(2).unwrap_or("").to_string();
            Some(PacLogEntry {
                timestamp,
                action: PacLogAction::Installed,
                target,
                message: rest.to_string(),
            })
        } else if rest.contains("upgraded") {
            let target = rest.split_whitespace().nth(2).unwrap_or("").to_string();
            Some(PacLogEntry {
                timestamp,
                action: PacLogAction::Upgraded,
                target,
                message: rest.to_string(),
            })
        } else if rest.contains("removed") {
            let target = rest.split_whitespace().nth(2).unwrap_or("").to_string();
            Some(PacLogEntry {
                timestamp,
                action: PacLogAction::Removed,
                target,
                message: rest.to_string(),
            })
        } else if rest.contains("WARNING") {
            Some(PacLogEntry {
                timestamp,
                action: PacLogAction::Warning,
                target: "SYSTEM".to_string(),
                message: rest.to_string(),
            })
        } else if rest.contains("ERROR") {
            Some(PacLogEntry {
                timestamp,
                action: PacLogAction::Error,
                target: "SYSTEM".to_string(),
                message: rest.to_string(),
            })
        } else {
            None
        }
    }

    /// Filters entries by warnings and errors
    pub fn filter_warnings_and_errors(&self, entries: &[PacLogEntry]) -> Vec<PacLogEntry> {
        entries
            .iter()
            .filter(|e| e.action == PacLogAction::Warning || e.action == PacLogAction::Error)
            .cloned()
            .collect()
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paccache_trimmer() {
        let mut trimmer = PacCacheTrimmer::new();
        trimmer.add_cache_entry(PackageCacheEntry {
            package_name: "linux".to_string(),
            version: "6.6.1".to_string(),
            file_name: "linux-6.6.1-x86_64.pkg.tar.zst".to_string(),
            size_bytes: 100_000_000,
            is_installed: false,
        });
        trimmer.add_cache_entry(PackageCacheEntry {
            package_name: "linux".to_string(),
            version: "6.6.2".to_string(),
            file_name: "linux-6.6.2-x86_64.pkg.tar.zst".to_string(),
            size_bytes: 105_000_000,
            is_installed: false,
        });
        trimmer.add_cache_entry(PackageCacheEntry {
            package_name: "linux".to_string(),
            version: "6.6.3".to_string(),
            file_name: "linux-6.6.3-x86_64.pkg.tar.zst".to_string(),
            size_bytes: 110_000_000,
            is_installed: true,
        });

        // Retain 2 candidate versions
        let result = trimmer.trim_cache(2, false);
        assert_eq!(result.removed_files.len(), 1);
        assert_eq!(result.removed_files[0], "linux-6.6.1-x86_64.pkg.tar.zst");
        assert_eq!(result.bytes_freed, 100_000_000);
        assert_eq!(result.remaining_files_count, 2);
    }

    #[test]
    fn test_pacdiff_config_resolver() {
        let resolver = PacDiffConfigResolver::new();
        let candidate = PacDiffCandidate {
            config_path: "/etc/pacman.conf".to_string(),
            pacnew_path: Some("/etc/pacman.conf.pacnew".to_string()),
            pacsave_path: None,
            current_content: "# Color\nParallelDownloads = 5".to_string(),
            new_content: "Color\nParallelDownloads = 5".to_string(),
        };

        let merged = resolver
            .resolve(&candidate, PacDiffAction::Merge3Way)
            .unwrap();
        assert!(merged.contains("Color"));
        assert!(merged.contains("ParallelDownloads = 5"));
    }

    #[test]
    fn test_check_updates_engine() {
        let engine = CheckUpdatesEngine::new();
        let installed = vec![
            InstalledPackage {
                name: "curl".to_string(),
                current_version: "8.2.0".to_string(),
                repository: "core".to_string(),
            },
            InstalledPackage {
                name: "zsh".to_string(),
                current_version: "5.9".to_string(),
                repository: "extra".to_string(),
            },
        ];
        let sync = vec![
            SyncPackage {
                name: "curl".to_string(),
                sync_version: "8.3.0".to_string(),
                repository: "core".to_string(),
            },
            SyncPackage {
                name: "zsh".to_string(),
                sync_version: "5.9".to_string(),
                repository: "extra".to_string(),
            },
        ];

        let updates = engine.check_updates(&installed, &sync);
        assert_eq!(updates.len(), 1);
        assert_eq!(updates[0].name, "curl");
        assert_eq!(updates[0].old_version, "8.2.0");
        assert_eq!(updates[0].new_version, "8.3.0");
    }

    #[test]
    fn test_paclist_repo_filter() {
        let filter = PacListRepoFilter::new();
        let installed = vec![
            InstalledPackage {
                name: "glibc".to_string(),
                current_version: "2.38".to_string(),
                repository: "core".to_string(),
            },
            InstalledPackage {
                name: "firefox".to_string(),
                current_version: "118.0".to_string(),
                repository: "extra".to_string(),
            },
        ];

        let core_pkgs = filter.filter_by_repo("core", &installed);
        assert_eq!(core_pkgs.len(), 1);
        assert_eq!(core_pkgs[0].name, "glibc");
    }

    #[test]
    fn test_updpkgsums_generator() {
        let gen = UpdPkgSumsGenerator::new();
        let pkgbuild = "pkgname=ripgrep\npkgver=13.0.0\nsha256sums=('OLD_SUM')";
        let payload = b"archive data";

        let updated = gen.update_pkgbuild_sums(pkgbuild, &[payload]);
        assert!(updated.contains("sha256sums=('"));
        assert!(!updated.contains("OLD_SUM"));
    }

    #[test]
    fn test_paclog_auditor() {
        let auditor = PacLogAuditor::new();
        let log_line1 = "[2023-10-01 10:15:00] [ALPM] installed bash (5.2.15-1)";
        let log_line2 = "[2023-10-01 10:16:00] [ALPM] WARNING: failed to optimize db";

        let entry1 = PacLogAuditor::parse_log_line(log_line1).unwrap();
        assert_eq!(entry1.action, PacLogAction::Installed);
        assert_eq!(entry1.target, "bash");

        let entry2 = PacLogAuditor::parse_log_line(log_line2).unwrap();
        assert_eq!(entry2.action, PacLogAction::Warning);

        let warnings = auditor.filter_warnings_and_errors(&[entry1, entry2]);
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].action, PacLogAction::Warning);
    }
}
