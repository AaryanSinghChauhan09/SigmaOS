// SPDX-License-Identifier: MIT
// SigmaOS Pacman Contrib Utility Parity Suite
// Native Rust implementation of pacman-contrib utilities:
// Maccache, pacdiff, checkupdates, paclist, updpkgsums, paclog,
// pacfiles, paccheck, pacreport, pkgfile, and pacman -Qi/-Si formatting

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec;
use std::vec::Vec;

use crate::sigpkg::arch_pacman_engine::ArchPacmanPackage;

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

    /// Plan cache pruning while retaining the newest `keep_versions` for each
    /// selected package. With `remove_uninstalled`, only packages with no
    /// installed cache entry are selected. Unparseable versions are retained.
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
            let any_installed = entries.iter().any(|e| e.is_installed);
            if remove_uninstalled && any_installed {
                remaining_count += entries.len();
                continue;
            }

            let mut versioned: Vec<&PackageCacheEntry> = Vec::new();
            let mut unclassified = 0;
            for entry in entries {
                if CheckUpdatesEngine::compare_versions(&entry.version, &entry.version).is_some() {
                    versioned.push(entry);
                } else {
                    // Do not destroy a cache file whose version cannot be ordered.
                    unclassified += 1;
                }
            }

            versioned.sort_by(|left, right| {
                CheckUpdatesEngine::compare_versions(&right.version, &left.version)
                    .unwrap_or(Ordering::Equal)
                    .then_with(|| left.file_name.cmp(&right.file_name))
            });

            let retained_versions = keep_versions.min(versioned.len());
            for entry in versioned.into_iter().skip(retained_versions) {
                removed_files.push(entry.file_name.clone());
                bytes_freed = bytes_freed.saturating_add(entry.size_bytes);
            }
            remaining_count += unclassified + retained_versions;
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
                (Some(cur), Some(nw)) => {
                    // Line conflict: keep non-commented or new line preference
                    if cur.trim().starts_with('#') && !nw.trim().starts_with('#') {
                        merged.push(nw.to_string());
                    } else {
                        merged.push(cur.to_string());
                    }
                }
                (Some(cur), None) => merged.push(cur.to_string()),
                (None, Some(nw)) => merged.push(nw.to_string()),
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

    /// Compare ALPM-style package versions, including epochs and pkgrel when
    /// both sides specify one. Invalid version strings return `None`.
    pub fn compare_versions(left: &str, right: &str) -> Option<Ordering> {
        fn split_version(version: &str) -> Option<(u64, &str, Option<&str>)> {
            let (epoch, body) = match version.split_once(':') {
                Some((epoch, body)) => (epoch.parse::<u64>().ok()?, body),
                None => (0, version),
            };
            if body.is_empty() {
                return None;
            }
            let (pkgver, pkgrel) = match body.split_once('-') {
                Some((pkgver, pkgrel)) if !pkgver.is_empty() && !pkgrel.is_empty() => {
                    (pkgver, Some(pkgrel))
                }
                Some(_) => return None,
                None => (body, None),
            };
            Some((epoch, pkgver, pkgrel))
        }

        fn next_segment<'a>(value: &'a [u8], cursor: &mut usize) -> Option<(&'a [u8], bool)> {
            while *cursor < value.len() && !value[*cursor].is_ascii_alphanumeric() {
                *cursor += 1;
            }
            if *cursor == value.len() {
                return None;
            }

            let start = *cursor;
            let numeric = value[start].is_ascii_digit();
            while *cursor < value.len()
                && value[*cursor].is_ascii_alphanumeric()
                && value[*cursor].is_ascii_digit() == numeric
            {
                *cursor += 1;
            }
            Some((&value[start..*cursor], numeric))
        }

        fn compare_part(left: &str, right: &str) -> Ordering {
            let left = left.as_bytes();
            let right = right.as_bytes();
            let (mut li, mut ri) = (0, 0);
            loop {
                let lhs = next_segment(left, &mut li);
                let rhs = next_segment(right, &mut ri);
                match (lhs, rhs) {
                    (None, None) => return Ordering::Equal,
                    (Some((_, true)), None) | (None, Some((_, false))) => return Ordering::Greater,
                    (Some((_, false)), None) | (None, Some((_, true))) => return Ordering::Less,
                    (Some((lhs, lhs_numeric)), Some((rhs, rhs_numeric))) => {
                        match (lhs_numeric, rhs_numeric) {
                            (true, true) => {
                                let lhs = lhs
                                    .iter()
                                    .position(|byte| *byte != b'0')
                                    .map(|offset| &lhs[offset..])
                                    .unwrap_or(&lhs[lhs.len()..]);
                                let rhs = rhs
                                    .iter()
                                    .position(|byte| *byte != b'0')
                                    .map(|offset| &rhs[offset..])
                                    .unwrap_or(&rhs[rhs.len()..]);
                                let order = lhs.len().cmp(&rhs.len()).then_with(|| lhs.cmp(rhs));
                                if order != Ordering::Equal {
                                    return order;
                                }
                            }
                            (false, false) => {
                                let order = lhs.cmp(rhs);
                                if order != Ordering::Equal {
                                    return order;
                                }
                            }
                            (true, false) => return Ordering::Greater,
                            (false, true) => return Ordering::Less,
                        }
                    }
                }
            }
        }

        let (left_epoch, left_pkgver, left_pkgrel) = split_version(left)?;
        let (right_epoch, right_pkgver, right_pkgrel) = split_version(right)?;
        let epoch_order = left_epoch.cmp(&right_epoch);
        if epoch_order != Ordering::Equal {
            return Some(epoch_order);
        }

        let pkgver_order = compare_part(left_pkgver, right_pkgver);
        if pkgver_order != Ordering::Equal {
            return Some(pkgver_order);
        }

        // libalpm ignores pkgrel if either version is in minimal form.
        Some(match (left_pkgrel, right_pkgrel) {
            (Some(left), Some(right)) => compare_part(left, right),
            _ => Ordering::Equal,
        })
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
                if Self::compare_versions(&sync.sync_version, &inst.current_version)
                    == Some(Ordering::Greater)
                {
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

    /// Computes the lowercase SHA-256 digest used by PKGBUILD sha256sums.
    pub fn compute_sha256(data: &[u8]) -> String {
        let digest = crate::crypto::primitives::sha256_hash(data);
        let mut hex = String::with_capacity(64);
        for byte in digest.data {
            hex.push_str(&format!("{:02x}", byte));
        }
        hex
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
// 7. PacFilesDb (pacfiles / pacman -Ql / pacman -Qo parity)
// ============================================================================

/// A single file owned by an installed package (`pacman -Ql` entry)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedPackageFile {
    pub path: String,
    pub size_bytes: u64,
    pub is_dir: bool,
}

/// Installed package file database (`pacfiles` and `pacman -Qo` parity)
#[derive(Debug, Default)]
pub struct PacFilesDb {
    /// package name -> files it owns
    pub packages: BTreeMap<String, Vec<OwnedPackageFile>>,
}

impl PacFilesDb {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register the file list of an installed package
    pub fn register_package(&mut self, package_name: &str, files: Vec<OwnedPackageFile>) {
        self.packages.insert(package_name.to_string(), files);
    }

    /// List files owned by a package (`pacman -Ql <pkg>`)
    pub fn files_of(&self, package_name: &str) -> Vec<&OwnedPackageFile> {
        self.packages
            .get(package_name)
            .map(|files| files.iter().collect())
            .unwrap_or_default()
    }

    /// Find which installed package owns a path (`pacman -Qo <path>`).
    /// An exact file match wins; otherwise the longest matching directory
    /// prefix is used, mirroring how pacman resolves ownership of paths
    /// beneath a package-owned directory.
    pub fn owner_of(&self, path: &str) -> Option<String> {
        let normalized = path.trim_end_matches('/');
        if normalized.is_empty() {
            return None;
        }

        for (pkg, files) in &self.packages {
            if files.iter().any(|f| !f.is_dir && f.path == normalized) {
                return Some(pkg.clone());
            }
        }

        let mut best: Option<(&str, usize)> = None;
        for (pkg, files) in &self.packages {
            for file in files {
                if !file.is_dir {
                    continue;
                }
                let dir = file.path.trim_end_matches('/');
                let matches = normalized == dir || normalized.starts_with(&format!("{}/", dir));
                if matches && best.map_or(true, |(_, len)| dir.len() > len) {
                    best = Some((pkg.as_str(), dir.len()));
                }
            }
        }
        best.map(|(pkg, _)| pkg.to_string())
    }

    /// Total tracked file count across all registered packages
    pub fn total_files(&self) -> usize {
        self.packages.values().map(Vec::len).sum()
    }
}

// ============================================================================
// 8. PacCheckVerifier (pacman -Qk / paccheck parity)
// ============================================================================

/// A file entry recorded in an installed package database with its expected
/// digest and size, mirroring `.MTREE` metadata used by `pacman -Qk`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrackedPackageFile {
    pub path: String,
    pub sha256: Option<String>,
    pub size_bytes: Option<u64>,
}

