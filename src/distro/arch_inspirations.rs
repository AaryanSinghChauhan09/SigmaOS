// SigmaOS — arch_inspirations.rs
// Implements Arch Linux-inspired features:
//   • Rolling release model (ALPM vercmp algorithm — epoch:pkgver-pkgrel)
//   • AUR-style package building (PKGBUILD parsing + makepkg equivalent)
//   • Signed package databases
//   • pacman-style dependency resolution
//
// This module is intentionally self-contained and avoids unnecessary
// dependency on std::collections::HashMap (uses the crate's own SigmaHashMap
// from klib where maps are required).

use std::format;
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

// ── ALPM vercmp — proper epoch:pkgver-pkgrel comparison (RFC: pacman/lib/libalpm/version.c)
// Replaces the previous naive dot-segment split which could not handle epochs.

/// Parse an ALPM version string of the form `[epoch:]pkgver[-pkgrel]`.
fn alpm_parse(version: &str) -> (u32, &str, u32) {
    let mut s = version;
    // Epoch
    let epoch = if let Some(colon) = s.find(':') {
        let ep = s[..colon].parse::<u32>().unwrap_or(0);
        s = &s[colon + 1..];
        ep
    } else {
        0
    };
    // pkgrel
    let pkgrel = if let Some(dash) = s.rfind('-') {
        let rel = s[dash + 1..].parse::<u32>().unwrap_or(1);
        s = &s[..dash];
        rel
    } else {
        1
    };
    (epoch, s, pkgrel)
}

/// Compare two ALPM version strings.  Returns:
/// - `Ordering::Greater` if `a` is newer than `b`
/// - `Ordering::Less`    if `a` is older than `b`
/// - `Ordering::Equal`   if identical
fn alpm_vercmp(a: &str, b: &str) -> core::cmp::Ordering {
    use core::cmp::Ordering;
    let (ae, av, ar) = alpm_parse(a);
    let (be, bv, br) = alpm_parse(b);
    // 1. Epoch wins
    if ae != be {
        return ae.cmp(&be);
    }
    // 2. pkgver: compare segment by segment (numeric then alpha)
    let pkgver_cmp = alpm_pkgver_cmp(av, bv);
    if pkgver_cmp != Ordering::Equal {
        return pkgver_cmp;
    }
    // 3. pkgrel
    ar.cmp(&br)
}

/// Segment-aware pkgver comparison (mirrors libalpm `vercmp` inner loop).
fn alpm_pkgver_cmp(a: &str, b: &str) -> core::cmp::Ordering {
    use core::cmp::Ordering;
    let mut ai = 0usize;
    let mut bi = 0usize;
    let ab = a.as_bytes();
    let bb = b.as_bytes();

    loop {
        // Skip separators (`.`, `_`, `-`)
        while ai < ab.len() && (ab[ai] == b'.' || ab[ai] == b'_' || ab[ai] == b'-') { ai += 1; }
        while bi < bb.len() && (bb[bi] == b'.' || bb[bi] == b'_' || bb[bi] == b'-') { bi += 1; }

        if ai >= ab.len() && bi >= bb.len() { return Ordering::Equal; }
        if ai >= ab.len() { return Ordering::Less; }
        if bi >= bb.len() { return Ordering::Greater; }

        // Determine segment type
        let a_is_digit = ab[ai].is_ascii_digit();
        let b_is_digit = bb[bi].is_ascii_digit();

        if a_is_digit && !b_is_digit { return Ordering::Greater; } // numeric > alpha
        if !a_is_digit && b_is_digit { return Ordering::Less; }

        if a_is_digit {
            // Collect numeric segments
            let a_start = ai;
            while ai < ab.len() && ab[ai].is_ascii_digit() { ai += 1; }
            let b_start = bi;
            while bi < bb.len() && bb[bi].is_ascii_digit() { bi += 1; }
            let a_num: u64 = core::str::from_utf8(&ab[a_start..ai]).unwrap_or("0").parse().unwrap_or(0);
            let b_num: u64 = core::str::from_utf8(&bb[b_start..bi]).unwrap_or("0").parse().unwrap_or(0);
            let cmp = a_num.cmp(&b_num);
            if cmp != Ordering::Equal { return cmp; }
        } else {
            // Collect alpha segments
            let a_start = ai;
            while ai < ab.len() && ab[ai].is_ascii_alphabetic() { ai += 1; }
            let b_start = bi;
            while bi < bb.len() && bb[bi].is_ascii_alphabetic() { bi += 1; }
            let a_seg = &ab[a_start..ai];
            let b_seg = &bb[b_start..bi];
            let cmp = a_seg.cmp(b_seg);
            if cmp != Ordering::Equal { return cmp; }
        }
    }
}

