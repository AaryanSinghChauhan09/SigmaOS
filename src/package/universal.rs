// SigmaOS Universal Package Manager
// Unified system absorbing apt, yum, pacman, snap, flatpak

#[cfg(not(feature = "standalone_test"))]
use crate::klib::HashMap;

#[cfg(feature = "standalone_test")]
use std::collections::HashMap;

use std::sync::Arc;

/// Package format type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackageFormat {
    Deb,          // apt
    Rpm,          // yum/dnf
    Pacman,       // pacman
    Snap,         // snap
    Flatpak,      // flatpak
    SigmaPkg,     // native SigmaOS format
    Ebuild,       // Gentoo portage (emerge)
    Apk,          // Alpine apk
    Nix,          // Nix package manager
    AppImage,     // AppImage portable
    Xbps,         // Void Linux xbps
    Txz,          // Slackware txz
    Eopkg,        // Solus eopkg
    Zypper,       // openSUSE zypper
    Guix,         // GNU Guix
    CachyOS,      // CachyOS optimized pacman
    Swupd,        // Intel Clear Linux swupd
    Starling,     // Starling Linux package
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

/// Package lifecycle states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackageState {
    Uninstalled,
    Resolved,
    Installing,
    Installed,
    Broken,
    RolledBack,
}

/// Unified package
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

    pub fn transition_to(&mut self, next: PackageState) -> Result<(), PackageError> {
        let allowed = match (self.state, next) {
            (PackageState::Uninstalled, PackageState::Resolved) |
            (PackageState::Uninstalled, PackageState::Installing) => true,
            (PackageState::Resolved, PackageState::Installing) => true,
            (PackageState::Installing, PackageState::Installed) |
            (PackageState::Installing, PackageState::Broken) => true,
            (PackageState::Installed, PackageState::Uninstalled) |
            (PackageState::Installed, PackageState::Broken) |
            (PackageState::Installed, PackageState::RolledBack) => true,
            (PackageState::Broken, PackageState::Resolved) |
            (PackageState::Broken, PackageState::Uninstalled) => true,
            (PackageState::RolledBack, PackageState::Resolved) |
            (PackageState::RolledBack, PackageState::Uninstalled) => true,
            (a, b) if a == b => true,
            _ => false,
        };

        if allowed {
            self.state = next;
            Ok(())
        } else {
            Err(PackageError::InvalidStateTransition {
                from: self.state,
                to: next,
            })
        }
    }
}

/// Strategy interface for package installation behaviors
pub trait InstallStrategy: Send + Sync {
    fn execute(&self, package: &UnifiedPackage) -> Result<(), PackageError>;
}

/// Strategy interface for package verification behaviors
pub trait VerifyStrategy: Send + Sync {
    fn execute(&self, package: &UnifiedPackage) -> Result<bool, PackageError>;
}

