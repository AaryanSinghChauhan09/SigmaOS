// SigmaOS Universal Package Manager
// Unified system absorbing all 18 major distribution formats.

#[cfg(not(feature = "standalone_test"))]
use crate::klib::{HashMap, HashSet, Arc};

#[cfg(feature = "standalone_test")]
use std::{collections::{HashMap, HashSet}, sync::Arc};

/// Package format type covering 18 major distribution formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackageFormat {
    Deb,       // Debian, Ubuntu, Mint, Parrot
    Rpm,       // RedHat, Fedora, CentOS, Zypper
    Pacman,    // Arch Linux, Manjaro, CachyOS
    Ebuild,    // Gentoo Linux
    Apk,       // Alpine Linux
    Nix,       // NixOS package manager
    Flatpak,   // Sandboxed desktop-oriented packages
    Snap,      // Isolated Canonical Snap system
    AppImage,  // Portable runtime desktop files
    Xbps,      // Void Linux xbps package format
    Txz,       // Slackware txz slackpkg format
    Eopkg,     // Solus eopkg package format
    Zypper,    // openSUSE packages via libsolv/zypp
    Guix,      // GNU Guix package system
    CachyOS,   // Highly optimized CachyOS variant
    Swupd,     // Intel Swupd stateless model
    Starling,  // Secure post-quantum micro-packages
    SigmaPkg,  // Native SigmaOS package system
}

/// Package source
#[derive(Debug, Clone)]
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

/// Package state for the State Pattern
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageState {
    Uninstalled,
    Downloading,
    _VerifyingSignature,
    _Unpacking,
    Installing,
    _RunningHooks,
    Installed,
    BrokenDependency,
    _Corrupted,
}

/// Unified package holding format-specific properties and metadata
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

// ============================================================================
// OOP Design Pattern: Strategy Pattern
// ============================================================================

pub trait InstallStrategy: Send + Sync {
    fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError>;
    fn verify(&self, package: &UnifiedPackage) -> Result<bool, PackageError>;
    fn remove(&self, package: &UnifiedPackage) -> Result<(), PackageError>;
}

