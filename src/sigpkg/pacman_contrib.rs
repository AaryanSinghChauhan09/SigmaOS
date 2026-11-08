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
// 12. PacTreeRenderer (pactree deep-tree parity)
// ============================================================================

/// Recursive dependency tree renderer (`pactree` parity).
/// Unlike the shallow `DependencyTreeVisualizer`, this walks the full
/// dependency graph with cycle detection.
pub struct PacTreeRenderer;

impl PacTreeRenderer {
    pub fn new() -> Self {
        Self
    }

    fn find<'a>(installed: &'a [ArchPacmanPackage], name: &str) -> Option<&'a ArchPacmanPackage> {
        installed.iter().find(|p| p.name == name)
    }

    fn deps_of(pkg: &ArchPacmanPackage) -> Vec<String> {
        pkg.depends
            .iter()
            .map(|d| dep_spec_base_name(d).to_string())
            .filter(|d| !d.is_empty())
            .collect()
    }

    fn reverse_deps_of<'a>(
        installed: &'a [ArchPacmanPackage],
        name: &str,
    ) -> Vec<&'a ArchPacmanPackage> {
        installed
            .iter()
            .filter(|p| p.depends.iter().any(|d| dep_spec_base_name(d) == name))
            .collect()
    }

    fn render_node(
        &self,
        name: &str,
        installed: &[ArchPacmanPackage],
        reverse: bool,
        path: &mut Vec<String>,
        lines: &mut Vec<String>,
        prefix: &str,
        is_last: bool,
        is_root: bool,
    ) {
        let connector = if is_root {
            String::new()
        } else if is_last {
            "└── ".to_string()
        } else {
            "├── ".to_string()
        };
        let cyclic = path.iter().any(|n| n == name);
        let suffix = if cyclic { " (*)" } else { "" };
        lines.push(format!("{}{}{}{}", prefix, connector, name, suffix));
        if cyclic {
            return;
        }

        path.push(name.to_string());
        let children: Vec<String> = if reverse {
            Self::reverse_deps_of(installed, name)
                .into_iter()
                .map(|p| p.name.clone())
                .collect()
        } else {
            Self::find(installed, name)
                .map(Self::deps_of)
                .unwrap_or_default()
        };

        let child_prefix = if is_root {
            prefix.to_string()
        } else {
            format!("{}{}", prefix, if is_last { "    " } else { "│   " })
        };
        let last_idx = children.len().saturating_sub(1);
        for (i, child) in children.iter().enumerate() {
            self.render_node(
                child,
                installed,
                reverse,
                path,
                lines,
                &child_prefix,
                i == last_idx,
                false,
            );
        }
        path.pop();
    }

    /// Render the full dependency tree for `pkg_name`.
    /// `reverse = true` renders the "required by" tree instead.
    /// Cycles are marked with `(*)` like real pactree output.
    pub fn render(&self, pkg_name: &str, installed: &[ArchPacmanPackage], reverse: bool) -> String {
        let mut lines = Vec::new();
        let mut path = Vec::new();
        self.render_node(
            pkg_name, installed, reverse, &mut path, &mut lines, "", true, true,
        );
        lines.join("\n") + "\n"
    }
}

impl Default for PacTreeRenderer {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 13. ExpacFormatter (expac parity)
// ============================================================================

/// Format-string expander for package database records (`expac` parity).
/// Supported tokens: %n name, %v version, %l licenses, %d depends,
/// %o optdepends, %s installed size, %a architecture, %u url,
/// %p packager, %g groups, %r repository (sync only), %% literal percent.
pub struct ExpacFormatter;

impl ExpacFormatter {
    pub fn new() -> Self {
        Self
    }

    /// Expand `fmt` against `pkg`. `repository` fills `%r` for sync records.
    pub fn format(&self, pkg: &ArchPacmanPackage, fmt: &str, repository: Option<&str>) -> String {
        let mut out = String::new();
        let mut chars = fmt.chars().peekable();
        while let Some(c) = chars.next() {
            if c != '%' {
                out.push(c);
                continue;
            }
            match chars.next() {
                None => out.push('%'),
                Some('%') => out.push('%'),
                Some('n') => out.push_str(&pkg.name),
                Some('v') => out.push_str(&pkg.version),
                Some('l') => out.push_str(&pkg.license.join(" ")),
                Some('d') => out.push_str(&pkg.depends.join(" ")),
                Some('o') => out.push_str(&pkg.optdepends.join(" ")),
                Some('s') => out.push_str(&pkg.installed_size.to_string()),
                Some('S') => out.push_str(&(pkg.installed_size / 1024).to_string()),
                Some('a') => out.push_str(&pkg.architecture),
                Some('u') => out.push_str(&pkg.url),
                Some('p') => out.push_str(&pkg.packager),
                Some('g') => out.push_str(&pkg.groups.join(" ")),
                Some('r') => out.push_str(repository.unwrap_or("")),
                Some(other) => {
                    out.push('%');
                    out.push(other);
                }
            }
        }
        out
    }
}

impl Default for ExpacFormatter {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 14. PacGraphRenderer (pacgraph / pactree --graph parity)
// ============================================================================

/// Renders the package dependency graph in Graphviz DOT format
/// (`pacgraph` / `pactree --graph` parity)
pub struct PacGraphRenderer;

impl PacGraphRenderer {
    pub fn new() -> Self {
        Self
    }

