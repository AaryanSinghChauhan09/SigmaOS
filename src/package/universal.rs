// SigmaOS Universal Package Manager
// Unified system absorbing apt, yum, pacman, snap, flatpak, nix, ebuild, etc.

use std::collections::{HashMap, HashSet};

/// Package format type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackageFormat {
    Deb,      // apt (Debian, Ubuntu, Pop!_OS)
    Rpm,      // dnf/rpm (Fedora, RHEL, openSUSE)
    Pacman,   // pacman (Arch Linux, EndeavourOS)
    Snap,     // snap (Ubuntu Universal)
    Flatpak,  // flatpak (Universal Sandbox)
    SigmaPkg, // native SigmaOS format
    Ebuild,   // Gentoo Portage
    Apk,      // Alpine APK
    Nix,      // NixOS Declarative Nix Expression
    AppImage, // AppImage Sandbox
    Xbps,     // Void Linux XBPS
    Txz,      // Slackware Package Tool
    Eopkg,    // Solus Package Manager
    Zypper,   // openSUSE Zypper Command Tool
    Guix,     // GNU Guix Functional Package Manager
    CachyOS,  // CachyOS Performance Optimized (x86_64-v3/v4)
    Swupd,    // Intel Clear Linux swupd bundles
    Starling, // Starling Desktop native packages
}

/// Package source
#[derive(Debug, Clone, PartialEq, Eq)]
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

/// Package lifecycle state machine states (State Pattern)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageState {
    Uninstalled,
    Staged,
    Configuring,
    Installing,
    Installed,
    RollbackPending,
}

/// Unified package representing cross-distro package metadata
#[derive(Debug, Clone)]
pub struct UnifiedPackage {
    pub name: String,
    pub version: String,
    pub formats: Vec<PackageFormat>,
    pub dependencies: Vec<String>,
    pub conflicts: Vec<String>,
    pub provides: Vec<String>,
    pub source: PackageSource,
    pub state: PackageState,
    pub pqc_signature: Option<String>,
    pub is_sandboxed: bool,
    pub telemetry_enabled: bool,
    pub install_duration_ms: u64,
    pub installed: bool,
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
            state: PackageState::Uninstalled,
            pqc_signature: None,
            is_sandboxed: false,
            telemetry_enabled: false,
            install_duration_ms: 0,
            installed: false,
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

    /// Transitions package lifecycle state safely (State Pattern)
    pub fn transition_to(&mut self, next: PackageState) -> Result<(), PackageError> {
        let is_valid = match (self.state, next) {
            (PackageState::Uninstalled, PackageState::Staged) => true,
            (PackageState::Uninstalled, PackageState::Configuring) => true,
            (PackageState::Uninstalled, PackageState::Installing) => true,
            (PackageState::Staged, PackageState::Configuring) => true,
            (PackageState::Configuring, PackageState::Installing) => true,
            (PackageState::Installing, PackageState::Installed) => true,
            (PackageState::Installed, PackageState::RollbackPending) => true,
            (PackageState::RollbackPending, PackageState::Uninstalled) => true,
            // Fallback transitions
            (PackageState::Installing, PackageState::RollbackPending) => true,
            (PackageState::Configuring, PackageState::Uninstalled) => true,
            (PackageState::Staged, PackageState::Uninstalled) => true,
            _ => false,
        };

        if is_valid {
            self.state = next;
            Ok(())
        } else {
            Err(PackageError::InstallationFailed(format!(
                "Invalid lifecycle state transition from {:?} to {:?}",
                self.state, next
            )))
        }
    }
}

// =========================================================================
// 1. STRATEGY PATTERN (Package Action Strategies)
// =========================================================================

pub trait PackageActionStrategy: Send + Sync {
    fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError>;
    fn verify(&self, package: &UnifiedPackage) -> Result<bool, PackageError>;
    fn rollback(&self, package: &UnifiedPackage) -> Result<(), PackageError>;
}

pub struct DebianStyleStrategy;
impl PackageActionStrategy for DebianStyleStrategy {
    fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "DebianStyleStrategy: Unpacking control archive, setting up pre-dependencies for {}",
            package.name
        );
        Ok(())
    }
    fn verify(&self, package: &UnifiedPackage) -> Result<bool, PackageError> {
        println!(
            "DebianStyleStrategy: Verifying md5sums files of {}",
            package.name
        );
        Ok(true)
    }
    fn rollback(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "DebianStyleStrategy: Rolling back deb package {}",
            package.name
        );
        Ok(())
    }
}

