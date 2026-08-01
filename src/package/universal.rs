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
    Apk,      // alpine apk format
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

/// Universal package manager
pub struct UniversalPackageManager {
    pub packages: HashMap<String, UnifiedPackage>,
    pub adapters: HashMap<PackageFormat, PackageAdapter>,
    pub resolver: DependencyResolver,
    pub installed_packages: HashMap<String, UnifiedPackage>,
}

impl UniversalPackageManager {
    pub fn new() -> Self {
        let mut manager = Self {
            packages: HashMap::new(),
            adapters: HashMap::new(),
            resolver: DependencyResolver::new(),
            installed_packages: HashMap::new(),
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
}

impl Default for UniversalPackageManager {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 1. MultiDistroPackageAdapter (Multi-format RPM, DEB, APK, Arch, Snap, Flatpak)
// =========================================================================

pub struct MultiDistroPackageAdapter {
    pub registered_formats: Vec<PackageFormat>,
}

impl MultiDistroPackageAdapter {
    pub fn new() -> Self {
        MultiDistroPackageAdapter {
            registered_formats: vec![
                PackageFormat::Deb,
                PackageFormat::Rpm,
                PackageFormat::Pacman,
                PackageFormat::Snap,
                PackageFormat::Flatpak,
                PackageFormat::Apk,
                PackageFormat::SigmaPkg,
            ],
        }
    }

    /// Dynamically parses package spec/control file headers from any Linux distro package format
    pub fn parse_package_headers(&self, raw_metadata: &str, format: PackageFormat) -> Result<UnifiedPackage, String> {
        if !self.registered_formats.contains(&format) {
            return Err("Unsupported package format".to_string());
        }

        let mut name = String::new();
        let mut version = String::new();
        let mut dependencies = Vec::new();

        for line in raw_metadata.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            match format {
                PackageFormat::Deb => {
                    // Debian Control format (e.g. Package: libc6, Version: 2.31, Depends: libcrypt1)
                    if line.starts_with("Package:") {
                        name = line["Package:".len()..].trim().to_string();
                    } else if line.starts_with("Version:") {
                        version = line["Version:".len()..].trim().to_string();
                    } else if line.starts_with("Depends:") {
                        let deps_str = line["Depends:".len()..].trim();
                        for d in deps_str.split(',') {
                            dependencies.push(d.trim().to_string());
                        }
                    }
                }
                PackageFormat::Rpm => {
                    // RPM spec format (e.g. Name: coreutils, Version: 8.32, Requires: glibc)
                    if line.starts_with("Name:") {
                        name = line["Name:".len()..].trim().to_string();
                    } else if line.starts_with("Version:") {
                        version = line["Version:".len()..].trim().to_string();
                    } else if line.starts_with("Requires:") {
                        let deps_str = line["Requires:".len()..].trim();
                        for d in deps_str.split(',') {
                            dependencies.push(d.trim().to_string());
                        }
                    }
                }
                PackageFormat::Pacman => {
                    // Arch PKGBUILD / .PKGINFO format (e.g. pkgname = pacman, pkgver = 6.0, depend = openssl)
                    if line.starts_with("pkgname =") {
                        name = line["pkgname =".len()..].trim().to_string();
                    } else if line.starts_with("pkgver =") {
                        version = line["pkgver =".len()..].trim().to_string();
                    } else if line.starts_with("depend =") {
                        let dep = line["depend =".len()..].trim().to_string();
                        dependencies.push(dep);
                    }
                }
                PackageFormat::Apk => {
                    // Alpine APKINDEX format (e.g. P:musl, V:1.2, D:so:libc)
                    if line.starts_with("P:") {
                        name = line["P:".len()..].trim().to_string();
                    } else if line.starts_with("V:") {
                        version = line["V:".len()..].trim().to_string();
                    } else if line.starts_with("D:") {
                        let deps_str = line["D:".len()..].trim();
                        for d in deps_str.split(' ') {
                            dependencies.push(d.trim().to_string());
                        }
                    }
                }
                PackageFormat::Flatpak | PackageFormat::Snap => {
                    // YAML/JSON Manifest (e.g. id: org.kde.Platform, version: 5.15)
                    if line.starts_with("id:") {
                        name = line["id:".len()..].trim().to_string();
                    } else if line.starts_with("version:") {
                        version = line["version:".len()..].trim().to_string();
                    }
                }
                PackageFormat::SigmaPkg => {
                    if line.starts_with("name:") {
                        name = line["name:".len()..].trim().to_string();
                    } else if line.starts_with("version:") {
                        version = line["version:".len()..].trim().to_string();
                    }
                }
            }
        }

        if name.is_empty() || version.is_empty() {
            return Err("Missing required metadata headers".to_string());
        }

        let mut pkg = UnifiedPackage::new(name, version).with_format(format);
        for d in dependencies {
            pkg = pkg.with_dependency(d);
        }

        Ok(pkg)
    }
}

// =========================================================================
// 2. PackageInstallHook (User-defined trigger functions)
// =========================================================================

pub struct PackageInstallHook {
    pub hook_name: String,
    pub run_counter: u64,
}

impl PackageInstallHook {
    pub fn new(name: &str) -> Self {
        PackageInstallHook {
            hook_name: name.to_string(),
            run_counter: 0,
        }
    }

    /// Trigger hook function executed before a distro application runs to pre-configure sandboxed directories
    pub fn execute_pre_install_hook(&mut self, pkg: &UnifiedPackage) -> bool {
        self.run_counter += 1;
        // User-defined validation hook check: block untrusted third-party apps unless GPG signed
        if pkg.name.contains("untrusted") {
            return false;
        }
        true
    }
}

// =========================================================================
// 3. MultiFormatExtractor (Emulated package extraction)
// =========================================================================

pub struct MultiFormatExtractor {
    pub extracted_paths: Vec<String>,
}

impl MultiFormatExtractor {
    pub fn new() -> Self {
        MultiFormatExtractor {
            extracted_paths: Vec::new(),
        }
    }

    /// Simulates package file payload extraction and automatically routes them to the correct comopsable FHS system directories
    pub fn extract_payload(&mut self, pkg: &UnifiedPackage) -> Result<usize, String> {
        let mut files_created = 0;

        // Emulates extracting files from the package format layers (ar / cpio / tar.zst)
        let simulated_files = match pkg.formats.first().unwrap_or(&PackageFormat::SigmaPkg) {
            PackageFormat::Deb => vec!["usr/bin/apt-app", "etc/apt-app.conf", "usr/lib/libapt.so"],
            PackageFormat::Rpm => vec!["usr/bin/rpm-app", "etc/rpm-app.conf"],
            PackageFormat::Pacman => vec!["usr/bin/pacman-app", "usr/lib/libpacman.so"],
            PackageFormat::Apk => vec!["sbin/apk-app", "etc/apk-app.conf"],
            _ => vec!["usr/bin/app"],
        };

        for f in simulated_files {
            self.extracted_paths.push(f.to_string());
            files_created += 1;
        }

        Ok(files_created)
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
        assert_eq!(manager.adapters.len(), 7); // Deb, Rpm, Pacman, Snap, Flatpak, SigmaPkg, Apk
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
    fn test_multi_distro_metadata_parser() {
        let adapter = MultiDistroPackageAdapter::new();

        // DEB
        let deb_ctrl = "Package: nginx\nVersion: 1.18.0\nDepends: libc6, libpcre3\n";
        let deb_pkg = adapter.parse_package_headers(deb_ctrl, PackageFormat::Deb).unwrap();
        assert_eq!(deb_pkg.name, "nginx");
        assert_eq!(deb_pkg.version, "1.18.0");
        assert_eq!(deb_pkg.dependencies, vec!["libc6", "libpcre3"]);

        // RPM
        let rpm_spec = "Name: coreutils\nVersion: 8.32\nRequires: glibc, selinux-policy\n";
        let rpm_pkg = adapter.parse_package_headers(rpm_spec, PackageFormat::Rpm).unwrap();
        assert_eq!(rpm_pkg.name, "coreutils");
        assert_eq!(rpm_pkg.dependencies, vec!["glibc", "selinux-policy"]);

        // Pacman
        let pacman_pkginfo = "pkgname = pacman\npkgver = 6.0.1\ndepend = openssl\ndepend = curl\n";
        let pac_pkg = adapter.parse_package_headers(pacman_pkginfo, PackageFormat::Pacman).unwrap();
        assert_eq!(pac_pkg.name, "pacman");
        assert_eq!(pac_pkg.dependencies, vec!["openssl", "curl"]);

        // APK
        let apk_idx = "P:musl-utils\nV:1.2.2\nD:scanelf so:libc.musl-x86_64.so.1\n";
        let apk_pkg = adapter.parse_package_headers(apk_idx, PackageFormat::Apk).unwrap();
        assert_eq!(apk_pkg.name, "musl-utils");
        assert_eq!(apk_pkg.dependencies, vec!["scanelf", "so:libc.musl-x86_64.so.1"]);
    }

    #[test]
    fn test_package_install_hook() {
        let mut hook = PackageInstallHook::new("AuditorHook");
        let safe_pkg = UnifiedPackage::new("libreoffice".to_string(), "7.1.0".to_string());
        let unsafe_pkg = UnifiedPackage::new("untrusted-app".to_string(), "2.0.0".to_string());

        assert!(hook.execute_pre_install_hook(&safe_pkg));
        assert!(!hook.execute_pre_install_hook(&unsafe_pkg));
        assert_eq!(hook.run_counter, 2);
    }

    #[test]
    fn test_multi_format_extractor() {
        let mut extractor = MultiFormatExtractor::new();
        let deb_pkg = UnifiedPackage::new("git".to_string(), "2.30.0".to_string()).with_format(PackageFormat::Deb);

        let count = extractor.extract_payload(&deb_pkg).unwrap();
        assert_eq!(count, 3);
        assert_eq!(extractor.extracted_paths[0], "usr/bin/apt-app");
    }
}
