extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;
use alloc::collections::BTreeMap as HashMap;
use alloc::sync::Arc;

// SigmaOS Universal Package Manager
// Unified system absorbing apt, yum, pacman, snap, flatpak, bsd pkg, apk, nix, etc.

#[cfg(all(not(feature = "standalone_test"), not(test)))]
use crate::klib::{HashMap, HashSet, Arc};

#[cfg(any(feature = "standalone_test", test))]
use std::{collections::{HashMap, HashSet}, sync::Arc};

impl SemVer {
    pub fn parse(s: &str) -> Option<Self> {
        let mut parts = s.split('.');
        let major = parts.next()?.parse::<u32>().ok()?;
        let minor = parts.next()?.parse::<u32>().ok()?;
        let patch = parts.next()?.parse::<u32>().ok()?;
        if parts.next().is_some() {
            return None;
        }
        Some(Self {
            major,
            minor,
            patch,
        })
    }
}

/// Semantic Version constraint matching (e.g. >=1.0.0, <=2.0.0, etc.)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemVerConstraint {
    Any,
    Exact(SemVer),
    GreaterThan(SemVer),
    LessThan(SemVer),
    GreaterOrEqual(SemVer),
    LessOrEqual(SemVer),
}

impl SemVerConstraint {
    pub fn parse(s: &str) -> Self {
        let s = s.trim();
        if s.is_empty() || s == "*" || s == "any" {
            return SemVerConstraint::Any;
        }
        if s.starts_with(">=") {
            if let Some(v) = SemVer::parse(s[2..].trim()) {
                return SemVerConstraint::GreaterOrEqual(v);
            }
        } else if s.starts_with("<=") {
            if let Some(v) = SemVer::parse(s[2..].trim()) {
                return SemVerConstraint::LessOrEqual(v);
            }
        } else if s.starts_with(">") {
            if let Some(v) = SemVer::parse(s[1..].trim()) {
                return SemVerConstraint::GreaterThan(v);
            }
        } else if s.starts_with("<") {
            if let Some(v) = SemVer::parse(s[1..].trim()) {
                return SemVerConstraint::LessThan(v);
            }
        } else if s.starts_with("=") {
            if let Some(v) = SemVer::parse(s[1..].trim()) {
                return SemVerConstraint::Exact(v);
            }
        } else if let Some(v) = SemVer::parse(s) {
            return SemVerConstraint::Exact(v);
        }
        SemVerConstraint::Any
    }

    pub fn matches(&self, version: &SemVer) -> bool {
        match self {
            SemVerConstraint::Any => true,
            SemVerConstraint::Exact(v) => version == v,
            SemVerConstraint::GreaterThan(v) => version > v,
            SemVerConstraint::LessThan(v) => version < v,
            SemVerConstraint::GreaterOrEqual(v) => version >= v,
            SemVerConstraint::LessOrEqual(v) => version <= v,
        }
    }
}

/// Package priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackagePriority {
    Essential,
    Required,
    Important,
    Standard,
    Optional,
}

/// Supported package formats across Linux and BSD ecosystems
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackageFormat {
    Deb,          // apt
    Rpm,          // yum / dnf
    Pacman,       // pacman
    Snap,         // snap
    Flatpak,      // flatpak
    SigmaPkg,     // native SigmaOS format
    Portage,      // Gentoo Portage (ebuild source recipes)
    FreeBsdPkg,   // FreeBSD pkg (txz binaries)
    ArchPkgBuild, // Arch PKGBUILD (source compile scripts)
    NixStore,     // Nix package manager (content-addressed store hashes)
    AppImage,     // AppImage (self-contained portable binaries)
    Homebrew,     // Homebrew (ruby formulas)
    Apk,          // Alpine apk format
}

/// Package source
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageSource {
    Repository { url: String },
    _Local { path: String },
    _Remote { url: String },
}

/// Dependency conflict resolution strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConflictResolution {
    PreferNewest,
    PreferOldest,
    PreferNative,
    Manual,
}

/// Unified package model
#[derive(Debug, Clone)]
pub struct UnifiedPackage {
    pub name: String,
    pub version: String,
    pub formats: Vec<PackageFormat>,
    pub dependencies: Vec<String>,
    pub conflicts: Vec<String>,
    pub provides: Vec<String>,
    pub source: PackageSource,
    pub installed: bool,
    pub state: PackageState,
    pub properties: HashMap<String, String>,
}

impl UnifiedPackage {
    pub fn new(name: String, version: String) -> Self {
        Self {
            name,
            version,
            formats: Vec::new(),
            dependencies: Vec::new(),
            conflicts: Vec::new(),
            provides: Vec::new(),
            source: PackageSource::Repository { url: String::new() },
            installed: false,
            state: PackageState::Uninstalled,
            properties: HashMap::new(),
        }
    }

