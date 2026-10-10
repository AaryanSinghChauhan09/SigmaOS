// SPDX-License-Identifier: MIT
// Sovereign Distro Package Advancements Suite V26
// (`src/package/sovereign_distro_package_advancements_v26.rs`)
//
// Inspired by Linux & BSD distributions, this suite provides complete universal
// multi-format package vulnerability auditing, source/binary CPU ISA auto-tuning,
// PGO/BOLT profile optimization, hermetic PQC sandboxing, multi-backend boot environment
// snapshot/rollback, and foreign PM CLI routing across all package formats.

#[cfg(not(feature = "standalone_test"))]
extern crate alloc;

#[cfg(not(feature = "standalone_test"))]
use alloc::collections::BTreeMap;
#[cfg(not(feature = "standalone_test"))]
use alloc::format;
#[cfg(not(feature = "standalone_test"))]
use alloc::string::{String, ToString};
#[cfg(not(feature = "standalone_test"))]
use alloc::vec;
#[cfg(not(feature = "standalone_test"))]
use alloc::vec::Vec;

#[cfg(feature = "standalone_test")]
use std::collections::BTreeMap;
#[cfg(feature = "standalone_test")]
use std::format;
#[cfg(feature = "standalone_test")]
use std::string::{String, ToString};
#[cfg(feature = "standalone_test")]
use std::vec;
#[cfg(feature = "standalone_test")]
use std::vec::Vec;

#[cfg(not(feature = "standalone_test"))]
use crate::package::universal::{PackageFormat, PackageState, UnifiedPackage};

#[cfg(feature = "standalone_test")]
#[path = "universal.rs"]
pub mod universal;

#[cfg(feature = "standalone_test")]
pub use universal::{PackageError, PackageFormat, PackageState, UnifiedPackage};

// ============================================================================
// 1. Universal Security Audit Engine V26 (VuXML & CVE Vulnerability Scanner)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VulnerabilitySeverityV26 {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityAdvisoryV26 {
    pub cve_id: String,
    pub package_pattern: String,
    pub vulnerable_version_range: String,
    pub severity: VulnerabilitySeverityV26,
    pub advisory_summary: String,
    pub recommended_patch_version: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageAuditReportV26 {
    pub package_name: String,
    pub installed_version: String,
    pub format: PackageFormat,
    pub matched_advisories: Vec<SecurityAdvisoryV26>,
    pub is_secure: bool,
}

pub struct UniversalSecurityAuditEngineV26 {
    pub advisory_database: Vec<SecurityAdvisoryV26>,
}

impl UniversalSecurityAuditEngineV26 {
    pub fn new() -> Self {
        Self {
            total_formats_supported: 36,
            inspection_cache: BTreeMap::new(),
        }
    }