    fn dot_id(name: &str) -> String {
        format!("\"{}\"", name.replace('\\', "\\\\").replace('"', "\\\""))
    }

    /// Generate a DOT digraph of hard dependencies. When
    /// `include_optdepends` is set, optional dependencies are emitted as
    /// dotted edges, mirroring `pactree --graph --optdeps`.
    pub fn to_dot(&self, installed: &[ArchPacmanPackage], include_optdepends: bool) -> String {
        let mut out = String::from("digraph packages {\n");
        out.push_str("    rankdir=LR;\n");
        for pkg in installed {
            out.push_str(&format!(
                "    {} [label=\"{}\"];\n",
                Self::dot_id(&pkg.name),
                pkg.name
            ));
            for dep in &pkg.depends {
                let base = dep_spec_base_name(dep);
                if installed.iter().any(|p| p.name == base) {
                    out.push_str(&format!(
                        "    {} -> {};\n",
                        Self::dot_id(&pkg.name),
                        Self::dot_id(base)
                    ));
                }
            }
            if include_optdepends {
                for optdep in &pkg.optdepends {
                    let base = optdep_spec_name(optdep);
                    if installed.iter().any(|p| p.name == base) {
                        out.push_str(&format!(
                            "    {} -> {} [style=dotted];\n",
                            Self::dot_id(&pkg.name),
                            Self::dot_id(base)
                        ));
                    }
                }
            }
        }
        out.push_str("}\n");
        out
    }
}

impl Default for PacGraphRenderer {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 15. DevelUpdateChecker (yay -Y --devel parity)
// ============================================================================

/// A VCS-tracked (devel) package with its local and upstream revisions
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DevelPackage {
    pub name: String,
    pub local_revision: String,
    pub latest_revision: String,
}

/// Detects pending updates for devel/VCS packages (`yay -Y --devel` parity)
#[derive(Debug, Default)]
pub struct DevelUpdateChecker;

impl DevelUpdateChecker {
    pub fn new() -> Self {
        Self
    }

    /// Returns packages whose upstream revision differs from the local one
    pub fn pending_updates<'a>(&self, packages: &'a [DevelPackage]) -> Vec<&'a DevelPackage> {
        packages
            .iter()
            .filter(|p| p.local_revision != p.latest_revision)
            .collect()
    }
}

// ============================================================================
// 16. PacNewsParser (pacnews parity)
// ============================================================================

/// A parsed Arch news announcement (`pacnews` parity)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacNewsItem {
    pub date: String,
    pub title: String,
    pub packages: Vec<String>,
}

/// Parses pacman-contrib `pacnews` output. Accepts both the JSON-ish news
/// feed format and the plain `YYYY-MM-DD title` headline format.
#[derive(Debug)]
pub struct PacNewsParser;

impl PacNewsParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse one news record. `header` is the headline line, `body` may
    /// contain a `packages: a, b` line listing affected packages.
    pub fn parse_record(header: &str, body: &str) -> Option<PacNewsItem> {
        let header = header.trim();
        let (date, title) = if let Some(rest) = header.strip_prefix('[') {
            let (d, t) = rest.split_once(']')?;
            (d.trim().to_string(), t.trim().to_string())
        } else {
            let (d, t) = header.split_once(' ')?;
            (d.to_string(), t.to_string())
        };
        if date.is_empty() || title.is_empty() {
            return None;
        }

        let mut packages = Vec::new();
        for line in body.lines() {
            let line = line.trim();
            if let Some(list) = line
                .strip_prefix("packages:")
                .or_else(|| line.strip_prefix("Packages:"))
            {
                packages = list
                    .split(',')
                    .map(|p| p.trim().to_string())
                    .filter(|p| !p.is_empty())
                    .collect();
            }
        }

        Some(PacNewsItem {
            date,
            title,
            packages,
        })
    }

    /// Parse a full news feed into items, one record per non-empty header
    /// line followed by optional `packages:` body lines.
    pub fn parse_feed(&self, feed: &str) -> Vec<PacNewsItem> {
        let mut items = Vec::new();
        let mut current_header: Option<String> = None;
        let mut current_body = String::new();

        for line in feed.lines() {
            let trimmed = line.trim();
            let looks_like_header = trimmed.starts_with('[')
                || trimmed.split_once(' ').map_or(false, |(d, _)| {
                    d.len() == 10 && d.chars().nth(4) == Some('-')
                });
            if looks_like_header {
                if let Some(header) = current_header.take() {
                    if let Some(item) = Self::parse_record(&header, &current_body) {
                        items.push(item);
                    }
                }
                current_header = Some(trimmed.to_string());
                current_body.clear();
            } else if current_header.is_some() {
                current_body.push_str(line);
                current_body.push('\n');
            }
        }
        if let Some(header) = current_header {
            if let Some(item) = Self::parse_record(&header, &current_body) {
                items.push(item);
            }
        }
        items
    }
}

impl Default for PacNewsParser {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 17. PacmanDbMarker (pacman -D --asexplicit/--asdeps parity)
// ============================================================================

/// Marks installed packages as explicitly or dependency installed
/// (`pacman -D --asexplicit` / `--asdeps` parity)
pub struct PacmanDbMarker;

impl PacmanDbMarker {
    pub fn new() -> Self {
        Self
    }

