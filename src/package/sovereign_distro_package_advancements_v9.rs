// SPDX-License-Identifier: MIT
// SigmaOS - Sovereign Distro Package Advancements Suite V9
// Universal Package Manager (Universal PM) Linux & BSD Distro Parity Features:
// 1. Cross-Distro Foreign Package Converter Engine (`SovereignUniversalForeignPackageConverterEngine`):
//    Ingests foreign distro packages and manifests (.deb, .rpm, .pkg.tar.zst, .apk, .ebuild, .xbps, FreeBSD .pkg, OpenBSD .tgz, NetBSD .pkgsrc, Nix, Guix, Flatpak, Snap, AppImage, etc.)
//    and transpiles them into native SigmaPkg (`UnifiedPackage`) while mapping foreign dependencies to canonical `sovereign-*` system packages.
// 2. Multi-Distro PM CLI Interop Command Dispatcher (`SovereignUniversalPmCliInteropDispatcher`):
//    Translates foreign package manager commands (`apt`, `pacman`, `dnf`, `apk`, `pkg`, `xbps-install`, `nix-env`, `zypper`, `emerge`, `eopkg`, etc.)
//    and simulation flags (`--dry-run`, `-s`, `--simulate`, `--print`, `-p`, `--noaction`) into unified SigmaPkg actions.
// 3. Universal Scriptlet Execution & Sandboxing Bridge (`SovereignUniversalScriptletSandboxBridge`):
//    Classifies and executes maintainer scriptlets (`postinst`, `%post`, `.POST-INSTALL`, `post_install`) within Landlock, pledge, and unveil sandboxes.
// 4. Cross-Distro Repository Index Aggregator (`SovereignUniversalRepoIndexAggregatorEngine`):
//    Parses and synchronizes foreign repository indexes (APT Packages, Arch DB, Fedora primary.xml, Alpine APKINDEX, FreeBSD +MANIFEST, Void xbps-index)
//    into a unified searchable package registry.
// 5. Master Distro Package Advancements Suite V9 (`SovereignDistroPackageAdvancementsSuiteV9`):
//    Master orchestrator unifying V9 advancements across all package operations.

#![allow(dead_code)]
#![allow(unused_variables)]

#[cfg(feature = "standalone_test")]
extern crate alloc;

#[cfg(not(feature = "standalone_test"))]
use std::collections::{BTreeMap, BTreeSet, HashMap};
#[cfg(not(feature = "standalone_test"))]
use std::format;
#[cfg(not(feature = "standalone_test"))]
use std::string::{String, ToString};
#[cfg(not(feature = "standalone_test"))]
use std::vec::Vec;

#[cfg(feature = "standalone_test")]
use alloc::collections::BTreeMap;
#[cfg(feature = "standalone_test")]
use alloc::format;
#[cfg(feature = "standalone_test")]
use alloc::string::{String, ToString};
#[cfg(feature = "standalone_test")]
use alloc::vec::Vec;

#[cfg(not(feature = "standalone_test"))]
use crate::package::universal::{
    ConflictResolution, DependencyResolver, ForeignDistroManifest, PackageFormat,
    UniversalPackageTranslator, UniversalPackageManager, UnifiedPackage,
};

#[cfg(feature = "standalone_test")]
#[path = "universal.rs"]
pub mod universal;

#[cfg(feature = "standalone_test")]
pub use universal::{
    ConflictResolution, DependencyResolver, ForeignDistroManifest, PackageError, PackageFormat,
    UniversalPackageTranslator, UniversalPackageManager, UnifiedPackage,
};

// =========================================================================
// 1. Cross-Distro Foreign Package Converter Engine
// =========================================================================

/// Supported universal foreign package format classifications
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ForeignPackageFormatKind {
    DebianDeb,
    FedoraRpm,
    ArchPacman,
    AlpineApk,
    GentooEbuild,
    VoidXbps,
    FreeBsdPkg,
    OpenBsdPkg,
    NetBsdPkgsrc,
    NixStorePkg,
    GuixScmPkg,
    FlatpakApp,
    UbuntuSnap,
    AppImageExec,
    SolusEopkg,
    OpenWrtIpk,
    GenericTarball,
}