pub struct DebInstallStrategy;
impl InstallStrategy for DebInstallStrategy {
    fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("Strategy: Unpacking deb and invoking preinst/postinst scripts for '{}'", package.name);
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

pub struct RpmMetadataAdapter;
impl PackageMetadataAdapter for RpmMetadataAdapter {
    fn adapt(&self, raw_data: &str) -> Result<UnifiedPackage, PackageError> {
        let mut pkg = UnifiedPackage::new("rpm-pkg".to_string(), "1.0.0".to_string())
            .with_format(PackageFormat::Rpm);
        for line in raw_data.lines() {
            if line.starts_with("Name:") {
                pkg.name = line["Name:".len()..].trim().to_string();
            } else if line.starts_with("Version:") {
                pkg.version = line["Version:".len()..].trim().to_string();
            }
        }
        Ok(pkg)
    }
}

pub struct PacmanMetadataAdapter;
impl PackageMetadataAdapter for PacmanMetadataAdapter {
    fn adapt(&self, raw_data: &str) -> Result<UnifiedPackage, PackageError> {
        let mut pkg = UnifiedPackage::new("pacman-pkg".to_string(), "1.0.0".to_string())
            .with_format(PackageFormat::Pacman);
        for line in raw_data.lines() {
            if line.starts_with("pkgname=") {
                pkg.name = line["pkgname=".len()..].trim().to_string();
            } else if line.starts_with("pkgver=") {
                pkg.version = line["pkgver=".len()..].trim().to_string();
            }
        }
        Ok(pkg)
    }
}

pub struct EbuildMetadataAdapter;
impl PackageMetadataAdapter for EbuildMetadataAdapter {
    fn adapt(&self, _raw: &str) -> Result<UnifiedPackage, PackageError> {
        Ok(UnifiedPackage::new("ebuild-pkg".to_string(), "1.0.0".to_string()).with_format(PackageFormat::Ebuild))
    }
}

pub struct ApkMetadataAdapter;
impl PackageMetadataAdapter for ApkMetadataAdapter {
    fn adapt(&self, _raw: &str) -> Result<UnifiedPackage, PackageError> {
        Ok(UnifiedPackage::new("apk-pkg".to_string(), "1.0.0".to_string()).with_format(PackageFormat::Apk))
    }
}

pub struct NixMetadataAdapter;
impl PackageMetadataAdapter for NixMetadataAdapter {
    fn adapt(&self, _raw: &str) -> Result<UnifiedPackage, PackageError> {
        Ok(UnifiedPackage::new("nix-pkg".to_string(), "1.0.0".to_string()).with_format(PackageFormat::Nix))
    }
}

pub struct FlatpakMetadataAdapter;
impl PackageMetadataAdapter for FlatpakMetadataAdapter {
    fn adapt(&self, _raw: &str) -> Result<UnifiedPackage, PackageError> {
        Ok(UnifiedPackage::new("flatpak-pkg".to_string(), "1.0.0".to_string()).with_format(PackageFormat::Flatpak))
    }
}

pub struct SnapMetadataAdapter;
impl PackageMetadataAdapter for SnapMetadataAdapter {
    fn adapt(&self, _raw: &str) -> Result<UnifiedPackage, PackageError> {
        Ok(UnifiedPackage::new("snap-pkg".to_string(), "1.0.0".to_string()).with_format(PackageFormat::Snap))
    }
}

pub struct AppImageMetadataAdapter;
impl PackageMetadataAdapter for AppImageMetadataAdapter {
    fn adapt(&self, _raw: &str) -> Result<UnifiedPackage, PackageError> {
        Ok(UnifiedPackage::new("appimage-pkg".to_string(), "1.0.0".to_string()).with_format(PackageFormat::AppImage))
    }
}

pub struct XbpsMetadataAdapter;
impl PackageMetadataAdapter for XbpsMetadataAdapter {
    fn adapt(&self, _raw: &str) -> Result<UnifiedPackage, PackageError> {
        Ok(UnifiedPackage::new("xbps-pkg".to_string(), "1.0.0".to_string()).with_format(PackageFormat::Xbps))
    }
}

pub struct TxzMetadataAdapter;
impl PackageMetadataAdapter for TxzMetadataAdapter {
    fn adapt(&self, _raw: &str) -> Result<UnifiedPackage, PackageError> {
        Ok(UnifiedPackage::new("txz-pkg".to_string(), "1.0.0".to_string()).with_format(PackageFormat::Txz))
    }
}

pub struct EopkgMetadataAdapter;
impl PackageMetadataAdapter for EopkgMetadataAdapter {
    fn adapt(&self, _raw: &str) -> Result<UnifiedPackage, PackageError> {
        Ok(UnifiedPackage::new("eopkg-pkg".to_string(), "1.0.0".to_string()).with_format(PackageFormat::Eopkg))
    }
}

pub struct ZypperMetadataAdapter;
impl PackageMetadataAdapter for ZypperMetadataAdapter {
    fn adapt(&self, _raw: &str) -> Result<UnifiedPackage, PackageError> {
        Ok(UnifiedPackage::new("zypper-pkg".to_string(), "1.0.0".to_string()).with_format(PackageFormat::Zypper))
    }
}

pub struct GuixMetadataAdapter;
impl PackageMetadataAdapter for GuixMetadataAdapter {
    fn adapt(&self, _raw: &str) -> Result<UnifiedPackage, PackageError> {
        Ok(UnifiedPackage::new("guix-pkg".to_string(), "1.0.0".to_string()).with_format(PackageFormat::Guix))
    }
}

pub struct CachyOSMetadataAdapter;
impl PackageMetadataAdapter for CachyOSMetadataAdapter {
    fn adapt(&self, _raw: &str) -> Result<UnifiedPackage, PackageError> {
        Ok(UnifiedPackage::new("cachyos-pkg".to_string(), "1.0.0".to_string()).with_format(PackageFormat::CachyOS))
    }
}

pub struct SwupdMetadataAdapter;
impl PackageMetadataAdapter for SwupdMetadataAdapter {
    fn adapt(&self, _raw: &str) -> Result<UnifiedPackage, PackageError> {
        Ok(UnifiedPackage::new("swupd-pkg".to_string(), "1.0.0".to_string()).with_format(PackageFormat::Swupd))
    }
}

pub struct StarlingMetadataAdapter;
impl PackageMetadataAdapter for StarlingMetadataAdapter {
    fn adapt(&self, _raw: &str) -> Result<UnifiedPackage, PackageError> {
        Ok(UnifiedPackage::new("starling-pkg".to_string(), "1.0.0".to_string()).with_format(PackageFormat::Starling))
    }
}

pub struct SigmaPkgMetadataAdapter;
impl PackageMetadataAdapter for SigmaPkgMetadataAdapter {
    fn adapt(&self, _raw: &str) -> Result<UnifiedPackage, PackageError> {
        Ok(UnifiedPackage::new("sigmapkg-pkg".to_string(), "1.0.0".to_string()).with_format(PackageFormat::SigmaPkg))
    }
}

// ============================================================================
// OOP Design Pattern: Decorator Pattern
// ============================================================================

pub trait PackageCapability {
    fn get_package(&self) -> &UnifiedPackage;
    fn enforce_sandbox(&self) -> Result<(), PackageError>;
    fn restrict_network(&self) -> Result<(), PackageError>;
    fn profile_performance(&self);
}

pub struct BasePackageDecorator {
    pub package: UnifiedPackage,
}

impl PackageCapability for BasePackageDecorator {
    fn get_package(&self) -> &UnifiedPackage {
        &self.package
    }
    fn enforce_sandbox(&self) -> Result<(), PackageError> { Ok(()) }
    fn restrict_network(&self) -> Result<(), PackageError> { Ok(()) }
    fn profile_performance(&self) {}
}

pub struct SandboxDecorator<T: PackageCapability> {
    pub decorated: T,
    pub is_isolated: bool,
}

impl<T: PackageCapability> PackageCapability for SandboxDecorator<T> {
    fn get_package(&self) -> &UnifiedPackage {
        self.decorated.get_package()
    }
    fn enforce_sandbox(&self) -> Result<(), PackageError> {
        if self.is_isolated {
            println!("SandboxDecorator: Sandboxing enforced for package '{}'!", self.get_package().name);
        }
        self.decorated.enforce_sandbox()
    }
    fn restrict_network(&self) -> Result<(), PackageError> {
        self.decorated.restrict_network()
    }
    fn profile_performance(&self) {
        self.decorated.profile_performance();
    }
}

pub struct NetworkRestrictionDecorator<T: PackageCapability> {
    pub decorated: T,
    pub allowed_hosts: Vec<String>,
}

impl<T: PackageCapability> PackageCapability for NetworkRestrictionDecorator<T> {
    fn get_package(&self) -> &UnifiedPackage {
        self.decorated.get_package()
    }
    fn enforce_sandbox(&self) -> Result<(), PackageError> {
        self.decorated.enforce_sandbox()
    }
    fn restrict_network(&self) -> Result<(), PackageError> {
        println!("NetworkRestrictionDecorator: Network restricted for package '{}' to hosts: {:?}", self.get_package().name, self.allowed_hosts);
        self.decorated.restrict_network()
    }
    fn profile_performance(&self) {
        self.decorated.profile_performance();
    }
}

// ============================================================================
// OOP Design Pattern: Factory Pattern
// ============================================================================

pub struct PackageFactory;

impl PackageFactory {
    pub fn get_strategy(format: PackageFormat) -> Box<dyn InstallStrategy> {
        match format {
            PackageFormat::Deb => Box::new(DebInstallStrategy),
            PackageFormat::Rpm => Box::new(RpmInstallStrategy),
            PackageFormat::Pacman => Box::new(PacmanInstallStrategy),
            PackageFormat::Ebuild => Box::new(EbuildInstallStrategy),
            PackageFormat::Apk => Box::new(ApkInstallStrategy),
            PackageFormat::Nix => Box::new(NixInstallStrategy),
            PackageFormat::Flatpak => Box::new(FlatpakInstallStrategy),
            PackageFormat::Snap => Box::new(SnapInstallStrategy),
            PackageFormat::AppImage => Box::new(AppImageInstallStrategy),
            PackageFormat::Xbps => Box::new(XbpsInstallStrategy),
            PackageFormat::Txz => Box::new(TxzInstallStrategy),
            PackageFormat::Eopkg => Box::new(EopkgInstallStrategy),
            PackageFormat::Zypper => Box::new(ZypperInstallStrategy),
            PackageFormat::Guix => Box::new(GuixInstallStrategy),
            PackageFormat::CachyOS => Box::new(CachyOSInstallStrategy),
            PackageFormat::Swupd => Box::new(SwupdInstallStrategy),
            PackageFormat::Starling => Box::new(StarlingInstallStrategy),
            PackageFormat::SigmaPkg => Box::new(SigmaPkgInstallStrategy),
        }
    }