    /// Set `is_explicit` on matching installed packages.
    /// Returns the names of packages whose flag actually changed;
    /// unknown names are ignored like pacman does with `-D`.
    pub fn set_explicit(
        &self,
        installed: &mut [ArchPacmanPackage],
        names: &[&str],
        explicit: bool,
    ) -> Vec<String> {
        let mut changed = Vec::new();
        for name in names {
            if let Some(pkg) = installed.iter_mut().find(|p| p.name == *name) {
                if pkg.is_explicit != explicit {
                    pkg.is_explicit = explicit;
                    changed.push(pkg.name.clone());
                }
            }
        }
        changed
    }

    /// `pacman -Qe` parity: list explicitly installed package names
    pub fn explicit_packages<'a>(installed: &'a [ArchPacmanPackage]) -> Vec<&'a str> {
        installed
            .iter()
            .filter(|p| p.is_explicit)
            .map(|p| p.name.as_str())
            .collect()
    }

    /// `pacman -Qd` parity: list dependency-installed package names
    pub fn dependency_packages<'a>(installed: &'a [ArchPacmanPackage]) -> Vec<&'a str> {
        installed
            .iter()
            .filter(|p| !p.is_explicit)
            .map(|p| p.name.as_str())
            .collect()
    }
}

impl Default for PacmanDbMarker {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 18. CheckpkgEngine (checkpkg parity)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckpkgSeverity {
    Error,
    Warning,
    Info,
}

/// A single lint finding produced by `checkpkg`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpkgFinding {
    pub severity: CheckpkgSeverity,
    pub message: String,
}

/// Extracts a scalar PKGBUILD variable's first quoted or bare value
fn pkgbuild_var<'a>(content: &'a str, key: &str) -> Option<&'a str> {
    for line in content.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix(key) {
            let rest = rest.trim_start();
            if rest.starts_with('=') {
                let value = rest[1..].trim();
                let value = value
                    .strip_prefix('"')
                    .and_then(|v| v.strip_suffix('"'))
                    .or_else(|| value.strip_prefix('\'').and_then(|v| v.strip_suffix('\'')))
                    .unwrap_or(value);
                return Some(value);
            }
        }
    }
    None
}

/// Splits a PKGBUILD array variable like `depends=(a "b>=1" c)` into items
fn pkgbuild_array(content: &str, key: &str) -> Vec<String> {
    let mut items = Vec::new();
    let mut in_array = false;
    let mut buf = String::new();
    for line in content.lines() {
        let line = line.trim();
        if !in_array {
            if let Some(rest) = line.strip_prefix(key) {
                let rest = rest.trim_start();
                if let Some(after_eq) = rest.strip_prefix('=') {
                    let after_eq = after_eq.trim_start();
                    if let Some(inner) = after_eq.strip_prefix('(') {
                        in_array = true;
                        buf.push_str(inner);
                        if inner.contains(')') {
                            in_array = false;
                        }
                        continue;
                    }
                }
            }
        } else {
            buf.push(' ');
            buf.push_str(line);
            if line.contains(')') {
                in_array = false;
            }
        }
    }
    if in_array || buf.is_empty() && !content.contains(&format!("{}=(", key)) {
        return items;
    }
    let body = buf.split(')').next().unwrap_or("");
    for tok in body.split_whitespace() {
        let tok = tok.trim_matches(|c| c == '"' || c == '\'');
        if !tok.is_empty() {
            items.push(tok.to_string());
        }
    }
    items
}

/// PKGBUILD linter (`checkpkg` parity). Checks packaging rules that go
/// beyond namcap's installed-package audit: dependency hygiene, version
/// format, license presence, and source/checksum consistency.
#[derive(Debug)]
pub struct CheckpkgEngine;

impl CheckpkgEngine {
    pub fn new() -> Self {
        Self
    }