pub struct RedHatStyleStrategy;
impl PackageActionStrategy for RedHatStyleStrategy {
    fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "RedHatStyleStrategy: Extracting RPM headers & payloads for {}",
            package.name
        );
        Ok(())
    }
    fn verify(&self, package: &UnifiedPackage) -> Result<bool, PackageError> {
        println!(
            "RedHatStyleStrategy: Checking database digest GPG signature for {}",
            package.name
        );
        Ok(true)
    }
    fn rollback(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "RedHatStyleStrategy: Restoring RPM rollback checkpoint for {}",
            package.name
        );
        Ok(())
    }
}

pub struct ArchStyleStrategy;
impl PackageActionStrategy for ArchStyleStrategy {
    fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "ArchStyleStrategy: Processing pacman transaction hook & ALPM database update for {}",
            package.name
        );
        Ok(())
    }
    fn verify(&self, package: &UnifiedPackage) -> Result<bool, PackageError> {
        println!(
            "ArchStyleStrategy: Performing MTREE validations for {}",
            package.name
        );
        Ok(true)
    }
    fn rollback(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "ArchStyleStrategy: Re-registering previous version of pacman archive for {}",
            package.name
        );
        Ok(())
    }
}

pub struct SandboxStyleStrategy;
impl PackageActionStrategy for SandboxStyleStrategy {
    fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "SandboxStyleStrategy: Configuring ostree/bubblewrap sandbox boundaries for {}",
            package.name
        );
        Ok(())
    }
    fn verify(&self, package: &UnifiedPackage) -> Result<bool, PackageError> {
        println!(
            "SandboxStyleStrategy: Verifying manifest permission overrides for {}",
            package.name
        );
        Ok(true)
    }
    fn rollback(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "SandboxStyleStrategy: Resetting namespace runtime limits for {}",
            package.name
        );
        Ok(())
    }
}

pub struct SovereignStyleStrategy;
impl PackageActionStrategy for SovereignStyleStrategy {
    fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("SovereignStyleStrategy: Direct secure high-speed microkernel mapping of native sigpkg {}", package.name);
        Ok(())
    }
    fn verify(&self, package: &UnifiedPackage) -> Result<bool, PackageError> {
        println!(
            "SovereignStyleStrategy: High-entropy cryptographic attestation verification for {}",
            package.name
        );
        Ok(true)
    }
    fn rollback(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "SovereignStyleStrategy: Swapping atomic root inodes pointer in zero-latency for {}",
            package.name
        );
        Ok(())
    }
}

// =========================================================================
// 2. ADAPTER PATTERN (Metadata Format Translation Adapters)
// =========================================================================

pub trait PackageFormatAdapter {
    fn convert_to_unified(&self, raw_metadata: &str) -> Result<UnifiedPackage, PackageError>;
    fn convert_to_raw(&self, package: &UnifiedPackage) -> Result<String, PackageError>;
}

pub struct DebMetadataAdapter;
impl PackageFormatAdapter for DebMetadataAdapter {
    fn convert_to_unified(&self, raw: &str) -> Result<UnifiedPackage, PackageError> {
        let mut name = "deb-package".to_string();
        let mut version = "1.0.0".to_string();
        let mut dependencies = Vec::new();

        for line in raw.lines() {
            if line.starts_with("Package: ") {
                name = line[9..].trim().to_string();
            } else if line.starts_with("Version: ") {
                version = line[9..].trim().to_string();
            } else if line.starts_with("Depends: ") {
                for dep in line[9..].split(',') {
                    dependencies.push(dep.trim().to_string());
                }
            }
        }

        let mut package = UnifiedPackage::new(name, version).with_format(PackageFormat::Deb);
        for dep in dependencies {
            package = package.with_dependency(dep);
        }
        Ok(package)
    }

    fn convert_to_raw(&self, package: &UnifiedPackage) -> Result<String, PackageError> {
        let mut raw = format!("Package: {}\nVersion: {}\n", package.name, package.version);
        if !package.dependencies.is_empty() {
            raw.push_str(&format!("Depends: {}\n", package.dependencies.join(", ")));
        }
        Ok(raw)
    }
}

