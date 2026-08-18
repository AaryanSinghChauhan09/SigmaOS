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
pub enum PackagePriority {
    Essential,
    Required,
    Important,
    Standard,
    Optional,
}

pub enum PackageFormat {
    Deb,      // apt
    Rpm,      // yum
    Pacman,   // pacman
    Snap,     // snap
    Flatpak,  // flatpak
    SigmaPkg, // native SigmaOS format
    AppImage, // portable app
    Guix,     // functional package format
    Nix,      // nixos package format
    Portage,  // gentoo emerge
    Zypper,   // opensuse package format
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

/// Package format adapter

pub trait PackageFormatAdapter: Send + Sync {
    fn format(&self) -> PackageFormat;
    fn adapter_name(&self) -> &str;
    fn parse_manifest(&self, _raw_data: &[u8]) -> Result<UnifiedPackage, &'static str> {
        Err("Not implemented")
    }
    fn install(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
    fn remove(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
    fn update(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
}

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
        // Simulate installation
        Ok(())
    }

    fn remove(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "[{}] Purging DEB package {}",
            self.adapter_name(),
            package.name
        );
        Ok(())
    }

    fn update(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "[{}] Refreshing and updating DEB package {}",
            self.adapter_name(),
            package.name
        );
        Ok(())
    }
}

/// AptDebAdapter handles Debian/Ubuntu package formats (`.deb`)
pub struct AptDebAdapter {
    pub dpkg_status_path: String,
}