/// Result of verifying an installed package against its recorded digests
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PackageVerificationReport {
    pub package_name: String,
    pub checked_files: usize,
    pub ok_files: usize,
    pub missing_files: Vec<String>,
    pub modified_files: Vec<String>,
}

impl PackageVerificationReport {
    /// True when every tracked file is present with matching metadata
    pub fn is_clean(&self) -> bool {
        self.missing_files.is_empty() && self.modified_files.is_empty()
    }
}

/// Installed package integrity verifier (`paccheck` / `pacman -Qk` parity)
pub struct PacCheckVerifier;

impl PacCheckVerifier {
    pub fn new() -> Self {
        Self
    }

    /// Verifies a package's tracked files against on-disk contents.
    /// `disk` maps absolute paths to file bytes; a missing key means the file
    /// is absent. When both size and digest metadata are recorded, any
    /// mismatch marks the file as modified.
    pub fn verify(
        &self,
        package_name: &str,
        tracked: &[TrackedPackageFile],
        disk: &BTreeMap<String, Vec<u8>>,
    ) -> PackageVerificationReport {
        let mut report = PackageVerificationReport {
            package_name: package_name.to_string(),
            ..PackageVerificationReport::default()
        };

        for file in tracked {
            report.checked_files += 1;
            match disk.get(&file.path) {
                None => report.missing_files.push(file.path.clone()),
                Some(contents) => {
                    let size_mismatch = file
                        .size_bytes
                        .map_or(false, |expected| expected != contents.len() as u64);
                    let digest_mismatch = file.sha256.as_ref().map_or(false, |expected| {
                        UpdPkgSumsGenerator::compute_sha256(contents) != *expected
                    });
                    if size_mismatch || digest_mismatch {
                        report.modified_files.push(file.path.clone());
                    } else {
                        report.ok_files += 1;
                    }
                }
            }
        }

        report.missing_files.sort();
        report.modified_files.sort();
        report
    }
}

