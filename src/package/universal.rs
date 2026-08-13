// SigmaOS Universal Package Manager
// Unified system absorbing apt, yum, pacman, snap, flatpak, apk

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
    Apk,      // alpine apk
    Ebuild,   // gentoo portage ebuild
    Xbps,     // void linux xbps
    Nix,      // nix/guix functional packages
    Eopkg,    // solus eopkg
    SlackPkg, // slackware standard packages
}

/// Version operator for dependency constraints
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VersionOp {
    Any,
    Eq,
    Ge,
    Le,
}

/// Structural package dependency constraint
#[derive(Debug, Clone)]
pub struct DependencyConstraint {
    pub name: String,
    pub op: VersionOp,
    pub version: String,
}

impl DependencyConstraint {
    pub fn parse(s: &str) -> Self {
        if let Some(pos) = s.find(">=") {
            let name = s[..pos].trim().to_string();
            let version = s[pos+2..].trim().to_string();
            DependencyConstraint { name, op: VersionOp::Ge, version }
        } else if let Some(pos) = s.find("<=") {
            let name = s[..pos].trim().to_string();
            let version = s[pos+2..].trim().to_string();
            DependencyConstraint { name, op: VersionOp::Le, version }
        } else if let Some(pos) = s.find("==") {
            let name = s[..pos].trim().to_string();
            let version = s[pos+2..].trim().to_string();
            DependencyConstraint { name, op: VersionOp::Eq, version }
        } else {
            DependencyConstraint {
                name: s.to_string(),
                op: VersionOp::Any,
                version: String::new(),
            }
        }
    }

    pub fn is_satisfied_by(&self, pkg_ver: &str) -> bool {
        match self.op {
            VersionOp::Any => true,
            VersionOp::Eq => pkg_ver == self.version,
            VersionOp::Ge => pkg_ver >= &self.version,
            VersionOp::Le => pkg_ver <= &self.version,
        }
    }
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
            let constraint = DependencyConstraint::parse(&current);
            let pkg_name = constraint.name.clone();

            if visited.contains(&pkg_name) {
                continue;
            }

            visited.insert(pkg_name.clone());

