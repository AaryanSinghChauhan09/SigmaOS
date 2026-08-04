// SigmaOS Universal Package Manager
// Unified system absorbing apt, yum, pacman, snap, flatpak

use std::collections::HashMap;

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
        let mut visited = std::collections::HashSet::new();

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
||||||| 43be3a7e8
/// Universal package manager
/// Package snapshot representing a saved system state of installed packages
#[derive(Debug, Clone)]
pub struct PackageSnapshot {
    pub id: usize,
    pub description: String,
    pub timestamp: u64,
    pub installed_packages: HashMap<String, UnifiedPackage>,
}

/// Universal package manager with transaction-safe snapshots & rollback mechanisms
pub struct UniversalPackageManager {
    pub packages: HashMap<String, UnifiedPackage>,
    pub adapters: HashMap<PackageFormat, PackageAdapter>,
    pub resolver: DependencyResolver,
    pub installed_packages: HashMap<String, UnifiedPackage>,
    pub transaction_history: TransactionalHistory,
    pub metadata_cache: HashMap<String, UnifiedPackage>,
||||||| 43be3a7e8
    pub snapshots: HashMap<usize, PackageSnapshot>,
    pub next_snapshot_id: usize,
||||||| 0ddf2eac7
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
||||||| 43be3a7e8
            snapshots: HashMap::new(),
            next_snapshot_id: 1,
||||||| 0ddf2eac7
            metadata_cache: HashMap::new(),
        };

        manager.add_default_adapters();
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
            if let Some(package) = self.packages.get(&dep_name) {
                // Find appropriate adapter
                for format in &package.formats {
                    if let Some(adapter) = self.adapters.get(format) {
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
                if let Some(adapter) = self.adapters.get(format) {
                    adapter.remove(package)?;
                    break;
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

    /// Create a snapshot of currently installed packages state
    pub fn create_snapshot(&mut self, description: String) -> usize {
        let id = self.next_snapshot_id;
        self.next_snapshot_id += 1;

        let snapshot = PackageSnapshot {
            id,
            description,
            timestamp: 0,
            installed_packages: self.installed_packages.clone(),
        };

        self.snapshots.insert(id, snapshot);
        id
    }

    /// Delete a package snapshot
    pub fn delete_snapshot(&mut self, id: usize) -> Result<(), PackageError> {
        if self.snapshots.remove(&id).is_none() {
            return Err(PackageError::PackageNotFound(format!("Snapshot ID {}", id)));
        }
        Ok(())
    }

    /// List all package snapshots
    pub fn list_snapshots(&self) -> Vec<(usize, String)> {
        let mut list = Vec::new();
        for (id, snap) in &self.snapshots {
            list.push((*id, snap.description.clone()));
        }
        list.sort_by_key(|&(id, _)| id);
        list
    }

    /// Rollback the active package state exactly to a previously saved snapshot
    pub fn rollback_to_snapshot(&mut self, id: usize) -> Result<(), PackageError> {
        let snapshot = self
            .snapshots
            .get(&id)
            .ok_or_else(|| PackageError::PackageNotFound(format!("Snapshot ID {}", id)))?
            .clone();

        // 1. Identify and uninstall packages currently installed but not in the snapshot
        let mut to_uninstall = Vec::new();
        for pkg_name in self.installed_packages.keys() {
            if !snapshot.installed_packages.contains_key(pkg_name) {
                to_uninstall.push(pkg_name.clone());
            }
        }

        for pkg_name in to_uninstall {
            self.remove(&pkg_name)?;
        }

        // 2. Identify and reinstall packages in the snapshot but not currently installed
        let mut to_install = Vec::new();
        for (pkg_name, _) in &snapshot.installed_packages {
            if !self.installed_packages.contains_key(pkg_name) {
                to_install.push(pkg_name.clone());
            }
        }

        for pkg_name in to_install {
            self.install(&pkg_name)?;
        }

        // 3. Sync full installed_packages state exactly with the snapshot
        self.installed_packages = snapshot.installed_packages;

        Ok(())
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
||||||| 43be3a7e8

    #[test]
    fn test_package_snapshots_and_rollback() {
        let mut manager = UniversalPackageManager::new();
        let pkg_v1 = UnifiedPackage::new("essential-tool".to_string(), "1.0.0".to_string())
            .with_format(PackageFormat::SigmaPkg);
        let pkg_v2 = UnifiedPackage::new("add-on-tool".to_string(), "2.0.0".to_string())
            .with_format(PackageFormat::SigmaPkg);

        manager.add_package(pkg_v1);
        manager.add_package(pkg_v2);

        // Install first package
        manager.install("essential-tool").unwrap();
        assert_eq!(manager.installed_packages.len(), 1);
        assert!(manager.installed_packages.contains_key("essential-tool"));

        // Create snapshot 1
        let snap_id = manager.create_snapshot("First stable package state".to_string());
        assert_eq!(manager.list_snapshots().len(), 1);

        // Install second package
        manager.install("add-on-tool").unwrap();
        assert_eq!(manager.installed_packages.len(), 2);
        assert!(manager.installed_packages.contains_key("add-on-tool"));

        // Rollback to snapshot 1
        manager.rollback_to_snapshot(snap_id).unwrap();

        // Verify state is reverted to exactly one package
        assert_eq!(manager.installed_packages.len(), 1);
        assert!(manager.installed_packages.contains_key("essential-tool"));
        assert!(!manager.installed_packages.contains_key("add-on-tool"));

        // Delete snapshot
        assert!(manager.delete_snapshot(snap_id).is_ok());
        assert!(manager.list_snapshots().is_empty());
    }
}