impl Default for PacCheckVerifier {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 9. PacReportAnalyzer (pacreport parity)
// ============================================================================

/// Analysis of the local package database (`pacreport` parity)
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PacReport {
    /// (package, optdep) pairs where the optdep is installed but not required
    /// by any installed package's hard dependencies
    pub unrequired_optdepends: Vec<(String, String)>,
    /// installed, non-explicit packages that nothing depends on (orphans)
    pub unrequired_dependencies: Vec<String>,
    /// (package name, installed size) sorted by size descending
    pub disk_usage: Vec<(String, u64)>,
    /// sum of all installed package sizes in bytes
    pub total_installed_size: u64,
}

/// Strips version constraints (`>=`, `<=`, `=`, `<`, `>`) from a dependency spec
fn dep_spec_base_name(spec: &str) -> &str {
    spec.split(|c| matches!(c, '<' | '>' | '='))
        .next()
        .unwrap_or(spec)
        .trim()
}

/// Strips the `: description` suffix and version constraint from an optdepend spec
fn optdep_spec_name(spec: &str) -> &str {
    dep_spec_base_name(spec.split(':').next().unwrap_or(spec))
}

/// Local database reporter (`pacreport` parity)
pub struct PacReportAnalyzer;

impl PacReportAnalyzer {
    pub fn new() -> Self {
        Self
    }