    /// Lint `pkgbuild_content`. `existing_files` optionally lists paths that
    /// exist on disk for backup-file verification.
    pub fn lint(&self, pkgbuild_content: &str, existing_files: &[&str]) -> Vec<CheckpkgFinding> {
        let mut findings = Vec::new();
        let mut push = |severity: CheckpkgSeverity, message: String| {
            findings.push(CheckpkgFinding { severity, message });
        };

        let pkgname = pkgbuild_var(pkgbuild_content, "pkgname").unwrap_or("");
        if pkgname.is_empty() {
            push(CheckpkgSeverity::Error, "pkgname is not set".to_string());
        } else {
            if pkgname
                .chars()
                .any(|c| c.is_uppercase() || c.is_whitespace())
            {
                push(
                    CheckpkgSeverity::Error,
                    format!("pkgname '{}' must be lowercase without spaces", pkgname),
                );
            }
            if pkgname.starts_with('-') || pkgname.ends_with('-') {
                push(
                    CheckpkgSeverity::Warning,
                    format!("pkgname '{}' should not start or end with '-'", pkgname),
                );
            }
        }

        if let Some(pkgver) = pkgbuild_var(pkgbuild_content, "pkgver") {
            if pkgver.contains('-') || pkgver.contains(':') {
                push(
                    CheckpkgSeverity::Error,
                    format!("pkgver '{}' must not contain '-' or ':'", pkgver),
                );
            }
        } else {
            push(CheckpkgSeverity::Error, "pkgver is not set".to_string());
        }

        if let Some(pkgrel) = pkgbuild_var(pkgbuild_content, "pkgrel") {
            match pkgrel.parse::<u32>() {
                Ok(0) | Err(_) => push(
                    CheckpkgSeverity::Error,
                    format!("pkgrel '{}' must be a positive integer", pkgrel),
                ),
                _ => {}
            }
        } else {
            push(CheckpkgSeverity::Error, "pkgrel is not set".to_string());
        }

        match pkgbuild_var(pkgbuild_content, "license") {
            None => push(
                CheckpkgSeverity::Warning,
                "license is not set; package may not be distributable".to_string(),
            ),
            Some(l) if l.contains("custom") => push(
                CheckpkgSeverity::Warning,
                "license 'custom' should reference the actual license terms".to_string(),
            ),
            _ => {}
        }

        let depends = pkgbuild_array(pkgbuild_content, "depends");
        let makedepends = pkgbuild_array(pkgbuild_content, "makedepends");
        for dep in &depends {
            let base = dep_spec_base_name(dep);
            if makedepends.iter().any(|m| dep_spec_base_name(m) == base) {
                push(
                    CheckpkgSeverity::Warning,
                    format!("'{}' is listed in both depends and makedepends", base),
                );
            }
        }

        if let Some(arch) = pkgbuild_var(pkgbuild_content, "arch") {
            if !arch.contains("x86_64") && !arch.contains("any") {
                push(
                    CheckpkgSeverity::Info,
                    format!("arch '{}' does not include x86_64 or 'any'", arch),
                );
            }
        }

        if let Some(url) = pkgbuild_var(pkgbuild_content, "url") {
            if !url.starts_with("http://") && !url.starts_with("https://") {
                push(
                    CheckpkgSeverity::Warning,
                    format!("url '{}' is not an http(s) URL", url),
                );
            }
        }

        let sources = pkgbuild_array(pkgbuild_content, "source");
        if !sources.is_empty() {
            let sums_line = pkgbuild_content
                .lines()
                .find(|l| l.trim_start().starts_with("sha256sums="));
            let sum_count = sums_line
                .map(|l| {
                    l.split('(')
                        .nth(1)
                        .and_then(|r| r.split(')').next())
                        .map(|inner| {
                            inner
                                .split_whitespace()
                                .filter(|t| !t.is_empty() && *t != "SKIP")
                                .count()
                        })
                        .unwrap_or(0)
                })
                .unwrap_or(0);
            let real_sources = sources
                .iter()
                .filter(|s| !s.ends_with(".install") && !s.ends_with(".patch"))
                .count();
            if sum_count > 0 && sum_count != real_sources {
                push(
                    CheckpkgSeverity::Warning,
                    format!(
                        "sha256sums has {} entries but source has {} downloadable files",
                        sum_count, real_sources
                    ),
                );
            }
        }

        for backup in pkgbuild_array(pkgbuild_content, "backup") {
            let path = dep_spec_base_name(&backup);
            if !existing_files.is_empty() && !existing_files.contains(&path) {
                push(
                    CheckpkgSeverity::Warning,
                    format!("backup file '{}' not found on disk", path),
                );
            }
        }

        findings
    }
}

impl Default for CheckpkgEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 19. MakechrootpkgOrchestrator (makechrootpkg / extra-x86_64-build parity)
// ============================================================================

/// Which devtools clean-chroot builder to emulate
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChrootBuilderProfile {
    /// `makechrootpkg` — builds in `$CHROOTDIR` with `-s` syncdeps
    Makechrootpkg,
    /// `extra-x86_64-build` — builds directly in `/` chroot for x86_64
    ExtraX86_64Build,
}

/// Ordered stages of a clean-chroot package build
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChrootBuildStage {
    ResolveBuildDeps,
    CreateChroot,
    BindMount,
    RunMakepkg,
    CollectArtifacts,
    Cleanup,
}

/// Result of planning or executing a clean-chroot build
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChrootBuildResult {
    pub success: bool,
    pub commands_run: Vec<String>,
    pub artifacts: Vec<String>,
    pub failed_stage: Option<ChrootBuildStage>,
}

/// Clean-chroot build orchestrator (`makechrootpkg` / `extra-x86_64-build` parity)
#[derive(Debug)]
pub struct MakechrootpkgOrchestrator {
    pub profile: ChrootBuilderProfile,
    pub chroot_dir: String,
    pub syncdeps: bool,
    pub clean: bool,
}

impl MakechrootpkgOrchestrator {
    pub fn new(profile: ChrootBuilderProfile) -> Self {
        let (chroot_dir, syncdeps) = match profile {
            ChrootBuilderProfile::Makechrootpkg => ("/chroot/x86_64".to_string(), true),
            ChrootBuilderProfile::ExtraX86_64Build => ("/".to_string(), false),
        };
        Self {
            profile,
            chroot_dir,
            syncdeps,
            clean: false,
        }
    }

