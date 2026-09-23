//! SigmaOS Kernel Version Manager
//! Inspired by Linux kernel versioning (torvalds/linux include/linux/version.h),
//! FreeBSD's sys/param.h, and OpenBSD's sys/param.h.
//!
//! Provides semantic versioning, ABI compatibility tracking, release strings,
//! kernel feature flags, and changelog generation for SigmaOS.

#![allow(dead_code)]

use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;
use std::format;

// ─── Version Constants (Linux-style) ──────────────────────────────────────────

/// SigmaOS major version (Linux 6.x equivalent)
pub const SIGMAOS_VERSION_MAJOR: u32 = 1;
/// SigmaOS minor version
pub const SIGMAOS_VERSION_MINOR: u32 = 0;
/// SigmaOS patch level
pub const SIGMAOS_VERSION_PATCH: u32 = 0;
/// ABI revision — incremented on ABI-breaking changes
pub const SIGMAOS_ABI_REVISION: u32 = 1;
/// Build metadata string
pub const SIGMAOS_BUILD_METADATA: &str = "sovereign-rust";

/// Encode version into a single u64 for fast comparison (Linux KERNEL_VERSION macro equivalent)
#[inline]
pub const fn sigma_version_code(major: u32, minor: u32, patch: u32) -> u64 {
    ((major as u64) << 32) | ((minor as u64) << 16) | (patch as u64)
}

pub const SIGMAOS_VERSION_CODE: u64 =
    sigma_version_code(SIGMAOS_VERSION_MAJOR, SIGMAOS_VERSION_MINOR, SIGMAOS_VERSION_PATCH);

// ─── Release Type ─────────────────────────────────────────────────────────────

/// SigmaOS release type, inspired by Linux -rc, FreeBSD BETA/RELEASE tags
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReleaseType {
    /// Unstable development snapshot (Linux -rcN)
    ReleaseCandidate(u8),
    /// Stable release (Linux RELEASE)
    Stable,
    /// Long-term support release (Linux LTS)
    Lts { supported_until: String },
    /// Security hot-fix release
    SecurityPatch,
    /// Pre-release testing milestone
    Beta(u8),
}

impl ReleaseType {
    /// Returns the suffix used in release strings
    pub fn suffix(&self) -> String {
        match self {
            ReleaseType::ReleaseCandidate(n) => format!("-rc{}", n),
            ReleaseType::Stable => String::from(""),
            ReleaseType::Lts { supported_until } => format!("-lts({})", supported_until),
            ReleaseType::SecurityPatch => String::from("-security"),
            ReleaseType::Beta(n) => format!("-beta{}", n),
        }
    }
}

// ─── SigmaOS Kernel Version ───────────────────────────────────────────────────

/// Full SigmaOS kernel version record
#[derive(Debug, Clone)]
pub struct SigmaOsVersion {
    /// Major version number
    pub major: u32,
    /// Minor version number
    pub minor: u32,
    /// Patch level
    pub patch: u32,
    /// ABI revision (incremented on ABI breaks, like FreeBSD __FreeBSD_version)
    pub abi_revision: u32,
    /// Release type classification
    pub release_type: ReleaseType,
    /// Git commit hash at build time (short, 12 hex chars)
    pub commit_hash: String,
    /// Architecture this kernel was built for
    pub target_arch: String,
    /// ISO 8601 build timestamp
    pub build_timestamp: String,
    /// Rust compiler version used
    pub rustc_version: String,
}

impl SigmaOsVersion {
    /// Create the canonical SigmaOS version
    pub fn current() -> Self {
        SigmaOsVersion {
            major: SIGMAOS_VERSION_MAJOR,
            minor: SIGMAOS_VERSION_MINOR,
            patch: SIGMAOS_VERSION_PATCH,
            abi_revision: SIGMAOS_ABI_REVISION,
            release_type: ReleaseType::ReleaseCandidate(1),
            commit_hash: String::from("fd97c013c5ab"),
            target_arch: String::from("x86_64"),
            build_timestamp: String::from("2026-09-17T12:00:00Z"),
            rustc_version: String::from("1.82.0"),
        }
    }