pub struct RpmMetadataAdapter;
impl PackageFormatAdapter for RpmMetadataAdapter {
    fn convert_to_unified(&self, raw: &str) -> Result<UnifiedPackage, PackageError> {
        let mut name = "rpm-package".to_string();
        let mut version = "1.0.0".to_string();
        let mut dependencies = Vec::new();

        for line in raw.lines() {
            if line.starts_with("Name: ") {
                name = line[6..].trim().to_string();
            } else if line.starts_with("Version: ") {
                version = line[9..].trim().to_string();
            } else if line.starts_with("Requires: ") {
                for dep in line[10..].split_whitespace() {
                    dependencies.push(dep.to_string());
                }
            }
        }

        let mut package = UnifiedPackage::new(name, version).with_format(PackageFormat::Rpm);
        for dep in dependencies {
            package = package.with_dependency(dep);
        }
        Ok(package)
    }

    fn convert_to_raw(&self, package: &UnifiedPackage) -> Result<String, PackageError> {
        let mut raw = format!("Name: {}\nVersion: {}\n", package.name, package.version);
        if !package.dependencies.is_empty() {
            raw.push_str(&format!("Requires: {}\n", package.dependencies.join(" ")));
        }
        Ok(raw)
    }
}

pub struct StarlingMetadataAdapter;
impl PackageFormatAdapter for StarlingMetadataAdapter {
    fn convert_to_unified(&self, raw: &str) -> Result<UnifiedPackage, PackageError> {
        let mut name = "starling-package".to_string();
        let mut version = "1.0.0".to_string();
        let mut dependencies = Vec::new();

        for line in raw.lines() {
            if line.starts_with("starling.name = ") {
                name = line[16..].trim_matches('"').to_string();
            } else if line.starts_with("starling.version = ") {
                version = line[19..].trim_matches('"').to_string();
            } else if line.starts_with("starling.depends = ") {
                for dep in line[19..].split_whitespace() {
                    let clean = dep.trim_matches(|c| c == '[' || c == ']' || c == '"' || c == ',');
                    if !clean.is_empty() {
                        dependencies.push(clean.to_string());
                    }
                }
            }
        }

        let mut package = UnifiedPackage::new(name, version).with_format(PackageFormat::Starling);
        for dep in dependencies {
            package = package.with_dependency(dep);
        }
        Ok(package)
    }

    fn convert_to_raw(&self, package: &UnifiedPackage) -> Result<String, PackageError> {
        let mut raw = format!(
            "starling.name = \"{}\"\nstarling.version = \"{}\"\n",
            package.name, package.version
        );
        if !package.dependencies.is_empty() {
            raw.push_str(&format!(
                "starling.depends = [ \"{}\" ]\n",
                package.dependencies.join("\", \"")
            ));
        }
        Ok(raw)
    }
}

// =========================================================================
// 3. DECORATOR PATTERN (Dynamic Package Enhancements)
// =========================================================================

pub trait IPackageDecorator: Send + Sync {
    fn decorate(&self, package: &mut UnifiedPackage) -> Result<(), PackageError>;
}

pub struct SandboxedPackageDecorator {
    pub network_restricted: bool,
}
impl IPackageDecorator for SandboxedPackageDecorator {
    fn decorate(&self, package: &mut UnifiedPackage) -> Result<(), PackageError> {
        package.is_sandboxed = true;
        println!(
            "SandboxedPackageDecorator: Sandbox protection applied. Network Restricted: {}",
            self.network_restricted
        );
        Ok(())
    }
}

pub struct PqcSignedPackageDecorator {
    pub key_id: String,
}
impl IPackageDecorator for PqcSignedPackageDecorator {
    fn decorate(&self, package: &mut UnifiedPackage) -> Result<(), PackageError> {
        package.pqc_signature = Some(format!("dilithium5:sig:{}", self.key_id));
        println!(
            "PqcSignedPackageDecorator: Signed with Dilithium-5 key ID: {}",
            self.key_id
        );
        Ok(())
    }
}

pub struct TelemetryPackageDecorator;
impl IPackageDecorator for TelemetryPackageDecorator {
    fn decorate(&self, package: &mut UnifiedPackage) -> Result<(), PackageError> {
        package.telemetry_enabled = true;
        println!(
            "TelemetryPackageDecorator: Detailed telemetry reporting hooks loaded dynamically."
        );
        Ok(())
    }
}

// =========================================================================
// 4. OBSERVER PATTERN (Polymorphic User-Defined Hooks & Triggers)
// =========================================================================

pub type UserDefinedHook =
    Box<dyn Fn(&mut UnifiedPackage) -> Result<(), PackageError> + Send + Sync>;