            if let Some(package) = self.packages.get(&pkg_name) {
                if !constraint.is_satisfied_by(&package.version) {
                    return Err(PackageError::InstallationFailed(format!(
                        "Dependency version constraint violation: {} require version satisfies constraint, but found version {}",
                        current, package.version
                    )));
                }

                for dep in &package.dependencies {
                    let dep_constraint = DependencyConstraint::parse(dep);
                    if !visited.contains(&dep_constraint.name) {
                        to_visit.push(dep.clone());
                    }
                }
                resolved.push(pkg_name);
            } else {
                return Err(PackageError::DependencyNotFound(pkg_name));
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

/// Transaction state snapshot of the package manager
#[derive(Debug, Clone)]
pub struct PackageManagerSnapshot {
    pub id: String,
    pub description: String,
    pub installed_versions: HashMap<String, String>, // package_name -> version
}

/// Universal package manager
pub struct UniversalPackageManager {
    pub packages: HashMap<String, UnifiedPackage>,
    pub adapters: HashMap<PackageFormat, PackageAdapter>,
    pub resolver: DependencyResolver,
    pub installed_packages: HashMap<String, UnifiedPackage>,
    pub snapshots: Vec<PackageManagerSnapshot>,
}

impl UniversalPackageManager {
    pub fn new() -> Self {
        let mut manager = Self {
            packages: HashMap::new(),
            adapters: HashMap::new(),
            resolver: DependencyResolver::new(),
            installed_packages: HashMap::new(),
            snapshots: Vec::new(),
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
        let apk_adapter = PackageAdapter::new(PackageFormat::Apk, "apk".to_string());
        let ebuild_adapter = PackageAdapter::new(PackageFormat::Ebuild, "portage".to_string());
        let xbps_adapter = PackageAdapter::new(PackageFormat::Xbps, "xbps".to_string());
        let nix_adapter = PackageAdapter::new(PackageFormat::Nix, "nix".to_string());
        let eopkg_adapter = PackageAdapter::new(PackageFormat::Eopkg, "eopkg".to_string());
        let slackpkg_adapter = PackageAdapter::new(PackageFormat::SlackPkg, "slackpkg".to_string());

        self.adapters.insert(PackageFormat::Deb, apt_adapter);
        self.adapters.insert(PackageFormat::Rpm, yum_adapter);
        self.adapters.insert(PackageFormat::Pacman, pacman_adapter);
        self.adapters.insert(PackageFormat::Snap, snap_adapter);
        self.adapters
            .insert(PackageFormat::Flatpak, flatpak_adapter);
        self.adapters
            .insert(PackageFormat::SigmaPkg, sigpkg_adapter);
        self.adapters
            .insert(PackageFormat::Apk, apk_adapter);
        self.adapters
            .insert(PackageFormat::Ebuild, ebuild_adapter);
        self.adapters
            .insert(PackageFormat::Xbps, xbps_adapter);
        self.adapters
            .insert(PackageFormat::Nix, nix_adapter);
        self.adapters
            .insert(PackageFormat::Eopkg, eopkg_adapter);
        self.adapters
            .insert(PackageFormat::SlackPkg, slackpkg_adapter);
    }

    pub fn add_package(&mut self, package: UnifiedPackage) {
        self.resolver.add_package(package.clone());
        self.packages.insert(package.name.clone(), package);
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

    /// Creates a transaction snapshot of currently installed packages and versions
    pub fn create_snapshot(&mut self, description: &str) -> String {
        let id = format!("snap-{}", self.snapshots.len() + 1);
        let mut installed_versions = HashMap::new();
        for (name, pkg) in &self.installed_packages {
            installed_versions.insert(name.clone(), pkg.version.clone());
        }
        self.snapshots.push(PackageManagerSnapshot {
            id: id.clone(),
            description: description.to_string(),
            installed_versions,
        });
        id
    }

    /// Rolls back the entire installed package state to the specified snapshot state
    pub fn rollback_to_snapshot(&mut self, snapshot_id: &str) -> Result<(), PackageError> {
        let snapshot = self.snapshots.iter()
            .find(|s| s.id == snapshot_id)
            .cloned()
            .ok_or_else(|| PackageError::InstallationFailed(format!("Snapshot not found: {}", snapshot_id)))?;

        self.installed_packages.clear();

        for (name, version) in snapshot.installed_versions {
            if let Some(pkg) = self.packages.get(&name) {
                let mut restored = pkg.clone();
                restored.version = version;
                restored.installed = true;
                self.installed_packages.insert(name, restored);
            }
        }
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
        assert_eq!(manager.adapters.len(), 12); // Deb, Rpm, Pacman, Snap, Flatpak, SigmaPkg, Apk, Ebuild, Xbps, Nix, Eopkg, SlackPkg
    }

    #[test]
    fn test_all_12_package_formats() {
        let mut manager = UniversalPackageManager::new();

        let pkg_gentoo = UnifiedPackage::new("sys-apps/dbus".to_string(), "1.14.0".to_string())
            .with_format(PackageFormat::Ebuild);
        let pkg_void = UnifiedPackage::new("void-runit".to_string(), "2.12".to_string())
            .with_format(PackageFormat::Xbps);
        let pkg_nix = UnifiedPackage::new("nix-coreutils".to_string(), "9.1".to_string())
            .with_format(PackageFormat::Nix);
        let pkg_solus = UnifiedPackage::new("solus-budgie".to_string(), "10.6".to_string())
            .with_format(PackageFormat::Eopkg);
        let pkg_slack = UnifiedPackage::new("slackware-glibc".to_string(), "2.36".to_string())
            .with_format(PackageFormat::SlackPkg);

        manager.add_package(pkg_gentoo);
        manager.add_package(pkg_void);
        manager.add_package(pkg_nix);
        manager.add_package(pkg_solus);
        manager.add_package(pkg_slack);

        manager.install("sys-apps/dbus").unwrap();
        manager.install("void-runit").unwrap();
        manager.install("nix-coreutils").unwrap();
        manager.install("solus-budgie").unwrap();
        manager.install("slackware-glibc").unwrap();

        assert_eq!(manager.installed_packages.len(), 5);
        assert!(manager.installed_packages.contains_key("sys-apps/dbus"));
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
    fn test_version_constraints_parsing_and_checking() {
        let c1 = DependencyConstraint::parse("openssl>=1.1.1");
        assert_eq!(c1.name, "openssl");
        assert_eq!(c1.op, VersionOp::Ge);
        assert_eq!(c1.version, "1.1.1");
        assert!(c1.is_satisfied_by("1.1.1"));
        assert!(c1.is_satisfied_by("1.2.0"));
        assert!(!c1.is_satisfied_by("1.1.0"));

        let c2 = DependencyConstraint::parse("glibc<=2.34");
        assert_eq!(c2.name, "glibc");
        assert_eq!(c2.op, VersionOp::Le);
        assert_eq!(c2.version, "2.34");
        assert!(c2.is_satisfied_by("2.31"));
        assert!(c2.is_satisfied_by("2.34"));
        assert!(!c2.is_satisfied_by("2.35"));

        let c3 = DependencyConstraint::parse("bash==5.1");
        assert_eq!(c3.name, "bash");
        assert_eq!(c3.op, VersionOp::Eq);
        assert_eq!(c3.version, "5.1");
        assert!(c3.is_satisfied_by("5.1"));
        assert!(!c3.is_satisfied_by("5.2"));

        let c4 = DependencyConstraint::parse("curl");
        assert_eq!(c4.name, "curl");
        assert_eq!(c4.op, VersionOp::Any);
        assert!(c4.is_satisfied_by("1.0"));
    }

    #[test]
    fn test_versioned_dependency_resolution_failures() {
        let mut resolver = DependencyResolver::new();
        let app = UnifiedPackage::new("app".to_string(), "1.0.0".to_string())
            .with_dependency("openssl>=1.1.1".to_string());

        // Register satisfying dependency
        let openssl_good = UnifiedPackage::new("openssl".to_string(), "1.1.1g".to_string());
        resolver.add_package(app.clone());
        resolver.add_package(openssl_good);

        let deps_good = resolver.resolve_dependencies("app").unwrap();
        assert!(deps_good.contains(&"openssl".to_string()));
        assert!(deps_good.contains(&"app".to_string()));

        // Reset and register violating dependency
        let mut resolver_bad = DependencyResolver::new();
        let openssl_bad = UnifiedPackage::new("openssl".to_string(), "1.0.2u".to_string());
        resolver_bad.add_package(app);
        resolver_bad.add_package(openssl_bad);

        let res = resolver_bad.resolve_dependencies("app");
        assert!(res.is_err());
        if let Err(PackageError::InstallationFailed(msg)) = res {
            assert!(msg.contains("Dependency version constraint violation"));
        } else {
            panic!("Expected constraint violation error");
        }
    }

    #[test]
    fn test_package_manager_state_snapshots_and_rollback() {
        let mut manager = UniversalPackageManager::new();
        let pkg_a = UnifiedPackage::new("curl".to_string(), "7.81.0".to_string())
            .with_format(PackageFormat::Apk);
        let pkg_b = UnifiedPackage::new("wget".to_string(), "1.21.1".to_string())
            .with_format(PackageFormat::SigmaPkg);

        manager.add_package(pkg_a);
        manager.add_package(pkg_b);

        // Install curl only
        manager.install("curl").unwrap();
        assert!(manager.installed_packages.contains_key("curl"));
        assert!(!manager.installed_packages.contains_key("wget"));

        // Snapshot 1 (contains only curl)
        let snap_id = manager.create_snapshot("Curl installed");
        assert_eq!(snap_id, "snap-1");
        assert_eq!(manager.snapshots.len(), 1);

        // Install wget
        manager.install("wget").unwrap();
        assert!(manager.installed_packages.contains_key("wget"));

        // Roll back to Snapshot 1
        manager.rollback_to_snapshot("snap-1").unwrap();
        assert!(manager.installed_packages.contains_key("curl"));
        assert!(!manager.installed_packages.contains_key("wget"));
    }

    #[test]
    fn test_multiple_formats_linux_distros() {
        let mut manager = UniversalPackageManager::new();
        let apk_pkg = UnifiedPackage::new("libssl".to_string(), "3.0.0".to_string())
            .with_format(PackageFormat::Apk);
        let rpm_pkg = UnifiedPackage::new("kernel-headers".to_string(), "6.1.0".to_string())
            .with_format(PackageFormat::Rpm);

        manager.add_package(apk_pkg);
        manager.add_package(rpm_pkg);

        manager.install("libssl").unwrap();
        manager.install("kernel-headers").unwrap();

        assert_eq!(manager.installed_packages.len(), 2);
    }
}