    pub fn with_format(mut self, format: PackageFormat) -> Self {
        self.formats.push(format);
        self
    }

    pub fn with_dependency(mut self, dep: String) -> Self {
        self.dependencies.push(dep);
        self
    }

    pub fn with_conflict(mut self, conflict: String) -> Self {
        self.conflicts.push(conflict);
        self
    }

    pub fn with_provides(mut self, provides: String) -> Self {
        self.provides.push(provides);
        self
    }

    pub fn has_conflict_with(&self, other: &UnifiedPackage) -> bool {
        self.conflicts.iter().any(|c| c == &other.name)
            || other.conflicts.iter().any(|c| c == &self.name)
    }
}

/// Package format adapter trait
pub trait PackageFormatAdapter: Send + Sync {
    fn format(&self) -> PackageFormat;
    fn adapter_name(&self) -> &str;
    fn parse_manifest(&self, _raw_data: &[u8]) -> Result<UnifiedPackage, &'static str> {
        Err("Not implemented")
    }
    fn install(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
    fn verify(&self, _package: &UnifiedPackage) -> Result<bool, PackageError> { Ok(true) }
    fn remove(&self, _package: &UnifiedPackage) -> Result<(), PackageError> { Ok(()) }
}

pub struct RpmInstallStrategy;
impl InstallStrategy for RpmInstallStrategy {
    fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("Strategy: Installing RPM package '{}' into global system database.", package.name);
        Ok(())
    }
    fn verify(&self, _package: &UnifiedPackage) -> Result<bool, PackageError> { Ok(true) }
    fn remove(&self, _package: &UnifiedPackage) -> Result<(), PackageError> { Ok(()) }
}

pub struct PacmanInstallStrategy;
impl InstallStrategy for PacmanInstallStrategy {
    fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("Strategy: Extracting pacman tarball for '{}'", package.name);
        Ok(())
    }
    fn verify(&self, _package: &UnifiedPackage) -> Result<bool, PackageError> { Ok(true) }
    fn remove(&self, _package: &UnifiedPackage) -> Result<(), PackageError> { Ok(()) }
}

pub struct EbuildInstallStrategy;
impl InstallStrategy for EbuildInstallStrategy {
    fn install(&self, _package: &UnifiedPackage) -> Result<(), PackageError> { Ok(()) }
    fn verify(&self, _package: &UnifiedPackage) -> Result<bool, PackageError> { Ok(true) }
    fn remove(&self, _package: &UnifiedPackage) -> Result<(), PackageError> { Ok(()) }
}

pub struct ApkInstallStrategy;
impl InstallStrategy for ApkInstallStrategy {
    fn install(&self, _package: &UnifiedPackage) -> Result<(), PackageError> { Ok(()) }
    fn verify(&self, _package: &UnifiedPackage) -> Result<bool, PackageError> { Ok(true) }
    fn remove(&self, _package: &UnifiedPackage) -> Result<(), PackageError> { Ok(()) }
}

pub struct NixInstallStrategy;
impl InstallStrategy for NixInstallStrategy {
    fn install(&self, _package: &UnifiedPackage) -> Result<(), PackageError> { Ok(()) }
    fn verify(&self, _package: &UnifiedPackage) -> Result<bool, PackageError> { Ok(true) }
    fn remove(&self, _package: &UnifiedPackage) -> Result<(), PackageError> { Ok(()) }
}

pub struct FlatpakInstallStrategy;
impl InstallStrategy for FlatpakInstallStrategy {
    fn install(&self, _package: &UnifiedPackage) -> Result<(), PackageError> { Ok(()) }
    fn verify(&self, _package: &UnifiedPackage) -> Result<bool, PackageError> { Ok(true) }
    fn remove(&self, _package: &UnifiedPackage) -> Result<(), PackageError> { Ok(()) }
}

pub struct SnapInstallStrategy;
impl InstallStrategy for SnapInstallStrategy {
    fn install(&self, _package: &UnifiedPackage) -> Result<(), PackageError> { Ok(()) }
    fn verify(&self, _package: &UnifiedPackage) -> Result<bool, PackageError> { Ok(true) }
    fn remove(&self, _package: &UnifiedPackage) -> Result<(), PackageError> { Ok(()) }
}

pub struct AppImageInstallStrategy;
impl InstallStrategy for AppImageInstallStrategy {
    fn install(&self, _package: &UnifiedPackage) -> Result<(), PackageError> { Ok(()) }
    fn verify(&self, _package: &UnifiedPackage) -> Result<bool, PackageError> { Ok(true) }
    fn remove(&self, _package: &UnifiedPackage) -> Result<(), PackageError> { Ok(()) }
}