    /// Plan the command sequence for building `pkgbuild_dir`.
    /// `makedepends`/`depends` come from the PKGBUILD; build deps are
    /// installed inside the chroot before invoking makepkg.
    pub fn plan_commands(&self, pkgbuild_dir: &str, build_deps: &[String]) -> Vec<String> {
        let mut cmds = Vec::new();
        match self.profile {
            ChrootBuilderProfile::Makechrootpkg => {
                cmds.push("mkarchroot -f /chroot/base base-devel".to_string());
                if self.syncdeps {
                    let mut dep_args = String::new();
                    for dep in build_deps {
                        dep_args.push_str(&format!(" {}", dep));
                    }
                    cmds.push(format!(
                        "arch-chroot /chroot/base pacman -Syu --needed --noconfirm{}",
                        dep_args
                    ));
                }
                cmds.push(format!(
                    "bind mount {} -> /chroot/x86_64/build",
                    pkgbuild_dir
                ));
                cmds.push(
                    "arch-chroot /chroot/x86_64 su builder -c 'cd /build && makepkg -srf --noconfirm'"
                        .to_string(),
                );
                cmds.push("arch-chroot /chroot/x86_64 pacman -U /build/*.pkg.tar.zst".to_string());
            }
            ChrootBuilderProfile::ExtraX86_64Build => {
                cmds.push("mkarchroot -f / chroot-base".to_string());
                let mut dep_args = String::new();
                for dep in build_deps {
                    dep_args.push_str(&format!(" {}", dep));
                }
                cmds.push(format!("pacstrap / base-devel{}", dep_args));
                cmds.push(format!("cp -a {} /pkg", pkgbuild_dir));
                cmds.push(
                    "arch-chroot / su builder -c 'cd /pkg && makepkg -srf --noconfirm'".to_string(),
                );
                cmds.push("pacman -U /pkg/*.pkg.tar.zst".to_string());
            }
        }
        cmds.push("umount chroot bind mounts".to_string());
        if self.clean {
            cmds.push(format!("rm -rf {}", self.chroot_dir));
        }
        cmds
    }

    /// Execute the planned build, returning the commands that ran and the
    /// produced package artifacts. Fails fast at `failed_stage` and still
    /// emits cleanup commands, mirroring devtools' trap-based cleanup.
    pub fn execute(
        &self,
        pkgbuild_dir: &str,
        build_deps: &[String],
        fail_at: Option<ChrootBuildStage>,
    ) -> ChrootBuildResult {
        let stage_cmds: Vec<(ChrootBuildStage, Vec<String>)> = vec![
            (
                ChrootBuildStage::ResolveBuildDeps,
                self.plan_commands(pkgbuild_dir, build_deps)
                    .into_iter()
                    .take(2)
                    .collect(),
            ),
            (
                ChrootBuildStage::CreateChroot,
                vec!["mkarchroot".to_string()],
            ),
            (
                ChrootBuildStage::BindMount,
                vec!["mount --bind".to_string()],
            ),
            (
                ChrootBuildStage::RunMakepkg,
                vec!["makepkg -srf".to_string()],
            ),
            (
                ChrootBuildStage::CollectArtifacts,
                vec!["pacman -U *.pkg.tar.zst".to_string()],
            ),
            (
                ChrootBuildStage::Cleanup,
                vec!["umount; rm -rf".to_string()],
            ),
        ];

        let mut commands_run = Vec::new();
        let mut artifacts = Vec::new();
        let mut failed_stage = None;

        for (stage, cmds) in stage_cmds {
            if failed_stage.is_some() {
                break;
            }
            if fail_at == Some(stage) {
                failed_stage = Some(stage);
                break;
            }
            commands_run.extend(cmds.iter().cloned());
            if stage == ChrootBuildStage::CollectArtifacts {
                artifacts.push("*.pkg.tar.zst".to_string());
            }
        }
        // cleanup always runs, even on failure (devtools trap semantics)
        commands_run.push("umount chroot bind mounts".to_string());
        if self.clean {
            commands_run.push(format!("rm -rf {}", self.chroot_dir));
        }

        ChrootBuildResult {
            success: failed_stage.is_none(),
            commands_run,
            artifacts,
            failed_stage,
        }
    }
}

// ============================================================================
// 20. RepoDbManager (repo-add / repo-remove parity)
// ============================================================================

/// A package entry inside an Arch repository database (repo-add output)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepoDbPackage {
    pub name: String,
    pub version: String,
    /// package file name, e.g. "foo-1.0-1-x86_64.pkg.tar.zst"
    pub file_name: String,
    pub is_debug: bool,
}

/// Manages a local Arch repository database directory.
/// `repo-add` adds built packages to the db; `repo-remove` removes them.
#[derive(Debug)]
pub struct RepoDbManager {
    pub packages: Vec<RepoDbPackage>,
}

impl RepoDbManager {
    pub fn new() -> Self {
        Self {
            packages: Vec::new(),
        }
    }