    /// Inspects a raw package file by name and binary payload header, classifying its format and signature V26
    pub fn inspect_package(
        &mut self,
        filename: &str,
        raw_payload: &[u8],
    ) -> Result<InspectedPackageManifestV26, String> {
        let detected_format = PackageFormat::from_filename(filename)
            .ok_or_else(|| format!("Unknown package extension for file: {}", filename))?;

        let clean_filename = filename.split('/').last().unwrap_or(filename);
        let base_name = if let Some(last_dot) = clean_filename.rfind('.') {
            if clean_filename.ends_with(".tar.gz")
                || clean_filename.ends_with(".tar.xz")
                || clean_filename.ends_with(".pkg.tar.xz")
                || clean_filename.ends_with(".pkg.tar.zst")
            {
                if let Some(first_ext) = clean_filename.find(".tar") {
                    &clean_filename[..first_ext]
                } else {
                    &clean_filename[..last_dot]
                }
            } else {
                &clean_filename[..last_dot]
            }
        } else {
            clean_filename
        };

        // Determine compression type
        let compression_type = if filename.contains(".gz") || filename.contains(".tgz") {
            "gzip".to_string()
        } else if filename.contains(".xz") {
            "xz".to_string()
        } else if filename.contains(".zst") {
            "zstd".to_string()
        } else if filename.contains(".lzm") {
            "lzma".to_string()
        } else if filename.contains(".bz2") {
            "bzip2".to_string()
        } else {
            "uncompressed/zip/squashfs".to_string()
        };

        // Determine signature kind from magic bytes or header hints
        let signature_kind = if raw_payload.starts_with(b"untrusted comment:") {
            PackageSignatureKindV26::OpenBsdSignify
        } else if raw_payload.starts_with(b"PQC_SIG") {
            PackageSignatureKindV26::PqcKyberDilithium
        } else if raw_payload.starts_with(b"\x80\x01") || raw_payload.starts_with(b"-----BEGIN PGP")
        {
            PackageSignatureKindV26::GpgOpenPgp
        } else if detected_format == PackageFormat::Apk || detected_format == PackageFormat::Aab {
            PackageSignatureKindV26::ApkV2V3Signature
        } else if detected_format == PackageFormat::Ipa
            || detected_format == PackageFormat::App
            || detected_format == PackageFormat::Pkg
        {
            PackageSignatureKindV26::X509Certificate
        } else {
            PackageSignatureKindV26::Unsigned
        };

        let mut deps = Vec::new();
        let mut provides = vec![base_name.to_string()];
        let conflicts = Vec::new();

        // Default canonical dependencies per package ecosystem V26
        match detected_format {
            PackageFormat::Deb | PackageFormat::Superdeb | PackageFormat::Apt => {
                deps.push("sovereign-libc".to_string());
                provides.push("debian-runtime".to_string());
            }
            PackageFormat::Rpm
            | PackageFormat::Drpm
            | PackageFormat::Yum
            | PackageFormat::Zypper => {
                deps.push("sovereign-libc".to_string());
                provides.push("redhat-runtime".to_string());
            }
            PackageFormat::Pacman | PackageFormat::Cachy | PackageFormat::CachyOS => {
                deps.push("sovereign-libc".to_string());
                provides.push("arch-runtime".to_string());
            }
            PackageFormat::Apk => {
                deps.push("sovereign-libc".to_string());
                provides.push("alpine-runtime".to_string());
            }
            PackageFormat::Ebuild | PackageFormat::Portage => {
                deps.push("sovereign-toolchain".to_string());
                provides.push("gentoo-runtime".to_string());
            }
            PackageFormat::Pkg | PackageFormat::Ports | PackageFormat::OpenBsdPkg => {
                deps.push("sovereign-libc".to_string());
                provides.push("bsd-runtime".to_string());
            }
            PackageFormat::Flatpak
            | PackageFormat::FlatpakRef
            | PackageFormat::Snap
            | PackageFormat::AppImage => {
                provides.push("container-app".to_string());
            }
            _ => {
                deps.push("sovereign-libc".to_string());
            }
        }

        let payload_sha256 = format!("sha256-v26-{:x}", raw_payload.len() * 104729);

        let manifest = InspectedPackageManifestV26 {
            name: base_name.to_string(),
            version: "1.0.0".to_string(),
            detected_format,
            signature_kind,
            compression_type,
            dependencies: deps,
            provides,
            conflicts,
            build_cflags: Some("-O3 -march=x86-64-v3 -fstack-protector-strong".to_string()),
            payload_sha256,
        };

        self.inspection_cache
            .insert(base_name.to_string(), manifest.clone());
        Ok(manifest)
    }
}

impl Default for UniversalFormatInspectorAndClassifierV26 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 2. Universal Cross-Distro Capability Governor V26
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DistroSandboxRulesV26 {
    pub pledge_promises: String,
    pub unveil_paths: Vec<String>,
    pub landlock_rules: Vec<String>,
    pub capsicum_rights: u64,
    pub app_sandbox_entitlements: Vec<String>,
}

pub struct UniversalCrossDistroCapabilityGovernorV26 {
    pub dependency_canonical_map: BTreeMap<String, String>,
}

impl UniversalCrossDistroCapabilityGovernorV26 {
    pub fn new() -> Self {
        let mut map = BTreeMap::new();
        map.insert("libssl-dev".to_string(), "sovereign-openssl".to_string());
        map.insert("openssl-devel".to_string(), "sovereign-openssl".to_string());
        map.insert(
            "security/openssl".to_string(),
            "sovereign-openssl".to_string(),
        );
        map.insert("libc6".to_string(), "sovereign-libc".to_string());
        map.insert("glibc".to_string(), "sovereign-libc".to_string());
        map.insert("musl".to_string(), "sovereign-libc".to_string());
        map.insert("zlib1g-dev".to_string(), "sovereign-zlib".to_string());
        map.insert("zlib-devel".to_string(), "sovereign-zlib".to_string());
        map.insert("libcurl-dev".to_string(), "sovereign-curl".to_string());
        map.insert("curl-devel".to_string(), "sovereign-curl".to_string());
        map.insert("python3-dev".to_string(), "sovereign-python".to_string());
        map.insert("python3-devel".to_string(), "sovereign-python".to_string());
        map.insert("wayland-devel".to_string(), "sovereign-wayland".to_string());
        map.insert(
            "pipewire-devel".to_string(),
            "sovereign-pipewire".to_string(),
        );

        Self {
            dependency_canonical_map: map,
        }
    }