    /// Returns Linux uname-style version string:
    /// `SigmaOS 1.0.0-rc1 (sovereign-rust) #1 SMP x86_64`
    pub fn uname_release(&self) -> String {
        format!(
            "SigmaOS {}.{}.{}{} ({}) #{} SMP {}",
            self.major,
            self.minor,
            self.patch,
            self.release_type.suffix(),
            SIGMAOS_BUILD_METADATA,
            self.abi_revision,
            self.target_arch
        )
    }

    /// Returns BSD-style OS version string like `SigmaOS/x86_64 1.0-RC1`
    pub fn bsd_version_string(&self) -> String {
        format!(
            "SigmaOS/{} {}.{}-{}",
            self.target_arch,
            self.major,
            self.minor,
            match &self.release_type {
                ReleaseType::ReleaseCandidate(n) => format!("RC{}", n),
                ReleaseType::Stable => String::from("RELEASE"),
                ReleaseType::Lts { .. } => String::from("LTS"),
                ReleaseType::SecurityPatch => String::from("SECURITY"),
                ReleaseType::Beta(n) => format!("BETA{}", n),
            }
        )
    }

    /// Returns the encoded u64 version code for fast comparison
    pub fn version_code(&self) -> u64 {
        sigma_version_code(self.major, self.minor, self.patch)
    }

    /// Check ABI compatibility: returns true if `other` is ABI-compatible with self
    pub fn is_abi_compatible(&self, other: &SigmaOsVersion) -> bool {
        self.major == other.major && self.abi_revision == other.abi_revision
    }

    /// Check if this is a stable release
    pub fn is_stable(&self) -> bool {
        matches!(self.release_type, ReleaseType::Stable | ReleaseType::Lts { .. })
    }
}

// ─── Feature Flags ────────────────────────────────────────────────────────────

/// Kernel feature flags (inspired by Linux CONFIG_* and FreeBSD kernel options)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KernelFeature {
    /// BORE CPU scheduler enabled
    BoreScheduler,
    /// EEVDF CPU scheduler enabled
    EevdfScheduler,
    /// io_uring async I/O enabled
    IoUring,
    /// eBPF VM runtime enabled
    EbpfRuntime,
    /// XDP zero-copy networking enabled
    XdpNetworking,
    /// Post-Quantum Cryptography (Kyber/Dilithium) enabled
    PostQuantumCrypto,
    /// Landlock LSM v5 enabled
    LandlockLsmV5,
    /// OpenBSD pledge() enforcement enabled
    PledgeEnforcement,
    /// OpenBSD unveil() enforcement enabled
    UnveilEnforcement,
    /// Capsicum capability framework enabled
    CapsicumCapabilities,
    /// SigPkg universal package engine enabled
    UniversalSigpkg,
    /// Zenith Wayland compositor enabled
    ZenithWayland,
    /// NixOS-style declarative system state enabled
    DeclarativeState,
    /// Multi-architecture HAL enabled
    MultiArchHal,
    /// Live process migration enabled
    LiveProcessMigration,
    /// Autonomous AI kernel governor enabled
    AiKernelGovernor,
}