    /// `repo-add` parity: register a built package. Returns the db entry
    /// written, or an error for invalid names/versions.
    pub fn add_package(
        &mut self,
        name: &str,
        version: &str,
        file_name: &str,
    ) -> Result<RepoDbPackage, &'static str> {
        if name.is_empty()
            || name
                .chars()
                .any(|c| c.is_whitespace() || c == '/' || c == '\\')
        {
            return Err("repo-add: invalid package name");
        }
        if CheckUpdatesEngine::compare_versions(version, version).is_none() {
            return Err("repo-add: invalid package version");
        }
        let entry = RepoDbPackage {
            name: name.to_string(),
            version: version.to_string(),
            file_name: file_name.to_string(),
            is_debug: file_name.contains("-debug"),
        };
        // repo-add replaces an existing entry of the same pkgrel line
        self.packages
            .retain(|p| p.name != entry.name || p.version != entry.version);
        self.packages.push(entry.clone());
        Ok(entry)
    }

    /// `repo-remove` parity: drop a package by name (and optional version).
    /// Returns true when something was removed.
    pub fn remove_package(&mut self, name: &str, version: Option<&str>) -> bool {
        let before = self.packages.len();
        self.packages
            .retain(|p| !(p.name == name && version.map_or(true, |v| p.version == v)));
        self.packages.len() != before
    }

    /// Render the repo `desc` database file content (one stanza per package)
    pub fn render_desc(&self) -> String {
        let mut out = String::new();
        for pkg in &self.packages {
            out.push_str(&format!("%NAME%\n{}\n", pkg.name));
            out.push_str(&format!("%VERSION%\n{}\n", pkg.version));
            out.push_str("%BASE%\n");
            out.push_str("%DESC%\n");
            if pkg.is_debug {
                out.push_str("%DEBUG%\n");
            }
        }
        out
    }

    /// File names currently in the repo, sorted for deterministic output
    pub fn file_list(&self) -> Vec<&str> {
        let mut files: Vec<&str> = self.packages.iter().map(|p| p.file_name.as_str()).collect();
        files.sort();
        files
    }
}

impl Default for RepoDbManager {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 21. SyncFileDbManager (pacman -Fy / pkgfile -u parity)
// ============================================================================

/// State of a downloaded sync file database
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncFileDb {
    /// repo name, e.g. "core", "extra"
    pub repo: String,
    /// remote URL the db was fetched from
    pub url: String,
    /// local cache path
    pub cache_path: String,
    /// entry count in the db
    pub entry_count: usize,
}

/// Manages download and refresh of pacman sync file databases
/// (`pacman -Fy` / `pkgfile -u` parity).
#[derive(Debug)]
pub struct SyncFileDbManager {
    pub databases: Vec<SyncFileDb>,
}

impl SyncFileDbManager {
    pub fn new() -> Self {
        Self {
            databases: Vec::new(),
        }
    }

    /// Register a sync db endpoint for a repo
    pub fn register_repo(&mut self, repo: &str, url: &str) {
        if !self.databases.iter().any(|d| d.repo == repo) {
            self.databases.push(SyncFileDb {
                repo: repo.to_string(),
                url: url.to_string(),
                cache_path: format!("/var/lib/pkgfile/{}.files", repo),
                entry_count: 0,
            });
        }
    }

    /// Record a completed download: updates entry count and marks fresh
    pub fn mark_synced(&mut self, repo: &str, entry_count: usize) {
        if let Some(db) = self.databases.iter_mut().find(|d| d.repo == repo) {
            db.entry_count = entry_count;
        }
    }

    /// `pacman -Fy` parity: refresh all registered sync file dbs.
    /// Returns the list of repos that would be fetched.
    pub fn refresh_all(&self) -> Vec<(String, String)> {
        self.databases
            .iter()
            .map(|d| (d.repo.clone(), d.url.clone()))
            .collect()
    }

    /// Merge a fetched db's entries into a PkgFileSearcher
    pub fn merge_into_searcher<'a>(
        &self,
        searcher: &mut PkgFileSearcher,
        repo: &str,
        entries: impl Iterator<Item = (&'a str, &'a str)>,
    ) {
        for (pkg, file) in entries {
            searcher.entries.push(SyncFileEntry {
                package_name: pkg.to_string(),
                repository: repo.to_string(),
                file_path: file.to_string(),
            });
        }
    }
}

impl Default for SyncFileDbManager {
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

    #[test]
    fn test_pactree_renderer() {
        let renderer = PacTreeRenderer::new();
        let mut a = sample_pkg("a");
        a.depends = vec!["b>=1.0".to_string(), "c".to_string()];
        let mut b = sample_pkg("b");
        b.depends = vec!["c".to_string()];
        let c = sample_pkg("c"); // no deps
        let installed = vec![a, b, c];

        let tree = renderer.render("a", &installed, false);
        assert!(tree.starts_with("a\n"));
        assert!(tree.contains("├── b"));
        assert!(tree.contains("│   └── c"));
        assert!(tree.contains("└── c"));

        // reverse tree: who requires c
        let rev = renderer.render("c", &installed, true);
        assert!(rev.contains("├── a") || rev.contains("└── a"));
        assert!(rev.contains("b"));
    }

    #[test]
    fn test_pactree_cycle_detection() {
        let renderer = PacTreeRenderer::new();
        let mut a = sample_pkg("a");
        a.depends = vec!["b".to_string()];
        let mut b = sample_pkg("b");
        b.depends = vec!["a".to_string()];
        let installed = vec![a, b];

        let tree = renderer.render("a", &installed, false);
        assert!(tree.contains("(*)"));
        // must terminate: only a bounded number of lines
        assert!(tree.lines().count() <= 6);
    }

    #[test]
    fn test_expac_formatter() {
        let fmt = ExpacFormatter::new();
        let mut pkg = sample_pkg("firefox");
        pkg.depends = vec!["gtk3".to_string(), "nss>=3.0".to_string()];
        pkg.optdepends = vec!["zenity: file dialogs".to_string()];

        assert_eq!(fmt.format(&pkg, "%n %v", None), "firefox 1.0.0-1");
        assert_eq!(fmt.format(&pkg, "%d", None), "gtk3 nss>=3.0");
        assert_eq!(fmt.format(&pkg, "%o", None), "zenity: file dialogs");
        assert_eq!(fmt.format(&pkg, "100%% %a", None), "100% x86_64");
        assert_eq!(fmt.format(&pkg, "%n@%r", Some("extra")), "firefox@extra");
        // unknown tokens pass through literally
        assert_eq!(fmt.format(&pkg, "%z", None), "%z");
    }