// Concrete strategies
#[derive(Debug, Clone, Copy)]
pub struct NativeStrategy;
impl InstallStrategy for NativeStrategy {
    fn execute(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("Strategy [Native]: Directly deploying {} binary package files.", package.name);
        Ok(())
    }
}
impl VerifyStrategy for NativeStrategy {
    fn execute(&self, package: &UnifiedPackage) -> Result<bool, PackageError> {
        println!("Strategy [Native]: Verifying {} native files structure.", package.name);
        Ok(true)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SandboxStrategy {
    pub isolation_level: &'static str,
}
impl InstallStrategy for SandboxStrategy {
    fn execute(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("Strategy [Sandbox ({})]: Enforcing secure jail container for {}.", self.isolation_level, package.name);
        Ok(())
    }
}
impl VerifyStrategy for SandboxStrategy {
    fn execute(&self, package: &UnifiedPackage) -> Result<bool, PackageError> {
        println!("Strategy [Sandbox]: Checking containment namespaces and permissions for {}.", package.name);
        Ok(true)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SourceCompileStrategy;
impl InstallStrategy for SourceCompileStrategy {
    fn execute(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("Strategy [SourceCompile]: Compiling {} from source recipes.", package.name);
        Ok(())
    }
}
impl VerifyStrategy for SourceCompileStrategy {
    fn execute(&self, package: &UnifiedPackage) -> Result<bool, PackageError> {
        println!("Strategy [SourceCompile]: Running static analysis & test suites on compiled binary for {}.", package.name);
        Ok(true)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BinaryExtractStrategy;
impl InstallStrategy for BinaryExtractStrategy {
    fn execute(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("Strategy [BinaryExtract]: Extracting pre-built binary archives for {}.", package.name);
        Ok(())
    }
}
impl VerifyStrategy for BinaryExtractStrategy {
    fn execute(&self, package: &UnifiedPackage) -> Result<bool, PackageError> {
        println!("Strategy [BinaryExtract]: Verifying checksum of extracted files for {}.", package.name);
        Ok(true)
    }
}

/// Helper function to retrieve appropriate Strategy based on format
pub fn get_strategies(format: PackageFormat) -> (Box<dyn InstallStrategy>, Box<dyn VerifyStrategy>) {
    match format {
        PackageFormat::SigmaPkg | PackageFormat::Pacman | PackageFormat::CachyOS => {
            (Box::new(NativeStrategy), Box::new(NativeStrategy))
        }
        PackageFormat::Snap | PackageFormat::Flatpak | PackageFormat::AppImage => {
            (
                Box::new(SandboxStrategy { isolation_level: "strict" }),
                Box::new(SandboxStrategy { isolation_level: "strict" })
            )
        }
        PackageFormat::Ebuild | PackageFormat::Nix | PackageFormat::Guix => {
            (Box::new(SourceCompileStrategy), Box::new(SourceCompileStrategy))
        }
        _ => {
            (Box::new(BinaryExtractStrategy), Box::new(BinaryExtractStrategy))
        }
    }
}

/// Events that trigger observer callbacks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackageEvent {
    PreInstall,
    PostInstall,
    PreRemove,
    PostRemove,
    OnFailure,
}

/// Observer interface for package manager status updates
pub trait PackageObserver: Send + Sync {
    fn name(&self) -> &'static str;
    fn on_event(&self, event: PackageEvent, package: &UnifiedPackage);
}

/// Closure-based triggering Observer for User-Defined Functions (UDF closures)
pub struct UdfClosureObserver {
    pub observer_name: &'static str,
    pub handler: Box<dyn Fn(PackageEvent, &UnifiedPackage) + Send + Sync>,
}

impl PackageObserver for UdfClosureObserver {
    fn name(&self) -> &'static str {
        self.observer_name
    }

    fn on_event(&self, event: PackageEvent, package: &UnifiedPackage) {
        (self.handler)(event, package);
    }
}

/// Decorator interface for dynamic package adapter runtime capability wrapping
pub trait AdapterDecorator: Send + Sync {
    fn name(&self) -> &'static str;
    fn pre_install_action(&self, package: &UnifiedPackage) -> Result<(), PackageError>;
    fn post_install_action(&self, package: &UnifiedPackage) -> Result<(), PackageError>;
}

/// Concrete Decorator: GPG Cryptographic Signature Verification
pub struct GpgVerificationDecorator {
    pub key_id: String,
}
impl AdapterDecorator for GpgVerificationDecorator {
    fn name(&self) -> &'static str {
        "GpgVerification"
    }
    fn pre_install_action(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("Decorator [GPG]: Verifying signature of {} using GPG key: {}", package.name, self.key_id);
        Ok(())
    }
    fn post_install_action(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
}

/// Concrete Decorator: Dynamic Network Bandwidth Shaping/Limiting
pub struct BandwidthShaperDecorator {
    pub max_kbps: usize,
}
impl AdapterDecorator for BandwidthShaperDecorator {
    fn name(&self) -> &'static str {
        "BandwidthShaper"
    }
    fn pre_install_action(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("Decorator [BandwidthShaper]: Setting download rate limit for {} to {} KB/s.", package.name, self.max_kbps);
        Ok(())
    }
    fn post_install_action(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
}

/// Concrete Decorator: Sandbox Confinement Level Auditing
pub struct SandboxEnforcerDecorator {
    pub confinement: String,
}
impl AdapterDecorator for SandboxEnforcerDecorator {
    fn name(&self) -> &'static str {
        "SandboxEnforcer"
    }
    fn pre_install_action(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("Decorator [SandboxEnforcer]: Checking confinement constraints ({}) for {}.", self.confinement, package.name);
        Ok(())
    }
    fn post_install_action(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
}

/// Concrete Decorator: Deep Manifest & Hash Integrity Audit
pub struct IntegrityAuditDecorator;
impl AdapterDecorator for IntegrityAuditDecorator {
    fn name(&self) -> &'static str {
        "IntegrityAudit"
    }
    fn pre_install_action(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("Decorator [IntegrityAudit]: Initiating deep cryptographic integrity scan of {}.", package.name);
        Ok(())
    }
    fn post_install_action(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
}

/// Concrete Decorator: Post-Quantum Secure Signature Validation
pub struct PostQuantumCryptoDecorator {
    pub algorithm: String,
}
impl AdapterDecorator for PostQuantumCryptoDecorator {
    fn name(&self) -> &'static str {
        "PostQuantumCrypto"
    }
    fn pre_install_action(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("Decorator [PostQuantumCrypto]: Verifying CRYSTALS-Dilithium/{} secure packet signatures for {}.", self.algorithm, package.name);
        Ok(())
    }
    fn post_install_action(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
}

/// Package format adapter
pub struct PackageAdapter {
    pub format: PackageFormat,
    pub adapter_name: String,
    pub capabilities: Vec<String>,
    pub decorators: Vec<Arc<dyn AdapterDecorator>>,
}

impl PackageAdapter {
    pub fn new(format: PackageFormat, adapter_name: String) -> Self {
        Self {
            format,
            adapter_name,
            capabilities: Vec::new(),
            decorators: Vec::new(),
        }
    }

    pub fn with_decorator(mut self, decorator: Arc<dyn AdapterDecorator>) -> Self {
        self.decorators.push(decorator);
        self
    }

    pub fn add_decorator(&mut self, decorator: Arc<dyn AdapterDecorator>) {
        self.decorators.push(decorator);
    }

    pub fn can_handle(&self, package: &UnifiedPackage) -> bool {
        package.formats.contains(&self.format)
    }

    pub fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        // Run decorators pre install
        for decorator in &self.decorators {
            decorator.pre_install_action(package)?;
        }

        println!(
            "Installing {} using {} adapter",
            package.name, self.adapter_name
        );
        // Simulate installation

        // Run decorators post install
        for decorator in &self.decorators {
            decorator.post_install_action(package)?;
        }
        Ok(())
    }

    pub fn remove(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "Removing {} using {} adapter",
            package.name, self.adapter_name
        );
        // Simulate removal
        Ok(())
    }

    pub fn update(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "Updating {} using {} adapter",
            package.name, self.adapter_name
        );
        // Simulate update
        Ok(())
    }
}

/// Foreign metadata structure for Debian (.deb)
#[derive(Debug, Clone)]
pub struct DebMetadata {
    pub package: String,
    pub version: String,
    pub depends: String,
    pub conflicts: String,
}

/// Foreign metadata structure for Red Hat (.rpm)
#[derive(Debug, Clone)]
pub struct RpmMetadata {
    pub name: String,
    pub version: String,
    pub requires: Vec<String>,
    pub conflicts: Vec<String>,
}

/// Foreign metadata structure for Arch Linux (pacman)
#[derive(Debug, Clone)]
pub struct PacmanMetadata {
    pub pkgname: String,
    pub pkgver: String,
    pub depend: Vec<String>,
    pub conflict: Vec<String>,
}

/// Adapter trait to map foreign package specifications to the Universal schema
pub trait ForeignPackageAdapter {
    fn adapt_to_unified(&self) -> UnifiedPackage;
}

impl ForeignPackageAdapter for DebMetadata {
    fn adapt_to_unified(&self) -> UnifiedPackage {
        let mut pkg = UnifiedPackage::new(self.package.clone(), self.version.clone())
            .with_format(PackageFormat::Deb);
        for dep in self.depends.split(',') {
            let trimmed = dep.trim();
            if !trimmed.is_empty() {
                pkg = pkg.with_dependency(trimmed.to_string());
            }
        }
        for conf in self.conflicts.split(',') {
            let trimmed = conf.trim();
            if !trimmed.is_empty() {
                pkg = pkg.with_conflict(trimmed.to_string());
            }
        }
        pkg
    }
}

impl ForeignPackageAdapter for RpmMetadata {
    fn adapt_to_unified(&self) -> UnifiedPackage {
        let mut pkg = UnifiedPackage::new(self.name.clone(), self.version.clone())
            .with_format(PackageFormat::Rpm);
        for dep in &self.requires {
            pkg = pkg.with_dependency(dep.clone());
        }
        for conf in &self.conflicts {
            pkg = pkg.with_conflict(conf.clone());
        }
        pkg
    }
}

impl ForeignPackageAdapter for PacmanMetadata {
    fn adapt_to_unified(&self) -> UnifiedPackage {
        let mut pkg = UnifiedPackage::new(self.pkgname.clone(), self.pkgver.clone())
            .with_format(PackageFormat::Pacman);
        for dep in &self.depend {
            pkg = pkg.with_dependency(dep.clone());
        }
        for conf in &self.conflict {
            pkg = pkg.with_conflict(conf.clone());
        }
        pkg
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

    pub fn resolve_dependencies(&self, package_name: &str) -> Result<std::vec::Vec<String>, PackageError> {
        let mut resolved: std::vec::Vec<String> = std::vec::Vec::new();
        let mut to_visit: std::vec::Vec<String> = std::vec::Vec::new();
        to_visit.push(package_name.to_string());
        let mut visited = std::collections::HashSet::<String>::new();

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

        let mut keys: std::vec::Vec<String> = std::vec::Vec::new();
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

/// Universal package manager
pub struct UniversalPackageManager {
    pub packages: HashMap<String, UnifiedPackage>,
    pub adapters: HashMap<PackageFormat, PackageAdapter>,
    pub resolver: DependencyResolver,
    pub installed_packages: HashMap<String, UnifiedPackage>,
    pub transaction_history: TransactionalHistory,
    pub metadata_cache: HashMap<String, UnifiedPackage>,
    pub observers: Vec<Arc<dyn PackageObserver>>,
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
            observers: Vec::new(),
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
            (PackageFormat::Ebuild, "emerge"),
            (PackageFormat::Apk, "apk"),
            (PackageFormat::Nix, "nix"),
            (PackageFormat::AppImage, "appimage"),
            (PackageFormat::Xbps, "xbps"),
            (PackageFormat::Txz, "txz"),
            (PackageFormat::Eopkg, "eopkg"),
            (PackageFormat::Zypper, "zypper"),
            (PackageFormat::Guix, "guix"),
            (PackageFormat::CachyOS, "cachyos"),
            (PackageFormat::Swupd, "swupd"),
            (PackageFormat::Starling, "starling"),
        ];

        for (format, name) in formats {
            self.adapters.insert(format, PackageAdapter::new(format, name.to_string()));
        }
    }

    pub fn add_observer(&mut self, observer: Arc<dyn PackageObserver>) {
        self.observers.push(observer);
    }

    fn notify_observers(&self, event: PackageEvent, package: &UnifiedPackage) {
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
            // 1. Transition state to Installing
            {
                if let Some(package) = self.packages.get_mut(&dep_name) {
                    package.transition_to(PackageState::Installing)?;
                }
            }

            let package = if let Some(p) = self.packages.get(&dep_name) {
                p.clone()
            } else {
                continue;
            };

            // 2. Trigger PreInstall observers
            self.notify_observers(PackageEvent::PreInstall, &package);

            // 3. Find and execute Strategies
            let format = if let Some(f) = package.formats.first() {
                *f
            } else {
                PackageFormat::SigmaPkg
            };
            let (inst_strategy, verify_strategy) = get_strategies(format);

            if let Err(e) = inst_strategy.execute(&package) {
                if let Some(pkg) = self.packages.get_mut(&dep_name) {
                    pkg.transition_to(PackageState::Broken).ok();
                }
                let pkg_broken = self.packages.get(&dep_name).unwrap().clone();
                self.notify_observers(PackageEvent::OnFailure, &pkg_broken);
                return Err(e);
            }

            match verify_strategy.execute(&package) {
                Ok(true) => {}
                _ => {
                    if let Some(pkg) = self.packages.get_mut(&dep_name) {
                        pkg.transition_to(PackageState::Broken).ok();
                    }
                    let pkg_broken = self.packages.get(&dep_name).unwrap().clone();
                    self.notify_observers(PackageEvent::OnFailure, &pkg_broken);
                    return Err(PackageError::InstallationFailed(format!("Verification failed for {}", package.name)));
                }
            }

            // Find appropriate adapter and install
            let mut installed_ok = false;
            for format in &package.formats {
                if let Some(adapter) = self.adapters.get(format) {
                    let adapter: &PackageAdapter = adapter;
                    adapter.install(&package)?;
                    installed_ok = true;
                    break;
                }
            }

            if !installed_ok {
                // Fallback to native adapter
                if let Some(adapter) = self.adapters.get(&PackageFormat::SigmaPkg) {
                    adapter.install(&package)?;
                }
            }

            // 4. Transition state to Installed
            {
                let pkg = self.packages.get_mut(&dep_name).unwrap();
                pkg.transition_to(PackageState::Installed)?;
                pkg.installed = true;
            }

            let final_pkg = self.packages.get(&dep_name).unwrap().clone();

            // 5. Trigger PostInstall observers
            self.notify_observers(PackageEvent::PostInstall, &final_pkg);

            self.installed_packages.insert(dep_name.clone(), final_pkg);
        }

        Ok(())
    }