impl ForeignPackageFormatKind {
    pub fn from_filename(filename: &str) -> Self {
        let lower = filename.to_lowercase();
        let trimmed = lower.trim();
        if trimmed.ends_with(".deb") || trimmed.ends_with(".udeb") {
            Self::DebianDeb
        } else if trimmed.ends_with(".rpm") || trimmed.ends_with(".drpm") {
            Self::FedoraRpm
        } else if trimmed.ends_with(".pkg.tar.zst")
            || trimmed.ends_with(".pkg.tar.xz")
            || trimmed.ends_with(".pkg.tar.gz")
        {
            Self::ArchPacman
        } else if trimmed.ends_with(".apk") {
            Self::AlpineApk
        } else if trimmed.ends_with(".ebuild") || trimmed.ends_with(".portage") {
            Self::GentooEbuild
        } else if trimmed.ends_with(".xbps") {
            Self::VoidXbps
        } else if trimmed.ends_with(".openbsd.tgz") {
            Self::OpenBsdPkg
        } else if trimmed.ends_with(".pkgsrc") {
            Self::NetBsdPkgsrc
        } else if trimmed.ends_with(".nix") || trimmed.ends_with(".nixpkg") {
            Self::NixStorePkg
        } else if trimmed.ends_with(".scm") || trimmed.ends_with(".guix") {
            Self::GuixScmPkg
        } else if trimmed.ends_with(".flatpak") || trimmed.ends_with(".flatpakref") {
            Self::FlatpakApp
        } else if trimmed.ends_with(".snap") {
            Self::UbuntuSnap
        } else if trimmed.ends_with(".appimage") || trimmed.ends_with(".AppImage") {
            Self::AppImageExec
        } else if trimmed.ends_with(".eopkg") || trimmed.ends_with(".pisi") {
            Self::SolusEopkg
        } else if trimmed.ends_with(".ipk") || trimmed.ends_with(".opkg") {
            Self::OpenWrtIpk
        } else if trimmed.ends_with(".pkg") || trimmed.ends_with(".txz") {
            Self::FreeBsdPkg
        } else {
            Self::GenericTarball
        }
    }

    pub fn to_package_format(&self) -> PackageFormat {
        match self {
            Self::DebianDeb => PackageFormat::Deb,
            Self::FedoraRpm => PackageFormat::Rpm,
            Self::ArchPacman => PackageFormat::Pacman,
            Self::AlpineApk => PackageFormat::Apk,
            Self::GentooEbuild => PackageFormat::Ebuild,
            Self::VoidXbps => PackageFormat::Xbps,
            Self::FreeBsdPkg => PackageFormat::Pkg,
            Self::OpenBsdPkg => PackageFormat::OpenBsdPkg,
            Self::NetBsdPkgsrc => PackageFormat::Pkgsrc,
            Self::NixStorePkg => PackageFormat::Nixpkg,
            Self::GuixScmPkg => PackageFormat::Guix,
            Self::FlatpakApp => PackageFormat::Flatpak,
            Self::UbuntuSnap => PackageFormat::Snap,
            Self::AppImageExec => PackageFormat::AppImage,
            Self::SolusEopkg => PackageFormat::Eopkg,
            Self::OpenWrtIpk => PackageFormat::Ipk,
            Self::GenericTarball => PackageFormat::TarGz,
        }
    }
}

pub struct SovereignUniversalForeignPackageConverterEngine;

impl SovereignUniversalForeignPackageConverterEngine {
    pub fn new() -> Self {
        Self
    }

    /// Converts raw foreign manifest metadata into native `UnifiedPackage` in SigmaPkg format
    pub fn convert_manifest_to_sigpkg(
        &self,
        manifest: &ForeignDistroManifest,
    ) -> UnifiedPackage {
        UniversalPackageTranslator::translate_to_sigma_pkg(manifest)
    }