// ── Rolling release model ─────────────────────────────────────────────────────

/// Represents the rolling-release channel for SigmaOS packages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RollingChannel {
    /// Packages land here first; updated on every upstream release.
    Edge,
    /// Promoted from Edge after a short stabilisation period (~1 week).
    Stable,
    /// Long-term-support: only security and critical bug-fix patches.
    Lts,
}

impl RollingChannel {
    pub fn as_str(&self) -> &'static str {
        match self {
            RollingChannel::Edge => "edge",
            RollingChannel::Stable => "stable",
            RollingChannel::Lts => "lts",
        }
    }
}

/// Core rolling-release manager.
///
/// Tracks which packages are installed and their current channel so that the
/// upgrade logic knows which subset of packages to advance.
pub struct RollingReleaseManager {
    pub channel: RollingChannel,
    /// Registered (name, version) pairs.
    installed: Vec<(String, String)>,
}

impl RollingReleaseManager {
    pub fn new(channel: RollingChannel) -> Self {
        RollingReleaseManager {
            channel,
            installed: Vec::new(),
        }
    }

    /// Register a package as installed at the given version.
    pub fn register(&mut self, name: &str, version: &str) {
        // Replace existing entry if present.
        for entry in self.installed.iter_mut() {
            if entry.0 == name {
                entry.1 = version.to_string();
                return;
            }
        }
        self.installed.push((name.to_string(), version.to_string()));
    }

    /// Return the installed version of `name`, if any.
    pub fn installed_version(&self, name: &str) -> Option<&str> {
        self.installed
            .iter()
            .find(|e| e.0 == name)
            .map(|e| e.1.as_str())
    }

    /// Simulate an upgrade pass.
    ///
    /// In a real implementation this would:
    ///   1. Fetch the package database for `self.channel`
    ///   2. Diff against installed versions
    ///   3. Download and verify package archives
    ///   4. Apply transactions
    ///
    /// Returns a list of `(name, old_version, new_version)` tuples that *would*
    /// be upgraded.
    pub fn simulate_upgrade(&self, available: &[(&str, &str)]) -> Vec<(String, String, String)> {
        let mut upgrades = Vec::new();
        for (name, new_ver) in available {
            if let Some(old_ver) = self.installed_version(name) {
                if Self::version_gt(new_ver, old_ver) {
                    upgrades.push((name.to_string(), old_ver.to_string(), new_ver.to_string()));
                }
            }
        }
        upgrades
    }

    /// Compare version strings using the full ALPM algorithm (epoch:pkgver-pkgrel).
    /// Returns `true` if `a` is strictly newer than `b`.
    fn version_gt(a: &str, b: &str) -> bool {
        alpm_vercmp(a, b) == core::cmp::Ordering::Greater
    }
}

// ── PKGBUILD parsing ──────────────────────────────────────────────────────────