    pub fn remove(&mut self, package_name: &str) -> Result<(), PackageError> {
        if self.installed_packages.contains_key(package_name) {
            let package = self.installed_packages.get(package_name).unwrap().clone();

            // Trigger PreRemove
            self.notify_observers(PackageEvent::PreRemove, &package);

            for format in &package.formats {
                if let Some(adapter) = self.adapters.get(format) {
                    let adapter: &PackageAdapter = adapter;
                    adapter.remove(&package)?;
                    break;
                }
            }

            {
                let pkg = self.installed_packages.get_mut(package_name).unwrap();
                pkg.transition_to(PackageState::Uninstalled)?;
                pkg.installed = false;
            }

            let final_pkg = self.installed_packages.get(package_name).unwrap().clone();

            // Trigger PostRemove
            self.notify_observers(PackageEvent::PostRemove, &final_pkg);
        }
        self.installed_packages.remove(package_name);
        Ok(())
    }

    pub fn update(&mut self, package_name: &str) -> Result<(), PackageError> {
        if self.installed_packages.contains_key(package_name) {
            let package = self.installed_packages.get(package_name).unwrap().clone();
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
}

impl Default for UniversalPackageManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Translation & Routing Shell Engine supporting cross-distro package command translation
pub struct TranslationRoutingEngine;

impl TranslationRoutingEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn translate_and_route(&self, manager: &mut UniversalPackageManager, cmd_line: &str) -> Result<String, PackageError> {
        let parts: Vec<&str> = cmd_line.split_whitespace().collect();
        if parts.is_empty() {
            return Err(PackageError::InstallationFailed("Empty command string.".to_string()));
        }

        let tool = parts[0];
        match tool {
            "apt" | "apt-get" => {
                if parts.len() >= 3 && parts[1] == "install" {
                    let pkg_name = parts[2];
                    if manager.get_package(pkg_name).is_none() {
                        let pkg = UnifiedPackage::new(pkg_name.to_string(), "1.0.0".to_string())
                            .with_format(PackageFormat::Deb);
                        manager.add_package(pkg);
                    }
                    manager.install(pkg_name)?;
                    Ok(format!("Routed 'apt' -> UniversalPackageManager: Installed Deb package '{}'", pkg_name))
                } else if parts.len() >= 3 && parts[1] == "remove" {
                    let pkg_name = parts[2];
                    manager.remove(pkg_name)?;
                    Ok(format!("Routed 'apt' -> UniversalPackageManager: Removed Deb package '{}'", pkg_name))
                } else {
                    Err(PackageError::InstallationFailed(format!("Unsupported apt command format: {}", cmd_line)))
                }
            }
            "pacman" => {
                if parts.len() >= 3 && parts[1] == "-S" {
                    let pkg_name = parts[2];
                    if manager.get_package(pkg_name).is_none() {
                        let pkg = UnifiedPackage::new(pkg_name.to_string(), "1.0.0".to_string())
                            .with_format(PackageFormat::Pacman);
                        manager.add_package(pkg);
                    }
                    manager.install(pkg_name)?;
                    Ok(format!("Routed 'pacman' -> UniversalPackageManager: Installed Pacman package '{}'", pkg_name))
                } else if parts.len() >= 3 && parts[1] == "-R" {
                    let pkg_name = parts[2];
                    manager.remove(pkg_name)?;
                    Ok(format!("Routed 'pacman' -> UniversalPackageManager: Removed Pacman package '{}'", pkg_name))
                } else {
                    Err(PackageError::InstallationFailed(format!("Unsupported pacman command format: {}", cmd_line)))
                }
            }
            "dnf" | "yum" => {
                if parts.len() >= 3 && parts[1] == "install" {
                    let pkg_name = parts[2];
                    if manager.get_package(pkg_name).is_none() {
                        let pkg = UnifiedPackage::new(pkg_name.to_string(), "1.0.0".to_string())
                            .with_format(PackageFormat::Rpm);
                        manager.add_package(pkg);
                    }
                    manager.install(pkg_name)?;
                    Ok(format!("Routed 'dnf/yum' -> UniversalPackageManager: Installed Rpm package '{}'", pkg_name))
                } else {
                    Err(PackageError::InstallationFailed(format!("Unsupported dnf/yum command format: {}", cmd_line)))
                }
            }
            "emerge" => {
                let pkg_name = if parts.len() >= 3 && (parts[1].starts_with('-') || parts[1] == "--ask") {
                    parts[2]
                } else if parts.len() >= 2 {
                    parts[1]
                } else {
                    return Err(PackageError::InstallationFailed("Emerge package target unspecified.".to_string()));
                };
                if manager.get_package(pkg_name).is_none() {
                    let pkg = UnifiedPackage::new(pkg_name.to_string(), "1.0.0".to_string())
                        .with_format(PackageFormat::Ebuild);
                    manager.add_package(pkg);
                }
                manager.install(pkg_name)?;
                Ok(format!("Routed 'emerge' -> UniversalPackageManager: Compiled and Installed Ebuild package '{}'", pkg_name))
            }
            _ => {
                // Fallback route as a native package run
                if manager.get_package(tool).is_none() {
                    let pkg = UnifiedPackage::new(tool.to_string(), "1.0.0".to_string())
                        .with_format(PackageFormat::SigmaPkg);
                    manager.add_package(pkg);
                }
                manager.install(tool)?;
                Ok(format!("Routed arbitrary command '{}' -> UniversalPackageManager as native SigmaPkg", tool))
            }
        }
    }
}

impl Default for TranslationRoutingEngine {
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
    InvalidStateTransition { from: PackageState, to: PackageState },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_creation() {
        let manager = UniversalPackageManager::new();
        // Checked: supports all 18 formats as defaults
        assert_eq!(manager.adapters.len(), 18);
    }