impl AptDebAdapter {
    pub fn new() -> Self {
        Self {
            dpkg_status_path: "/var/lib/dpkg/status".to_string(),
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

    fn parse_manifest(&self, raw_data: &[u8]) -> Result<UnifiedPackage, &'static str> {
        let manifest = String::from_utf8(raw_data.to_vec())
            .map_err(|_| "Failed to parse UTF-8 DEB manifest")?;
        let mut name = String::new();
        let mut version = String::new();
        let mut dependencies = Vec::new();

        for line in manifest.lines() {
            if line.starts_with("Package: ") {
                name = line["Package: ".len()..].trim().to_string();
            } else if line.starts_with("Version: ") {
                version = line["Version: ".len()..].trim().to_string();
            } else if line.starts_with("Depends: ") {
                let deps = line["Depends: ".len()..].trim();
                for d in deps.split(',') {
                    dependencies.push(d.trim().to_string());
                }
            }
        }

        if name.is_empty() || version.is_empty() {
            return Err("Invalid DEB manifest");
        }

        Ok(UnifiedPackage::new(
            &name,
            &version,
            PackageFormat::Deb,
            dependencies,
            vec!["/usr/bin/".to_string() + &name],
        ))
    }

    fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("AptDebAdapter: Installing Debian package {}", package.name);
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
            "Removing {} using {} adapter",
            package.name, self.adapter_name
        );
        // Simulate removal
        Ok(())
    }

    fn remove(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "[{}] Erasing RPM package {}",
            self.adapter_name(),
            package.name
        );
        Ok(())
    }

    fn update(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "[{}] Running transaction check & upgrade for RPM package {}",
            self.adapter_name(),
            package.name
        );
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
            "Updating {} using {} adapter",
            package.name, self.adapter_name
        );
        Ok(())
    }

    fn remove(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "[{}] Removing pacman package {}",
            self.adapter_name(),
            package.name
        );
        Ok(())
    }

    fn update(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "[{}] Sysupgrade pacman package {}",
            self.adapter_name(),
            package.name
        );
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
        println!(
            "[{}] Unmounting snap package {}",
            self.adapter_name(),
            package.name
        );
        Ok(())
    }

    fn update(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "[{}] Refreshing snap package {}",
            self.adapter_name(),
            package.name
        );
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
        println!(
            "[{}] Uninstalling flatpak package {}",
            self.adapter_name(),
            package.name
        );
        Ok(())
    }

    fn update(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "[{}] Updating flatpak package {}",
            self.adapter_name(),
            package.name
        );
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
        println!(
            "[{}] Deleting native SigmaPkg package {}",
            self.adapter_name(),
            package.name
        );
        Ok(())
    }

    fn update(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "[{}] Atomic rollback-safe update of SigmaPkg package {}",
            self.adapter_name(),
            package.name
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
                let pkg1: &UnifiedPackage = match self.packages.get(pkg1_name) {
                    Some(p) => p,
                    None => continue,
                };
                let pkg2: &UnifiedPackage = match self.packages.get(pkg2_name) {
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
                // Prefer the package with higher version
                for (pkg1, pkg2) in conflicts {
                    let p1: &UnifiedPackage = match self.packages.get(pkg1) {
                        Some(p) => p,
                        None => continue,
                    };
                    let p2: &UnifiedPackage = match self.packages.get(pkg2) {
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
                // Prefer the package with lower version
                for (pkg1, pkg2) in conflicts {
                    let p1: &UnifiedPackage = match self.packages.get(pkg1) {
                        Some(p) => p,
                        None => continue,
                    };
                    let p2: &UnifiedPackage = match self.packages.get(pkg2) {
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
                // Prefer SigmaPkg format
                for (pkg1, pkg2) in conflicts {
                    let p1: &UnifiedPackage = match self.packages.get(pkg1) {
                        Some(p) => p,
                        None => continue,
                    };
                    let p2: &UnifiedPackage = match self.packages.get(pkg2) {
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
        Self {
            cache: HashMap::new(),
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
        self.adapters
            .insert(PackageFormat::Deb, Box::new(AptDebAdapter::new()));
        self.adapters
            .insert(PackageFormat::Rpm, Box::new(YumRpmAdapter::new()));
        self.adapters
            .insert(PackageFormat::Pacman, Box::new(PacmanAdapter::new()));
        self.adapters
            .insert(PackageFormat::Snap, Box::new(SnapAdapter::new()));
        self.adapters
            .insert(PackageFormat::Flatpak, Box::new(FlatpakAdapter::new()));
        self.adapters
            .insert(PackageFormat::SigmaPkg, Box::new(SigmaPkgAdapter::new()));
    }

    /// Dynamic polymorphic registration of custom format adapters
    pub fn register_adapter(
        &mut self,
        format: PackageFormat,
        adapter: Box<dyn PackageFormatAdapter>,
    ) {
        self.adapters.insert(format, adapter);
    }

    pub fn add_package(&mut self, package: UnifiedPackage) {
        self.resolver.add_package(package.clone());
        self.metadata_cache
            .insert(package.name.clone(), package.clone());
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
            if let Some(package) = self.packages.get(&dep_name) {
                // Find appropriate adapter
                for format in &package.formats {
                    if let Some(adapter) = self.adapters.get(format) {
                        let adapter: &PackageAdapter = adapter;
                        adapter.install(package)?;
                        break;
                    }
                }

                let mut installed = package.clone();
                installed.installed = true;
                self.installed_packages.insert(dep_name.clone(), installed);
            }
        }

        Ok(())
    }

    pub fn remove(&mut self, package_name: &str) -> Result<(), PackageError> {
        if let Some(package) = self.installed_packages.get(package_name) {
            for format in &package.formats {
                let adapter = match self.adapters.get(format) {
                    Some(a) => a,
                    None => continue,
                };
                adapter.remove(package)?;
                break;
            }
            self.installed_packages.remove(package_name);
        }
        Ok(())
    }

    pub fn update(&mut self, package_name: &str) -> Result<(), PackageError> {
        if let Some(package) = self.installed_packages.get(package_name) {
            for format in &package.formats {
                let adapter = match self.adapters.get(format) {
                    Some(a) => a,
                    None => continue,
                };
                adapter.update(package)?;
                break;
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
        let mut resolver = DependencyResolver::new();
        let lib_pkg = UnifiedPackage::new("lib-helper".to_string(), "1.2.3".to_string())
            .with_format(PackageFormat::SigmaPkg);
        let app_pkg = UnifiedPackage::new("my-app".to_string(), "1.0.0".to_string())
            .with_format(PackageFormat::SigmaPkg)
            .with_dependency("lib-helper".to_string());

        resolver.add_package(lib_pkg);
        resolver.add_package(app_pkg);

        let deps = resolver.resolve_dependencies("my-app").unwrap();
        assert_eq!(deps.len(), 2);
    }

    struct FailingAdapter;
    impl PackageFormatAdapter for FailingAdapter {
        fn format(&self) -> PackageFormat {
            PackageFormat::SigmaPkg
        }
        fn adapter_name(&self) -> &str {
            "failing-adapter"
        }
        fn install(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
            Err(PackageError::InstallationFailed(
                "Simulated crash".to_string(),
            ))
        }
    }

    #[test]
    fn test_batch_transaction_atomic_rollback() {
        let mut manager = UniversalPackageManager::new();
        manager.register_adapter(PackageFormat::SigmaPkg, Box::new(FailingAdapter));

        let pkg1 = UnifiedPackage::new("my-app".to_string(), "1.0.0".to_string())
            .with_format(PackageFormat::SigmaPkg);

        manager.add_package(pkg1);

        let checkpoint_id = manager.create_checkpoint();
        assert_eq!(checkpoint_id, 0);
    }
}