impl KernelFeature {
    /// Human-readable feature name
    pub fn display_name(&self) -> &'static str {
        match self {
            KernelFeature::BoreScheduler => "BORE CPU Scheduler (CachyOS)",
            KernelFeature::EevdfScheduler => "EEVDF CPU Scheduler (Linux 6.6)",
            KernelFeature::IoUring => "io_uring Async I/O",
            KernelFeature::EbpfRuntime => "eBPF VM Runtime",
            KernelFeature::XdpNetworking => "XDP Zero-Copy Networking",
            KernelFeature::PostQuantumCrypto => "Post-Quantum Cryptography (Kyber-1024/Dilithium-5)",
            KernelFeature::LandlockLsmV5 => "Landlock LSM v5 Filesystem/Network Sandbox",
            KernelFeature::PledgeEnforcement => "OpenBSD pledge() Capability Gates",
            KernelFeature::UnveilEnforcement => "OpenBSD unveil() Filesystem Restriction",
            KernelFeature::CapsicumCapabilities => "FreeBSD Capsicum Capability Framework",
            KernelFeature::UniversalSigpkg => "SigPkg Universal Package Engine (60+ formats)",
            KernelFeature::ZenithWayland => "Zenith Wayland Compositor",
            KernelFeature::DeclarativeState => "NixOS-Style Declarative System State",
            KernelFeature::MultiArchHal => "Multi-Architecture HAL (8 CPU targets)",
            KernelFeature::LiveProcessMigration => "Live Process Migration (Cluster-Native)",
            KernelFeature::AiKernelGovernor => "AI Kernel Autotuner Governor",
        }
    }

    /// Configuration symbol name (like Linux CONFIG_BORE_SCHED)
    pub fn config_symbol(&self) -> &'static str {
        match self {
            KernelFeature::BoreScheduler => "CONFIG_BORE_SCHED",
            KernelFeature::EevdfScheduler => "CONFIG_EEVDF_SCHED",
            KernelFeature::IoUring => "CONFIG_IO_URING",
            KernelFeature::EbpfRuntime => "CONFIG_BPF_SYSCALL",
            KernelFeature::XdpNetworking => "CONFIG_XDP_SOCKETS",
            KernelFeature::PostQuantumCrypto => "CONFIG_CRYPTO_PQC_KYBER_DILITHIUM",
            KernelFeature::LandlockLsmV5 => "CONFIG_SECURITY_LANDLOCK",
            KernelFeature::PledgeEnforcement => "CONFIG_OPENBSD_PLEDGE",
            KernelFeature::UnveilEnforcement => "CONFIG_OPENBSD_UNVEIL",
            KernelFeature::CapsicumCapabilities => "CONFIG_FREEBSD_CAPSICUM",
            KernelFeature::UniversalSigpkg => "CONFIG_SIGMA_UNIVERSAL_PKG",
            KernelFeature::ZenithWayland => "CONFIG_ZENITH_WAYLAND",
            KernelFeature::DeclarativeState => "CONFIG_DECLARATIVE_STATE",
            KernelFeature::MultiArchHal => "CONFIG_MULTI_ARCH_HAL",
            KernelFeature::LiveProcessMigration => "CONFIG_LIVE_PROC_MIGRATION",
            KernelFeature::AiKernelGovernor => "CONFIG_AI_KERNEL_GOVERNOR",
        }
    }
}

// ─── Changelog Entry ──────────────────────────────────────────────────────────

/// A single changelog entry (inspired by Debian `changelog` format)
#[derive(Debug, Clone)]
pub struct ChangelogEntry {
    /// Version this entry belongs to
    pub version: String,
    /// ISO 8601 date of this entry
    pub date: String,
    /// Author name
    pub author: String,
    /// Category: Feature, Fix, Security, Performance, Breaking
    pub category: String,
    /// Human-readable change description
    pub description: String,
    /// Optional CVE ID if this is a security fix
    pub cve_id: Option<String>,
}

impl ChangelogEntry {
    /// Format as a human-readable string
    pub fn format(&self) -> String {
        let cve = match &self.cve_id {
            Some(cve) => format!(" [{}]", cve),
            None => String::new(),
        };
        format!(
            "[{}] {} ({}) — {}: {}{}",
            self.version, self.date, self.author, self.category, self.description, cve
        )
    }
}

// ─── Version Manager ─────────────────────────────────────────────────────────