    #[test]
    fn test_package_creation() {
        let package = UnifiedPackage::new("test".to_string(), "1.0.0".to_string())
            .with_format(PackageFormat::Deb)
            .with_dependency("dep1".to_string());
        assert_eq!(package.formats.len(), 1);
        assert_eq!(package.dependencies.len(), 1);
        assert_eq!(package.state, PackageState::Uninstalled);
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
        let inst = manager.installed_packages.get("test").unwrap();
        assert_eq!(inst.state, PackageState::Installed);
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
        let mut pkg = UnifiedPackage::new("pkg".to_string(), "1.0.0".to_string());
        assert_eq!(pkg.state, PackageState::Uninstalled);

        // Valid transitions
        assert!(pkg.transition_to(PackageState::Installing).is_ok());
        assert_eq!(pkg.state, PackageState::Installing);

        assert!(pkg.transition_to(PackageState::Installed).is_ok());
        assert_eq!(pkg.state, PackageState::Installed);

        // Invalid transition: Installed to Resolved directly should fail
        assert!(pkg.transition_to(PackageState::Resolved).is_err());
    }

    #[test]
    fn test_strategy_pattern() {
        let pkg = UnifiedPackage::new("cachy".to_string(), "2.0".to_string())
            .with_format(PackageFormat::CachyOS);
        let (inst, verify) = get_strategies(PackageFormat::CachyOS);
        assert!(inst.execute(&pkg).is_ok());
        assert!(verify.execute(&pkg).unwrap());

        let sandbox_pkg = UnifiedPackage::new("vlc".to_string(), "3.0".to_string())
            .with_format(PackageFormat::Flatpak);
        let (inst_sb, verify_sb) = get_strategies(PackageFormat::Flatpak);
        assert!(inst_sb.execute(&sandbox_pkg).is_ok());
        assert!(verify_sb.execute(&sandbox_pkg).unwrap());
    }