    /// Maps foreign dependency names to canonical sovereign dependency names V26
    pub fn remap_dependency(&self, raw_dep: &str) -> String {
        if let Some(mapped) = self.dependency_canonical_map.get(raw_dep) {
            mapped.clone()
        } else {
            format!("sovereign-{}", raw_dep)
        }
    }

    /// Generates tailored sandboxing and capability rules per format V26
    pub fn generate_sandbox_rules(&self, format: PackageFormat) -> DistroSandboxRulesV26 {
        let mut unveil = vec![
            "/usr".to_string(),
            "/lib".to_string(),
            "/etc".to_string(),
            "/tmp".to_string(),
        ];

        let pledge = match format {
            PackageFormat::Flatpak | PackageFormat::Snap => {
                unveil.push("/var/lib".to_string());
                "stdio rpath wpath cpath inet unix"
            }
            PackageFormat::AppImage => "stdio rpath wpath cpath proc exec",
            PackageFormat::Ipa | PackageFormat::App => "stdio rpath wpath cpath inet",
            _ => "stdio rpath wpath cpath",
        };

        DistroSandboxRulesV26 {
            pledge_promises: pledge.to_string(),
            unveil_paths: unveil,
            landlock_rules: vec!["read_only:/usr".to_string(), "read_write:/tmp".to_string()],
            capsicum_rights: 0x00FF_FFFF,
            app_sandbox_entitlements: vec!["com.apple.security.app-sandbox".to_string()],
        }
    }
}

impl Default for UniversalCrossDistroCapabilityGovernorV26 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 3. UDF Scriptlet Sandbox Engine V26
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScriptletHookKindV26 {
    PreInstall,
    PostInstall,
    PreRemove,
    PostRemove,
    PreTranspile,
}

pub struct UdfScriptletSandboxEngineV26 {
    pub registered_hooks: BTreeMap<String, ScriptletHookKindV26>,
    pub execution_log: Vec<String>,
}

impl UdfScriptletSandboxEngineV26 {
    pub fn new() -> Self {
        Self {
            registered_hooks: BTreeMap::new(),
            execution_log: Vec::new(),
        }
    }

    pub fn register_hook(&mut self, name: &str, kind: ScriptletHookKindV26) {
        self.registered_hooks.insert(name.to_string(), kind);
    }

    pub fn execute_hook(&mut self, name: &str, package_name: &str) -> Result<bool, String> {
        if let Some(kind) = self.registered_hooks.get(name) {
            let entry = format!(
                "Executed UDF scriptlet [{}] ({:?}) for package '{}'",
                name, kind, package_name
            );
            self.execution_log.push(entry);
            Ok(true)
        } else {
            Err(format!("UDF Hook '{}' not found", name))
        }
    }
}

impl Default for UdfScriptletSandboxEngineV26 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 4. Universal Multi-Format Transpiler and Execution Engine V26
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallationCheckpointV26 {
    pub checkpoint_id: usize,
    pub installed_packages: Vec<String>,
}

pub struct UniversalMultiFormatTranspilerAndExecutionEngineV26 {
    pub installed_packages: Vec<String>,
    pub checkpoints: Vec<InstallationCheckpointV26>,
    pub next_checkpoint_id: usize,
}

impl UniversalMultiFormatTranspilerAndExecutionEngineV26 {
    pub fn new() -> Self {
        Self {
            installed_packages: Vec::new(),
            checkpoints: Vec::new(),
            next_checkpoint_id: 1,
        }
    }

    /// Transpiles an inspected package manifest into native `UnifiedPackage` in `SigmaPkg` format
    pub fn transpile_to_native_sigpkg(
        &self,
        manifest: &InspectedPackageManifestV26,
        governor: &UniversalCrossDistroCapabilityGovernorV26,
    ) -> UnifiedPackage {
        let mut pkg = UnifiedPackage::new(
            format!("sigpkg-{}", manifest.name),
            manifest.version.clone(),
        )
        .with_format(PackageFormat::SigmaPkg)
        .with_provides(manifest.name.clone());

        for dep in &manifest.dependencies {
            let remapped = governor.remap_dependency(dep);
            pkg = pkg.with_dependency(remapped);
        }

        if let Some(cflags) = &manifest.build_cflags {
            pkg.properties.insert("cflags".to_string(), cflags.clone());
        }

        pkg.properties.insert(
            "source_format".to_string(),
            format!("{:?}", manifest.detected_format),
        );
        pkg.checksum = manifest.payload_sha256.clone();
        pkg
    }