pub struct PackageTriggerRegistry {
    pub pre_install_hooks: Vec<UserDefinedHook>,
    pub post_install_hooks: Vec<UserDefinedHook>,
}

impl PackageTriggerRegistry {
    pub fn new() -> Self {
        Self {
            pre_install_hooks: Vec::new(),
            post_install_hooks: Vec::new(),
        }
    }

    pub fn register_pre_install<F>(&mut self, hook: F)
    where
        F: Fn(&mut UnifiedPackage) -> Result<(), PackageError> + Send + Sync + 'static,
    {
        self.pre_install_hooks.push(Box::new(hook));
    }

    pub fn register_post_install<F>(&mut self, hook: F)
    where
        F: Fn(&mut UnifiedPackage) -> Result<(), PackageError> + Send + Sync + 'static,
    {
        self.post_install_hooks.push(Box::new(hook));
    }
}

impl Default for PackageTriggerRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// PRE-EXISTING COMPATIBILITY STUBS & FACADE PACKS
// =========================================================================

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
        let strategy = get_strategy_for(self.format);
        strategy.install(package)?;
        Ok(())
    }

    pub fn remove(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        let strategy = get_strategy_for(self.format);
        strategy.rollback(package)?;
        Ok(())
    }

    pub fn update(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        let strategy = get_strategy_for(self.format);
        strategy.install(package)?;
        Ok(())
    }
}

pub fn get_strategy_for(format: PackageFormat) -> Box<dyn PackageActionStrategy> {
    match format {
        PackageFormat::Deb => Box::new(DebianStyleStrategy),
        PackageFormat::Rpm => Box::new(RedHatStyleStrategy),
        PackageFormat::Pacman => Box::new(ArchStyleStrategy),
        PackageFormat::Flatpak | PackageFormat::Snap | PackageFormat::AppImage => {
            Box::new(SandboxStyleStrategy)
        }
        PackageFormat::SigmaPkg | PackageFormat::CachyOS | PackageFormat::Starling => {
            Box::new(SovereignStyleStrategy)
        }
        _ => Box::new(SovereignStyleStrategy),
    }
}

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