    /// Analyzes the installed package set for unrequired optdepends,
    /// orphaned dependencies, and per-package disk usage.
    pub fn analyze(&self, installed: &[ArchPacmanPackage]) -> PacReport {
        let installed_names: Vec<&str> = installed.iter().map(|p| p.name.as_str()).collect();

        let mut required: Vec<String> = Vec::new();
        for pkg in installed {
            for dep in &pkg.depends {
                let base = dep_spec_base_name(dep);
                if !base.is_empty() && !required.iter().any(|r| r == base) {
                    required.push(base.to_string());
                }
            }
        }

        let mut unrequired_optdepends = Vec::new();
        let mut unrequired_dependencies = Vec::new();
        for pkg in installed {
            for optdep in &pkg.optdepends {
                let name = optdep_spec_name(optdep);
                if !name.is_empty()
                    && installed_names.contains(&name)
                    && !required.iter().any(|r| r.as_str() == name)
                {
                    unrequired_optdepends.push((pkg.name.clone(), name.to_string()));
                }
            }
            if !pkg.is_explicit && !required.iter().any(|r| r.as_str() == pkg.name.as_str()) {
                unrequired_dependencies.push(pkg.name.clone());
            }
        }

        let mut disk_usage: Vec<(String, u64)> = installed
            .iter()
            .map(|p| (p.name.clone(), p.installed_size))
            .collect();
        disk_usage.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

        unrequired_optdepends.sort();
        unrequired_optdepends.dedup();
        unrequired_dependencies.sort();

        PacReport {
            unrequired_optdepends,
            unrequired_dependencies,
            total_installed_size: installed.iter().map(|p| p.installed_size).sum(),
            disk_usage,
        }
    }
}

impl Default for PacReportAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 10. PkgFileSearcher (pkgfile / pacman -F parity)
// ============================================================================

/// A file entry in a sync repository database (`pkgfile` / `pacman -F`)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncFileEntry {
    pub package_name: String,
    pub repository: String,
    pub file_path: String,
}

/// Searches sync databases for which package provides a file
/// (`pkgfile` and `pacman -F` parity)
#[derive(Debug, Default)]
pub struct PkgFileSearcher {
    pub entries: Vec<SyncFileEntry>,
}

impl PkgFileSearcher {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a package's file list from a sync database
    pub fn register_package_files(&mut self, package_name: &str, repository: &str, files: &[&str]) {
        for file in files {
            self.entries.push(SyncFileEntry {
                package_name: package_name.to_string(),
                repository: repository.to_string(),
                file_path: (*file).to_string(),
            });
        }
    }

    /// Exact path lookup (`pacman -F /usr/bin/foo`)
    pub fn find_exact(&self, path: &str) -> Vec<&SyncFileEntry> {
        self.entries
            .iter()
            .filter(|e| e.file_path == path)
            .collect()
    }

    /// Glob lookup supporting `*` (any sequence) and `?` (single character),
    /// matching `pkgfile` pattern semantics (`pkgfile '*gcc*'`)
    pub fn find_glob(&self, pattern: &str) -> Vec<&SyncFileEntry> {
        self.entries
            .iter()
            .filter(|e| glob_match(pattern, &e.file_path))
            .collect()
    }
}

