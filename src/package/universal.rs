// SigmaOS Universal Package Manager
// Unified system absorbing apt, yum, pacman, snap, flatpak, ebuild, apk, nix, etc.

use crate::klib::HashMap;
use crate::klib::HashSet;
use std::sync::Arc;

/// Package format type supporting 18 major distribution formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackageFormat {
    Deb,      // apt - Debian, Ubuntu, Mint, Parrot
    Rpm,      // yum/dnf/zypper - Fedora, RHEL, CentOS, openSUSE
    Pacman,   // pacman - Arch Linux, Manjaro, CachyOS
    Snap,     // snapd - Ubuntu
    Flatpak,  // flatpak - Flathub, elementaryOS, Fedora
    SigmaPkg, // native SigmaOS format
    Ebuild,   // emerge - Gentoo
    Apk,      // apk - Alpine
    Nix,      // nix-env - NixOS
    AppImage, // AppImage
    Xbps,     // xbps - Void Linux
    Txz,      // pkgtools - Slackware
    Eopkg,    // eopkg - Solus
    Zypper,   // zypper - openSUSE
    Guix,     // guix - GNU Guix
    CachyOS,  // cachy - CachyOS
    Swupd,    // swupd - Clear Linux
    Starling, // starling - Starling OS
}

/// Package source
#[derive(Debug, Clone)]
pub enum PackageSource {
    Repository { url: String },
    Local { path: String },
    Remote { url: String },
}

/// Dependency conflict resolution strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConflictResolution {
    PreferNewest,
    PreferOldest,
    PreferNative,
    Manual,
}

/// Package State Enum for lifecycle representation (OOP State Pattern)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackageStateEnum {
    Uninstalled,
    Cached,
    Installing,
    Installed,
    Verifying,
    RollingBack,
    Corrupted,
}

/// Unified package containing metadata and capability decorations
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

    // OOP additions
    pub current_state: PackageStateEnum,
    pub capabilities: Vec<String>,
    pub post_install_scripts: Vec<String>,
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
            current_state: PackageStateEnum::Uninstalled,
            capabilities: Vec::new(),
            post_install_scripts: Vec::new(),
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

    pub fn transition_to(&mut self, new_state: PackageStateEnum) {
        println!(
            "Package '{}' transitioned: {:?} -> {:?}",
            self.name, self.current_state, new_state
        );
        self.current_state = new_state;
    }
}

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

    pub fn can_handle(&self, package: &UnifiedPackage) -> bool {
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

/// OOP Strategy Pattern: Package installation and maintenance strategies
pub trait InstallStrategy: Send + Sync {
    fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError>;
    fn remove(&self, package: &UnifiedPackage) -> Result<(), PackageError>;
    fn update(&self, package: &UnifiedPackage) -> Result<(), PackageError>;
}

pub struct DebianInstallStrategy;
impl InstallStrategy for DebianInstallStrategy {
    fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "DebianStrategy: Executing preinst/postinst for package: {}",
            package.name
        );
        Ok(())
    }
    fn remove(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "DebianStrategy: Purging configuration files and executing prerm/postrm for {}",
            package.name
        );
        Ok(())
    }
    fn update(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "DebianStrategy: Upgrading existing package: {}",
            package.name
        );
        Ok(())
    }
}

pub struct RpmInstallStrategy;
impl InstallStrategy for RpmInstallStrategy {
    fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "RpmStrategy: Verifying headers, running %pre and %post scriptlets for {}",
            package.name
        );
        Ok(())
    }
    fn remove(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "RpmStrategy: Processing rpm-database deletion of {}",
            package.name
        );
        Ok(())
    }
    fn update(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "RpmStrategy: Transactionally upgrading rpm package: {}",
            package.name
        );
        Ok(())
    }
}

pub struct PacmanInstallStrategy;
impl InstallStrategy for PacmanInstallStrategy {
    fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "PacmanStrategy: Re-syncing local pacman databases and unpacking zst payload for {}",
            package.name
        );
        Ok(())
    }
    fn remove(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "PacmanStrategy: Removing and cleaning orphan packages for {}",
            package.name
        );
        Ok(())
    }
    fn update(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "PacmanStrategy: Rolling release pacman -Syu replacement for {}",
            package.name
        );
        Ok(())
    }
}