    #[test]
    fn test_pacgraph_dot() {
        let renderer = PacGraphRenderer::new();
        let mut a = sample_pkg("a");
        a.depends = vec!["b>=1".to_string(), "missing-dep".to_string()];
        a.optdepends = vec!["c: optional".to_string()];
        let b = sample_pkg("b");
        let c = sample_pkg("c");
        let installed = vec![a, b, c];

        let dot = renderer.to_dot(&installed, true);
        assert!(dot.starts_with("digraph packages {"));
        assert!(dot.contains("\"a\" -> \"b\";"));
        assert!(dot.contains("\"a\" -> \"c\" [style=dotted];"));
        // deps not installed are excluded
        assert!(!dot.contains("missing-dep\";"));
        // without optdeps flag, dotted edges are absent
        let dot2 = renderer.to_dot(&installed, false);
        assert!(!dot2.contains("style=dotted"));
    }

    #[test]
    fn test_devel_update_checker() {
        let checker = DevelUpdateChecker::new();
        let pkgs = vec![
            DevelPackage {
                name: "neovim-git".to_string(),
                local_revision: "abc123".to_string(),
                latest_revision: "def456".to_string(),
            },
            DevelPackage {
                name: "linux-git".to_string(),
                local_revision: "aaa".to_string(),
                latest_revision: "aaa".to_string(),
            },
        ];
        let pending = checker.pending_updates(&pkgs);
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].name, "neovim-git");
    }

    #[test]
    fn test_pacnews_parser() {
        let parser = PacNewsParser::new();
        let feed = "[2026-11-01] linux 6.17 upgrade\npackages: linux, linux-headers\n\n2026-10-30 openssl security fix\npackages: openssl\n";
        let items = parser.parse_feed(feed);
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].date, "2026-11-01");
        assert_eq!(items[0].title, "linux 6.17 upgrade");
        assert_eq!(items[0].packages, vec!["linux", "linux-headers"]);
        assert_eq!(items[1].title, "openssl security fix");
    }

    #[test]
    fn test_pacman_db_marker() {
        let marker = PacmanDbMarker::new();
        let mut a = sample_pkg("a");
        a.is_explicit = false;
        let mut b = sample_pkg("b");
        b.is_explicit = true;
        let mut installed = vec![a, b];

        // pacman -D --asexplicit a
        let changed = marker.set_explicit(&mut installed, &["a", "nonexistent"], true);
        assert_eq!(changed, vec!["a".to_string()]);
        assert!(installed[0].is_explicit);

        // pacman -Qe / -Qd listings
        assert_eq!(
            PacmanDbMarker::explicit_packages(&installed),
            vec!["a", "b"]
        );
        assert!(PacmanDbMarker::dependency_packages(&installed).is_empty());

        let changed2 = marker.set_explicit(&mut installed, &["b"], false);
        assert_eq!(changed2, vec!["b".to_string()]);
        assert_eq!(PacmanDbMarker::dependency_packages(&installed), vec!["b"]);
    }

    const CLEAN_PKGBUILD: &str = r#"