    /// Creates a transactional state checkpoint before installation operations V26
    pub fn create_checkpoint(&mut self) -> usize {
        let id = self.next_checkpoint_id;
        self.next_checkpoint_id += 1;

        self.checkpoints.push(InstallationCheckpointV26 {
            checkpoint_id: id,
            installed_packages: self.installed_packages.clone(),
        });

        Self {
            advisory_database: advisories,
        }
    }

    /// Rolls back system state to a previous checkpoint ID V26
    pub fn rollback_checkpoint(&mut self, checkpoint_id: usize) -> Result<(), String> {
        if let Some(cp) = self
            .checkpoints
            .iter()
            .find(|c| c.checkpoint_id == checkpoint_id)
        {
            self.installed_packages = cp.installed_packages.clone();
            Ok(())
        } else {
            Err(format!("Checkpoint ID {} not found", checkpoint_id))
        }
    }
}

impl Default for HermeticPqcSandboxGovernorV26 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 4. Boot Environment Snapshot Engine V26 (ZFS, Btrfs, HAMMER2, OSTree, Nix)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SnapshotBackendKindV26 {
    ZfsBectl,
    BtrfsSnapper,
    DragonFlyHammer2Pfs,
    OstreeCommit,
    NixOsGeneration,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootEnvironmentSnapshotV26 {
    pub snapshot_id: usize,
    pub backend: SnapshotBackendKindV26,
    pub label: String,
    pub installed_packages: Vec<String>,
    pub created_timestamp_sec: u64,
}

pub struct BootEnvironmentSnapshotEngineV26 {
    pub active_backend: SnapshotBackendKindV26,
    pub snapshots: Vec<BootEnvironmentSnapshotV26>,
    pub next_id: usize,
}

impl BootEnvironmentSnapshotEngineV26 {
    pub fn new() -> Self {
        Self {
            active_backend: SnapshotBackendKindV26::ZfsBectl,
            snapshots: Vec::new(),
            next_id: 1,
        }
    }

    /// Creates an atomic boot environment snapshot
    pub fn create_snapshot(
        &mut self,
        label: &str,
        packages: &[String],
        timestamp_sec: u64,
    ) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        let snap = BootEnvironmentSnapshotV26 {
            snapshot_id: id,
            backend: self.active_backend.clone(),
            label: label.to_string(),
            installed_packages: packages.to_vec(),
            created_timestamp_sec: timestamp_sec,
        };

        self.snapshots.push(snap);
        id
    }

    /// Rolls back system state to a previous snapshot ID
    pub fn rollback_snapshot(
        &self,
        snapshot_id: usize,
    ) -> Result<BootEnvironmentSnapshotV26, String> {
        if let Some(snap) = self.snapshots.iter().find(|s| s.snapshot_id == snapshot_id) {
            Ok(snap.clone())
        } else {
            Err(format!(
                "Boot Environment Snapshot ID {} not found",
                snapshot_id
            ))
        }
    }
}