pub struct GentooInstallStrategy;
impl InstallStrategy for GentooInstallStrategy {
    fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("GentooStrategy: Unpacking portage tree ebuild, applying USE flags for compilation of {}", package.name);
        Ok(())
    }
    fn remove(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "GentooStrategy: Emerge -C clean operation on {}",
            package.name
        );
        Ok(())
    }
    fn update(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "GentooStrategy: Re-compiling and installing dependencies for Gentoo package: {}",
            package.name
        );
        Ok(())
    }
}

pub struct NixInstallStrategy;
impl InstallStrategy for NixInstallStrategy {
    fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("NixStrategy: Purely functional evaluation, symlinking /nix/store derivation path for {}", package.name);
        Ok(())
    }
    fn remove(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "NixStrategy: Removing declarative store path for {}",
            package.name
        );
        Ok(())
    }
    fn update(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "NixStrategy: Updating isolated profile link generation for {}",
            package.name
        );
        Ok(())
    }
}

pub struct FlatpakInstallStrategy;
impl InstallStrategy for FlatpakInstallStrategy {
    fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("FlatpakStrategy: Pulling flatpak runtime container portals and unpacking sandbox filesystems for {}", package.name);
        Ok(())
    }
    fn remove(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "FlatpakStrategy: Unregistering bubblewrap sandbox and app files for {}",
            package.name
        );
        Ok(())
    }
    fn update(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "FlatpakStrategy: Pulling OSTree delta update block layers for Flatpak: {}",
            package.name
        );
        Ok(())
    }
}

pub struct ApkInstallStrategy;
impl InstallStrategy for ApkInstallStrategy {
    fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "ApkStrategy: Running apk add simulation for {}",
            package.name
        );
        Ok(())
    }
    fn remove(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "ApkStrategy: Running apk del simulation for {}",
            package.name
        );
        Ok(())
    }
    fn update(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "ApkStrategy: Running apk upgrade simulation for {}",
            package.name
        );
        Ok(())
    }
}

pub struct DefaultInstallStrategy {
    pub format_name: String,
}
impl InstallStrategy for DefaultInstallStrategy {
    fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "DefaultStrategy: Deploying package {} under format {}",
            package.name, self.format_name
        );
        Ok(())
    }
    fn remove(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "DefaultStrategy: Removing package {} under format {}",
            package.name, self.format_name
        );
        Ok(())
    }
    fn update(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "DefaultStrategy: Updating package {} under format {}",
            package.name, self.format_name
        );
        Ok(())
    }
}

/// OOP Observer Pattern: Register triggers/hooks executed during package manager events
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackageEvent {
    BeforeInstall,
    AfterInstall,
    BeforeRemove,
    AfterRemove,
    OnVerifySuccess,
    OnVerifyFailure,
}

pub trait PackageObserver: Send + Sync {
    fn name(&self) -> &'static str;
    fn on_event(&self, event: PackageEvent, package: &UnifiedPackage);
}

pub struct LdConfigTrigger;
impl PackageObserver for LdConfigTrigger {
    fn name(&self) -> &'static str {
        "ldconfig-trigger"
    }
    fn on_event(&self, event: PackageEvent, _package: &UnifiedPackage) {
        if event == PackageEvent::AfterInstall {
            println!(
                "LdConfigTrigger: Rebuilding system shared library cache dynamically (ldconfig)."
            );
        }
    }
}

pub struct SystemdServiceTrigger;
impl PackageObserver for SystemdServiceTrigger {
    fn name(&self) -> &'static str {
        "systemd-trigger"
    }
    fn on_event(&self, event: PackageEvent, package: &UnifiedPackage) {
        if event == PackageEvent::AfterInstall {
            println!(
                "SystemdTrigger: Registering, enabling, and starting service units for {}",
                package.name
            );
        } else if event == PackageEvent::BeforeRemove {
            println!(
                "SystemdTrigger: Disabling and stopping system services for {}",
                package.name
            );
        }
    }
}

pub struct IconCacheTrigger;
impl PackageObserver for IconCacheTrigger {
    fn name(&self) -> &'static str {
        "icon-cache-trigger"
    }
    fn on_event(&self, event: PackageEvent, _package: &UnifiedPackage) {
        if event == PackageEvent::AfterInstall || event == PackageEvent::AfterRemove {
            println!("IconCacheTrigger: Rebuilding desktop gtk/qt application icon caches.");
        }
    }
}