/// Minimal glob matcher supporting `*` and `?` wildcards
fn glob_match(pattern: &str, text: &str) -> bool {
    fn inner(p: &[u8], t: &[u8]) -> bool {
        match p.first() {
            None => t.is_empty(),
            Some(b'*') => {
                // collapse consecutive stars
                let mut rest = &p[1..];
                while rest.first() == Some(&b'*') {
                    rest = &rest[1..];
                }
                (0..=t.len()).any(|skip| inner(rest, &t[skip..]))
            }
            Some(b'?') => !t.is_empty() && inner(&p[1..], &t[1..]),
            Some(&c) => t.first() == Some(&c) && inner(&p[1..], &t[1..]),
        }
    }
    inner(pattern.as_bytes(), text.as_bytes())
}

// ============================================================================
// 11. PacmanQueryFormatter (pacman -Qi / pacman -Si parity)
// ============================================================================

/// Formats package database records the way `pacman -Qi` / `pacman -Si` do
pub struct PacmanQueryFormatter;

impl PacmanQueryFormatter {
    pub fn new() -> Self {
        Self
    }

    fn join_or_none(items: &[String]) -> String {
        if items.is_empty() {
            "None".to_string()
        } else {
            items.join(", ")
        }
    }

    fn line(label: &str, value: &str) -> String {
        format!("{:<16}: {}", label, value)
    }

    /// `pacman -Qi` parity: installed package record
    pub fn format_installed(&self, pkg: &ArchPacmanPackage) -> String {
        let mut out = String::new();
        out.push_str(&Self::line("Name", &pkg.name));
        out.push('\n');
        out.push_str(&Self::line("Version", &pkg.version));
        out.push('\n');
        out.push_str(&Self::line("Description", &pkg.description));
        out.push('\n');
        out.push_str(&Self::line("Architecture", &pkg.architecture));
        out.push('\n');
        out.push_str(&Self::line("URL", &pkg.url));
        out.push('\n');
        out.push_str(&Self::line("Licenses", &Self::join_or_none(&pkg.license)));
        out.push('\n');
        out.push_str(&Self::line("Groups", &Self::join_or_none(&pkg.groups)));
        out.push('\n');
        out.push_str(&Self::line("Provides", &Self::join_or_none(&pkg.provides)));
        out.push('\n');
        out.push_str(&Self::line("Depends On", &Self::join_or_none(&pkg.depends)));
        out.push('\n');
        out.push_str(&Self::line(
            "Optional Deps",
            &Self::join_or_none(&pkg.optdepends),
        ));
        out.push('\n');
        out.push_str(&Self::line("Required By", "None"));
        out.push('\n');
        out.push_str(&Self::line("Optional For", "None"));
        out.push('\n');
        out.push_str(&Self::line(
            "Conflicts With",
            &Self::join_or_none(&pkg.conflicts),
        ));
        out.push('\n');
        out.push_str(&Self::line("Replaces", &Self::join_or_none(&pkg.replaces)));
        out.push('\n');
        out.push_str(&Self::line(
            "Installed Size",
            &format!("{} KiB", pkg.installed_size / 1024),
        ));
        out.push('\n');
        out.push_str(&Self::line("Packager", &pkg.packager));
        out.push('\n');
        out.push_str(&Self::line("Build Date", &pkg.build_date));
        out.push('\n');
        out.push_str(&Self::line("Install Date", &pkg.install_date));
        out.push('\n');
        out
    }