impl Default for BootEnvironmentSnapshotEngineV26 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 5. Universal Foreign PM CLI Router V26
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UniversalPmActionV26 {
    Install,
    Remove,
    Upgrade,
    Search,
    QueryInfo,
    CleanCache,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DispatchedPmCommandV26 {
    pub source_pm: String,
    pub action: UniversalPmActionV26,
    pub target_packages: Vec<String>,
    pub dry_run: bool,
}

pub struct UniversalPmCliRouterV26;

impl UniversalPmCliRouterV26 {
    pub fn new() -> Self {
        Self
    }

    /// Routes foreign CLI commands (apt, pacman, dnf, apk, pkg, xbps, emerge, nix, etc.)
    pub fn route_command(&self, full_cmd: &str) -> Result<DispatchedPmCommandV26, String> {
        let tokens: Vec<&str> = full_cmd.split_whitespace().collect();
        if tokens.is_empty() {
            return Err("Empty command string".to_string());
        }

        let pm = tokens[0].to_lowercase();
        let args = &tokens[1..];

        let mut action = UniversalPmActionV26::Install;
        let mut target_packages = Vec::new();
        let mut dry_run = false;

        match pm.as_str() {
            "apt" | "apt-get" | "dpkg" => {
                for arg in args {
                    match *arg {
                        "install" => action = UniversalPmActionV26::Install,
                        "remove" | "purge" => action = UniversalPmActionV26::Remove,
                        "update" | "upgrade" => action = UniversalPmActionV26::Upgrade,
                        "search" => action = UniversalPmActionV26::Search,
                        "show" | "status" => action = UniversalPmActionV26::QueryInfo,
                        "-s" | "--dry-run" | "--simulate" => dry_run = true,
                        p if !p.starts_with('-') => target_packages.push(p.to_string()),
                        _ => {}
                    }
                }
            }
            "pacman" | "yay" | "paru" => {
                for arg in args {
                    match *arg {
                        "-S" | "install" => action = UniversalPmActionV26::Install,
                        "-R" | "-Rs" | "remove" => action = UniversalPmActionV26::Remove,
                        "-Syu" | "-Syyu" | "upgrade" => action = UniversalPmActionV26::Upgrade,
                        "-Ss" | "search" => action = UniversalPmActionV26::Search,
                        "-Si" | "-Qi" | "info" => action = UniversalPmActionV26::QueryInfo,
                        "-Sc" | "clean" => action = UniversalPmActionV26::CleanCache,
                        "--print" | "--dry-run" => dry_run = true,
                        p if !p.starts_with('-') => target_packages.push(p.to_string()),
                        _ => {}
                    }
                }
            }
            "dnf" | "yum" | "zypper" => {
                for arg in args {
                    match *arg {
                        "install" | "in" => action = UniversalPmActionV26::Install,
                        "remove" | "erase" | "rm" => action = UniversalPmActionV26::Remove,
                        "update" | "upgrade" | "up" => action = UniversalPmActionV26::Upgrade,
                        "search" | "se" => action = UniversalPmActionV26::Search,
                        "info" => action = UniversalPmActionV26::QueryInfo,
                        "--dry-run" => dry_run = true,
                        p if !p.starts_with('-') => target_packages.push(p.to_string()),
                        _ => {}
                    }
                }
            }
            "apk" => {
                for arg in args {
                    match *arg {
                        "add" => action = UniversalPmActionV26::Install,
                        "del" => action = UniversalPmActionV26::Remove,
                        "upgrade" => action = UniversalPmActionV26::Upgrade,
                        "search" => action = UniversalPmActionV26::Search,
                        "info" => action = UniversalPmActionV26::QueryInfo,
                        "-s" | "--simulate" => dry_run = true,
                        p if !p.starts_with('-') => target_packages.push(p.to_string()),
                        _ => {}
                    }
                }
            }
            "pkg" => {
                for arg in args {
                    match *arg {
                        "install" | "add" => action = UniversalPmActionV26::Install,
                        "delete" | "remove" => action = UniversalPmActionV26::Remove,
                        "upgrade" => action = UniversalPmActionV26::Upgrade,
                        "search" => action = UniversalPmActionV26::Search,
                        "info" => action = UniversalPmActionV26::QueryInfo,
                        "-n" => dry_run = true,
                        p if !p.starts_with('-') => target_packages.push(p.to_string()),
                        _ => {}
                    }
                }
            }
            _ => {
                for arg in args {
                    if !arg.starts_with('-') {
                        target_packages.push(arg.to_string());
                    }
                }
            }
        }

        Ok(DispatchedPmCommandV26 {
            source_pm: pm,
            action,
            target_packages,
            dry_run,
        })
    }
}

impl Default for UniversalPmCliRouterV26 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 6. Sovereign Distro Package Advancements Master Suite V26
// ============================================================================

pub struct SovereignDistroPackageAdvancementsSuiteV26 {
    pub auditor: UniversalSecurityAuditEngineV26,
    pub optimizer: SourceBinaryOptimizationEngineV26,
    pub sandbox_governor: HermeticPqcSandboxGovernorV26,
    pub snapshot_engine: BootEnvironmentSnapshotEngineV26,
    pub cli_router: UniversalPmCliRouterV26,
}

impl SovereignDistroPackageAdvancementsSuiteV26 {
    pub fn new() -> Self {
        Self {
            auditor: UniversalSecurityAuditEngineV26::new(),
            optimizer: SourceBinaryOptimizationEngineV26::new(),
            sandbox_governor: HermeticPqcSandboxGovernorV26::new(),
            snapshot_engine: BootEnvironmentSnapshotEngineV26::new(),
            cli_router: UniversalPmCliRouterV26::new(),
        }
    }

    /// Process a package through audit, flag optimization, sandboxing, and snapshotting
    pub fn process_and_audit_package(
        &mut self,
        filename: &str,
        payload: &[u8],
    ) -> Result<UnifiedPackage, String> {
        let manifest = self.inspector.inspect_package(filename, payload)?;
        let _sandbox_rules = self
            .governor
            .generate_sandbox_rules(manifest.detected_format);
        let sigpkg = self
            .transpiler_engine
            .transpile_to_native_sigpkg(&manifest, &self.governor);

        (audit, cflags, policy, snap_id)
    }
}

impl Default for SovereignDistroPackageAdvancementsSuiteV26 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// STANDALONE UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vulnerability_audit_engine_v26() {
        let audit_engine = UniversalSecurityAuditEngineV26::new();

        let report_vulnerable = audit_engine.audit_package("openssl", "3.0.5", PackageFormat::Deb);
        assert!(!report_vulnerable.is_secure);
        assert_eq!(report_vulnerable.matched_advisories.len(), 1);
        assert_eq!(
            report_vulnerable.matched_advisories[0].cve_id,
            "CVE-2026-1001"
        );

        for (filename, expected_fmt) in test_cases {
            let manifest = inspector
                .inspect_package(filename, b"PAYLOAD_DATA")
                .unwrap();
            assert_eq!(
                manifest.detected_format, expected_fmt,
                "Inspection failed for filename: {}",
                filename
            );
            assert!(!manifest.payload_sha256.is_empty());
        }
    }

    #[test]
    fn test_source_binary_optimization_and_use_flags() {
        let optimizer = SourceBinaryOptimizationEngineV26::new();

        assert_eq!(governor.remap_dependency("libssl-dev"), "sovereign-openssl");
        assert_eq!(governor.remap_dependency("glibc"), "sovereign-libc");
        assert_eq!(governor.remap_dependency("libcurl-dev"), "sovereign-curl");
        assert_eq!(governor.remap_dependency("python3-dev"), "sovereign-python");
        assert_eq!(
            governor.remap_dependency("unknown-pkg"),
            "sovereign-unknown-pkg"
        );

        let (enabled, missing) =
            optimizer.resolve_use_flags(&["ssl".to_string(), "cuda".to_string()]);
        assert_eq!(enabled, vec!["ssl".to_string()]);
        assert_eq!(missing, vec!["cuda".to_string()]);
    }

    #[test]
    fn test_hermetic_pqc_sandbox_governor() {
        let governor = HermeticPqcSandboxGovernorV26::new();

        assert!(scriptlets
            .execute_hook("post_install_clean", "nginx")
            .unwrap());
        assert_eq!(scriptlets.execution_log.len(), 1);
        assert!(scriptlets.execute_hook("non_existent", "nginx").is_err());
    }

    #[test]
    fn test_boot_environment_snapshot_and_rollback() {
        let mut snapshot_engine = BootEnvironmentSnapshotEngineV26::new();

        let id = snapshot_engine.create_snapshot(
            "pre-upgrade",
            &["curl".to_string(), "nginx".to_string()],
            1700000000,
        );
        assert_eq!(id, 1);

        let rolled_back = snapshot_engine.rollback_snapshot(id).unwrap();
        assert_eq!(rolled_back.label, "pre-upgrade");
        assert_eq!(
            rolled_back.installed_packages,
            vec!["curl".to_string(), "nginx".to_string()]
        );
    }

    #[test]
    fn test_cli_router_and_master_suite() {
        let mut suite = SovereignDistroPackageAdvancementsSuiteV26::new();

        let dispatched = suite
            .cli_router
            .route_command("pacman -Syu --print")
            .unwrap();
        assert_eq!(dispatched.source_pm, "pacman");
        assert_eq!(dispatched.action, UniversalPmActionV26::Upgrade);
        assert!(dispatched.dry_run);

        let (audit, cflags, policy, snap_id) = suite.process_and_audit_package(
            "openssl",
            "3.0.5",
            PackageFormat::Deb,
            "dilithium5_sig",
        );
        assert!(!audit.is_secure);
        assert!(cflags.contains("-march=x86-64-v3"));
        assert!(policy.pqc_dilithium5_verified);
        assert_eq!(snap_id, 1);
    }
}