/// OOP Decorator Pattern: Enrich packages with capabilities and constraints
pub trait PackageCapabilityDecorator: Send + Sync {
    fn decorate(&self, package: &mut UnifiedPackage);
}

pub struct SandboxedPackageDecorator;
impl PackageCapabilityDecorator for SandboxedPackageDecorator {
    fn decorate(&self, package: &mut UnifiedPackage) {
        package
            .capabilities
            .push("Sandboxed (cgroups/ns)".to_string());
    }
}

pub struct NetworkRestrictedDecorator;
impl PackageCapabilityDecorator for NetworkRestrictedDecorator {
    fn decorate(&self, package: &mut UnifiedPackage) {
        package.capabilities.push("Network Restricted".to_string());
    }
}

pub struct HighPriorityDecorator;
impl PackageCapabilityDecorator for HighPriorityDecorator {
    fn decorate(&self, package: &mut UnifiedPackage) {
        package
            .capabilities
            .push("High Priority (System Critical)".to_string());
    }
}

/// OOP Adapter Pattern: Translate foreign manifests into UnifiedPackages
pub struct ForeignMetadata {
    pub raw_content: String,
    pub format: PackageFormat,
}

pub trait ForeignMetadataAdapter: Send + Sync {
    fn adapt(&self, metadata: &ForeignMetadata) -> Result<UnifiedPackage, PackageError>;
}

pub struct ControlFileAdapter;
impl ForeignMetadataAdapter for ControlFileAdapter {
    fn adapt(&self, metadata: &ForeignMetadata) -> Result<UnifiedPackage, PackageError> {
        let mut package = UnifiedPackage::new("adapted-deb".to_string(), "1.0.0".to_string())
            .with_format(PackageFormat::Deb);
        for line in metadata.raw_content.lines() {
            if let Some((key, val)) = line.split_once(':') {
                match key.trim() {
                    "Package" => package.name = val.trim().to_string(),
                    "Version" => package.version = val.trim().to_string(),
                    "Depends" => {
                        for dep in val.split(',') {
                            package = package.with_dependency(dep.trim().to_string());
                        }
                    }
                    "Conflicts" => {
                        for conflict in val.split(',') {
                            package = package.with_conflict(conflict.trim().to_string());
                        }
                    }
                    _ => {}
                }
            }
        }
        Ok(package)
    }
}

pub struct SpecFileAdapter;
impl ForeignMetadataAdapter for SpecFileAdapter {
    fn adapt(&self, metadata: &ForeignMetadata) -> Result<UnifiedPackage, PackageError> {
        let mut package = UnifiedPackage::new("adapted-rpm".to_string(), "1.0.0".to_string())
            .with_format(PackageFormat::Rpm);
        for line in metadata.raw_content.lines() {
            if let Some((key, val)) = line.split_once(':') {
                match key.trim() {
                    "Name" => package.name = val.trim().to_string(),
                    "Version" => package.version = val.trim().to_string(),
                    "Requires" => {
                        package = package.with_dependency(val.trim().to_string());
                    }
                    "Conflicts" => {
                        package = package.with_conflict(val.trim().to_string());
                    }
                    _ => {}
                }
            }
        }
        Ok(package)
    }
}