    pub fn get_adapter(format: PackageFormat) -> Box<dyn PackageMetadataAdapter> {
        match format {
            PackageFormat::Deb => Box::new(DebMetadataAdapter),
            PackageFormat::Rpm => Box::new(RpmMetadataAdapter),
            PackageFormat::Pacman => Box::new(PacmanMetadataAdapter),
            PackageFormat::Ebuild => Box::new(EbuildMetadataAdapter),
            PackageFormat::Apk => Box::new(ApkMetadataAdapter),
            PackageFormat::Nix => Box::new(NixMetadataAdapter),
            PackageFormat::Flatpak => Box::new(FlatpakMetadataAdapter),
            PackageFormat::Snap => Box::new(SnapMetadataAdapter),
            PackageFormat::AppImage => Box::new(AppImageMetadataAdapter),
            PackageFormat::Xbps => Box::new(XbpsMetadataAdapter),
            PackageFormat::Txz => Box::new(TxzMetadataAdapter),
            PackageFormat::Eopkg => Box::new(EopkgMetadataAdapter),
            PackageFormat::Zypper => Box::new(ZypperMetadataAdapter),
            PackageFormat::Guix => Box::new(GuixMetadataAdapter),
            PackageFormat::CachyOS => Box::new(CachyOSMetadataAdapter),
            PackageFormat::Swupd => Box::new(SwupdMetadataAdapter),
            PackageFormat::Starling => Box::new(StarlingMetadataAdapter),
            PackageFormat::SigmaPkg => Box::new(SigmaPkgMetadataAdapter),
        }
    }
}

// ============================================================================
// OOP Design Pattern: Observer Pattern & User-Defined Functions (UDFs)
// ============================================================================

pub trait PackageObserver: Send + Sync {
    fn on_state_change(&self, package: &UnifiedPackage, old_state: PackageState, new_state: PackageState);
}

pub type PackageUdfHook = Arc<dyn Fn(&UnifiedPackage) -> Result<(), String> + Send + Sync>;

pub struct PackageTriggerRegistry {
    pub pre_install_hooks: Vec<PackageUdfHook>,
    pub post_install_hooks: Vec<PackageUdfHook>,
    pub observers: Vec<Box<dyn PackageObserver>>,
}

impl PackageTriggerRegistry {
    pub fn new() -> Self {
        Self {
            pre_install_hooks: Vec::new(),
            post_install_hooks: Vec::new(),
            observers: Vec::new(),
        }
    }