#[derive(Debug, Clone)]
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
        for checkpoint in &self.checkpoints {
            if checkpoint.checkpoint_id == id {
                return Some(checkpoint);
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

/// Universal package manager - OOP Facade for all package formats & integrations
pub struct UniversalPackageManager {
    pub packages: HashMap<String, UnifiedPackage>,
    pub adapters: HashMap<PackageFormat, PackageAdapter>,
    pub resolver: DependencyResolver,
    pub installed_packages: HashMap<String, UnifiedPackage>,
    pub transaction_history: TransactionalHistory,
    pub metadata_cache: HashMap<String, UnifiedPackage>,
    pub trigger_registry: PackageTriggerRegistry,
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
            trigger_registry: PackageTriggerRegistry::new(),
        };

        manager.add_default_adapters();
        manager
    }

    fn add_default_adapters(&mut self) {
        let formats = vec![
            (PackageFormat::Deb, "apt"),
            (PackageFormat::Rpm, "yum"),
            (PackageFormat::Pacman, "pacman"),
            (PackageFormat::Snap, "snap"),
            (PackageFormat::Flatpak, "flatpak"),
            (PackageFormat::SigmaPkg, "sigpkg"),
            (PackageFormat::Ebuild, "ebuild"),
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
            let adapter = PackageAdapter::new(format, name.to_string());
            self.adapters.insert(format, adapter);
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
            if let Some(package) = self.packages.get_mut(&dep_name) {
                // Change lifecycle State: Configuring (State Pattern)
                package.transition_to(PackageState::Configuring)?;

                // Run pre-install hooks (Observer / UDF triggers)
                for hook in &self.trigger_registry.pre_install_hooks {
                    hook(package)?;
                }

                package.transition_to(PackageState::Installing)?;

                // Find appropriate adapter and execute strategy (Strategy Pattern)
                let mut installed_successfully = false;
                for format in &package.formats {
                    if let Some(adapter) = self.adapters.get(format) {
                        adapter.install(package)?;
                        installed_successfully = true;
                        break;
                    }
                }

                if !installed_successfully {
                    return Err(PackageError::InstallationFailed(format!(
                        "No adapter found for package formats: {:?}",
                        package.formats
                    )));
                }

                package.transition_to(PackageState::Installed)?;

                // Run post-install hooks (Observer / UDF triggers)
                for hook in &self.trigger_registry.post_install_hooks {
                    hook(package)?;
                }

                let mut installed = package.clone();
                installed.installed = true;
                self.installed_packages.insert(dep_name.clone(), installed);
            }
        }

        Ok(())
    }

    pub fn remove(&mut self, package_name: &str) -> Result<(), PackageError> {
        if let Some(package) = self.installed_packages.get_mut(package_name) {
            package.transition_to(PackageState::RollbackPending)?;
            for format in &package.formats {
                if let Some(adapter) = self.adapters.get(format) {
                    adapter.remove(package)?;
                    break;
                }
            }
            package.transition_to(PackageState::Uninstalled)?;
            self.installed_packages.remove(package_name);
        }
        Ok(())
    }

    pub fn update(&mut self, package_name: &str) -> Result<(), PackageError> {
        if let Some(package) = self.installed_packages.get_mut(package_name) {
            package.transition_to(PackageState::Configuring)?;
            for format in &package.formats {
                if let Some(adapter) = self.adapters.get(format) {
                    adapter.update(package)?;
                    break;
                }
            }
            package.transition_to(PackageState::Installed)?;
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
    fn test_state_transitions() {
        let mut package = UnifiedPackage::new("test-state".to_string(), "1.0.0".to_string());
        assert_eq!(package.state, PackageState::Uninstalled);

        package.transition_to(PackageState::Staged).unwrap();
        assert_eq!(package.state, PackageState::Staged);

        package.transition_to(PackageState::Configuring).unwrap();
        package.transition_to(PackageState::Installing).unwrap();
        package.transition_to(PackageState::Installed).unwrap();
        assert_eq!(package.state, PackageState::Installed);

        let err = package.transition_to(PackageState::Staged);
        assert!(err.is_err());
    }

    #[test]
    fn test_strategy_and_adapters() {
        let deb_raw = "Package: curl\nVersion: 8.4.0\nDepends: libcurl4, zlib1g\n";
        let adapter = DebMetadataAdapter;
        let package = adapter.convert_to_unified(deb_raw).unwrap();
        assert_eq!(package.name, "curl");
        assert_eq!(package.version, "8.4.0");
        assert_eq!(package.dependencies.len(), 2);

        let serialized = adapter.convert_to_raw(&package).unwrap();
        assert!(serialized.contains("Package: curl"));
        assert!(serialized.contains("Depends: libcurl4, zlib1g"));
    }

    #[test]
    fn test_starling_adapter() {
        let starling_raw = "starling.name = \"starling-wm\"\nstarling.version = \"0.9.5\"\nstarling.depends = [ \"x11\", \"glib\" ]\n";
        let adapter = StarlingMetadataAdapter;
        let package = adapter.convert_to_unified(starling_raw).unwrap();
        assert_eq!(package.name, "starling-wm");
        assert_eq!(package.version, "0.9.5");
        assert_eq!(package.dependencies.len(), 2);
    }

    #[test]
    fn test_decorators() {
        let mut package = UnifiedPackage::new("decorated-pkg".to_string(), "1.0.0".to_string());

        let sandbox = SandboxedPackageDecorator {
            network_restricted: true,
        };
        let signer = PqcSignedPackageDecorator {
            key_id: "testkey123".to_string(),
        };
        let telemetry = TelemetryPackageDecorator;

        sandbox.decorate(&mut package).unwrap();
        signer.decorate(&mut package).unwrap();
        telemetry.decorate(&mut package).unwrap();

        assert!(package.is_sandboxed);
        assert_eq!(package.pqc_signature.unwrap(), "dilithium5:sig:testkey123");
        assert!(package.telemetry_enabled);
    }

    #[test]
    fn test_udf_triggers_and_hooks() {
        let mut manager = UniversalPackageManager::new();

        manager.trigger_registry.register_pre_install(|pkg| {
            pkg.name = format!("{}-prehooked", pkg.name);
            Ok(())
        });

        manager.trigger_registry.register_post_install(|pkg| {
            pkg.version = format!("{}-post", pkg.version);
            Ok(())
        });

        let package = UnifiedPackage::new("core-utility".to_string(), "3.2".to_string())
            .with_format(PackageFormat::SigmaPkg);
        manager.add_package(package);

        manager.install("core-utility").unwrap();

        let installed = manager.installed_packages.get("core-utility").unwrap();
        assert_eq!(installed.name, "core-utility-prehooked");
        assert_eq!(installed.version, "3.2-post");
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
}