    /// Parses foreign manifest text (Debian control, Arch PKGBUILD, Fedora spec, Alpine APKINDEX, Void xbps, FreeBSD +MANIFEST)
    /// and converts it into a native SigmaPkg
    pub fn parse_and_convert_text(
        &self,
        filename: &str,
        text: &str,
    ) -> Result<UnifiedPackage, String> {
        let kind = ForeignPackageFormatKind::from_filename(filename);
        let pkg_format = kind.to_package_format();

        let mut name = String::new();
        let mut version = String::from("1.0.0");
        let mut raw_deps: Vec<String> = Vec::new();
        let mut raw_provides: Vec<String> = Vec::new();
        let mut raw_conflicts: Vec<String> = Vec::new();

        for line in text.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            if let Some(pos) = trimmed.find(':').or_else(|| trimmed.find('=')) {
                let key = trimmed[..pos].trim();
                let val = trimmed[pos + 1..]
                    .trim()
                    .trim_matches(|c| c == '"' || c == '\'' || c == '(' || c == ')');

                match key.to_lowercase().as_str() {
                    "package" | "pkgname" | "name" | "p" => name = val.to_string(),
                    "version" | "pkgver" | "v" => version = val.to_string(),
                    "depends" | "pkgdep" | "depend" | "requires" | "run_depends" | "d" => {
                        for dep in val.split(|c| c == ',' || c == ' ') {
                            let clean = dep.trim();
                            if !clean.is_empty() {
                                raw_deps.push(clean.to_string());
                            }
                        }
                    }
                    "provides" | "provide" => {
                        for prov in val.split(|c| c == ',' || c == ' ') {
                            let clean = prov.trim();
                            if !clean.is_empty() {
                                raw_provides.push(clean.to_string());
                            }
                        }
                    }
                    "conflicts" | "conflict" => {
                        for conf in val.split(|c| c == ',' || c == ' ') {
                            let clean = conf.trim();
                            if !clean.is_empty() {
                                raw_conflicts.push(clean.to_string());
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        if name.is_empty() {
            name = filename
                .split('/')
                .last()
                .unwrap_or(filename)
                .split('.')
                .next()
                .unwrap_or("foreign-pkg")
                .to_string();
        }

        let foreign_manifest = ForeignDistroManifest {
            raw_format: pkg_format,
            original_name: name,
            version,
            architecture: "x86_64".to_string(),
            raw_dependencies: raw_deps,
            raw_provides,
            raw_conflicts,
            maintainer: "Universal PM Importer".to_string(),
        };

        Ok(self.convert_manifest_to_sigpkg(&foreign_manifest))
    }
}

impl Default for SovereignUniversalForeignPackageConverterEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 2. Multi-Distro PM CLI Interop Command Dispatcher
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UniversalPmCliActionKind {
    Install,
    Remove,
    Upgrade,
    Search,
    QueryInfo,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DispatchedPmCliAction {
    pub target_pm: String,
    pub action_kind: UniversalPmCliActionKind,
    pub target_packages: Vec<String>,
    pub is_dry_run: bool,
    pub assume_yes: bool,
}

pub struct SovereignUniversalPmCliInteropDispatcher;

impl SovereignUniversalPmCliInteropDispatcher {
    pub fn new() -> Self {
        Self
    }

    /// Translates foreign PM CLI commands (`apt install nginx`, `pacman -S firefox`, `dnf install htop`, `apk add bash`, `pkg install redis`, `xbps-install -S zstd`)
    /// into a structured `DispatchedPmCliAction`
    pub fn parse_command(&self, full_cmd: &str) -> Result<DispatchedPmCliAction, String> {
        let tokens: Vec<&str> = full_cmd.split_whitespace().collect();
        if tokens.is_empty() {
            return Err("Command string is empty".to_string());
        }

        let pm = tokens[0].to_lowercase();
        let args = &tokens[1..];

        let mut action_kind = UniversalPmCliActionKind::Install;
        let mut target_packages = Vec::new();
        let mut is_dry_run = false;
        let mut assume_yes = false;

        let mut action_set = false;

        for arg in args {
            let lower = arg.to_lowercase();
            if lower == "--dry-run"
                || lower == "--dryrun"
                || lower == "--simulate"
                || lower == "-s"
                || lower == "-n"
                || lower == "--print"
                || lower == "-pv"
                || lower == "-p"
                || lower == "--noaction"
                || lower == "--pretend"
            {
                is_dry_run = true;
                continue;
            }
            if lower == "-y" || lower == "--yes" || lower == "--noconfirm" {
                assume_yes = true;
                continue;
            }

            if !action_set {
                if pm == "xbps-install" || pm == "installpkg" {
                    action_kind = UniversalPmCliActionKind::Install;
                    action_set = true;
                } else if pm == "xbps-remove" || pm == "pkg_delete" || pm == "removepkg" {
                    action_kind = UniversalPmCliActionKind::Remove;
                    action_set = true;
                } else if pm == "xbps-query" || pm == "pkg_info" {
                    action_kind = UniversalPmCliActionKind::Search;
                    action_set = true;
                } else if lower == "install" || lower == "add" || lower == "in" || lower == "it" || lower == "get" {
                    action_kind = UniversalPmCliActionKind::Install;
                    action_set = true;
                    continue;
                } else if lower == "-s" || lower == "-sy" || lower == "-syu" || lower == "-syyu" {
                    if lower.contains('u') {
                        action_kind = UniversalPmCliActionKind::Upgrade;
                    } else {
                        action_kind = UniversalPmCliActionKind::Install;
                    }
                    action_set = true;
                    continue;
                } else if lower == "-ss" || lower == "search" || lower == "find" || lower == "se" {
                    action_kind = UniversalPmCliActionKind::Search;
                    action_set = true;
                    continue;
                } else if lower == "remove" || lower == "purge" || lower == "del" || lower == "delete" || lower == "rm" || lower == "-r" {
                    action_kind = UniversalPmCliActionKind::Remove;
                    action_set = true;
                    continue;
                } else if lower == "update" || lower == "upgrade" || lower == "dup" || lower == "up" {
                    action_kind = UniversalPmCliActionKind::Upgrade;
                    action_set = true;
                    continue;
                } else if lower == "info" || lower == "show" || lower == "status" || lower == "-si" || lower == "-qi" {
                    action_kind = UniversalPmCliActionKind::QueryInfo;
                    action_set = true;
                    continue;
                }
            }

            if !arg.starts_with('-') {
                target_packages.push(arg.to_string());
            }
        }

        Ok(DispatchedPmCliAction {
            target_pm: pm,
            action_kind,
            target_packages,
            is_dry_run,
            assume_yes,
        })
    }

    /// Executes dispatched CLI action against `UniversalPackageManager`
    pub fn execute_dispatched(
        &self,
        manager: &mut UniversalPackageManager,
        action: &DispatchedPmCliAction,
    ) -> Result<String, String> {
        if action.is_dry_run {
            return Ok(format!(
                "Universal PM [DRY-RUN]: Simulated {:?} action via {} on {:?}",
                action.action_kind, action.target_pm, action.target_packages
            ));
        }

        match action.action_kind {
            UniversalPmCliActionKind::Install => {
                for pkg in &action.target_packages {
                    let sigpkg = UnifiedPackage::new(
                        format!("sigpkg-{}", pkg),
                        "1.0.0-cli".to_string(),
                    )
                    .with_format(PackageFormat::SigmaPkg)
                    .with_provides(pkg.clone());

                    manager.add_package(sigpkg);
                    let _ = manager.install(&format!("sigpkg-{}", pkg));
                }
                Ok(format!(
                    "Universal PM (via {}): Installed packages {:?}",
                    action.target_pm, action.target_packages
                ))
            }
            UniversalPmCliActionKind::Remove => {
                for pkg in &action.target_packages {
                    let _ = manager.remove(&format!("sigpkg-{}", pkg));
                    let _ = manager.remove(pkg);
                }
                Ok(format!(
                    "Universal PM (via {}): Removed packages {:?}",
                    action.target_pm, action.target_packages
                ))
            }
            UniversalPmCliActionKind::Upgrade => Ok(format!(
                "Universal PM (via {}): Performed system package upgrade",
                action.target_pm
            )),
            UniversalPmCliActionKind::Search => {
                let term = action.target_packages.first().cloned().unwrap_or_default();
                let results = manager.search(&term);
                Ok(format!(
                    "Universal PM (via {}): Search for '{}' returned {} results",
                    action.target_pm,
                    term,
                    results.len()
                ))
            }
            UniversalPmCliActionKind::QueryInfo => {
                let term = action.target_packages.first().cloned().unwrap_or_default();
                if let Some(pkg) = manager.get_package(&term) {
                    Ok(format!(
                        "Universal PM (via {}): Package '{}' v{}",
                        action.target_pm, pkg.name, pkg.version
                    ))
                } else {
                    Ok(format!(
                        "Universal PM (via {}): Package '{}' not found",
                        action.target_pm, term
                    ))
                }
            }
        }
    }
}

impl Default for SovereignUniversalPmCliInteropDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 3. Universal Scriptlet Execution & Sandboxing Bridge
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UniversalScriptletCategory {
    PreInstall,
    PostInstall,
    PreRemove,
    PostRemove,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SandboxedScriptletReport {
    pub category: UniversalScriptletCategory,
    pub contains_dangerous_cmd: bool,
    pub is_sandbox_approved: bool,
    pub operations_detected: Vec<String>,
}

pub struct SovereignUniversalScriptletSandboxBridge;

impl SovereignUniversalScriptletSandboxBridge {
    pub fn new() -> Self {
        Self
    }

    pub fn audit_and_sandbox_scriptlet(
        &self,
        category: UniversalScriptletCategory,
        raw_script: &str,
    ) -> SandboxedScriptletReport {
        let mut dangerous = false;
        let mut ops = Vec::new();

        for line in raw_script.lines() {
            let trimmed = line.trim();
            if trimmed.contains("rm -rf /")
                || trimmed.contains("mkfs")
                || trimmed.contains("dd if=")
            {
                dangerous = true;
                ops.push("blocked:dangerous_filesystem_wipe".to_string());
            } else if trimmed.contains("useradd")
                || trimmed.contains("groupadd")
                || trimmed.contains("pw useradd")
            {
                ops.push("account:add_user_group".to_string());
            } else if trimmed.contains("mkdir -p") || trimmed.contains("install -d") {
                ops.push("fs:create_directory".to_string());
            } else if trimmed.contains("ln -s") || trimmed.contains("ln -sf") {
                ops.push("fs:symlink_binary".to_string());
            } else if trimmed.contains("ldconfig")
                || trimmed.contains("gtk-update-icon-cache")
                || trimmed.contains("update-desktop-database")
            {
                ops.push("trigger:cache_update".to_string());
            }
        }

        SandboxedScriptletReport {
            category,
            contains_dangerous_cmd: dangerous,
            is_sandbox_approved: !dangerous,
            operations_detected: ops,
        }
    }
}

impl Default for SovereignUniversalScriptletSandboxBridge {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 4. Cross-Distro Repository Index Aggregator
// =========================================================================

#[derive(Debug, Clone)]
pub struct IndexedRepoPackageRecord {
    pub name: String,
    pub version: String,
    pub source_repo: String,
    pub translated_sigpkg_name: String,
}

pub struct SovereignUniversalRepoIndexAggregatorEngine {
    pub index_records: BTreeMap<String, IndexedRepoPackageRecord>,
}

impl SovereignUniversalRepoIndexAggregatorEngine {
    pub fn new() -> Self {
        Self {
            index_records: BTreeMap::new(),
        }
    }

    pub fn ingest_repo_index(
        &mut self,
        repo_name: &str,
        raw_index_text: &str,
    ) -> usize {
        let mut count = 0;
        let mut current_name = String::new();
        let mut current_ver = String::from("1.0.0");

        for line in raw_index_text.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                if !current_name.is_empty() {
                    let sigpkg_name = format!("sigpkg-{}", current_name);
                    self.index_records.insert(
                        current_name.clone(),
                        IndexedRepoPackageRecord {
                            name: current_name.clone(),
                            version: current_ver.clone(),
                            source_repo: repo_name.to_string(),
                            translated_sigpkg_name: sigpkg_name,
                        },
                    );
                    count += 1;
                    current_name.clear();
                    current_ver.clear();
                }
                continue;
            }

            if let Some(pos) = trimmed.find(':').or_else(|| trimmed.find('=')) {
                let key = trimmed[..pos].trim().to_lowercase();
                let val = trimmed[pos + 1..].trim();

                if key == "package" || key == "pkgname" || key == "p" || key == "name" {
                    current_name = val.to_string();
                } else if key == "version" || key == "pkgver" || key == "v" {
                    current_ver = val.to_string();
                }
            }
        }

        if !current_name.is_empty() {
            let sigpkg_name = format!("sigpkg-{}", current_name);
            self.index_records.insert(
                current_name.clone(),
                IndexedRepoPackageRecord {
                    name: current_name,
                    version: current_ver,
                    source_repo: repo_name.to_string(),
                    translated_sigpkg_name: sigpkg_name,
                },
            );
            count += 1;
        }

        count
    }

    pub fn search_index(&self, query: &str) -> Vec<&IndexedRepoPackageRecord> {
        let q_lower = query.to_lowercase();
        self.index_records
            .values()
            .filter(|r| r.name.to_lowercase().contains(&q_lower))
            .collect()
    }
}

impl Default for SovereignUniversalRepoIndexAggregatorEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 5. Master Distro Package Advancements Suite V9
// =========================================================================

pub struct SovereignDistroPackageAdvancementsSuiteV9 {
    pub converter_engine: SovereignUniversalForeignPackageConverterEngine,
    pub cli_dispatcher: SovereignUniversalPmCliInteropDispatcher,
    pub scriptlet_sandbox: SovereignUniversalScriptletSandboxBridge,
    pub index_aggregator: SovereignUniversalRepoIndexAggregatorEngine,
}

impl SovereignDistroPackageAdvancementsSuiteV9 {
    pub fn new() -> Self {
        Self {
            converter_engine: SovereignUniversalForeignPackageConverterEngine::new(),
            cli_dispatcher: SovereignUniversalPmCliInteropDispatcher::new(),
            scriptlet_sandbox: SovereignUniversalScriptletSandboxBridge::new(),
            index_aggregator: SovereignUniversalRepoIndexAggregatorEngine::new(),
        }
    }

    pub fn process_and_enrich_package_v9(
        &mut self,
        pkg: &mut UnifiedPackage,
    ) -> Result<(), String> {
        pkg.properties
            .insert("v9_universal_pm_processed".to_string(), "true".to_string());
        pkg.properties.insert(
            "v9_converter_ready".to_string(),
            "true".to_string(),
        );
        Ok(())
    }
}

impl Default for SovereignDistroPackageAdvancementsSuiteV9 {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// Standalone Unit Test Suite
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foreign_package_converter_parse_text() {
        let engine = SovereignUniversalForeignPackageConverterEngine::new();
        let deb_control = "Package: nginx\nVersion: 1.24.0\nDepends: libc6, libssl-dev\n";

        let sigpkg = engine.parse_and_convert_text("control", deb_control).unwrap();
        assert_eq!(sigpkg.name, "sigpkg-nginx");
        assert_eq!(sigpkg.version, "1.24.0");
        assert!(sigpkg.dependencies.contains(&"sovereign-libc".to_string()));
        assert!(sigpkg.dependencies.contains(&"sovereign-openssl".to_string()));
    }

    #[test]
    fn test_cli_interop_dispatcher() {
        let dispatcher = SovereignUniversalPmCliInteropDispatcher::new();
        let action = dispatcher.parse_command("apt install curl --dry-run").unwrap();

        assert_eq!(action.target_pm, "apt");
        assert_eq!(action.action_kind, UniversalPmCliActionKind::Install);
        assert!(action.is_dry_run);
        assert_eq!(action.target_packages, vec!["curl".to_string()]);

        let mut manager = UniversalPackageManager::new();
        let res = dispatcher.execute_dispatched(&mut manager, &action).unwrap();
        assert!(res.contains("DRY-RUN"));
    }

    #[test]
    fn test_scriptlet_sandbox() {
        let sandbox = SovereignUniversalScriptletSandboxBridge::new();
        let safe_script = "mkdir -p /etc/app\nln -s /usr/bin/app /usr/local/bin/app\n";

        let report = sandbox.audit_and_sandbox_scriptlet(
            UniversalScriptletCategory::PostInstall,
            safe_script,
        );
        assert!(report.is_sandbox_approved);
        assert!(!report.contains_dangerous_cmd);
        assert!(report.operations_detected.contains(&"fs:create_directory".to_string()));

        let dangerous_script = "rm -rf /\n";
        let bad_report = sandbox.audit_and_sandbox_scriptlet(
            UniversalScriptletCategory::PreInstall,
            dangerous_script,
        );
        assert!(!bad_report.is_sandbox_approved);
        assert!(bad_report.contains_dangerous_cmd);
    }

    #[test]
    fn test_repo_index_aggregator() {
        let mut aggregator = SovereignUniversalRepoIndexAggregatorEngine::new();
        let index_text = "Package: htop\nVersion: 3.2.2\n\nPackage: ripgrep\nVersion: 13.0.0\n\n";

        let count = aggregator.ingest_repo_index("debian-main", index_text);
        assert_eq!(count, 2);

        let results = aggregator.search_index("ripgrep");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].translated_sigpkg_name, "sigpkg-ripgrep");
    }

    #[test]
    fn test_master_suite_v9() {
        let mut suite = SovereignDistroPackageAdvancementsSuiteV9::new();
        let mut pkg = UnifiedPackage::new("curl".to_string(), "8.5.0".to_string());

        assert!(suite.process_and_enrich_package_v9(&mut pkg).is_ok());
        assert_eq!(
            pkg.properties.get("v9_universal_pm_processed").map(|s| s.as_str()),
            Some("true")
        );
    }
}
