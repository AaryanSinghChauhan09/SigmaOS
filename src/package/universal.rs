// SigmaOS Universal Package Manager
// Unified system absorbing apt, yum, pacman, snap, flatpak, and Alpine apk

use std::collections::HashMap;

/// Package format type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackageFormat {
    Deb,      // apt (Debian/Ubuntu)
    Rpm,      // yum/dnf (RHEL/Fedora/openSUSE)
    Pacman,   // pacman (Arch Linux)
    Apk,      // apk (Alpine Linux)
    Snap,     // snap (Ubuntu Sandbox)
    Flatpak,  // flatpak (Sandbox Desktop)
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
    pub fhs_redirect_required: bool,
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
            fhs_redirect_required: false,
        }
    }

    pub fn with_format(mut self, format: PackageFormat) -> Self {
        self.formats.push(format);
        if format != PackageFormat::SigmaPkg {
            self.fhs_redirect_required = true;
        }
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
        let mut capabilities = vec!["metadata_parsing".to_string(), "dependency_mapping".to_string()];
        if format != PackageFormat::SigmaPkg {
            capabilities.push("fhs_redirection_sandbox".to_string());
        }
        Self {
            format,
            adapter_name,
            capabilities,
        }
    }

    pub fn can_handle(&self, package: &UnifiedPackage) -> bool {
        package.formats.contains(&self.format)
    }

    /// Redirect standard Linux FHS paths to the secure SigmaOS sandbox directory
    pub fn redirect_fhs_path(&self, path: &str) -> String {
        if self.format == PackageFormat::SigmaPkg {
            return path.to_string();
        }

        let path_clean = path.trim();
        if path_clean.starts_with("/usr/bin") {
            format!("/sandbox/fhs_compat/bin/{}", &path_clean[8..].trim_start_matches('/'))
        } else if path_clean.starts_with("/etc") {
            format!("/sandbox/fhs_compat/etc/{}", &path_clean[4..].trim_start_matches('/'))
        } else if path_clean.starts_with("/var") {
            format!("/sandbox/fhs_compat/var/{}", &path_clean[4..].trim_start_matches('/'))
        } else if path_clean.starts_with("/lib64") {
            format!("/sandbox/fhs_compat/lib/{}", &path_clean[6..].trim_start_matches('/'))
        } else if path_clean.starts_with("/lib") {
            format!("/sandbox/fhs_compat/lib/{}", &path_clean[4..].trim_start_matches('/'))
        } else {
            format!("/sandbox/fhs_compat/{}", path_clean.trim_start_matches('/'))
        }
    }

    /// Maps external Linux library dependencies to native SigmaOS capability-gated modules
    pub fn map_linux_dependency(&self, external_dep: &str) -> &'static str {
        match external_dep.trim() {
            "libc6" | "glibc" | "musl" | "libc.so.6" | "libc.so" => "sigma_libc",
            "libssl" | "libcrypto" | "libssl.so" | "libssl.so.1.1" | "libssl.so.3" => "sigma_crypto",
            "libm" | "libm.so.6" | "libm.so" => "sigma_math",
            "libpthread" | "libpthread.so.0" | "libpthread.so" => "sigma_threads",
            "librt" | "librt.so.1" => "sigma_rt",
            "libdl" | "libdl.so.2" => "sigma_loader",
            "bash" | "sh" | "ash" | "zsh" => "sigma_sh",
            _ => "sigma_legacy_compat", // Default fallback security-wrapped compatibility module
        }
    }

    /// Translate foreign package metadata format into native UnifiedPackage structures
    pub fn translate_foreign_metadata(&self, foreign_data: &str) -> Result<UnifiedPackage, PackageError> {
        match self.format {
            PackageFormat::Deb => {
                // Parse Debian control file format (key-value pairs separated by colons)
                let mut name = String::new();
                let mut version = String::new();
                let mut dependencies = Vec::new();

                for line in foreign_data.lines() {
                    let parts: Vec<&str> = line.splitn(2, ':').collect();
                    if parts.len() == 2 {
                        let key = parts[0].trim().to_lowercase();
                        let val = parts[1].trim();
                        match key.as_str() {
                            "package" => name = val.to_string(),
                            "version" => version = val.to_string(),
                            "depends" => {
                                for dep in val.split(',') {
                                    let clean_dep = dep.trim().split(' ').next().unwrap_or("").to_string();
                                    if !clean_dep.is_empty() {
                                        dependencies.push(self.map_linux_dependency(&clean_dep).to_string());
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }

                if name.is_empty() || version.is_empty() {
                    return Err(PackageError::InstallationFailed("Malformed Debian control file".to_string()));
                }

                let mut pkg = UnifiedPackage::new(name, version).with_format(PackageFormat::Deb);
                for dep in dependencies {
                    pkg = pkg.with_dependency(dep);
                }
                Ok(pkg)
            }
            PackageFormat::Rpm => {
                // Parse RPM Spec file parameters
                let mut name = String::new();
                let mut version = String::new();
                let mut dependencies = Vec::new();

                for line in foreign_data.lines() {
                    let parts: Vec<&str> = line.splitn(2, ':').collect();
                    if parts.len() == 2 {
                        let key = parts[0].trim().to_lowercase();
                        let val = parts[1].trim();
                        match key.as_str() {
                            "name" => name = val.to_string(),
                            "version" => version = val.to_string(),
                            "requires" => {
                                for dep in val.split(',') {
                                    let clean_dep = dep.trim().split(' ').next().unwrap_or("").to_string();
                                    if !clean_dep.is_empty() {
                                        dependencies.push(self.map_linux_dependency(&clean_dep).to_string());
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }

                if name.is_empty() || version.is_empty() {
                    return Err(PackageError::InstallationFailed("Malformed RPM Spec file".to_string()));
                }

                let mut pkg = UnifiedPackage::new(name, version).with_format(PackageFormat::Rpm);
                for dep in dependencies {
                    pkg = pkg.with_dependency(dep);
                }
                Ok(pkg)
            }
            PackageFormat::Pacman => {
                // Parse Arch .PKGINFO or PKGBUILD format
                let mut name = String::new();
                let mut version = String::new();
                let mut dependencies = Vec::new();

                for line in foreign_data.lines() {
                    let parts: Vec<&str> = line.splitn(2, '=').collect();
                    if parts.len() == 2 {
                        let key = parts[0].trim().to_lowercase();
                        let val = parts[1].trim();
                        match key.as_str() {
                            "pkgname" => name = val.to_string(),
                            "pkgver" => version = val.to_string(),
                            "depend" => {
                                let clean_dep = val.split('>').next().unwrap_or("").split('<').next().unwrap_or("").trim().to_string();
                                if !clean_dep.is_empty() {
                                    dependencies.push(self.map_linux_dependency(&clean_dep).to_string());
                                }
                            }
                            _ => {}
                        }
                    }
                }

                if name.is_empty() || version.is_empty() {
                    return Err(PackageError::InstallationFailed("Malformed PKGINFO file".to_string()));
                }

                let mut pkg = UnifiedPackage::new(name, version).with_format(PackageFormat::Pacman);
                for dep in dependencies {
                    pkg = pkg.with_dependency(dep);
                }
                Ok(pkg)
            }
            PackageFormat::Apk => {
                // Parse Alpine APKINDEX meta lines
                let mut name = String::new();
                let mut version = String::new();
                let mut dependencies = Vec::new();

                for line in foreign_data.lines() {
                    if line.starts_with('P') {
                        name = line[2..].to_string();
                    } else if line.starts_with('V') {
                        version = line[2..].to_string();
                    } else if line.starts_with('D') {
                        for dep in line[2..].split(' ') {
                            let clean_dep = dep.split('<').next().unwrap_or("").split('>').next().unwrap_or("").split('=').next().unwrap_or("").trim().to_string();
                            if !clean_dep.is_empty() {
                                dependencies.push(self.map_linux_dependency(&clean_dep).to_string());
                            }
                        }
                    }
                }

                if name.is_empty() || version.is_empty() {
                    return Err(PackageError::InstallationFailed("Malformed APKINDEX entry".to_string()));
                }

                let mut pkg = UnifiedPackage::new(name, version).with_format(PackageFormat::Apk);
                for dep in dependencies {
                    pkg = pkg.with_dependency(dep);
                }
                Ok(pkg)
            }
            _ => {
                // Return default SigmaPkg unchanged
                let pkg = UnifiedPackage::new("untranslated".to_string(), "1.0.0".to_string()).with_format(PackageFormat::SigmaPkg);
                Ok(pkg)
            }
        }
    }

    pub fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "Installing {} using {} adapter",
            package.name, self.adapter_name
        );
        if package.fhs_redirect_required {
            println!(
                "Redirecting paths for {} with {} adapter to security sandbox prefix /sandbox/fhs_compat",
                package.name, self.adapter_name
            );
        }
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
        let apk_adapter = PackageAdapter::new(PackageFormat::Apk, "apk".to_string());
        let snap_adapter = PackageAdapter::new(PackageFormat::Snap, "snap".to_string());
        let flatpak_adapter = PackageAdapter::new(PackageFormat::Flatpak, "flatpak".to_string());
        let sigpkg_adapter = PackageAdapter::new(PackageFormat::SigmaPkg, "sigpkg".to_string());

        self.adapters.insert(PackageFormat::Deb, apt_adapter);
        self.adapters.insert(PackageFormat::Rpm, yum_adapter);
        self.adapters.insert(PackageFormat::Pacman, pacman_adapter);
        self.adapters.insert(PackageFormat::Apk, apk_adapter);
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
        assert_eq!(manager.adapters.len(), 7);
    }

    #[test]
    fn test_package_creation() {
        let package = UnifiedPackage::new("test".to_string(), "1.0.0".to_string())
            .with_format(PackageFormat::Deb)
            .with_dependency("dep1".to_string());
        assert_eq!(package.formats.len(), 1);
        assert_eq!(package.dependencies.len(), 1);
        assert!(package.fhs_redirect_required);
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
    fn test_fhs_path_redirection() {
        let adapter = PackageAdapter::new(PackageFormat::Deb, "apt".to_string());
        assert_eq!(adapter.redirect_fhs_path("/usr/bin/cool-app"), "/sandbox/fhs_compat/bin/cool-app");
        assert_eq!(adapter.redirect_fhs_path("/etc/config.json"), "/sandbox/fhs_compat/etc/config.json");
    }

    #[test]
    fn test_dependency_mapping() {
        let adapter = PackageAdapter::new(PackageFormat::Rpm, "yum".to_string());
        assert_eq!(adapter.map_linux_dependency("libc.so.6"), "sigma_libc");
        assert_eq!(adapter.map_linux_dependency("libssl.so.3"), "sigma_crypto");
        assert_eq!(adapter.map_linux_dependency("unrelated_lib"), "sigma_legacy_compat");
    }

    #[test]
    fn test_debian_control_translation() {
        let adapter = PackageAdapter::new(PackageFormat::Deb, "apt".to_string());
        let foreign_data = "Package: cool-utility\nVersion: 4.2.1\nDepends: libc6, libssl\n";
        let translated = adapter.translate_foreign_metadata(foreign_data).unwrap();
        assert_eq!(translated.name, "cool-utility");
        assert_eq!(translated.version, "4.2.1");
        assert_eq!(translated.dependencies, vec!["sigma_libc", "sigma_crypto"]);
    }

    #[test]
    fn test_rpm_spec_translation() {
        let adapter = PackageAdapter::new(PackageFormat::Rpm, "yum".to_string());
        let foreign_data = "Name: nice-tool\nVersion: 1.0.5\nRequires: glibc, bash\n";
        let translated = adapter.translate_foreign_metadata(foreign_data).unwrap();
        assert_eq!(translated.name, "nice-tool");
        assert_eq!(translated.version, "1.0.5");
        assert_eq!(translated.dependencies, vec!["sigma_libc", "sigma_sh"]);
    }

    #[test]
    fn test_arch_pkginfo_translation() {
        let adapter = PackageAdapter::new(PackageFormat::Pacman, "pacman".to_string());
        let foreign_data = "pkgname = arch-app\npkgver = 2.0.0\ndepend = musl>=1.2\ndepend = openssl\n";
        let translated = adapter.translate_foreign_metadata(foreign_data).unwrap();
        assert_eq!(translated.name, "arch-app");
        assert_eq!(translated.version, "2.0.0");
        assert_eq!(translated.dependencies, vec!["sigma_libc", "sigma_crypto"]);
    }

    #[test]
    fn test_alpine_apkindex_translation() {
        let adapter = PackageAdapter::new(PackageFormat::Apk, "apk".to_string());
        let foreign_data = "C:sha256:hash\nP:alpine-app\nV:1.15.2\nD:musl>=1.2 libssl3 sh\n";
        let translated = adapter.translate_foreign_metadata(foreign_data).unwrap();
        assert_eq!(translated.name, "alpine-app");
        assert_eq!(translated.version, "1.15.2");
        assert_eq!(translated.dependencies, vec!["sigma_libc", "sigma_crypto", "sigma_sh"]);
    }
}