    /// `pacman -Si` parity: sync database record including repository
    pub fn format_sync(&self, pkg: &ArchPacmanPackage, repository: &str) -> String {
        let mut out = String::new();
        out.push_str(&Self::line("Repository", repository));
        out.push('\n');
        out.push_str(&Self::line("Name", &pkg.name));
        out.push('\n');
        out.push_str(&Self::line("Version", &pkg.version));
        out.push('\n');
        out.push_str(&Self::line("Description", &pkg.description));
        out.push('\n');
        out.push_str(&Self::line("Architecture", &pkg.architecture));
        out.push('\n');
        out.push_str(&Self::line("URL", &pkg.url));
        out.push('\n');
        out.push_str(&Self::line("Licenses", &Self::join_or_none(&pkg.license)));
        out.push('\n');
        out.push_str(&Self::line("Groups", &Self::join_or_none(&pkg.groups)));
        out.push('\n');
        out.push_str(&Self::line("Provides", &Self::join_or_none(&pkg.provides)));
        out.push('\n');
        out.push_str(&Self::line("Depends On", &Self::join_or_none(&pkg.depends)));
        out.push('\n');
        out.push_str(&Self::line(
            "Optional Deps",
            &Self::join_or_none(&pkg.optdepends),
        ));
        out.push('\n');
        out.push_str(&Self::line(
            "Conflicts With",
            &Self::join_or_none(&pkg.conflicts),
        ));
        out.push('\n');
        out.push_str(&Self::line("Replaces", &Self::join_or_none(&pkg.replaces)));
        out.push('\n');
        out.push_str(&Self::line(
            "Download Size",
            &format!("{} KiB", pkg.installed_size / 1024),
        ));
        out.push('\n');
        out.push_str(&Self::line("Installed Size", "Unknown"));
        out.push('\n');
        out.push_str(&Self::line("Packager", &pkg.packager));
        out.push('\n');
        out.push_str(&Self::line("Build Date", &pkg.build_date));
        out.push('\n');
        out
    }
}