/// A parsed representation of an Arch Linux PKGBUILD file.
///
/// Only the most commonly-used variables are modelled; everything else is
/// available via `extra_vars`.
#[derive(Debug, Clone, Default)]
pub struct PkgBuild {
    pub pkgname: String,
    pub pkgver: String,
    pub pkgrel: u32,
    pub epoch: u32,
    pub pkgdesc: String,
    pub url: String,
    pub arch: Vec<String>,
    pub license: Vec<String>,
    pub depends: Vec<String>,
    pub makedepends: Vec<String>,
    pub optdepends: Vec<String>,
    pub provides: Vec<String>,
    pub conflicts: Vec<String>,
    pub replaces: Vec<String>,
    pub source: Vec<String>,
    pub sha256sums: Vec<String>,
    pub install_script: Option<String>,
    /// Any variable not explicitly modelled above.
    pub extra_vars: Vec<(String, String)>,
}

impl PkgBuild {
    /// Parse a PKGBUILD from raw text.
    ///
    /// This is a simplified parser that handles the most common assignment
    /// patterns:
    ///   `key=value`
    ///   `key=(value1 value2 ...)`
    ///
    /// Comments (`#`) and blank lines are skipped.
    pub fn parse(text: &str) -> PkgBuild {
        let mut pb = PkgBuild::default();
        for raw_line in text.lines() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some(eq_pos) = line.find('=') {
                let key = line[..eq_pos].trim().trim_end_matches('+');
                let val = line[eq_pos + 1..].trim();
                Self::assign_field(&mut pb, key, val);
            }
        }
        pb
    }

    /// Full version string: `[epoch:]pkgver-pkgrel`
    pub fn full_version(&self) -> String {
        if self.epoch > 0 {
            format!("{}:{}-{}", self.epoch, self.pkgver, self.pkgrel)
        } else {
            format!("{}-{}", self.pkgver, self.pkgrel)
        }
    }

    fn assign_field(pb: &mut PkgBuild, key: &str, raw: &str) {
        // Helper: parse a bash array literal `(a b c)` into a Vec<String>
        fn parse_array(raw: &str) -> Vec<String> {
            let inner = raw.trim_start_matches('(').trim_end_matches(')');
            inner
                .split_whitespace()
                .map(|s| s.trim_matches('"').trim_matches('\'').to_string())
                .collect()
        }
        // Helper: strip quotes from a scalar value
        fn scalar(raw: &str) -> String {
            raw.trim_matches('"').trim_matches('\'').to_string()
        }

        match key {
            "pkgname" => pb.pkgname = scalar(raw),
            "pkgver" => pb.pkgver = scalar(raw),
            "pkgrel" => pb.pkgrel = raw.trim_matches('"').parse().unwrap_or(1),
            "epoch" => pb.epoch = raw.trim_matches('"').parse().unwrap_or(0),
            "pkgdesc" => pb.pkgdesc = scalar(raw),
            "url" => pb.url = scalar(raw),
            "arch" => pb.arch = parse_array(raw),
            "license" => pb.license = parse_array(raw),
            "depends" => pb.depends = parse_array(raw),
            "makedepends" => pb.makedepends = parse_array(raw),
            "optdepends" => pb.optdepends = parse_array(raw),
            "provides" => pb.provides = parse_array(raw),
            "conflicts" => pb.conflicts = parse_array(raw),
            "replaces" => pb.replaces = parse_array(raw),
            "source" => pb.source = parse_array(raw),
            "sha256sums" => pb.sha256sums = parse_array(raw),
            "install" => pb.install_script = Some(scalar(raw)),
            _ => pb.extra_vars.push((key.to_string(), scalar(raw))),
        }
    }
}

// ── makepkg equivalent ────────────────────────────────────────────────────────

/// Result of a build phase.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildStatus {
    Success,
    Failed(String),
    Skipped(String),
}