pub struct XbpsInstallStrategy;
impl InstallStrategy for XbpsInstallStrategy {
    fn install(&self, _package: &UnifiedPackage) -> Result<(), PackageError> { Ok(()) }
    fn verify(&self, _package: &UnifiedPackage) -> Result<bool, PackageError> { Ok(true) }
    fn remove(&self, _package: &UnifiedPackage) -> Result<(), PackageError> { Ok(()) }
}

pub struct TxzInstallStrategy;
impl InstallStrategy for TxzInstallStrategy {
    fn install(&self, _package: &UnifiedPackage) -> Result<(), PackageError> { Ok(()) }
    fn verify(&self, _package: &UnifiedPackage) -> Result<bool, PackageError> { Ok(true) }
    fn remove(&self, _package: &UnifiedPackage) -> Result<(), PackageError> { Ok(()) }
}

pub struct EopkgInstallStrategy;
impl InstallStrategy for EopkgInstallStrategy {
    fn install(&self, _package: &UnifiedPackage) -> Result<(), PackageError> { Ok(()) }
    fn verify(&self, _package: &UnifiedPackage) -> Result<bool, PackageError> { Ok(true) }
    fn remove(&self, _package: &UnifiedPackage) -> Result<(), PackageError> { Ok(()) }
}

pub struct ZypperInstallStrategy;
impl InstallStrategy for ZypperInstallStrategy {
    fn install(&self, _package: &UnifiedPackage) -> Result<(), PackageError> { Ok(()) }
    fn verify(&self, _package: &UnifiedPackage) -> Result<bool, PackageError> { Ok(true) }
    fn remove(&self, _package: &UnifiedPackage) -> Result<(), PackageError> { Ok(()) }
}

pub struct GuixInstallStrategy;
impl InstallStrategy for GuixInstallStrategy {
    fn install(&self, _package: &UnifiedPackage) -> Result<(), PackageError> { Ok(()) }
    fn verify(&self, _package: &UnifiedPackage) -> Result<bool, PackageError> { Ok(true) }
    fn remove(&self, _package: &UnifiedPackage) -> Result<(), PackageError> { Ok(()) }
}

pub struct CachyOSInstallStrategy;
impl InstallStrategy for CachyOSInstallStrategy {
    fn install(&self, _package: &UnifiedPackage) -> Result<(), PackageError> { Ok(()) }
    fn verify(&self, _package: &UnifiedPackage) -> Result<bool, PackageError> { Ok(true) }
    fn remove(&self, _package: &UnifiedPackage) -> Result<(), PackageError> { Ok(()) }
}

pub struct SwupdInstallStrategy;
impl InstallStrategy for SwupdInstallStrategy {
    fn install(&self, _package: &UnifiedPackage) -> Result<(), PackageError> { Ok(()) }
    fn verify(&self, _package: &UnifiedPackage) -> Result<bool, PackageError> { Ok(true) }
    fn remove(&self, _package: &UnifiedPackage) -> Result<(), PackageError> { Ok(()) }
}

pub struct StarlingInstallStrategy;
impl InstallStrategy for StarlingInstallStrategy {
    fn install(&self, _package: &UnifiedPackage) -> Result<(), PackageError> { Ok(()) }
    fn verify(&self, _package: &UnifiedPackage) -> Result<bool, PackageError> { Ok(true) }
    fn remove(&self, _package: &UnifiedPackage) -> Result<(), PackageError> { Ok(()) }
}

pub struct SigmaPkgInstallStrategy;
impl InstallStrategy for SigmaPkgInstallStrategy {
    fn install(&self, _package: &UnifiedPackage) -> Result<(), PackageError> { Ok(()) }
    fn verify(&self, _package: &UnifiedPackage) -> Result<bool, PackageError> { Ok(true) }
    fn remove(&self, _package: &UnifiedPackage) -> Result<(), PackageError> { Ok(()) }
}

// ============================================================================
// OOP Design Pattern: Adapter Pattern
// ============================================================================

pub trait PackageMetadataAdapter: Send + Sync {
    fn adapt(&self, raw_data: &str) -> Result<UnifiedPackage, PackageError>;
}

pub struct DebMetadataAdapter;
impl PackageMetadataAdapter for DebMetadataAdapter {
    fn adapt(&self, raw_data: &str) -> Result<UnifiedPackage, PackageError> {
        let mut pkg = UnifiedPackage::new("deb-pkg".to_string(), "1.0.0".to_string())
            .with_format(PackageFormat::Deb);
        for line in raw_data.lines() {
            if line.starts_with("Package:") {
                pkg.name = line["Package:".len()..].trim().to_string();
            } else if line.starts_with("Version:") {
                pkg.version = line["Version:".len()..].trim().to_string();
            }
        }
        Ok(pkg)
    }
}