impl Default for PacmanQueryFormatter {
    fn default() -> Self {
        Self::new()
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
            version: "6.6.3".to_string(),
            file_name: "linux-6.6.3-x86_64.pkg.tar.zst".to_string(),
            size_bytes: 110_000_000,
            is_installed: true,
        });
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

    fn sample_pkg(name: &str) -> ArchPacmanPackage {
        ArchPacmanPackage {
            name: name.to_string(),
            version: "1.0.0-1".to_string(),
            description: format!("{} package", name),
            url: String::new(),
            architecture: "x86_64".to_string(),
            license: vec!["MIT".to_string()],
            groups: Vec::new(),
            depends: Vec::new(),
            optdepends: Vec::new(),
            makedepends: Vec::new(),
            checkdepends: Vec::new(),
            provides: Vec::new(),
            conflicts: Vec::new(),
            replaces: Vec::new(),
            backup: Vec::new(),
            installed_size: 1024,
            packager: "SigmaOS".to_string(),
            build_date: "2026-01-01".to_string(),
            install_date: "2026-01-02".to_string(),
            is_explicit: true,
        }
    }

    #[test]
    fn test_pacfiles_db() {
        let mut db = PacFilesDb::new();
        db.register_package(
            "bash",
            vec![
                OwnedPackageFile {
                    path: "/usr/bin".to_string(),
                    size_bytes: 0,
                    is_dir: true,
                },
                OwnedPackageFile {
                    path: "/usr/bin/bash".to_string(),
                    size_bytes: 1_234_567,
                    is_dir: false,
                },
            ],
        );
        db.register_package(
            "coreutils",
            vec![OwnedPackageFile {
                path: "/usr/bin/ls".to_string(),
                size_bytes: 100,
                is_dir: false,
            }],
        );

        assert_eq!(db.files_of("bash").len(), 2);
        assert_eq!(db.owner_of("/usr/bin/ls"), Some("coreutils".to_string()));
        assert_eq!(db.owner_of("/usr/bin/bash"), Some("bash".to_string()));
        // directory-prefix ownership like pacman -Qo
        assert_eq!(
            db.owner_of("/usr/bin/bash/subshell"),
            Some("bash".to_string())
        );
        assert_eq!(db.owner_of("/etc/hostname"), None);
        assert_eq!(db.total_files(), 3);
    }

    #[test]
    fn test_paccheck_verifier() {
        let verifier = PacCheckVerifier::new();
        let contents = b"#!/bin/sh\necho hi\n";
        let sha = UpdPkgSumsGenerator::compute_sha256(contents);
        let tracked = vec![
            TrackedPackageFile {
                path: "/usr/bin/bash".to_string(),
                sha256: Some(sha),
                size_bytes: Some(contents.len() as u64),
            },
            TrackedPackageFile {
                path: "/usr/share/bash/README".to_string(),
                sha256: None,
                size_bytes: None,
            },
            TrackedPackageFile {
                path: "/usr/bin/bash.bak".to_string(),
                sha256: Some(UpdPkgSumsGenerator::compute_sha256(b"other")),
                size_bytes: None,
            },
        ];

        let mut disk = BTreeMap::new();
        disk.insert("/usr/bin/bash".to_string(), contents.to_vec());
        disk.insert("/usr/bin/bash.bak".to_string(), contents.to_vec());

        let report = verifier.verify("bash", &tracked, &disk);
        assert_eq!(report.checked_files, 3);
        assert_eq!(report.ok_files, 1);
        assert_eq!(
            report.missing_files,
            vec!["/usr/share/bash/README".to_string()]
        );
        assert_eq!(report.modified_files, vec!["/usr/bin/bash.bak".to_string()]);
        assert!(!report.is_clean());

        disk.insert("/usr/share/bash/README".to_string(), b"readme".to_vec());
        let clean = verifier.verify("bash", &tracked[..1], &disk);
        assert!(clean.is_clean());
    }

    #[test]
    fn test_pacreport_analyzer() {
        let analyzer = PacReportAnalyzer::new();
        let mut shell = sample_pkg("zsh");
        shell.depends = vec!["bash>=5.0".to_string()];
        let mut bash = sample_pkg("bash");
        bash.is_explicit = true;
        let mut git = sample_pkg("git");
        git.optdepends = vec!["zsh-completions: zsh completion".to_string()];
        git.installed_size = 2048;
        let mut helper = sample_pkg("helper");
        helper.is_explicit = false; // orphan: nothing depends on it
        let mut compl = sample_pkg("zsh-completions");
        compl.is_explicit = true; // installed optdep, not required by depends

        let report = analyzer.analyze(&[shell, bash, git, helper, compl]);
        assert_eq!(
            report.unrequired_optdepends,
            vec![("git".to_string(), "zsh-completions".to_string())]
        );
        assert_eq!(report.unrequired_dependencies, vec!["helper".to_string()]);
        assert_eq!(report.disk_usage[0], ("git".to_string(), 2048));
        assert_eq!(report.total_installed_size, 5 * 1024 + 1024);
    }

    #[test]
    fn test_pkgfile_searcher() {
        let mut searcher = PkgFileSearcher::new();
        searcher.register_package_files("gcc", "core", &["/usr/bin/gcc", "/usr/lib/libgcc.so"]);
        searcher.register_package_files("clang", "extra", &["/usr/bin/clang"]);

        assert_eq!(searcher.find_exact("/usr/bin/gcc").len(), 1);
        assert_eq!(searcher.find_exact("/usr/bin/missing").len(), 0);

        let glob = searcher.find_glob("/usr/bin/*cc");
        assert_eq!(glob.len(), 1);
        let qmark = searcher.find_glob("/usr/bin/gcc?");
        assert!(qmark.is_empty());
        let qmark2 = searcher.find_glob("/usr/bin/g?c");
        assert_eq!(qmark2.len(), 1);
    }

    #[test]
    fn test_pacman_query_formatter() {
        let formatter = PacmanQueryFormatter::new();
        let mut pkg = sample_pkg("bash");
        pkg.depends = vec!["readline".to_string(), "ncurses>=6.0".to_string()];
        pkg.optdepends = vec!["bash-completion: completions".to_string()];

        let installed = formatter.format_installed(&pkg);
        assert!(installed.contains("Name            : bash"));
        assert!(installed.contains("Depends On      : readline, ncurses>=6.0"));
        assert!(installed.contains("Optional Deps   : bash-completion: completions"));
        assert!(installed.contains("Installed Size  : 1 KiB"));

        let sync = formatter.format_sync(&pkg, "core");
        assert!(sync.contains("Repository      : core"));
        assert!(sync.contains("Download Size   : 1 KiB"));
    }
}