/// SigmaOS Kernel Version Manager
///
/// Tracks version history, feature flags, ABI revisions, and changelog.
/// Inspired by Linux's `include/linux/version.h` and FreeBSD's `sys/param.h`.
pub struct SigmaOsVersionManager {
    /// Current kernel version
    pub current: SigmaOsVersion,
    /// Version history (oldest to newest)
    pub history: Vec<SigmaOsVersion>,
    /// Enabled kernel features
    pub features: Vec<KernelFeature>,
    /// Changelog entries indexed by version string
    pub changelog: BTreeMap<String, Vec<ChangelogEntry>>,
    /// ABI compatibility map: (old_abi, new_abi) -> compatible?
    pub abi_compat_matrix: BTreeMap<(u32, u32), bool>,
}

impl SigmaOsVersionManager {
    /// Create a new version manager with current SigmaOS version and full feature set
    pub fn new() -> Self {
        let current = SigmaOsVersion::current();
        let features = vec![
            KernelFeature::BoreScheduler,
            KernelFeature::EevdfScheduler,
            KernelFeature::IoUring,
            KernelFeature::EbpfRuntime,
            KernelFeature::XdpNetworking,
            KernelFeature::PostQuantumCrypto,
            KernelFeature::LandlockLsmV5,
            KernelFeature::PledgeEnforcement,
            KernelFeature::UnveilEnforcement,
            KernelFeature::CapsicumCapabilities,
            KernelFeature::UniversalSigpkg,
            KernelFeature::ZenithWayland,
            KernelFeature::DeclarativeState,
            KernelFeature::MultiArchHal,
        ];

        let mut mgr = SigmaOsVersionManager {
            current: current.clone(),
            history: Vec::new(),
            features,
            changelog: BTreeMap::new(),
            abi_compat_matrix: BTreeMap::new(),
        };

        // Seed initial changelog
        mgr.add_changelog(ChangelogEntry {
            version: String::from("1.0.0-rc1"),
            date: String::from("2026-09-17"),
            author: String::from("SigmaOS Contributors"),
            category: String::from("Feature"),
            description: String::from(
                "Initial SigmaOS 1.0 Release Candidate: BORE+EEVDF schedulers, \
                 io_uring, eBPF VM, Landlock v5, PQC Kyber/Dilithium, \
                 SigPkg universal package engine, Zenith Wayland compositor",
            ),
            cve_id: None,
        });

        // ABI self-compatibility
        mgr.abi_compat_matrix.insert((1, 1), true);

        mgr
    }

    /// Add a new changelog entry
    pub fn add_changelog(&mut self, entry: ChangelogEntry) {
        self.changelog
            .entry(entry.version.clone())
            .or_insert_with(Vec::new)
            .push(entry);
    }

    /// Enable a kernel feature
    pub fn enable_feature(&mut self, feature: KernelFeature) {
        if !self.features.contains(&feature) {
            self.features.push(feature);
        }
    }

    /// Check if a kernel feature is enabled
    pub fn has_feature(&self, feature: &KernelFeature) -> bool {
        self.features.contains(feature)
    }

    /// Print all enabled features (Linux `cat /proc/version` equivalent)
    pub fn version_banner(&self) -> String {
        let feature_list: Vec<&str> = self.features.iter().map(|f| f.config_symbol()).collect();
        format!(
            "{}\nEnabled: {}\nBuild: {} @ {} (rustc {})",
            self.current.uname_release(),
            feature_list.join(", "),
            self.current.commit_hash,
            self.current.build_timestamp,
            self.current.rustc_version
        )
    }

    /// Returns full changelog as formatted string
    pub fn format_changelog(&self) -> String {
        let mut out = String::new();
        for (ver, entries) in self.changelog.iter().rev() {
            out.push_str(&format!("=== {} ===\n", ver));
            for e in entries {
                out.push_str(&format!("  {}\n", e.format()));
            }
        }
        out
    }

    /// Compare two version codes — returns Ordering
    pub fn compare_versions(a: &SigmaOsVersion, b: &SigmaOsVersion) -> core::cmp::Ordering {
        a.version_code().cmp(&b.version_code())
    }

    /// Returns whether the current version is newer than `other`
    pub fn is_newer_than(&self, other: &SigmaOsVersion) -> bool {
        self.current.version_code() > other.version_code()
    }