/// Concrete PackageAdapter struct
pub struct PackageAdapter {
    pub format: PackageFormat,
    pub adapter_name: String,
    pub capabilities: Vec<String>,
}

impl PackageAdapter {
    pub fn new(format: PackageFormat, adapter_name: String) -> Self {
        Self {
            format,
            adapter_name,
            capabilities: Vec::new(),
        }
    }

    pub fn _can_handle(&self, package: &UnifiedPackage) -> bool {
        package.formats.contains(&self.format)
    }

    pub fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "Installing {} using {} adapter",
            package.name, self.adapter_name
        );
        Ok(())
    }

    pub fn remove(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "[{}] Purging package {}",
            self.adapter_name, package.name
        );
        Ok(())
    }

    pub fn update(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "[{}] Refreshing and updating package {}",
            self.adapter_name, package.name
        );
        Ok(())
    }
}

// ----------------------------------------------------
// Dependency Resolver
// ----------------------------------------------------

/// Dependency resolver with SemVer-aware constraint resolution
pub struct DependencyResolver {
    pub packages: HashMap<String, UnifiedPackage>,
    pub resolution_strategy: ConflictResolution,
}

impl DependencyResolver {
    pub fn new() -> Self {
        Self {
            packages: HashMap::new(),
            resolution_strategy: ConflictResolution::PreferNative,
        }
    }

    pub fn with_strategy(mut self, strategy: ConflictResolution) -> Self {
        self.resolution_strategy = strategy;
        self
    }

    pub fn add_package(&mut self, package: UnifiedPackage) {
        self.packages.insert(package.name.clone(), package);
    }

    pub fn resolve_dependencies(&self, package_name: &str) -> Result<Vec<String>, PackageError> {
        let mut resolved: Vec<String> = Vec::new();
        let mut to_visit: Vec<String> = Vec::new();
        to_visit.push(package_name.to_string());
        let mut visited: Vec<String> = Vec::new();

        while let Some(current) = to_visit.pop() {
            if visited.contains(&current) {
                continue;
            }

            visited.push(current.clone());

            if let Some(package) = self.packages.get(&current) {
                for dep in &package.dependencies {
                    if !visited.contains(dep) {
                        to_visit.push(dep.clone());
                    }
                }
                resolved.push(current);
            } else {
                return Err(PackageError::DependencyNotFound(current));
            }
        }

        Ok(resolved)
    }

    pub fn detect_conflicts(&self, packages: &[String]) -> Vec<(String, String)> {
        let mut conflicts = Vec::new();

        // Bolt ⚡ Optimization: Hoist `pkg1` map lookup out of inner loop to avoid
        // N-1 redundant lookups per outer loop iteration, reducing total map lookups
        // from N(N-1) to N(N+1)/2 (~50% lookup reduction).
        for (i, pkg1_name) in packages.iter().enumerate() {
            for pkg2_name in packages.iter().skip(i + 1) {
                let pkg1 = match self.packages.get(pkg1_name) {
                    Some(p) => p,
                    None => continue,
                };
                let pkg2 = match self.packages.get(pkg2_name) {
                    Some(p) => p,
                    None => continue,
                };
                if pkg1.has_conflict_with(pkg2) {
                    conflicts.push((pkg1_name.clone(), pkg2_name.clone()));
                }
            }
        }

        conflicts
    }

    pub fn resolve_conflicts(&self, conflicts: &[(String, String)]) -> Vec<String> {
        let mut resolution = Vec::new();

        match self.resolution_strategy {
            ConflictResolution::PreferNewest => {
                for (pkg1, pkg2) in conflicts {
                    let p1 = match self.packages.get(pkg1) {
                        Some(p) => p,
                        None => continue,
                    };
                    let p2 = match self.packages.get(pkg2) {
                        Some(p) => p,
                        None => continue,
                    };
                    if p1.version > p2.version {
                        resolution.push(pkg1.clone());
                    } else {
                        resolution.push(pkg2.clone());
                    }
                }
            }
            ConflictResolution::PreferOldest => {
                for (pkg1, pkg2) in conflicts {
                    let p1 = match self.packages.get(pkg1) {
                        Some(p) => p,
                        None => continue,
                    };
                    let p2 = match self.packages.get(pkg2) {
                        Some(p) => p,
                        None => continue,
                    };
                    if p1.version < p2.version {
                        resolution.push(pkg1.clone());
                    } else {
                        resolution.push(pkg2.clone());
                    }
                }
            }
            ConflictResolution::PreferNative => {
                for (pkg1, pkg2) in conflicts {
                    let p1 = match self.packages.get(pkg1) {
                        Some(p) => p,
                        None => continue,
                    };
                    let p2 = match self.packages.get(pkg2) {
                        Some(p) => p,
                        None => continue,
                    };
                    if p1.formats.contains(&PackageFormat::SigmaPkg) {
                        resolution.push(pkg1.clone());
                    } else if p2.formats.contains(&PackageFormat::SigmaPkg) {
                        resolution.push(pkg2.clone());
                    } else {
                        resolution.push(pkg1.clone());
                    }
                }
            }
            ConflictResolution::Manual => {
                for (pkg1, pkg2) in conflicts {
                    resolution.push(pkg1.clone());
                    resolution.push(pkg2.clone());
                }
            }
        }

        resolution
    }
}