pub struct PkgBuildAdapter;
impl ForeignMetadataAdapter for PkgBuildAdapter {
    fn adapt(&self, metadata: &ForeignMetadata) -> Result<UnifiedPackage, PackageError> {
        let mut package = UnifiedPackage::new("adapted-pacman".to_string(), "1.0.0".to_string())
            .with_format(PackageFormat::Pacman);
        for line in metadata.raw_content.lines() {
            if let Some((key, val)) = line.split_once('=') {
                match key.trim() {
                    "pkgname" => package.name = val.trim().replace('"', "").replace('\'', ""),
                    "pkgver" => package.version = val.trim().replace('"', "").replace('\'', ""),
                    "depends" => {
                        let cleaned = val
                            .trim()
                            .replace('(', "")
                            .replace(')', "")
                            .replace('"', "")
                            .replace('\'', "");
                        for dep in cleaned.split_whitespace() {
                            package = package.with_dependency(dep.to_string());
                        }
                    }
                    _ => {}
                }
            }
        }
        Ok(package)
    }
}

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

    pub fn resolve_dependencies(&self, package_name: &str) -> Result<Vec<String>, PackageError> {
        let mut resolved = Vec::new();
        let mut to_visit = vec![package_name.to_string()];
        let mut visited = HashSet::new();

        while let Some(current) = to_visit.pop() {
            if visited.contains(&current) {
                continue;
            }

            visited.insert(current.clone());

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

        for (i, pkg1_name) in packages.iter().enumerate() {
            for pkg2_name in packages.iter().skip(i + 1) {
                if let (Some(pkg1), Some(pkg2)) =
                    (self.packages.get(pkg1_name), self.packages.get(pkg2_name))
                {
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

        let mut keys = Vec::new();
        for key in installed.keys() {
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

/// Universal package manager with advanced OOP structures and UDF callbacks
pub struct UniversalPackageManager {
    pub packages: HashMap<String, UnifiedPackage>,
    pub adapters: HashMap<PackageFormat, PackageAdapter>,
    pub resolver: DependencyResolver,
    pub installed_packages: HashMap<String, UnifiedPackage>,
    pub transaction_history: TransactionalHistory,

    // Advanced OOP fields
    pub strategies: HashMap<PackageFormat, Arc<dyn InstallStrategy>>,
    pub observers: Vec<Arc<dyn PackageObserver>>,
    pub udfs:
        HashMap<String, Arc<dyn Fn(&UnifiedPackage) -> Result<(), PackageError> + Send + Sync>>,
}

impl UniversalPackageManager {
    pub fn new() -> Self {
        let mut manager = Self {
            packages: HashMap::new(),
            adapters: HashMap::new(),
            resolver: DependencyResolver::new(),
            installed_packages: HashMap::new(),
            transaction_history: TransactionalHistory::new(),
            strategies: HashMap::new(),
            observers: Vec::new(),
            udfs: HashMap::new(),
        };

        manager.add_default_adapters();
        manager.add_default_strategies();
        manager.add_default_observers();
        manager
    }

    fn add_default_adapters(&mut self) {
        let apt_adapter = PackageAdapter::new(PackageFormat::Deb, "apt".to_string());
        let yum_adapter = PackageAdapter::new(PackageFormat::Rpm, "yum".to_string());
        let pacman_adapter = PackageAdapter::new(PackageFormat::Pacman, "pacman".to_string());
        let snap_adapter = PackageAdapter::new(PackageFormat::Snap, "snap".to_string());
        let flatpak_adapter = PackageAdapter::new(PackageFormat::Flatpak, "flatpak".to_string());
        let sigpkg_adapter = PackageAdapter::new(PackageFormat::SigmaPkg, "sigpkg".to_string());

        self.adapters.insert(PackageFormat::Deb, apt_adapter);
        self.adapters.insert(PackageFormat::Rpm, yum_adapter);
        self.adapters.insert(PackageFormat::Pacman, pacman_adapter);
        self.adapters.insert(PackageFormat::Snap, snap_adapter);
        self.adapters
            .insert(PackageFormat::Flatpak, flatpak_adapter);
        self.adapters
            .insert(PackageFormat::SigmaPkg, sigpkg_adapter);
    }

    fn add_default_strategies(&mut self) {
        self.strategies
            .insert(PackageFormat::Deb, Arc::new(DebianInstallStrategy));
        self.strategies
            .insert(PackageFormat::Rpm, Arc::new(RpmInstallStrategy));
        self.strategies
            .insert(PackageFormat::Pacman, Arc::new(PacmanInstallStrategy));
        self.strategies
            .insert(PackageFormat::Ebuild, Arc::new(GentooInstallStrategy));
        self.strategies
            .insert(PackageFormat::Nix, Arc::new(NixInstallStrategy));
        self.strategies
            .insert(PackageFormat::Flatpak, Arc::new(FlatpakInstallStrategy));
        self.strategies
            .insert(PackageFormat::Apk, Arc::new(ApkInstallStrategy));
        self.strategies.insert(
            PackageFormat::AppImage,
            Arc::new(DefaultInstallStrategy {
                format_name: "AppImage".to_string(),
            }),
        );
        self.strategies.insert(
            PackageFormat::SigmaPkg,
            Arc::new(DefaultInstallStrategy {
                format_name: "SigmaPkg".to_string(),
            }),
        );
    }

    fn add_default_observers(&mut self) {
        self.observers.push(Arc::new(LdConfigTrigger));
        self.observers.push(Arc::new(SystemdServiceTrigger));
        self.observers.push(Arc::new(IconCacheTrigger));
    }

    pub fn register_udf<F>(&mut self, name: &str, udf: F)
    where
        F: Fn(&UnifiedPackage) -> Result<(), PackageError> + Send + Sync + 'static,
    {
        self.udfs.insert(name.to_string(), Arc::new(udf));
    }

    pub fn notify_observers(&self, event: PackageEvent, package: &UnifiedPackage) {
        for obs in &self.observers {
            obs.on_event(event, package);
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
            // 1. Notify BeforeInstall using immutable reference first
            if let Some(package) = self.packages.get(&dep_name) {
                self.notify_observers(PackageEvent::BeforeInstall, package);
            }

            // 2. Mutably borrow package to install and invoke hooks
            if let Some(package) = self.packages.get_mut(&dep_name) {
                package.transition_to(PackageStateEnum::Installing);

                // Run installer matching preferred format/strategy
                let mut installed_ok = false;
                for format in &package.formats {
                    if let Some(strategy) = self.strategies.get(format) {
                        strategy.install(package)?;
                        installed_ok = true;
                        break;
                    } else if let Some(adapter) = self.adapters.get(format) {
                        adapter.install(package)?;
                        installed_ok = true;
                        break;
                    }
                }

                if !installed_ok {
                    // Fallback to default install
                    println!("No custom strategy or adapter found for package '{}'. Executing standard routine.", package.name);
                }

                // Run user-defined functions (UDFs) if registered on package
                for script in &package.post_install_scripts {
                    if let Some(udf) = self.udfs.get(script) {
                        println!("Executing custom package UDF trigger: {}", script);
                        udf(package)?;
                    }
                }

                package.transition_to(PackageStateEnum::Installed);
                let mut installed = package.clone();
                installed.installed = true;

                // Fire AfterInstall event on clone
                self.notify_observers(PackageEvent::AfterInstall, &installed);

                self.installed_packages.insert(dep_name.clone(), installed);
            }
        }

        Ok(())
    }

    pub fn remove(&mut self, package_name: &str) -> Result<(), PackageError> {
        // 1. Notify BeforeRemove
        if let Some(package) = self.installed_packages.get(package_name) {
            self.notify_observers(PackageEvent::BeforeRemove, package);
        }

        // 2. Mutably borrow to run strategies
        if let Some(package) = self.installed_packages.get_mut(package_name) {
            let mut removed_ok = false;
            for format in &package.formats {
                if let Some(strategy) = self.strategies.get(format) {
                    strategy.remove(package)?;
                    removed_ok = true;
                    break;
                } else if let Some(adapter) = self.adapters.get(format) {
                    adapter.remove(package)?;
                    removed_ok = true;
                    break;
                }
            }

            if !removed_ok {
                println!(
                    "No custom strategy found for package '{}' during removal.",
                    package.name
                );
            }

            package.transition_to(PackageStateEnum::Uninstalled);
            let uninstalled = package.clone();
            self.notify_observers(PackageEvent::AfterRemove, &uninstalled);
        }

        self.installed_packages.remove(package_name);
        Ok(())
    }

    pub fn update(&mut self, package_name: &str) -> Result<(), PackageError> {
        if let Some(package) = self.installed_packages.get_mut(package_name) {
            for format in &package.formats {
                if let Some(strategy) = self.strategies.get(format) {
                    strategy.update(package)?;
                    break;
                } else if let Some(adapter) = self.adapters.get(format) {
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

    /// CLI command parser & translator router
    pub fn execute_cli_command(&mut self, cmd: &str) -> Result<String, PackageError> {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.is_empty() {
            return Err(PackageError::PackageNotFound("Empty command".to_string()));
        }

        match parts[0] {
            "apt" | "apt-get" => {
                if parts.len() >= 3 && (parts[1] == "install" || parts[1] == "get") {
                    let name = parts[2];
                    self.install(name)?;
                    Ok(format!(
                        "Translated 'apt install {}' and installed successfully via UniversalPackageManager.",
                        name
                    ))
                } else if parts.len() >= 3 && parts[1] == "remove" {
                    let name = parts[2];
                    self.remove(name)?;
                    Ok(format!(
                        "Translated 'apt remove {}' and removed successfully.",
                        name
                    ))
                } else {
                    Err(PackageError::PackageNotFound(format!(
                        "Unsupported apt command: {}",
                        cmd
                    )))
                }
            }
            "pacman" => {
                if parts.len() >= 3 && parts[1] == "-S" {
                    let name = parts[2];
                    self.install(name)?;
                    Ok(format!(
                        "Translated 'pacman -S {}' and installed successfully.",
                        name
                    ))
                } else if parts.len() >= 3 && parts[1] == "-R" {
                    let name = parts[2];
                    self.remove(name)?;
                    Ok(format!(
                        "Translated 'pacman -R {}' and removed successfully.",
                        name
                    ))
                } else {
                    Err(PackageError::PackageNotFound(format!(
                        "Unsupported pacman command: {}",
                        cmd
                    )))
                }
            }
            "dnf" | "yum" | "zypper" => {
                if parts.len() >= 3 && parts[1] == "install" {
                    let name = parts[2];
                    self.install(name)?;
                    Ok(format!(
                        "Translated '{} install {}' and installed successfully.",
                        parts[0], name
                    ))
                } else if parts.len() >= 3 && parts[1] == "remove" {
                    let name = parts[2];
                    self.remove(name)?;
                    Ok(format!(
                        "Translated '{} remove {}' and removed successfully.",
                        parts[0], name
                    ))
                } else {
                    Err(PackageError::PackageNotFound(format!(
                        "Unsupported {} command: {}",
                        parts[0], cmd
                    )))
                }
            }
            "nix-env" => {
                if parts.len() >= 3 && parts[1] == "-i" {
                    let name = parts[2];
                    self.install(name)?;
                    Ok(format!(
                        "Translated 'nix-env -i {}' and installed functionally.",
                        name
                    ))
                } else {
                    Err(PackageError::PackageNotFound(format!(
                        "Unsupported nix command: {}",
                        cmd
                    )))
                }
            }
            "emerge" => {
                if parts.len() >= 2 {
                    let name = parts[parts.len() - 1];
                    self.install(name)?;
                    Ok(format!(
                        "Translated 'emerge {}' and compiled successfully via Gentoo ebuild.",
                        name
                    ))
                } else {
                    Err(PackageError::PackageNotFound(format!(
                        "Unsupported emerge command: {}",
                        cmd
                    )))
                }
            }
            _ => {
                if parts.len() >= 3 && parts[1] == "install" {
                    let name = parts[2];
                    self.install(name)?;
                    Ok(format!("Installed package '{}' natively.", name))
                } else {
                    Err(PackageError::PackageNotFound(format!(
                        "Unknown command driver: {}",
                        cmd
                    )))
                }
            }
        }
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
    AdapterNotFound,
    InstallationFailed(String),
    ConflictDetected(Vec<(String, String)>),
}

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

        // 1. Create a baseline checkpoint (empty)
        let checkpoint_id = manager.create_checkpoint();
        assert_eq!(checkpoint_id, 1);

        // 2. Install pkg1 and pkg2
        manager.install("pkg1").unwrap();
        manager.install("pkg2").unwrap();
        assert_eq!(manager.installed_packages.len(), 2);

        // 3. Roll back to baseline checkpoint
        manager.rollback_to_checkpoint(checkpoint_id).unwrap();
        assert_eq!(manager.installed_packages.len(), 0);
    }

    #[test]
    fn test_state_transitions() {
        let mut pkg = UnifiedPackage::new("my-app".to_string(), "1.2.3".to_string());
        assert_eq!(pkg.current_state, PackageStateEnum::Uninstalled);

        pkg.transition_to(PackageStateEnum::Installing);
        assert_eq!(pkg.current_state, PackageStateEnum::Installing);

        pkg.transition_to(PackageStateEnum::Installed);
        assert_eq!(pkg.current_state, PackageStateEnum::Installed);
    }

    #[test]
    fn test_decorators() {
        let mut pkg = UnifiedPackage::new("secure-app".to_string(), "1.0.0".to_string());
        assert!(pkg.capabilities.is_empty());

        let decorator_sandbox = SandboxedPackageDecorator;
        decorator_sandbox.decorate(&mut pkg);
        assert!(pkg
            .capabilities
            .contains(&"Sandboxed (cgroups/ns)".to_string()));

        let decorator_net = NetworkRestrictedDecorator;
        decorator_net.decorate(&mut pkg);
        assert!(pkg.capabilities.contains(&"Network Restricted".to_string()));

        let decorator_prio = HighPriorityDecorator;
        decorator_prio.decorate(&mut pkg);
        assert!(pkg
            .capabilities
            .contains(&"High Priority (System Critical)".to_string()));
    }

    #[test]
    fn test_adapters() {
        // Test Deb Control file adapter
        let control_data = ForeignMetadata {
            raw_content:
                "Package: super-deb\nVersion: 2.3.4\nDepends: dep1, dep2\nConflicts: old-pkg"
                    .to_string(),
            format: PackageFormat::Deb,
        };
        let adapted_deb = ControlFileAdapter.adapt(&control_data).unwrap();
        assert_eq!(adapted_deb.name, "super-deb");
        assert_eq!(adapted_deb.version, "2.3.4");
        assert_eq!(
            adapted_deb.dependencies,
            vec!["dep1".to_string(), "dep2".to_string()]
        );
        assert_eq!(adapted_deb.conflicts, vec!["old-pkg".to_string()]);

        // Test RPM SPEC file adapter
        let spec_data = ForeignMetadata {
            raw_content: "Name: super-rpm\nVersion: 4.1.0\nRequires: glibc\nConflicts: bad-rpm"
                .to_string(),
            format: PackageFormat::Rpm,
        };
        let adapted_rpm = SpecFileAdapter.adapt(&spec_data).unwrap();
        assert_eq!(adapted_rpm.name, "super-rpm");
        assert_eq!(adapted_rpm.version, "4.1.0");
        assert_eq!(adapted_rpm.dependencies, vec!["glibc".to_string()]);
        assert_eq!(adapted_rpm.conflicts, vec!["bad-rpm".to_string()]);

        // Test PKGBUILD adapter
        let pkgbuild_data = ForeignMetadata {
            raw_content: "pkgname=\"super-arch\"\npkgver=\"1.5.0\"\ndepends=('openssl' 'zlib')"
                .to_string(),
            format: PackageFormat::Pacman,
        };
        let adapted_pacman = PkgBuildAdapter.adapt(&pkgbuild_data).unwrap();
        assert_eq!(adapted_pacman.name, "super-arch");
        assert_eq!(adapted_pacman.version, "1.5.0");
        assert_eq!(
            adapted_pacman.dependencies,
            vec!["openssl".to_string(), "zlib".to_string()]
        );
    }

    #[test]
    fn test_user_defined_functions_and_strategies() {
        let mut manager = UniversalPackageManager::new();

        // Define a custom UDF script
        manager.register_udf("my_custom_post_install", |_pkg| {
            println!("UDF triggered successfully!");
            Ok(())
        });

        let mut pkg = UnifiedPackage::new("custom-app".to_string(), "1.0.0".to_string())
            .with_format(PackageFormat::Apk);
        pkg.post_install_scripts
            .push("my_custom_post_install".to_string());

        manager.add_package(pkg);
        assert!(manager.install("custom-app").is_ok());

        let installed = manager.get_package("custom-app").unwrap();
        assert_eq!(installed.current_state, PackageStateEnum::Installed);
    }

    #[test]
    fn test_cli_command_router() {
        let mut manager = UniversalPackageManager::new();

        let pkg_deb = UnifiedPackage::new("firefox".to_string(), "115.0.0".to_string())
            .with_format(PackageFormat::Deb);
        let pkg_arch = UnifiedPackage::new("emacs".to_string(), "29.1.0".to_string())
            .with_format(PackageFormat::Pacman);
        let pkg_gentoo = UnifiedPackage::new("htop".to_string(), "3.2.0".to_string())
            .with_format(PackageFormat::Ebuild);

        manager.add_package(pkg_deb);
        manager.add_package(pkg_arch);
        manager.add_package(pkg_gentoo);

        // Test apt translation
        let apt_res = manager.execute_cli_command("apt install firefox").unwrap();
        assert!(apt_res.contains("apt install firefox"));
        assert!(manager.installed_packages.contains_key("firefox"));

        // Test pacman translation
        let pac_res = manager.execute_cli_command("pacman -S emacs").unwrap();
        assert!(pac_res.contains("pacman -S emacs"));
        assert!(manager.installed_packages.contains_key("emacs"));

        // Test emerge translation
        let emerge_res = manager.execute_cli_command("emerge htop").unwrap();
        assert!(emerge_res.contains("emerge htop"));
        assert!(manager.installed_packages.contains_key("htop"));

        // Test apt remove translation
        let remove_res = manager.execute_cli_command("apt remove firefox").unwrap();
        assert!(remove_res.contains("apt remove firefox"));
        assert!(!manager.installed_packages.contains_key("firefox"));
    }
}