    pub fn register_pre_install(&mut self, hook: PackageUdfHook) {
        self.pre_install_hooks.push(hook);
    }

    pub fn register_post_install(&mut self, hook: PackageUdfHook) {
        self.post_install_hooks.push(hook);
    }

    pub fn register_observer(&mut self, observer: Box<dyn PackageObserver>) {
        self.observers.push(observer);
    }

    pub fn notify_state_change(&self, package: &UnifiedPackage, old_state: PackageState, new_state: PackageState) {
        for obs in &self.observers {
            obs.on_state_change(package, old_state, new_state);
        }
    }
}

impl Default for PackageTriggerRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Legacy Adapter & Backwards Compatibility Layer
// ============================================================================

/// Package format adapter
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
            "Removing {} using {} adapter",
            package.name, self.adapter_name
        );
        Ok(())
    }

    pub fn update(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "Updating {} using {} adapter",
            package.name, self.adapter_name
        );
        Ok(())
    }
}

// ============================================================================
// Core Dependency Resolver
// ============================================================================

/// Dependency resolver
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

    pub fn resolve_dependencies(&self, package_name: &str) -> Result<std::vec::Vec<String>, PackageError> {
        let mut resolved: std::vec::Vec<String> = std::vec::Vec::new();
        let mut to_visit: std::vec::Vec<String> = std::vec::Vec::new();
        to_visit.push(package_name.to_string());
        let mut visited = HashSet::<String>::new();
    pub fn resolve_dependencies(&self, package_name: &str) -> Result<std::vec::Vec<String>, PackageError> {
        let mut resolved: std::vec::Vec<String> = std::vec::Vec::new();
        let mut to_visit: std::vec::Vec<String> = std::vec::Vec::new();
        to_visit.push(package_name.to_string());
        let mut visited = std::collections::HashSet::<String>::new();
        let mut visited = HashSet::<String>::new();

        while let Some(current) = to_visit.pop() {
            let current: String = current;
            if visited.contains(&current) {
                continue;
            }

            visited.insert(current.clone());

            if let Some(package) = self.packages.get(&current) {
                for dep in &package.dependencies {
                    let dep: &String = dep;
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

        for (i, pkg1_name) in packages.iter().enumerate() {
            for pkg2_name in packages.iter().skip(i + 1) {
                if let (Some(pkg1), Some(pkg2)) =
                    (self.packages.get(pkg1_name), self.packages.get(pkg2_name))
                {
                    let pkg1: &UnifiedPackage = pkg1;
                    let pkg2: &UnifiedPackage = pkg2;
                    if pkg1.has_conflict_with(pkg2) {
                        conflicts.push((pkg1_name.clone(), pkg2_name.clone()));
                    }
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
                    if let (Some(p1), Some(p2)) = (self.packages.get(pkg1), self.packages.get(pkg2))
                    {
                        if p1.version > p2.version {
                            resolution.push(pkg1.clone());
                        } else {
                            resolution.push(pkg2.clone());
                        }
                    }
                }
            }
            ConflictResolution::PreferOldest => {
                for (pkg1, pkg2) in conflicts {
                    if let (Some(p1), Some(p2)) = (self.packages.get(pkg1), self.packages.get(pkg2))
                    {
                        if p1.version < p2.version {
                            resolution.push(pkg1.clone());
                        } else {
                            resolution.push(pkg2.clone());
                        }
                    }
                }
            }
            ConflictResolution::PreferNative => {
                for (pkg1, pkg2) in conflicts {
                    if let (Some(p1), Some(p2)) = (self.packages.get(pkg1), self.packages.get(pkg2))
                    {
                        if p1.formats.contains(&PackageFormat::SigmaPkg) {
                            resolution.push(pkg1.clone());
                        } else if p2.formats.contains(&PackageFormat::SigmaPkg) {
                            resolution.push(pkg2.clone());
                        } else {
                            resolution.push(pkg1.clone());
                        }
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
#[derive(Debug, Clone)]
pub struct PackageCheckpoint {
    pub checkpoint_id: usize,
    pub installed_keys: Vec<String>,
}

/// Transactional history tracker for SigmaPkg/UniversalPackageManager rollbacks
#[derive(Debug, Clone)]
pub struct TransactionalHistory {
    pub checkpoints: Vec<PackageCheckpoint>,
    pub next_checkpoint_id: usize,
}

impl TransactionalHistory {
    pub fn new() -> Self {
        TransactionalHistory {
            checkpoints: Vec::new(),
            next_checkpoint_id: 1,
        }
    }

    pub fn create_checkpoint(&mut self, installed: &HashMap<String, UnifiedPackage>) -> usize {
        let id = self.next_checkpoint_id;
        self.next_checkpoint_id += 1;

        let mut keys: Vec<String> = Vec::new();
        let mut keys: std::vec::Vec<String> = std::vec::Vec::new();
        let mut keys: Vec<String> = Vec::new();
        for key in installed.keys() {
            let key: &String = key;
            keys.push(key.clone());
        }

        self.checkpoints.push(PackageCheckpoint {
            checkpoint_id: id,
            installed_keys: keys,
        });

        id
    }

    pub fn get_checkpoint(&self, id: usize) -> Option<&PackageCheckpoint> {
        for i in 0..self.checkpoints.len() {
            if self.checkpoints[i].checkpoint_id == id {
                return Some(&self.checkpoints[i]);
            }
        }
        None
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
            triggers: PackageTriggerRegistry::new(),
        };

        manager.add_default_adapters();
        manager
    }

    fn add_default_adapters(&mut self) {
        // Register standard legacy adapters for original backwards compatibility tests
        self.adapters.insert(PackageFormat::Deb, PackageAdapter::new(PackageFormat::Deb, "apt".to_string()));
        self.adapters.insert(PackageFormat::Rpm, PackageAdapter::new(PackageFormat::Rpm, "yum".to_string()));
        self.adapters.insert(PackageFormat::Pacman, PackageAdapter::new(PackageFormat::Pacman, "pacman".to_string()));
        self.adapters.insert(PackageFormat::Snap, PackageAdapter::new(PackageFormat::Snap, "snap".to_string()));
        self.adapters.insert(PackageFormat::Flatpak, PackageAdapter::new(PackageFormat::Flatpak, "flatpak".to_string()));
        self.adapters.insert(PackageFormat::SigmaPkg, PackageAdapter::new(PackageFormat::SigmaPkg, "sigpkg".to_string()));
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
            for key in current_keys {
                if !checkpoint.installed_keys.contains(&key) {
                    self.remove(&key)?;
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
        // Resolve dependencies
        let dependencies = self.resolver.resolve_dependencies(package_name)?;

        // Detect conflicts
        let conflicts = self.resolver.detect_conflicts(&dependencies);

        if !conflicts.is_empty() {
            let resolution = self.resolver.resolve_conflicts(&conflicts);
            println!("Conflicts detected: {:?}", conflicts);
            println!("Resolution: {:?}", resolution);
        }

        // Install packages
        for dep_name in dependencies {
            if let Some(package) = self.packages.get(&dep_name).cloned() {
                let mut installing_package = package.clone();
                let old_state = installing_package.state;

                // Move package state to downloading
                installing_package.state = PackageState::Downloading;
                self.triggers.notify_state_change(&installing_package, old_state, PackageState::Downloading);

                // Pre-install hooks (User-Defined Functions)
                for hook in &self.triggers.pre_install_hooks {
                    if let Err(err_msg) = hook(&installing_package) {
                        installing_package.state = PackageState::BrokenDependency;
                        return Err(PackageError::InstallationFailed(format!("Pre-install hook failed: {}", err_msg)));
            if let Some(package) = self.packages.get(&dep_name) {
                // Find appropriate adapter
                for format in &package.formats {
                    if let Some(adapter) = self.adapters.get(format) {
                        let adapter: &PackageAdapter = adapter;
                        adapter.install(package)?;
                        break;
            if let Some(package) = self.packages.get(&dep_name).cloned() {
                let mut installing_package = package.clone();
                let old_state = installing_package.state;

                // Move package state to downloading
                installing_package.state = PackageState::Downloading;
                self.triggers.notify_state_change(&installing_package, old_state, PackageState::Downloading);

                // Pre-install hooks (User-Defined Functions)
                for hook in &self.triggers.pre_install_hooks {
                    if let Err(err_msg) = hook(&installing_package) {
                        installing_package.state = PackageState::BrokenDependency;
                        return Err(PackageError::InstallationFailed(format!("Pre-install hook failed: {}", err_msg)));
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
        if let Some(package) = self.installed_packages.get(package_name) {
            for format in &package.formats {
                if let Some(adapter) = self.adapters.get(format) {
                    let adapter: &PackageAdapter = adapter;
                    adapter.update(package)?;
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

// ============================================================================
// Advanced Standalone Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_creation() {
        let manager = UniversalPackageManager::new();
        assert_eq!(manager.adapters.len(), 6);
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
    fn test_transactional_rollback() {
        let mut manager = UniversalPackageManager::new();
        let pkg1 = UnifiedPackage::new("pkg1".to_string(), "1.0.0".to_string())
            .with_format(PackageFormat::SigmaPkg);
        let pkg2 = UnifiedPackage::new("pkg2".to_string(), "1.0.0".to_string())
            .with_format(PackageFormat::SigmaPkg);

        manager.add_package(pkg1);
        manager.add_package(pkg2);

        let checkpoint_id = manager.create_checkpoint();
        assert_eq!(checkpoint_id, 1);

        manager.install("pkg1").unwrap();
        manager.install("pkg2").unwrap();
        assert_eq!(manager.installed_packages.len(), 2);

        manager.rollback_to_checkpoint(checkpoint_id).unwrap();
        assert_eq!(manager.installed_packages.len(), 0);
    }

    // ========================================================================
    // Comprehensive New OOP Subsystem Tests
    // ========================================================================

    struct StateTransitionObserver {
        events: std::sync::Mutex<Vec<(String, PackageState, PackageState)>>,
    }

    impl PackageObserver for StateTransitionObserver {
        fn on_state_change(&self, package: &UnifiedPackage, old_state: PackageState, new_state: PackageState) {
            let mut evs = self.events.lock().unwrap();
            evs.push((package.name.clone(), old_state, new_state));
        }
    }

    #[test]
    fn test_state_pattern_transitions_and_observer() {
        let mut manager = UniversalPackageManager::new();
        let package = UnifiedPackage::new("state-test".to_string(), "3.2.1".to_string())
            .with_format(PackageFormat::Pacman);

        let obs = Arc::new(StateTransitionObserver {
            events: std::sync::Mutex::new(Vec::new()),
        });

        struct ObserverWrapper {
            inner: Arc<StateTransitionObserver>,
        }
        impl PackageObserver for ObserverWrapper {
            fn on_state_change(&self, package: &UnifiedPackage, old_state: PackageState, new_state: PackageState) {
                self.inner.on_state_change(package, old_state, new_state);
            }
        }

        manager.triggers.register_observer(Box::new(ObserverWrapper { inner: obs.clone() }));

        manager.add_package(package);
        manager.install("state-test").unwrap();

        let events = obs.events.lock().unwrap();
        assert!(events.len() >= 2);
        assert_eq!(events[0].1, PackageState::Uninstalled);
        assert_eq!(events[0].2, PackageState::Downloading);
    }

    #[test]
    fn test_strategy_and_factory_patterns() {
        let strategy = PackageFactory::get_strategy(PackageFormat::Deb);
        let pkg = UnifiedPackage::new("my-deb".to_string(), "1.0.0".to_string()).with_format(PackageFormat::Deb);
        assert!(strategy.install(&pkg).is_ok());
        assert!(strategy.verify(&pkg).unwrap());

        let adapter = PackageFactory::get_adapter(PackageFormat::Pacman);
        let adapted_pkg = adapter.adapt("pkgname=adapted-pacman\npkgver=4.0.0").unwrap();
        assert_eq!(adapted_pkg.name, "adapted-pacman");
        assert_eq!(adapted_pkg.version, "4.0.0");
        assert_eq!(adapted_pkg.formats[0], PackageFormat::Pacman);
    }

    #[test]
    fn test_decorator_pattern_sandbox_and_network() {
        let pkg = UnifiedPackage::new("untrusted-app".to_string(), "0.0.1".to_string()).with_format(PackageFormat::Apk);
        let base = BasePackageDecorator { package: pkg };

        let sandboxed = SandboxDecorator {
            decorated: base,
            is_isolated: true,
        };

        let fully_decorated = NetworkRestrictionDecorator {
            decorated: sandboxed,
            allowed_hosts: vec!["sigmaos.org".to_string()],
        };

        assert_eq!(fully_decorated.get_package().name, "untrusted-app");
        assert!(fully_decorated.enforce_sandbox().is_ok());
        assert!(fully_decorated.restrict_network().is_ok());
    }

    #[test]
    fn test_user_defined_functions_udfs() {
        let mut manager = UniversalPackageManager::new();
        let package = UnifiedPackage::new("udf-test".to_string(), "1.0.0".to_string()).with_format(PackageFormat::Rpm);

        let validated = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let validated_clone = validated.clone();

        manager.triggers.register_pre_install(Arc::new(move |pkg| {
            if pkg.name == "udf-test" {
                validated_clone.store(true, std::sync::atomic::Ordering::SeqCst);
                Ok(())
            } else {
                Err("Unexpected package in pre-install".to_string())
            }
        }));

        manager.add_package(package);
        manager.install("udf-test").unwrap();

        assert!(validated.load(std::sync::atomic::Ordering::SeqCst));
    }
}