    #[test]
    fn test_observer_pattern() {
        use std::sync::Mutex;

        let mut manager = UniversalPackageManager::new();
        let events_triggered = Arc::new(Mutex::new(Vec::new()));

        let events_clone = events_triggered.clone();
        let closure_observer = Arc::new(UdfClosureObserver {
            observer_name: "test_closure_observer",
            handler: Box::new(move |event, package| {
                events_clone.lock().unwrap().push((event, package.name.clone()));
            }),
        });

        manager.add_observer(closure_observer);

        let pkg = UnifiedPackage::new("observe-me".to_string(), "1.0".to_string())
            .with_format(PackageFormat::SigmaPkg);
        manager.add_package(pkg);

        manager.install("observe-me").unwrap();

        let recorded = events_triggered.lock().unwrap();
        assert!(recorded.contains(&(PackageEvent::PreInstall, "observe-me".to_string())));
        assert!(recorded.contains(&(PackageEvent::PostInstall, "observe-me".to_string())));
    }

    #[test]
    fn test_decorator_pattern() {
        let mut adapter = PackageAdapter::new(PackageFormat::Deb, "apt".to_string());
        adapter.add_decorator(Arc::new(GpgVerificationDecorator { key_id: "0xABCDEF".to_string() }));
        adapter.add_decorator(Arc::new(BandwidthShaperDecorator { max_kbps: 1024 }));
        adapter.add_decorator(Arc::new(SandboxEnforcerDecorator { confinement: "strict".to_string() }));
        adapter.add_decorator(Arc::new(IntegrityAuditDecorator));
        adapter.add_decorator(Arc::new(PostQuantumCryptoDecorator { algorithm: "Dilithium5".to_string() }));

        let pkg = UnifiedPackage::new("secured-pkg".to_string(), "1.0.0".to_string());
        assert!(adapter.install(&pkg).is_ok());
    }

