#![allow(unused_variables)]
// SigmaOS Universal Package Manager
// Unified system absorbing apt, yum, pacman, snap, flatpak

use std::collections::HashMap;

/// Semantic Version (SemVer representation)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemVer {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl SemVer {
    pub fn parse(s: &str) -> Option<Self> {
        let mut parts = s.split('.');
        let major = parts.next()?.parse::<u32>().ok()?;
        let minor = parts.next()?.parse::<u32>().ok()?;
        let patch = parts.next()?.parse::<u32>().ok()?;
        if parts.next().is_some() {
            return None;
        }
        Some(Self { major, minor, patch })
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
        } else {
            if let Some(v) = SemVer::parse(s) {
                return SemVerConstraint::Exact(v);
            }
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

/// Package format type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackageFormat {
    Deb,      // apt
    Rpm,      // yum
    Pacman,   // pacman
    Snap,     // snap
    Flatpak,  // flatpak
    SigmaPkg, // native SigmaOS format
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
    pub checksum: String,
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
            checksum: String::new(),
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

    pub fn with_checksum(mut self, checksum: String) -> Self {
        self.checksum = checksum;
        self
    }

    pub fn has_conflict_with(&self, other: &UnifiedPackage) -> bool {
        self.conflicts.iter().any(|c| c == &other.name)
            || other.conflicts.iter().any(|c| c == &self.name)
    }

    pub fn verify_integrity(&self) -> bool {
        if self.checksum.is_empty() {
            true
        } else {
            // Simulated validation of cryptographic checksum
            self.checksum.len() >= 8
        }
    }
}

/// Polymorphic Package Format Adapter (OOP & Modularity design)
pub trait PackageFormatAdapter {
    fn format(&self) -> PackageFormat;
    fn adapter_name(&self) -> &str;
    fn can_handle(&self, package: &UnifiedPackage) -> bool {
        package.formats.contains(&self.format())
    }
    fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError>;
    fn remove(&self, package: &UnifiedPackage) -> Result<(), PackageError>;
    fn update(&self, package: &UnifiedPackage) -> Result<(), PackageError>;
}

// ----------------------------------------------------
// Concrete Implementations of Distro Adapters
// ----------------------------------------------------

/// AptDebAdapter handles Debian/Ubuntu package formats (`.deb`)
pub struct AptDebAdapter {
    pub cache_dir: String,
    pub gpg_check_enabled: bool,
}

impl AptDebAdapter {
    pub fn new() -> Self {
        Self {
            cache_dir: "/var/cache/apt/archives".to_string(),
            gpg_check_enabled: true,
        }
    }
}

impl PackageFormatAdapter for AptDebAdapter {
    fn format(&self) -> PackageFormat {
        PackageFormat::Deb
    }

    fn adapter_name(&self) -> &str {
        "apt"
    }

    fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "[{}] GPG validation status: {}. Installing DEB package {} to {}",
            self.adapter_name(),
            self.gpg_check_enabled,
            package.name,
            self.cache_dir
        );
        Ok(())
    }

    fn remove(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("[{}] Purging DEB package {}", self.adapter_name(), package.name);
        Ok(())
    }

    fn update(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("[{}] Refreshing and updating DEB package {}", self.adapter_name(), package.name);
        Ok(())
    }
}

/// YumRpmAdapter handles RedHat/Fedora package formats (`.rpm`)
pub struct YumRpmAdapter {
    pub repo_metadata_path: String,
}

impl YumRpmAdapter {
    pub fn new() -> Self {
        Self {
            repo_metadata_path: "/var/lib/yum/repos".to_string(),
        }
    }
}

impl PackageFormatAdapter for YumRpmAdapter {
    fn format(&self) -> PackageFormat {
        PackageFormat::Rpm
    }

    fn adapter_name(&self) -> &str {
        "yum"
    }

    fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "[{}] Reading repo metadata from {}. Installing RPM package {}",
            self.adapter_name(),
            self.repo_metadata_path,
            package.name
        );
        Ok(())
    }

    fn remove(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("[{}] Erasing RPM package {}", self.adapter_name(), package.name);
        Ok(())
    }

    fn update(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("[{}] Running transaction check & upgrade for RPM package {}", self.adapter_name(), package.name);
        Ok(())
    }
}

/// PacmanAdapter handles Arch Linux package formats
pub struct PacmanAdapter {
    pub sync_db_path: String,
}

impl PacmanAdapter {
    pub fn new() -> Self {
        Self {
            sync_db_path: "/var/lib/pacman/sync".to_string(),
        }
    }
}

impl PackageFormatAdapter for PacmanAdapter {
    fn format(&self) -> PackageFormat {
        PackageFormat::Pacman
    }

    fn adapter_name(&self) -> &str {
        "pacman"
    }

    fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "[{}] Synchronizing DB from {}. Installing package {}",
            self.adapter_name(),
            self.sync_db_path,
            package.name
        );
        Ok(())
    }

    fn remove(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("[{}] Removing pacman package {}", self.adapter_name(), package.name);
        Ok(())
    }

    fn update(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("[{}] Sysupgrade pacman package {}", self.adapter_name(), package.name);
        Ok(())
    }
}

/// SnapAdapter handles Canonical Snap packages
pub struct SnapAdapter {
    pub confinement_level: String,
}

impl SnapAdapter {
    pub fn new() -> Self {
        Self {
            confinement_level: "strict".to_string(),
        }
    }
}

impl PackageFormatAdapter for SnapAdapter {
    fn format(&self) -> PackageFormat {
        PackageFormat::Snap
    }

    fn adapter_name(&self) -> &str {
        "snap"
    }

    fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "[{}] Setting confinement: {}. Mounting snap package {}",
            self.adapter_name(),
            self.confinement_level,
            package.name
        );
        Ok(())
    }

    fn remove(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("[{}] Unmounting snap package {}", self.adapter_name(), package.name);
        Ok(())
    }

    fn update(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("[{}] Refreshing snap package {}", self.adapter_name(), package.name);
        Ok(())
    }
}

/// FlatpakAdapter handles Flatpak sandboxed packages
pub struct FlatpakAdapter {
    pub ostree_repo: String,
}

impl FlatpakAdapter {
    pub fn new() -> Self {
        Self {
            ostree_repo: "/var/lib/flatpak/repo".to_string(),
        }
    }
}

impl PackageFormatAdapter for FlatpakAdapter {
    fn format(&self) -> PackageFormat {
        PackageFormat::Flatpak
    }

    fn adapter_name(&self) -> &str {
        "flatpak"
    }

    fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "[{}] Pulling from OSTree repo: {}. Installing flatpak package {}",
            self.adapter_name(),
            self.ostree_repo,
            package.name
        );
        Ok(())
    }

    fn remove(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("[{}] Uninstalling flatpak package {}", self.adapter_name(), package.name);
        Ok(())
    }

    fn update(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("[{}] Updating flatpak package {}", self.adapter_name(), package.name);
        Ok(())
    }
}

/// SigmaPkgAdapter handles native SigmaOS packages
pub struct SigmaPkgAdapter {
    pub secure_integrity_check: bool,
}

impl SigmaPkgAdapter {
    pub fn new() -> Self {
        Self {
            secure_integrity_check: true,
        }
    }
}

impl PackageFormatAdapter for SigmaPkgAdapter {
    fn format(&self) -> PackageFormat {
        PackageFormat::SigmaPkg
    }

    fn adapter_name(&self) -> &str {
        "sigpkg"
    }

    fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "[{}] Integrity check status: {}. Unpacking native SigmaPkg package {}",
            self.adapter_name(),
            self.secure_integrity_check,
            package.name
        );
        Ok(())
    }

    fn remove(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("[{}] Deleting native SigmaPkg package {}", self.adapter_name(), package.name);
        Ok(())
    }

    fn update(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("[{}] Atomic rollback-safe update of SigmaPkg package {}", self.adapter_name(), package.name);
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

    /// Parse a dependency string (e.g. "curl>=7.81.0" or just "curl") into package name and constraint
    pub fn parse_dependency(dep_str: &str) -> (String, SemVerConstraint) {
        let operators = [">=", "<=", ">", "<", "="];
        for op in &operators {
            if let Some(idx) = dep_str.find(op) {
                let name = dep_str[..idx].trim().to_string();
                let constraint_str = &dep_str[idx..];
                let constraint = SemVerConstraint::parse(constraint_str);
                return (name, constraint);
            }
        }
        (dep_str.trim().to_string(), SemVerConstraint::Any)
    }

    pub fn resolve_dependencies(&self, package_name: &str) -> Result<Vec<String>, PackageError> {
        let mut resolved = Vec::new();
        let mut to_visit = vec![package_name.to_string()];
        let mut visited = std::collections::HashSet::new();

        while let Some(current) = to_visit.pop() {
            let (name, constraint) = Self::parse_dependency(&current);
            if visited.contains(&name) {
                continue;
            }

            visited.insert(name.clone());

            if let Some(package) = self.packages.get(&name) {
                // Verify SemVer constraint
                if let Some(pkg_ver) = SemVer::parse(&package.version) {
                    if !constraint.matches(&pkg_ver) {
                        return Err(PackageError::VersionMismatch(
                            name,
                            package.version.clone(),
                            format!("{:?}", constraint),
                        ));
                    }
                }

                // Push dependencies of this package
                for dep in &package.dependencies {
                    let (dep_name, _) = Self::parse_dependency(dep);
                    if !visited.contains(&dep_name) {
                        to_visit.push(dep.clone());
                    }
                }
                resolved.push(name);
            } else {
                return Err(PackageError::DependencyNotFound(name));
            }
        }

        Ok(resolved)
    }

    pub fn detect_conflicts(&self, packages: &[String]) -> Vec<(String, String)> {
        let mut conflicts = Vec::new();
        // Optimize: pre-resolve packages to avoid repetitive, redundant O(N^2) hash map lookups.
        // This reduces hash map lookup overhead from O(N^2) to flat O(N).
        let resolved_packages: Vec<(&String, &UnifiedPackage)> = packages
            .iter()
            .filter_map(|name| self.packages.get(name).map(|pkg| (name, pkg)))
            .collect();

        for (i, (pkg1_name, pkg1)) in resolved_packages.iter().enumerate() {
            for (pkg2_name, pkg2) in resolved_packages.iter().skip(i + 1) {
                if pkg1.has_conflict_with(pkg2) {
                    conflicts.push(((*pkg1_name).clone(), (*pkg2_name).clone()));
                }
            }
        }

        conflicts
    }

    pub fn resolve_conflicts(&self, conflicts: &[(String, String)]) -> Vec<String> {
        let mut resolution = Vec::new();

        match self.resolution_strategy {
            ConflictResolution::PreferNewest => {
                // Prefer the package with higher version
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
                // Prefer the package with lower version
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
                // Prefer SigmaPkg format
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
                // Return conflicts for manual resolution
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

/// Universal package manager
pub struct UniversalPackageManager {
    pub packages: HashMap<String, UnifiedPackage>,
    pub adapters: HashMap<PackageFormat, Box<dyn PackageFormatAdapter>>,
    pub resolver: DependencyResolver,
    pub installed_packages: HashMap<String, UnifiedPackage>,
    pub transaction_history: TransactionalHistory,
    pub metadata_cache: HashMap<String, UnifiedPackage>,
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
        };

        manager.add_default_adapters();
        manager
    }

    fn add_default_adapters(&mut self) {
        self.adapters.insert(PackageFormat::Deb, Box::new(AptDebAdapter::new()));
        self.adapters.insert(PackageFormat::Rpm, Box::new(YumRpmAdapter::new()));
        self.adapters.insert(PackageFormat::Pacman, Box::new(PacmanAdapter::new()));
        self.adapters.insert(PackageFormat::Snap, Box::new(SnapAdapter::new()));
        self.adapters.insert(PackageFormat::Flatpak, Box::new(FlatpakAdapter::new()));
        self.adapters.insert(PackageFormat::SigmaPkg, Box::new(SigmaPkgAdapter::new()));
    }

    /// Dynamic polymorphic registration of custom format adapters
    pub fn register_adapter(&mut self, format: PackageFormat, adapter: Box<dyn PackageFormatAdapter>) {
        self.adapters.insert(format, adapter);
    }

    pub fn add_package(&mut self, package: UnifiedPackage) {
        self.resolver.add_package(package.clone());
        self.metadata_cache.insert(package.name.clone(), package.clone());
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

        let mut installed_in_this_transaction = Vec::new();

        // Install packages
        for dep_name in dependencies {
            if let Some(package) = self.packages.get(&dep_name) {
                // Verify package integrity / cryptographic validation
                if !package.verify_integrity() {
                    self.rollback_transaction(&installed_in_this_transaction);
                    return Err(PackageError::InstallationFailed(format!(
                        "Integrity validation failed for {}",
                        dep_name
                    )));
                }

                // Find appropriate adapter
                let mut installed_by_adapter = false;
                for format in &package.formats {
                    if let Some(adapter) = self.adapters.get(format) {
                        match adapter.install(package) {
                            Ok(_) => {
                                installed_by_adapter = true;
                                break;
                            }
                            Err(e) => {
                                self.rollback_transaction(&installed_in_this_transaction);
                                return Err(e);
                            }
                        }
                    }
                }

                if !installed_by_adapter {
                    self.rollback_transaction(&installed_in_this_transaction);
                    return Err(PackageError::AdapterNotFound);
                }

                let mut installed = package.clone();
                installed.installed = true;
                self.installed_packages.insert(dep_name.clone(), installed);
                installed_in_this_transaction.push(dep_name);
            }
        }

        Ok(())
    }

    fn rollback_transaction(&mut self, installed: &[String]) {
        println!("Executing atomic rollback for transaction...");
        for pkg_name in installed {
            if let Some(package) = self.installed_packages.remove(pkg_name) {
                for format in &package.formats {
                    if let Some(adapter) = self.adapters.get(format) {
                        let _ = adapter.remove(&package);
                        break;
                    }
                }
            }
        }
    }

    pub fn remove(&mut self, package_name: &str) -> Result<(), PackageError> {
        if let Some(package) = self.installed_packages.get(package_name) {
            let mut removed_by_adapter = false;
            for format in &package.formats {
                if let Some(adapter) = self.adapters.get(format) {
                    adapter.remove(package)?;
                    removed_by_adapter = true;
                    break;
                }
            }
            if !removed_by_adapter {
                return Err(PackageError::AdapterNotFound);
            }
            self.installed_packages.remove(package_name);
        }
        Ok(())
    }

    pub fn update(&mut self, package_name: &str) -> Result<(), PackageError> {
        if let Some(package) = self.installed_packages.get(package_name) {
            let mut updated_by_adapter = false;
            for format in &package.formats {
                if let Some(adapter) = self.adapters.get(format) {
                    adapter.update(package)?;
                    updated_by_adapter = true;
                    break;
                }
            }
            if !updated_by_adapter {
                return Err(PackageError::AdapterNotFound);
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

    pub fn rollback_snapshot(&mut self, package_name: &str) -> Result<(), PackageError> {
        if let Some(package) = self.packages.get(package_name) {
            println!("Rolling back package snapshot: {}", package.name);
            Ok(())
        } else {
            Err(PackageError::PackageNotFound(package_name.to_string()))
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
    VersionMismatch(String, String, String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeatureType {
    Binary,
    Library,
    Source,
}

pub trait PackageAdapter {
    fn adapter_name(&self) -> &str;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_creation() {
        let manager = UniversalPackageManager::new();
        assert_eq!(manager.adapters.len(), 11);
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
}
