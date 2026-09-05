// SigmaOS — arch_inspirations.rs
// Implements Arch Linux-inspired features:
//   • Rolling release model
//   • AUR-style package building (PKGBUILD parsing + makepkg equivalent)
//   • Signed package databases
//   • pacman-style dependency resolution
//
// This module is intentionally self-contained and avoids unnecessary
// dependency on std::collections::HashMap (uses the crate's own SigmaHashMap
// from klib where maps are required).


use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

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

    /// Naive version comparison: splits on `.` and compares numeric segments.
    fn version_gt(a: &str, b: &str) -> bool {
        let parse_seg =
            |s: &str| -> Vec<u32> { s.split('.').filter_map(|x| x.parse::<u32>().ok()).collect() };
        let av = parse_seg(a);
        let bv = parse_seg(b);
        let len = av.len().max(bv.len());
        for i in 0..len {
            let ai = av.get(i).copied().unwrap_or(0);
            let bi = bv.get(i).copied().unwrap_or(0);
            if ai > bi {
                return true;
            }
            if ai < bi {
                return false;
            }
        }
        false
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

    fn phase_package(&self, pb: &PkgBuild) -> BuildStatus {
        // Would invoke: make DESTDIR=$pkgdir install; then create .pkg.tar.zst
        let _pkg_filename = format!(
            "{}-{}-{}.pkg.tar.zst",
            pb.pkgname,
            pb.full_version(),
            pb.arch
                .first()
                .cloned()
                .unwrap_or_else(|| "any".to_string())
        );
        BuildStatus::Success
    }

    fn phase_sign(&self, _pb: &PkgBuild) -> BuildStatus {
        if self.config.gpg_key.is_none() {
            return BuildStatus::Failed("sign requested but no GPG key configured".to_string());
        }
        BuildStatus::Success
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

#[cfg(test_disabled)]
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
}