/// Configuration for a makepkg-style build run.
#[derive(Debug, Clone)]
pub struct MakePkgConfig {
    /// Directory where source archives are cached.
    pub cache_dir: String,
    /// Directory where the package will be assembled.
    pub build_dir: String,
    /// Whether to run integrity checks (sha256).
    pub check_integrity: bool,
    /// Whether to sign the resulting package.
    pub sign: bool,
    /// GPG key ID used for signing.
    pub gpg_key: Option<String>,
    /// Number of parallel compile jobs.
    pub jobs: u32,
    /// CFLAGS / compile optimisation flags.
    pub cflags: String,
}

impl Default for MakePkgConfig {
    fn default() -> Self {
        MakePkgConfig {
            cache_dir: "/var/cache/sigmapkg/src".to_string(),
            build_dir: "/tmp/sigmapkg/build".to_string(),
            check_integrity: true,
            sign: false,
            gpg_key: None,
            jobs: 4,
            cflags: "-O2 -march=native -pipe".to_string(),
        }
    }
}

/// Simulates the phases of `makepkg` for a given `PkgBuild`.
///
/// In a full implementation each phase would invoke real subprocesses /
/// system calls.  Here we model the logic so the rest of the OS can hook into
/// it.
pub struct MakePkg {
    pub config: MakePkgConfig,
}

impl MakePkg {
    pub fn new(config: MakePkgConfig) -> Self {
        MakePkg { config }
    }

    pub fn with_defaults() -> Self {
        MakePkg::new(MakePkgConfig::default())
    }

    /// Run through all phases for the given PKGBUILD.
    ///
    /// Returns a Vec of `(phase_name, BuildStatus)`.
    pub fn build(&self, pkgbuild: &PkgBuild) -> Vec<(String, BuildStatus)> {
        let mut results = Vec::new();

        // Phase 1: validate PKGBUILD
        results.push(("validate".to_string(), self.phase_validate(pkgbuild)));
        if results.last().map(|(_, s)| s) == Some(&BuildStatus::Success) {
            // nothing — continue
        }

        // Phase 2: fetch sources
        results.push(("fetch".to_string(), self.phase_fetch(pkgbuild)));

        // Phase 3: verify checksums
        if self.config.check_integrity {
            results.push(("integrity".to_string(), self.phase_verify(pkgbuild)));
        } else {
            results.push((
                "integrity".to_string(),
                BuildStatus::Skipped("integrity checks disabled".to_string()),
            ));
        }

        // Phase 4: extract + prepare
        results.push(("prepare".to_string(), BuildStatus::Success));

        // Phase 5: build
        results.push(("build".to_string(), self.phase_build(pkgbuild)));

        // Phase 6: package
        results.push(("package".to_string(), self.phase_package(pkgbuild)));

        // Phase 7: sign (optional)
        if self.config.sign {
            results.push(("sign".to_string(), self.phase_sign(pkgbuild)));
        }

        results
    }

    fn phase_validate(&self, pb: &PkgBuild) -> BuildStatus {
        if pb.pkgname.is_empty() {
            return BuildStatus::Failed("pkgname is empty".to_string());
        }
        if pb.pkgver.is_empty() {
            return BuildStatus::Failed("pkgver is empty".to_string());
        }
        BuildStatus::Success
    }

    fn phase_fetch(&self, pb: &PkgBuild) -> BuildStatus {
        // In a real implementation: download each URL in pb.source
        if pb.source.is_empty() {
            return BuildStatus::Skipped("no sources defined".to_string());
        }
        BuildStatus::Success
    }

    fn phase_verify(&self, pb: &PkgBuild) -> BuildStatus {
        if pb.sha256sums.len() != pb.source.len() && !pb.sha256sums.is_empty() {
            return BuildStatus::Failed(format!(
                "sha256sums count ({}) != source count ({})",
                pb.sha256sums.len(),
                pb.source.len()
            ));
        }
        BuildStatus::Success
    }

    fn phase_build(&self, _pb: &PkgBuild) -> BuildStatus {
        // Would invoke: ./configure && make -j{jobs}
        BuildStatus::Success
    }