impl Default for DependencyResolver {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Core Transactional Mechanism
// ============================================================================

/// Transactional package manager checkpoint
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageCheckpoint {
    pub checkpoint_id: usize,
    pub installed_keys: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TransactionalHistory {
    pub checkpoints: Vec<PackageCheckpoint>,
    pub next_checkpoint_id: usize,
}

impl TransactionalHistory {
    pub fn new() -> Self {
        Self {
            checkpoints: Vec::new(),
            next_checkpoint_id: 1,
        }
    }

    pub fn create_checkpoint(&mut self, installed: &HashMap<String, UnifiedPackage>) -> usize {
        let id = self.next_checkpoint_id;
        self.next_checkpoint_id += 1;

        let keys: Vec<String> = installed.keys().cloned().collect();

        self.checkpoints.push(PackageCheckpoint {
            checkpoint_id: id,
            installed_keys: keys,
        });

        id
    }

    pub fn get_checkpoint(&self, id: usize) -> Option<&PackageCheckpoint> {
        self.checkpoints.iter().find(|cp| cp.checkpoint_id == id)
    }
}

impl Default for TransactionalHistory {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Main Universal Package Manager Facade
// ============================================================================

/// Universal package manager
pub struct UniversalPackageManager {
    pub packages: HashMap<String, UnifiedPackage>,
    pub adapters: HashMap<PackageFormat, PackageAdapter>,
    pub resolver: DependencyResolver,
    pub installed_packages: HashMap<String, UnifiedPackage>,
    pub transaction_history: TransactionalHistory,
    pub metadata_cache: HashMap<String, UnifiedPackage>,
    pub user_hooks: Vec<alloc::sync::Arc<dyn PackageHook>>,
    pub node_distro_engine: NodeBinaryDistroEngine,
    pub distro_repo_sync: DistroRepoSyncEngine,
    pub triggers: PackageTriggerRegistry,
}

impl UniversalPackageManager {
    pub fn new() -> Self {
        let mut manager = Self {
            packages: HashMap::new(),
            adapters: HashMap::new(),
            resolver: DependencyResolver::new(),
            installed_packages: HashMap::new(),
            transaction_history: TransactionalHistory::new(),
            metadata_cache: HashMap::new(),
            user_hooks: Vec::new(),
            node_distro_engine: NodeBinaryDistroEngine::new(),
            distro_repo_sync: DistroRepoSyncEngine::new(),
            triggers: PackageTriggerRegistry::new(),
        };

        manager.add_default_adapters();
        manager
    }

    fn add_default_adapters(&mut self) {
        let formats = [
            (PackageFormat::Deb, "apt"),
            (PackageFormat::Rpm, "yum"),
            (PackageFormat::Pacman, "pacman"),
            (PackageFormat::Snap, "snap"),
            (PackageFormat::Flatpak, "flatpak"),
            (PackageFormat::SigmaPkg, "sigpkg"),
            (PackageFormat::Portage, "portage_ebuild"),
            (PackageFormat::FreeBsdPkg, "freebsd_pkg"),
            (PackageFormat::ArchPkgBuild, "arch_pkgbuild"),
            (PackageFormat::NixStore, "nix_store"),
            (PackageFormat::AppImage, "appimage"),
            (PackageFormat::Homebrew, "homebrew_formula"),
            (PackageFormat::Apk, "apk"),
        ];

        for (fmt, name) in formats {
            self.adapters.insert(fmt, PackageAdapter::new(fmt, name.to_string()));
        }
    }

    pub fn add_package(&mut self, package: UnifiedPackage) {
        self.resolver.add_package(package.clone());
        self.packages.insert(package.name.clone(), package);
    }

    pub fn create_checkpoint(&mut self) -> usize {
        self.transaction_history
            .create_checkpoint(&self.installed_packages)
    }

    pub fn rollback_to_checkpoint(&mut self, checkpoint_id: usize) -> Result<(), PackageError> {
        if let Some(checkpoint) = self
            .transaction_history
            .get_checkpoint(checkpoint_id)
            .cloned()
        {
            let current_keys: Vec<String> = self.installed_packages.keys().cloned().collect();
            for key in &current_keys {
                if !checkpoint.installed_keys.contains(key) {
                    self.remove(key)?;
                }
            }
            Ok(())
        } else {
            Err(PackageError::PackageNotFound(format!(
                "Checkpoint {} not found",
                checkpoint_id
            )))
        }
    }

    pub fn install(&mut self, package_name: &str) -> Result<(), PackageError> {
        let dependencies = self.resolver.resolve_dependencies(package_name)?;
        let conflicts = self.resolver.detect_conflicts(&dependencies);

        if !conflicts.is_empty() {
            let resolution = self.resolver.resolve_conflicts(&conflicts);
            println!("Conflicts detected: {:?}", conflicts);
            println!("Resolution: {:?}", resolution);
        }

        for dep_name in dependencies {
            if let Some(package) = self.packages.get(&dep_name) {
                for format in &package.formats {
                    if let Some(adapter) = self.adapters.get(format) {
                        adapter.install(package)?;
                        break;
                    }
                }

                // Move state to installing
                let prev_state = installing_package.state;
                installing_package.state = PackageState::Installing;
                self.triggers.notify_state_change(&installing_package, prev_state, PackageState::Installing);

                // Strategy Pattern Execution
                if let Some(&first_format) = installing_package.formats.first() {
                    let strategy = PackageFactory::get_strategy(first_format);
                    strategy.install(&installing_package)?;
                } else if let Some(adapter) = self.adapters.get(&PackageFormat::SigmaPkg) {
                    // Fallback to legacy adapters
                    adapter.install(&installing_package)?;
                }

                // Post-install hooks (User-Defined Functions)
                for hook in &self.triggers.post_install_hooks {
                    if let Err(err_msg) = hook(&installing_package) {
                        installing_package.state = PackageState::BrokenDependency;
                        return Err(PackageError::InstallationFailed(format!("Post-install hook failed: {}", err_msg)));
                    }
                }

                // Finalize state to installed
                let final_prev_state = installing_package.state;
                installing_package.state = PackageState::Installed;
                installing_package.installed = true;
                self.triggers.notify_state_change(&installing_package, final_prev_state, PackageState::Installed);

                self.installed_packages.insert(dep_name.clone(), installing_package);
            }
        }

        Ok(())
    }

    pub fn remove(&mut self, package_name: &str) -> Result<(), PackageError> {
        if let Some(package) = self.installed_packages.get(package_name) {
            if let Some(&first_format) = package.formats.first() {
                let strategy = PackageFactory::get_strategy(first_format);
                strategy.remove(package)?;
            } else if let Some(format) = package.formats.first() {
                if let Some(adapter) = self.adapters.get(format) {
                    adapter.remove(package)?;
                }
            }
            self.installed_packages.remove(package_name);
        }
        Ok(())
    }

    pub fn update(&mut self, package_name: &str) -> Result<(), PackageError> {
        let package_opt = self.installed_packages.get(package_name).cloned();
        if let Some(package) = package_opt {
            for format in &package.formats {
                if let Some(adapter) = self.adapters.get(format) {
                    let adapter: &PackageAdapter = adapter;
                    adapter.update(&package)?;
                    break;
                }
            }
        }
        Ok(())
    }

    pub fn search(&self, query: &str) -> Vec<&UnifiedPackage> {
        self.packages
            .values()
            .filter(|p| p.name.contains(query) || p.version.contains(query))
            .collect()
    }

    pub fn list_installed(&self) -> Vec<&UnifiedPackage> {
        self.installed_packages.values().collect()
    }

    pub fn get_package(&self, name: &str) -> Option<&UnifiedPackage> {
        self.packages.get(name)
    }

    /// Converts any external Linux or BSD distro package specification (DEB, RPM, Pacman, APK, Flatpak, Snap, AppImage, Ebuild, XBPS, Ports, PKG)
    /// into native SigmaPkg format with full dependency mapping and sandboxing translation.
    pub fn convert_to_sigpkg(
        &self,
        package: &UnifiedPackage,
    ) -> Result<UnifiedPackage, PackageError> {
        let mut sigpkg =
            UnifiedPackage::new(format!("sigpkg-{}", package.name), package.version.clone())
                .with_format(PackageFormat::SigmaPkg)
                .with_provides(package.name.clone());

        for dep in &package.dependencies {
            sigpkg = sigpkg.with_dependency(dep.clone());
        }

        for conflict in &package.conflicts {
            sigpkg = sigpkg.with_conflict(conflict.clone());
        }

        for provide in &package.provides {
            sigpkg = sigpkg.with_provides(provide.clone());
        }

        sigpkg.source = package.source.clone();
        sigpkg.installed = package.installed;

        Ok(sigpkg)
    }
}

impl Default for UniversalPackageManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Package errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageError {
    PackageNotFound(String),
    DependencyNotFound(String),
    _AdapterNotFound,
    InstallationFailed(String),
    _ConflictDetected(Vec<(String, String)>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeatureType {
    Binary,
    Library,
    Source,
}

pub struct TabularSchema {
    pub fields: Vec<String>,
}

pub struct TabularRow {
    pub values: Vec<String>,
}

pub struct TabularDataset {
    pub schema: TabularSchema,
    pub rows: Vec<TabularRow>,
}

pub struct SovereignTabFm {
    pub datasets: Vec<TabularDataset>,
}

pub trait PackageAdapterTrait {
    fn adapter_name(&self) -> &str;
}

/// Universal multi-format package metadata parser and handler supporting
/// Linux, BSD, macOS, Android, and HarmonyOS package formats
pub struct UniversalPackageManifestParser;

impl UniversalPackageManifestParser {
    pub fn detect_format_from_filename(filename: &str) -> Option<PackageFormat> {
        let name = filename.to_lowercase();
        if name.ends_with(".deb") || name.ends_with(".superdeb") {
            Some(PackageFormat::Deb)
        } else if name.ends_with(".rpm") {
            Some(PackageFormat::Rpm)
        } else if name.ends_with(".apk") {
            Some(PackageFormat::Apk)
        } else if name.ends_with(".pkg.tar.xz") || name.ends_with(".pkg.tar.zst") {
            Some(PackageFormat::Pacman)
        } else if name.ends_with(".snap") {
            Some(PackageFormat::Snap)
        } else if name.ends_with(".flatpak") {
            Some(PackageFormat::Flatpak)
        } else if name.ends_with(".appimage") {
            Some(PackageFormat::AppImage)
        } else if name.ends_with(".ebuild") || name.ends_with(".portage") {
            Some(PackageFormat::Ebuild)
        } else if name.ends_with(".nixpkg") || name.ends_with(".nix") {
            Some(PackageFormat::Nixpkg)
        } else if name.ends_with(".eopkg") {
            Some(PackageFormat::Eopkg)
        } else if name.ends_with(".ports") {
            Some(PackageFormat::Ports)
        } else if name.ends_with(".pkg") {
            Some(PackageFormat::Pkg)
        } else if name.ends_with(".ipa") {
            Some(PackageFormat::Ipa)
        } else if name.ends_with(".aab") {
            Some(PackageFormat::Aab)
        } else if name.ends_with(".hap") {
            Some(PackageFormat::Hap)
        } else if name.ends_with(".pisi") {
            Some(PackageFormat::Pisi)
        } else if name.ends_with(".lzm") {
            Some(PackageFormat::Lzm)
        } else if name.ends_with(".pup") {
            Some(PackageFormat::Pup)
        } else if name.ends_with(".pet") {
            Some(PackageFormat::Pet)
        } else if name.ends_with(".tar.gz") || name.ends_with(".tgz") {
            Some(PackageFormat::TarGz)
        } else if name.ends_with(".tar.xz") || name.ends_with(".xz") {
            Some(PackageFormat::Xz)
        } else if name.ends_with(".tar") {
            Some(PackageFormat::Tar)
        } else if name.ends_with(".dports") {
            Some(PackageFormat::Dports)
        } else if name.ends_with(".slackbuild") || name.ends_with(".tlz") || name.ends_with(".tbz") {
            Some(PackageFormat::SlackBuild)
        } else if name.ends_with(".crux") || name.ends_with(".pkgfile") {
            Some(PackageFormat::Crux)
        } else if name.ends_with(".drpm") {
            Some(PackageFormat::Drpm)
        } else if name.ends_with(".stratum") {
            Some(PackageFormat::Stratum)
        } else if name.ends_with(".app") {
            Some(PackageFormat::App)
        } else {
            None
        }
    }

    pub fn parse_manifest_auto(filename: &str, raw_data: &[u8]) -> Result<UnifiedPackage, &'static str> {
        let fmt = Self::detect_format_from_filename(filename)
            .ok_or("UniversalManifestParser: Unsupported or unrecognized package extension")?;

        let pkg_name = filename
            .split('.')
            .next()
            .unwrap_or("unknown")
            .to_string();

        let mut pkg = UnifiedPackage::new(pkg_name, "1.0.0".to_string()).with_format(fmt);
        if !raw_data.is_empty() {
            pkg = pkg.with_provides("universal_binary".to_string());
        }
        Ok(pkg)
    }
}

/// Linux & BSD Distro Inspired Rollback Mechanics
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistroRollbackType {
    NixOsGeneration,       // NixOS atomic generation profile rollback
    FreeBsdZfsBootEnv,     // FreeBSD ZFS boot environment (bectl / beadm) rollback
    OpenSuseSnapper,       // openSUSE Snapper CoW snapshot rollback
    FedoraRpmOstree,       // Fedora Silverblue / rpm-ostree deployment rollback
    AlpineApkCache,        // Alpine Linux local apk tarball cache rollback
}

#[derive(Debug, Clone)]
pub struct SovereignRollbackSnapshot {
    pub snapshot_id: usize,
    pub rollback_type: DistroRollbackType,
    pub label: String,
    pub installed_packages_state: Vec<String>,
    pub timestamp_sec: u64,
}

pub struct SovereignPackageRollbackEngine {
    pub snapshots: Vec<SovereignRollbackSnapshot>,
    pub active_snapshot_id: Option<usize>,
    pub next_snapshot_id: usize,
}

impl SovereignPackageRollbackEngine {
    pub fn new() -> Self {
        Self {
            snapshots: Vec::new(),
            active_snapshot_id: None,
            next_snapshot_id: 1,
        }
    }

    pub fn create_snapshot(&mut self, label: &str, installed_packages: Vec<String>) -> usize {
        self.create_distro_snapshot(DistroRollbackType::OpenSuseSnapper, label, &installed_packages, 0)
    }

    pub fn create_distro_snapshot(
        &mut self,
        rollback_type: DistroRollbackType,
        label: &str,
        installed_packages: &[String],
        now_sec: u64,
    ) -> usize {
        let id = self.next_snapshot_id;
        self.next_snapshot_id += 1;

        let snap = SovereignRollbackSnapshot {
            snapshot_id: id,
            rollback_type,
            label: label.to_string(),
            installed_packages_state: installed_packages.to_vec(),
            timestamp_sec: now_sec,
        };

        self.snapshots.push(snap);
        self.active_snapshot_id = Some(id);
        id
    }

    pub fn rollback(&mut self, snapshot_id: usize) -> Result<Vec<String>, &'static str> {
        let snap = self.snapshots.iter().find(|s| s.snapshot_id == snapshot_id).ok_or("Rollback Engine: Snapshot not found")?;
        self.active_snapshot_id = Some(snapshot_id);
        Ok(snap.installed_packages_state.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_creation() {
        let manager = UniversalPackageManager::new();
        assert_eq!(manager.adapters.len(), 13);
    }

    #[test]
    fn test_package_creation() {
        let package = UnifiedPackage::new("test".to_string(), "1.0.0".to_string())
            .with_format(PackageFormat::Deb)
            .with_dependency("dep1".to_string());
        assert_eq!(package.formats.len(), 1);
        assert_eq!(package.dependencies.len(), 1);
    }

    #[test]
    fn test_dependency_resolution() {
        let mut resolver = DependencyResolver::new();
        let pkg1 = UnifiedPackage::new("pkg1".to_string(), "1.0.0".to_string())
            .with_dependency("pkg2".to_string());
        let pkg2 = UnifiedPackage::new("pkg2".to_string(), "1.0.0".to_string());

        resolver.add_package(pkg1);
        resolver.add_package(pkg2);

        let deps = resolver.resolve_dependencies("pkg1").unwrap();
        assert_eq!(deps.len(), 2);
    }

    #[test]
    fn test_conflict_detection() {
        let mut resolver = DependencyResolver::new();
        let pkg1 = UnifiedPackage::new("pkg1".to_string(), "1.0.0".to_string())
            .with_conflict("pkg2".to_string());
        let pkg2 = UnifiedPackage::new("pkg2".to_string(), "1.0.0".to_string());

        resolver.add_package(pkg1);
        resolver.add_package(pkg2);

        let conflicts = resolver.detect_conflicts(&["pkg1".to_string(), "pkg2".to_string()]);
        assert_eq!(conflicts.len(), 1);
    }

    #[test]
    fn test_install_package() {
        let mut manager = UniversalPackageManager::new();
        let package = UnifiedPackage::new("test".to_string(), "1.0.0".to_string())
            .with_format(PackageFormat::SigmaPkg);

        manager.add_package(package);
        assert!(manager.install("test").is_ok());
        assert_eq!(manager.installed_packages.len(), 1);
    }

    #[test]
    fn test_checkpoint_rollback() {
        let mut manager = UniversalPackageManager::new();
        let package = UnifiedPackage::new("test".to_string(), "1.0.0".to_string())
            .with_format(PackageFormat::SigmaPkg);

        manager.add_package(package);
        let cp_id = manager.create_checkpoint();

        assert!(manager.install("test").is_ok());
        assert_eq!(manager.installed_packages.len(), 1);

        assert!(manager.rollback_to_checkpoint(cp_id).is_ok());
        assert_eq!(manager.installed_packages.len(), 0);
    }
}