pkgname=testpkg
pkgver=1.2.3
pkgrel=1
pkgdesc="A test package"
arch=("x86_64")
url="https://example.com"
license=('MIT')
depends=("glibc" "zlib>=1.0")
makedepends=("cmake")
source=("https://example.com/src.tar.gz")
sha256sums=('abc123')
backup=("etc/testpkg.conf")
"#;

    #[test]
    fn test_checkpkg_clean_pkgbuild() {
        let engine = CheckpkgEngine::new();
        let findings = engine.lint(CLEAN_PKGBUILD, &["etc/testpkg.conf"]);
        assert!(findings.is_empty(), "unexpected findings: {:?}", findings);
    }

    #[test]
    fn test_checkpkg_finds_violations() {
        let engine = CheckpkgEngine::new();
        let bad = r#"
pkgname=Bad_Pkg
pkgver=1.0-rc1
pkgrel=0
license=('custom')
depends=("glibc" "cmake")
makedepends=("cmake")
arch=("aarch64")
url="ftp://example.com"
source=("https://example.com/a.tar.gz" "https://example.com/b.tar.gz")
sha256sums=('aaa' 'bbb' 'ccc')
backup=("etc/missing.conf")
"#;
        let findings = engine.lint(bad, &["etc/other.conf"]);
        let msgs: Vec<&str> = findings.iter().map(|f| f.message.as_str()).collect();
        assert!(msgs.iter().any(|m| m.contains("lowercase")));
        assert!(msgs.iter().any(|m| m.contains("pkgver")));
        assert!(msgs.iter().any(|m| m.contains("pkgrel")));
        assert!(msgs.iter().any(|m| m.contains("license 'custom'")));
        assert!(msgs
            .iter()
            .any(|m| m.contains("both depends and makedepends")));
        assert!(msgs.iter().any(|m| m.contains("does not include x86_64")));
        assert!(msgs.iter().any(|m| m.contains("http(s)")));
        assert!(msgs.iter().any(|m| m.contains("sha256sums has 3 entries")));
        assert!(msgs.iter().any(|m| m.contains("backup file")));
        assert!(findings
            .iter()
            .any(|f| f.severity == CheckpkgSeverity::Error));
    }

    #[test]
    fn test_checkpkg_multiline_array() {
        let engine = CheckpkgEngine::new();
        let pkgbuild = r#"
pkgname=multi
pkgver=1.0
pkgrel=1
license=('MIT')
depends=(
    "foo"
    "bar>=2.0"
)
source=("https://x/y.tar.gz")
sha256sums=('sum')
"#;
        let findings = engine.lint(pkgbuild, &[]);
        // no depends/makedepends conflict, no missing required fields
        assert!(!findings.iter().any(|f| f.message.contains("pkgname")));
        assert!(!findings.iter().any(|f| f.message.contains("pkgver")));
        assert!(!findings.iter().any(|f| f.message.contains("pkgrel")));
    }

    #[test]
    fn test_makechrootpkg_plan() {
        let orch = MakechrootpkgOrchestrator::new(ChrootBuilderProfile::Makechrootpkg);
        let deps = vec!["base-devel".to_string(), "cmake".to_string()];
        let cmds = orch.plan_commands("/home/user/pkgbuild", &deps);
        assert!(cmds.iter().any(|c| c.contains("mkarchroot")));
        assert!(cmds.iter().any(|c| c.contains("cmake")));
        assert!(cmds.iter().any(|c| c.contains("makepkg")));
        assert!(cmds.iter().any(|c| c.contains("umount")));
    }

    #[test]
    fn test_extra_x86_64_build_plan() {
        let orch = MakechrootpkgOrchestrator::new(ChrootBuilderProfile::ExtraX86_64Build);
        let cmds = orch.plan_commands("/src/pkg", &[]);
        assert!(cmds.iter().any(|c| c.contains("pacstrap /")));
        assert!(cmds.iter().any(|c| c.contains("arch-chroot /")));
    }

    #[test]
    fn test_makechrootpkg_execute_success_and_failure() {
        let mut orch = MakechrootpkgOrchestrator::new(ChrootBuilderProfile::Makechrootpkg);
        orch.clean = true;
        let deps = vec!["base-devel".to_string()];

        let ok = orch.execute("/build/dir", &deps, None);
        assert!(ok.success);
        assert!(!ok.artifacts.is_empty());
        assert!(ok.commands_run.iter().any(|c| c.contains("rm -rf")));

        let failed = orch.execute("/build/dir", &deps, Some(ChrootBuildStage::RunMakepkg));
        assert!(!failed.success);
        assert_eq!(failed.failed_stage, Some(ChrootBuildStage::RunMakepkg));
        // cleanup still runs on failure (trap semantics)
        assert!(failed.commands_run.iter().any(|c| c.contains("umount")));
    }

    #[test]
    fn test_repo_db_manager() {
        let mut repo = RepoDbManager::new();
        repo.add_package("foo", "1.0.0-1", "foo-1.0.0-1-x86_64.pkg.tar.zst")
            .unwrap();
        repo.add_package("foo", "1.1.0-1", "foo-1.1.0-1-x86_64.pkg.tar.zst")
            .unwrap();
        repo.add_package("bar", "2.0-1", "bar-2.0-1-x86_64.pkg.tar.zst")
            .unwrap();
        repo.add_package("baz", "3.0-1", "baz-3.0-1-x86_64-debug.pkg.tar.zst")
            .unwrap();

        assert_eq!(repo.packages.len(), 4);
        assert!(repo.file_list().contains(&"bar-2.0-1-x86_64.pkg.tar.zst"));
        let desc = repo.render_desc();
        assert!(desc.contains("%NAME%\nfoo\n"));
        assert!(desc.contains("%DEBUG%")); // debug package flagged

        // invalid inputs rejected
        assert!(repo.add_package("bad name", "1.0", "x").is_err());
        assert!(repo.add_package("ok", "1.0-", "x").is_err());

        // repo-remove by name and by name+version
        assert!(repo.remove_package("bar", None));
        assert!(!repo.remove_package("bar", None));
        assert!(repo.remove_package("foo", Some("1.0.0-1")));
        assert_eq!(repo.packages.len(), 2);
        assert!(repo.packages.iter().all(|p| p.name != "bar"));
    }

    #[test]
    fn test_sync_file_db_manager() {
        let mut mgr = SyncFileDbManager::new();
        mgr.register_repo("core", "https://mirror/core.files");
        mgr.register_repo("extra", "https://mirror/extra.files");
        mgr.register_repo("core", "https://mirror/core.files"); // dedup
        assert_eq!(mgr.databases.len(), 2);

        mgr.mark_synced("core", 15234);
        assert_eq!(mgr.databases[0].entry_count, 15234);

        let refresh = mgr.refresh_all();
        assert_eq!(refresh.len(), 2);

        let mut searcher = PkgFileSearcher::new();
        mgr.merge_into_searcher(
            &mut searcher,
            "core",
            [("bash", "/usr/bin/bash"), ("bash", "/etc/bash.bashrc")].into_iter(),
        );
        assert_eq!(searcher.find_exact("/usr/bin/bash").len(), 1);
        assert_eq!(searcher.find_exact("/usr/bin/bash")[0].repository, "core");
    }
}