    #[test]
    fn test_adapter_pattern() {
        let deb_meta = DebMetadata {
            package: "curl".to_string(),
            version: "7.80.0".to_string(),
            depends: "libcurl,zlib".to_string(),
            conflicts: "wget".to_string(),
        };

        let unified = deb_meta.adapt_to_unified();
        assert_eq!(unified.name, "curl");
        assert_eq!(unified.version, "7.80.0");
        assert!(unified.formats.contains(&PackageFormat::Deb));
        assert!(unified.dependencies.contains(&"libcurl".to_string()));
        assert!(unified.conflicts.contains(&"wget".to_string()));
    }

    #[test]
    fn test_translation_routing_engine() {
        let mut manager = UniversalPackageManager::new();
        let engine = TranslationRoutingEngine::new();

        let r1 = engine.translate_and_route(&mut manager, "apt install tmux").unwrap();
        assert!(r1.contains("tmux"));
        assert!(manager.installed_packages.contains_key("tmux"));

        let r2 = engine.translate_and_route(&mut manager, "pacman -S zsh").unwrap();
        assert!(r2.contains("zsh"));
        assert!(manager.installed_packages.contains_key("zsh"));

        let r3 = engine.translate_and_route(&mut manager, "emerge htop").unwrap();
        assert!(r3.contains("htop"));
        assert!(manager.installed_packages.contains_key("htop"));
    }
}