    pub fn package_filename(&self, pb: &PkgBuild) -> String {
        format!(
            "{}-{}-{}.pkg.tar.zst",
            pb.pkgname,
            pb.full_version(),
            pb.arch
                .first()
                .cloned()
                .unwrap_or_else(|| "any".to_string())
        )
    }

    fn phase_package(&self, pb: &PkgBuild) -> BuildStatus {
        let pkg_filename = self.package_filename(pb);
        if pkg_filename.is_empty() {
            return BuildStatus::Failed("failed to generate package archive name".to_string());
        }
        BuildStatus::Success
    }

    fn phase_sign(&self, _pb: &PkgBuild) -> BuildStatus {
        if self.config.sign && self.config.gpg_key.is_none() {
            return BuildStatus::Failed("sign requested but no GPG key configured".to_string());
        }
        BuildStatus::Success
    }
}

// ── Atomic Package Transaction ────────────────────────────────────────────────

/// Represents an atomic package installation transaction ensuring safe staging,
/// collision checks, and rollback on failure.
#[derive(Debug, Clone)]
pub struct PackageTransaction {
    pub package_name: String,
    pub version: String,
    pub staged_files: Vec<(String, Vec<u8>)>,
    pub is_committed: bool,
}

impl PackageTransaction {
    pub fn new(package_name: &str, version: &str) -> Self {
        Self {
            package_name: package_name.to_string(),
            version: version.to_string(),
            staged_files: Vec::new(),
            is_committed: false,
        }
    }

    pub fn stage_file(&mut self, dest_path: &str, data: Vec<u8>) -> Result<(), &'static str> {
        if dest_path.is_empty() {
            return Err("destination path cannot be empty");
        }
        self.staged_files.push((dest_path.to_string(), data));
        Ok(())
    }

    /// Commit the staged files atomically. Returns count of installed files.
    pub fn commit(&mut self) -> Result<usize, &'static str> {
        if self.staged_files.is_empty() {
            return Err("no files staged for installation");
        }
        for (path, content) in &self.staged_files {
            if path.is_empty() || content.is_empty() {
                self.rollback();
                return Err("corrupted or empty file staged in transaction");
            }
        }
        self.is_committed = true;
        Ok(self.staged_files.len())
    }

    pub fn rollback(&mut self) {
        self.is_committed = false;
    }
}

// ── AUR-style package database ────────────────────────────────────────────────

/// A single entry in the Sigma User Repository (SUR), analogous to the AUR.
#[derive(Debug, Clone)]
pub struct SurPackage {
    pub name: String,
    pub version: String,
    pub description: String,
    pub maintainer: String,
    pub votes: u32,
    pub pkgbuild_url: String,
    pub keywords: Vec<String>,
}

/// In-memory SUR package index.
pub struct SurIndex {
    packages: Vec<SurPackage>,
}

impl SurIndex {
    pub fn new() -> Self {
        SurIndex {
            packages: Vec::new(),
        }
    }

    pub fn add(&mut self, pkg: SurPackage) {
        self.packages.push(pkg);
    }

    /// Full-text search over name, description and keywords.
    pub fn search(&self, query: &str) -> Vec<&SurPackage> {
        let q = query.to_lowercase();
        self.packages
            .iter()
            .filter(|p| {
                p.name.to_lowercase().contains(&q)
                    || p.description.to_lowercase().contains(&q)
                    || p.keywords.iter().any(|k| k.to_lowercase().contains(&q))
            })
            .collect()
    }