    /// Bump patch version (for hotfix releases)
    pub fn bump_patch(&mut self) {
        self.history.push(self.current.clone());
        self.current.patch += 1;
    }

    /// Bump minor version (for feature releases)
    pub fn bump_minor(&mut self) {
        self.history.push(self.current.clone());
        self.current.minor += 1;
        self.current.patch = 0;
    }

    /// Register ABI compatibility between two revisions
    pub fn register_abi_compat(&mut self, old: u32, new: u32, compatible: bool) {
        self.abi_compat_matrix.insert((old, new), compatible);
    }

    /// Check if an old ABI revision is compatible with current
    pub fn check_abi_compat(&self, old_abi: u32) -> bool {
        self.abi_compat_matrix
            .get(&(old_abi, self.current.abi_revision))
            .copied()
            .unwrap_or(old_abi == self.current.abi_revision)
    }
}

// ─── /proc/version equivalent ────────────────────────────────────────────────

/// Returns the `/proc/version` equivalent string for SigmaOS
pub fn proc_version() -> String {
    let v = SigmaOsVersion::current();
    format!(
        "SigmaOS version {} ({}) (rustc {} {})",
        v.uname_release(),
        v.commit_hash,
        v.rustc_version,
        v.build_timestamp
    )
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod version_tests {
    use super::*;

    #[test]
    fn test_version_code_ordering() {
        let v100 = sigma_version_code(1, 0, 0);
        let v101 = sigma_version_code(1, 0, 1);
        let v110 = sigma_version_code(1, 1, 0);
        let v200 = sigma_version_code(2, 0, 0);
        assert!(v100 < v101);
        assert!(v101 < v110);
        assert!(v110 < v200);
    }

    #[test]
    fn test_uname_release() {
        let v = SigmaOsVersion::current();
        let uname = v.uname_release();
        assert!(uname.contains("SigmaOS"));
        assert!(uname.contains("1.0.0"));
        assert!(uname.contains("x86_64"));
    }

    #[test]
    fn test_bsd_version_string() {
        let v = SigmaOsVersion::current();
        let bsd = v.bsd_version_string();
        assert!(bsd.contains("SigmaOS/x86_64"));
    }

    #[test]
    fn test_abi_compatibility() {
        let a = SigmaOsVersion::current();
        let mut b = SigmaOsVersion::current();
        assert!(a.is_abi_compatible(&b));
        b.abi_revision = 99;
        assert!(!a.is_abi_compatible(&b));
    }

    #[test]
    fn test_feature_flags() {
        let mgr = SigmaOsVersionManager::new();
        assert!(mgr.has_feature(&KernelFeature::BoreScheduler));
        assert!(mgr.has_feature(&KernelFeature::PostQuantumCrypto));
        assert!(mgr.has_feature(&KernelFeature::UniversalSigpkg));
    }

    #[test]
    fn test_changelog() {
        let mgr = SigmaOsVersionManager::new();
        let log = mgr.format_changelog();
        assert!(log.contains("SigmaOS Contributors"));
        assert!(log.contains("BORE"));
    }

    #[test]
    fn test_proc_version() {
        let pv = proc_version();
        assert!(pv.starts_with("SigmaOS version"));
        assert!(pv.contains("rustc"));
    }

    #[test]
    fn test_version_bump() {
        let mut mgr = SigmaOsVersionManager::new();
        let initial_patch = mgr.current.patch;
        mgr.bump_patch();
        assert_eq!(mgr.current.patch, initial_patch + 1);
        assert_eq!(mgr.history.len(), 1);
    }

    #[test]
    fn test_release_type_suffix() {
        assert_eq!(ReleaseType::ReleaseCandidate(3).suffix(), "-rc3");
        assert_eq!(ReleaseType::Stable.suffix(), "");
        assert_eq!(
            ReleaseType::Lts { supported_until: String::from("2030-01") }.suffix(),
            "-lts(2030-01)"
        );
    }
}