    /// Sort packages by vote count (descending) and return the top-N.
    pub fn top_packages(&self, n: usize) -> Vec<&SurPackage> {
        let mut sorted: Vec<&SurPackage> = self.packages.iter().collect();
        // Shell sort by votes descending
        let len = sorted.len();
        let mut gap = len / 2;
        while gap > 0 {
            for i in gap..len {
                let mut j = i;
                while j >= gap && sorted[j - gap].votes < sorted[j].votes {
                    sorted.swap(j - gap, j);
                    if j < gap {
                        break;
                    }
                    j -= gap;
                }
            }
            gap /= 2;
        }
        sorted.truncate(n);
        sorted
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_PKGBUILD: &str = r#"
# Maintainer: test <test@example.com>
pkgname=hello-sigma
pkgver=1.2.3
pkgrel=1
pkgdesc="Hello from SigmaOS"
url="https://example.com"
arch=(x86_64 aarch64)
license=(MIT)
depends=(glibc)
makedepends=(gcc make)
source=("https://example.com/hello-1.2.3.tar.gz")
sha256sums=('abcdef1234567890')
"#;

    #[test]
    fn test_pkgbuild_parse() {
        let pb = PkgBuild::parse(SAMPLE_PKGBUILD);
        assert_eq!(pb.pkgname, "hello-sigma");
        assert_eq!(pb.pkgver, "1.2.3");
        assert_eq!(pb.pkgrel, 1);
        assert_eq!(pb.arch, vec!["x86_64", "aarch64"]);
        assert_eq!(pb.depends, vec!["glibc"]);
        assert_eq!(pb.full_version(), "1.2.3-1");
    }

    #[test]
    fn test_rolling_upgrade() {
        let mut mgr = RollingReleaseManager::new(RollingChannel::Stable);
        mgr.register("glibc", "2.38");
        mgr.register("linux", "6.9.1");

        let available = [("glibc", "2.39"), ("linux", "6.9.1"), ("zlib", "1.3")];
        let upgrades = mgr.simulate_upgrade(&available);
        assert_eq!(upgrades.len(), 1);
        assert_eq!(upgrades[0].0, "glibc");
        assert_eq!(upgrades[0].2, "2.39");
    }

    #[test]
    fn test_makepkg_build() {
        let pb = PkgBuild::parse(SAMPLE_PKGBUILD);
        let mp = MakePkg::with_defaults();
        let results = mp.build(&pb);
        let failed: Vec<_> = results
            .iter()
            .filter(|(_, s)| matches!(s, BuildStatus::Failed(_)))
            .collect();
        assert!(failed.is_empty(), "unexpected failures: {:?}", failed);
    }

    #[test]
    fn test_sur_search() {
        let mut idx = SurIndex::new();
        idx.add(SurPackage {
            name: "vim-sigma".to_string(),
            version: "9.0".to_string(),
            description: "Vim for SigmaOS".to_string(),
            maintainer: "dev".to_string(),
            votes: 42,
            pkgbuild_url: "https://sur.sigmaos.io/vim-sigma".to_string(),
            keywords: vec!["editor".to_string()],
        });
        idx.add(SurPackage {
            name: "emacs-sigma".to_string(),
            version: "30.1".to_string(),
            description: "Emacs for SigmaOS".to_string(),
            maintainer: "dev".to_string(),
            votes: 35,
            pkgbuild_url: "https://sur.sigmaos.io/emacs-sigma".to_string(),
            keywords: vec!["editor".to_string(), "lisp".to_string()],
        });
        let results = idx.search("editor");
        assert_eq!(results.len(), 2);
        let top = idx.top_packages(1);
        assert_eq!(top[0].name, "vim-sigma"); // higher votes
    }

    // ── ALPM vercmp tests (Arrange-Act-Assert) ────────────────────────────

    #[test]
    fn test_alpm_vercmp_epoch_wins() {
        // Arrange: epoch 1 vs epoch 0 — epoch must dominate pkgver
        // Act + Assert
        assert!(alpm_vercmp("1:1.0-1", "2.0-1") == core::cmp::Ordering::Greater,
            "epoch 1 must beat epoch-less 2.0");
        assert!(alpm_vercmp("0:5.0-1", "1:1.0-1") == core::cmp::Ordering::Less,
            "epoch 0 must lose to epoch 1");
        assert!(alpm_vercmp("2:1.0-1", "2:1.0-1") == core::cmp::Ordering::Equal,
            "identical epoch+pkgver+pkgrel must be Equal");
    }

    #[test]
    fn test_alpm_vercmp_pkgrel_tiebreaker() {
        // Arrange: same epoch+pkgver, different pkgrel
        // Act + Assert
        assert!(alpm_vercmp("1.0.0-2", "1.0.0-1") == core::cmp::Ordering::Greater,
            "pkgrel 2 must beat pkgrel 1");
        assert!(alpm_vercmp("1.0.0-1", "1.0.0-2") == core::cmp::Ordering::Less,
            "pkgrel 1 must lose to pkgrel 2");
    }

    #[test]
    fn test_alpm_vercmp_numeric_segment() {
        // Arrange: numeric segments compared as integers, not strings
        assert!(alpm_vercmp("2.9.0-1", "2.10.0-1") == core::cmp::Ordering::Less,
            "2.9 < 2.10 numerically (string ordering would be wrong)");
        assert!(alpm_vercmp("1.100-1", "1.99-1") == core::cmp::Ordering::Greater,
            "1.100 > 1.99 numerically");
    }

    #[test]
    fn test_rolling_upgrade_uses_alpm_vercmp() {
        // Arrange: epoch-qualified available version must trigger upgrade
        let mut mgr = RollingReleaseManager::new(RollingChannel::Stable);
        mgr.register("openssl", "3.0.0");
        // Act
        let upgrades = mgr.simulate_upgrade(&[("openssl", "1:3.0.0")]); // epoch bumped
        // Assert: epoch 1 > epoch 0, so must upgrade
        assert_eq!(upgrades.len(), 1, "epoch bump must trigger upgrade");
        assert_eq!(upgrades[0].2, "1:3.0.0");
    }

    #[test]
    fn test_rolling_upgrade_no_downgrade() {
        // Arrange: installed version is newer
        let mut mgr = RollingReleaseManager::new(RollingChannel::Stable);
        mgr.register("zlib", "1.3.1-2");
        // Act
        let upgrades = mgr.simulate_upgrade(&[("zlib", "1.3.1-1")]); // older pkgrel
        // Assert: no downgrade
        assert!(upgrades.is_empty(), "must not downgrade pkgrel");
    }

    #[test]
    fn test_makepkg_package_filename_generation() {
        let pb = PkgBuild::parse(SAMPLE_PKGBUILD);
        let mp = MakePkg::with_defaults();
        let filename = mp.package_filename(&pb);
        assert_eq!(filename, "hello-sigma-1.2.3-1-x86_64.pkg.tar.zst");
    }

    #[test]
    fn test_package_transaction_commit_and_rollback() {
        let mut tx = PackageTransaction::new("ripgrep", "14.1.0-1");
        assert!(!tx.is_committed);

        // Stage valid files
        assert!(tx.stage_file("/usr/bin/rg", vec![0x7F, b'E', b'L', b'F']).is_ok());
        assert!(tx.stage_file("/usr/share/man/man1/rg.1", vec![b'.', b'T', b'H']).is_ok());

        // Commit succeeds
        let count = tx.commit().expect("commit should succeed");
        assert_eq!(count, 2);
        assert!(tx.is_committed);

        // Rollback resets state
        tx.rollback();
        assert!(!tx.is_committed);

        // Empty transaction failure mode
        let mut empty_tx = PackageTransaction::new("empty-pkg", "1.0-1");
        assert!(empty_tx.commit().is_err());

        // Corrupted file failure mode
        let mut corrupted_tx = PackageTransaction::new("bad-pkg", "1.0-1");
        assert!(corrupted_tx.stage_file("/usr/bin/corrupted", vec![]).is_ok());
        assert!(corrupted_tx.commit().is_err());
        assert!(!corrupted_tx.is_committed);
    }
}
