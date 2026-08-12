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
    Nix,      // nix expression
    Ebuild,   // gentoo ebuild
    Apk,      // alpine apk
    Txz,      // slackware pkgtool
    Xbps,     // void xbps
    Cachyos,  // CachyOS optimized format
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

/// NixAdapter handles Nix declarative packages
pub struct NixAdapter {
    pub store_path: String,
}

impl NixAdapter {
    pub fn new() -> Self {
        Self {
            store_path: "/nix/store".to_string(),
        }
    }
}

impl PackageFormatAdapter for NixAdapter {
    fn format(&self) -> PackageFormat {
        PackageFormat::Nix
    }

    fn adapter_name(&self) -> &str {
        "nix"
    }

    fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "[{}] Realizing Nix derivation. Mounting sandbox store path {} for package {}",
            self.adapter_name(),
            self.store_path,
            package.name
        );
        Ok(())
    }

    fn remove(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("[{}] Garbage collecting Nix store path for package {}", self.adapter_name(), package.name);
        Ok(())
    }

    fn update(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("[{}] Rebuilding Nix environment generation for package {}", self.adapter_name(), package.name);
        Ok(())
    }
}

/// EbuildAdapter handles Gentoo Portage ebuild formats
pub struct EbuildAdapter {
    pub portage_tree_path: String,
}

impl EbuildAdapter {
    pub fn new() -> Self {
        Self {
            portage_tree_path: "/var/db/repos/gentoo".to_string(),
        }
    }
}

impl PackageFormatAdapter for EbuildAdapter {
    fn format(&self) -> PackageFormat {
        PackageFormat::Ebuild
    }

    fn adapter_name(&self) -> &str {
        "portage"
    }

    fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "[{}] Compiling from source in portage tree {}. Installing ebuild package {}",
            self.adapter_name(),
            self.portage_tree_path,
            package.name
        );
        Ok(())
    }

    fn remove(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("[{}] Unmerging ebuild package {}", self.adapter_name(), package.name);
        Ok(())
    }

    fn update(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("[{}] Re-emerging and updating ebuild package {}", self.adapter_name(), package.name);
        Ok(())
    }
}

/// ApkAdapter handles Alpine Linux APK package formats
pub struct ApkAdapter {
    pub alpine_keys_dir: String,
}

impl ApkAdapter {
    pub fn new() -> Self {
        Self {
            alpine_keys_dir: "/etc/apk/keys".to_string(),
        }
    }
}

impl PackageFormatAdapter for ApkAdapter {
    fn format(&self) -> PackageFormat {
        PackageFormat::Apk
    }

    fn adapter_name(&self) -> &str {
        "apk"
    }

    fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "[{}] Validating Alpine keys at {}. Unpacking APK package {}",
            self.adapter_name(),
            self.alpine_keys_dir,
            package.name
        );
        Ok(())
    }

    fn remove(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("[{}] Removing APK package {}", self.adapter_name(), package.name);
        Ok(())
    }

    fn update(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("[{}] Upgrading APK package {}", self.adapter_name(), package.name);
        Ok(())
    }
}

/// TxzAdapter handles Slackware package formats
pub struct TxzAdapter {
    pub slackware_pkg_db: String,
}

impl TxzAdapter {
    pub fn new() -> Self {
        Self {
            slackware_pkg_db: "/var/log/packages".to_string(),
        }
    }
}

impl PackageFormatAdapter for TxzAdapter {
    fn format(&self) -> PackageFormat {
        PackageFormat::Txz
    }

    fn adapter_name(&self) -> &str {
        "slackware"
    }

    fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "[{}] Logging Slackware installation entry in {}. Installing txz package {}",
            self.adapter_name(),
            self.slackware_pkg_db,
            package.name
        );
        Ok(())
    }

    fn remove(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("[{}] Running Slackware removepkg for txz package {}", self.adapter_name(), package.name);
        Ok(())
    }

    fn update(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("[{}] Running Slackware upgradepkg for txz package {}", self.adapter_name(), package.name);
        Ok(())
    }
}

/// XbpsAdapter handles Void Linux XBPS packages
pub struct XbpsAdapter {
    pub void_repo_cache: String,
}

impl XbpsAdapter {
    pub fn new() -> Self {
        Self {
            void_repo_cache: "/var/cache/xbps".to_string(),
        }
    }
}

impl PackageFormatAdapter for XbpsAdapter {
    fn format(&self) -> PackageFormat {
        PackageFormat::Xbps
    }

    fn adapter_name(&self) -> &str {
        "xbps"
    }

    fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "[{}] Downloading to XBPS repo cache {}. Unpacking package {}",
            self.adapter_name(),
            self.void_repo_cache,
            package.name
        );
        Ok(())
    }

    fn remove(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("[{}] Purging Void XBPS package {}", self.adapter_name(), package.name);
        Ok(())
    }

    fn update(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("[{}] Re-syncing and upgrading Void XBPS package {}", self.adapter_name(), package.name);
        Ok(())
    }
}

/// CachyosAdapter handles CachyOS architecture-optimized packages
pub struct CachyosAdapter {
    pub microarch_optimization_level: u32,
}

impl CachyosAdapter {
    pub fn new() -> Self {
        Self {
            microarch_optimization_level: 3,
        }
    }
}

impl PackageFormatAdapter for CachyosAdapter {
    fn format(&self) -> PackageFormat {
        PackageFormat::Cachyos
    }

    fn adapter_name(&self) -> &str {
        "cachyos"
    }

    fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "[{}] Installing architecture-optimized package {} (Target optimization level: v{})",
            self.adapter_name(),
            package.name,
            self.microarch_optimization_level
        );
        Ok(())
    }

    fn remove(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("[{}] Removing optimized CachyOS package {}", self.adapter_name(), package.name);
        Ok(())
    }

    fn update(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("[{}] Upgrading optimized CachyOS package {}", self.adapter_name(), package.name);
        Ok(())
    }
}

/// User-defined extensible hook trait for package format actions
pub trait PackageHook: Send + Sync {
    fn hook_name(&self) -> &str;
    fn execute(&self, package: &UnifiedPackage) -> Result<(), PackageError>;
}

/// A standard hook implementation wrapping a closure/function
pub struct CustomPackageHook {
    pub name: String,
    pub func: Box<dyn Fn(&UnifiedPackage) -> Result<(), PackageError> + Send + Sync>,
}

impl CustomPackageHook {
    pub fn new<F>(name: &str, func: F) -> Self
    where
        F: Fn(&UnifiedPackage) -> Result<(), PackageError> + Send + Sync + 'static,
    {
        Self {
            name: name.to_string(),
            func: Box::new(func),
        }
    }
}

impl PackageHook for CustomPackageHook {
    fn hook_name(&self) -> &str {
        &self.name
    }

    fn execute(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        (self.func)(package)
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

// ----------------------------------------------------
// Local Metadata Cache
// ----------------------------------------------------

pub struct LocalMetadataCache {
    pub cache: HashMap<String, UnifiedPackage>,
}

impl LocalMetadataCache {
    pub fn new() -> Self {
        Self { cache: HashMap::new() }
    }
    pub fn insert(&mut self, name: String, package: UnifiedPackage) {
        self.cache.insert(name, package);
    }
    pub fn get(&self, name: &str) -> Option<&UnifiedPackage> {
        self.cache.get(name)
    }
}

// ----------------------------------------------------
// Universal Package Manager
// ----------------------------------------------------

#[derive(Debug, Clone)]
pub struct PackageCheckpoint {
    pub id: usize,
    pub installed_keys: std::collections::HashSet<String>,
}

#[derive(Debug, Clone)]
pub struct TransactionHistory {
    pub checkpoints: Vec<PackageCheckpoint>,
    pub next_id: usize,
}

impl TransactionHistory {
    pub fn new() -> Self {
        Self {
            checkpoints: Vec::new(),
            next_id: 1,
        }
    }

    pub fn create_checkpoint(&mut self, installed_packages: &HashMap<String, UnifiedPackage>) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        let installed_keys = installed_packages.keys().cloned().collect();
        self.checkpoints.push(PackageCheckpoint { id, installed_keys });
        id
    }

    pub fn get_checkpoint(&self, id: usize) -> Option<&PackageCheckpoint> {
        self.checkpoints.iter().find(|c| c.id == id)
    }
}

/// Universal package manager using dynamic dispatch to modularly handle various package format adapters
pub struct UniversalPackageManager {
    pub packages: HashMap<String, UnifiedPackage>,
    pub adapters: HashMap<PackageFormat, Box<dyn PackageFormatAdapter>>,
    pub resolver: DependencyResolver,
    pub installed_packages: HashMap<String, UnifiedPackage>,
    pub metadata_cache: LocalMetadataCache,
    pub transaction_history: TransactionHistory,
    pub pre_install_hooks: Vec<std::sync::Arc<dyn PackageHook>>,
    pub post_install_hooks: Vec<std::sync::Arc<dyn PackageHook>>,
    pub pre_remove_hooks: Vec<std::sync::Arc<dyn PackageHook>>,
    pub post_remove_hooks: Vec<std::sync::Arc<dyn PackageHook>>,
}

impl UniversalPackageManager {
    pub fn new() -> Self {
        let mut manager = Self {
            packages: HashMap::new(),
            adapters: HashMap::new(),
            resolver: DependencyResolver::new(),
            installed_packages: HashMap::new(),
            metadata_cache: LocalMetadataCache::new(),
            transaction_history: TransactionHistory::new(),
            pre_install_hooks: Vec::new(),
            post_install_hooks: Vec::new(),
            pre_remove_hooks: Vec::new(),
            post_remove_hooks: Vec::new(),
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
        self.adapters.insert(PackageFormat::Nix, Box::new(NixAdapter::new()));
        self.adapters.insert(PackageFormat::Ebuild, Box::new(EbuildAdapter::new()));
        self.adapters.insert(PackageFormat::Apk, Box::new(ApkAdapter::new()));
        self.adapters.insert(PackageFormat::Txz, Box::new(TxzAdapter::new()));
        self.adapters.insert(PackageFormat::Xbps, Box::new(XbpsAdapter::new()));
        self.adapters.insert(PackageFormat::Cachyos, Box::new(CachyosAdapter::new()));
    }

    /// Dynamic polymorphic registration of custom format adapters
    pub fn register_adapter(&mut self, format: PackageFormat, adapter: Box<dyn PackageFormatAdapter>) {
        self.adapters.insert(format, adapter);
    }

    pub fn add_pre_install_hook(&mut self, hook: std::sync::Arc<dyn PackageHook>) {
        self.pre_install_hooks.push(hook);
    }

    pub fn add_post_install_hook(&mut self, hook: std::sync::Arc<dyn PackageHook>) {
        self.post_install_hooks.push(hook);
    }

    pub fn add_pre_remove_hook(&mut self, hook: std::sync::Arc<dyn PackageHook>) {
        self.pre_remove_hooks.push(hook);
    }

    pub fn add_post_remove_hook(&mut self, hook: std::sync::Arc<dyn PackageHook>) {
        self.post_remove_hooks.push(hook);
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

                // Execute pre-install hooks
                for hook in &self.pre_install_hooks {
                    if let Err(e) = hook.execute(package) {
                        self.rollback_transaction(&installed_in_this_transaction);
                        return Err(e);
                    }
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
                self.installed_packages.insert(dep_name.clone(), installed.clone());
                installed_in_this_transaction.push(dep_name);

                // Execute post-install hooks
                for hook in &self.post_install_hooks {
                    if let Err(e) = hook.execute(&installed) {
                        self.rollback_transaction(&installed_in_this_transaction);
                        return Err(e);
                    }
                }
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
        if let Some(package) = self.installed_packages.get(package_name).cloned() {
            // Execute pre-remove hooks
            for hook in &self.pre_remove_hooks {
                hook.execute(&package)?;
            }

            let mut removed_by_adapter = false;
            for format in &package.formats {
                if let Some(adapter) = self.adapters.get(format) {
                    adapter.remove(&package)?;
                    removed_by_adapter = true;
                    break;
                }
            }
            if !removed_by_adapter {
                return Err(PackageError::AdapterNotFound);
            }
            self.installed_packages.remove(package_name);

            // Execute post-remove hooks
            for hook in &self.post_remove_hooks {
                hook.execute(&package)?;
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_creation() {
        let manager = UniversalPackageManager::new();
        assert_eq!(manager.adapters.len(), 12);
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
    fn test_apt_deb_adapter_flow() {
        let adapter = AptDebAdapter::new();
        assert_eq!(adapter.format(), PackageFormat::Deb);
        assert_eq!(adapter.adapter_name(), "apt");

        let package = UnifiedPackage::new("curl".to_string(), "7.81.0".to_string())
            .with_format(PackageFormat::Deb);

        assert!(adapter.can_handle(&package));
        assert!(adapter.install(&package).is_ok());
        assert!(adapter.update(&package).is_ok());
        assert!(adapter.remove(&package).is_ok());
    }

    #[test]
    fn test_yum_rpm_adapter_flow() {
        let adapter = YumRpmAdapter::new();
        assert_eq!(adapter.format(), PackageFormat::Rpm);
        assert_eq!(adapter.adapter_name(), "yum");

        let package = UnifiedPackage::new("nginx".to_string(), "1.20.1".to_string())
            .with_format(PackageFormat::Rpm);

        assert!(adapter.can_handle(&package));
        assert!(adapter.install(&package).is_ok());
        assert!(adapter.update(&package).is_ok());
        assert!(adapter.remove(&package).is_ok());
    }

    struct MockCustomAdapter;
    impl PackageFormatAdapter for MockCustomAdapter {
        fn format(&self) -> PackageFormat {
            PackageFormat::Deb
        }
        fn adapter_name(&self) -> &str {
            "custom-mock"
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

    #[test]
    fn test_universal_manager_polymorphism() {
        let mut manager = UniversalPackageManager::new();
        // Dynamic registration under Open-Closed/Polymorphism OOP principles
        manager.register_adapter(PackageFormat::Deb, Box::new(MockCustomAdapter));

        let package = UnifiedPackage::new("custom-app".to_string(), "1.0.0".to_string())
            .with_format(PackageFormat::Deb);

        manager.add_package(package);
        assert!(manager.install("custom-app").is_ok());
        assert_eq!(manager.installed_packages.len(), 1);
    }

    #[test]
    fn test_version_constraint_resolution() {
        let mut resolver = DependencyResolver::new();

        // Valid setup
        let lib_pkg = UnifiedPackage::new("lib-helper".to_string(), "1.2.3".to_string())
            .with_format(PackageFormat::SigmaPkg);
        let app_pkg = UnifiedPackage::new("my-app".to_string(), "1.0.0".to_string())
            .with_format(PackageFormat::SigmaPkg)
            .with_dependency("lib-helper>=1.1.0".to_string());

        resolver.add_package(lib_pkg);
        resolver.add_package(app_pkg);

        // This should pass since 1.2.3 matches >=1.1.0
        let deps = resolver.resolve_dependencies("my-app").unwrap();
        assert_eq!(deps.len(), 2);

        // Invalid version setup (fails constraint check)
        let mut resolver_err = DependencyResolver::new();
        let lib_old = UnifiedPackage::new("lib-helper".to_string(), "1.0.5".to_string())
            .with_format(PackageFormat::SigmaPkg);
        let app_pkg2 = UnifiedPackage::new("my-app".to_string(), "1.0.0".to_string())
            .with_format(PackageFormat::SigmaPkg)
            .with_dependency("lib-helper>=1.1.0".to_string());

        resolver_err.add_package(lib_old);
        resolver_err.add_package(app_pkg2);

        let err = resolver_err.resolve_dependencies("my-app").unwrap_err();
        assert!(matches!(err, PackageError::VersionMismatch(_, _, _)));
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
            Err(PackageError::InstallationFailed("Simulated crash".to_string()))
        }
        fn remove(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
            Ok(())
        }
        fn update(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
            Ok(())
        }
    }

    #[test]
    fn test_batch_transaction_atomic_rollback() {
        let mut manager = UniversalPackageManager::new();
        manager.register_adapter(PackageFormat::SigmaPkg, Box::new(FailingAdapter));

        let package = UnifiedPackage::new("my-app".to_string(), "1.0.0".to_string())
            .with_format(PackageFormat::SigmaPkg);

        manager.add_package(package);

        // Since it's FailingAdapter, installation will fail and trigger transaction rollback
        let result = manager.install("my-app");
        assert!(result.is_err());
        // Verify installed count is 0
        assert_eq!(manager.list_installed().len(), 0);
    }

    #[test]
    fn test_new_distro_adapters_flow() {
        let manager = UniversalPackageManager::new();

        // 1. Nix
        let nix_pkg = UnifiedPackage::new("hello-nix".to_string(), "2.10.0".to_string())
            .with_format(PackageFormat::Nix);
        let nix_adapter = manager.adapters.get(&PackageFormat::Nix).unwrap();
        assert_eq!(nix_adapter.format(), PackageFormat::Nix);
        assert_eq!(nix_adapter.adapter_name(), "nix");
        assert!(nix_adapter.can_handle(&nix_pkg));
        assert!(nix_adapter.install(&nix_pkg).is_ok());
        assert!(nix_adapter.update(&nix_pkg).is_ok());
        assert!(nix_adapter.remove(&nix_pkg).is_ok());

        // 2. Portage/Ebuild
        let ebuild_pkg = UnifiedPackage::new("gentoo-sources".to_string(), "6.1.0".to_string())
            .with_format(PackageFormat::Ebuild);
        let ebuild_adapter = manager.adapters.get(&PackageFormat::Ebuild).unwrap();
        assert_eq!(ebuild_adapter.format(), PackageFormat::Ebuild);
        assert_eq!(ebuild_adapter.adapter_name(), "portage");
        assert!(ebuild_adapter.can_handle(&ebuild_pkg));
        assert!(ebuild_adapter.install(&ebuild_pkg).is_ok());

        // 3. APK
        let apk_pkg = UnifiedPackage::new("alpine-baselayout".to_string(), "3.4.0".to_string())
            .with_format(PackageFormat::Apk);
        let apk_adapter = manager.adapters.get(&PackageFormat::Apk).unwrap();
        assert_eq!(apk_adapter.format(), PackageFormat::Apk);
        assert_eq!(apk_adapter.adapter_name(), "apk");
        assert!(apk_adapter.can_handle(&apk_pkg));
        assert!(apk_adapter.install(&apk_pkg).is_ok());

        // 4. Txz/Slackware
        let txz_pkg = UnifiedPackage::new("slack-base".to_string(), "15.0.0".to_string())
            .with_format(PackageFormat::Txz);
        let txz_adapter = manager.adapters.get(&PackageFormat::Txz).unwrap();
        assert_eq!(txz_adapter.format(), PackageFormat::Txz);
        assert_eq!(txz_adapter.adapter_name(), "slackware");
        assert!(txz_adapter.can_handle(&txz_pkg));
        assert!(txz_adapter.install(&txz_pkg).is_ok());

        // 5. XBPS
        let xbps_pkg = UnifiedPackage::new("void-runit".to_string(), "2.12.0".to_string())
            .with_format(PackageFormat::Xbps);
        let xbps_adapter = manager.adapters.get(&PackageFormat::Xbps).unwrap();
        assert_eq!(xbps_adapter.format(), PackageFormat::Xbps);
        assert_eq!(xbps_adapter.adapter_name(), "xbps");
        assert!(xbps_adapter.can_handle(&xbps_pkg));
        assert!(xbps_adapter.install(&xbps_pkg).is_ok());

        // 6. CachyOS
        let cachy_pkg = UnifiedPackage::new("linux-cachyos".to_string(), "6.4.2".to_string())
            .with_format(PackageFormat::Cachyos);
        let cachy_adapter = manager.adapters.get(&PackageFormat::Cachyos).unwrap();
        assert_eq!(cachy_adapter.format(), PackageFormat::Cachyos);
        assert_eq!(cachy_adapter.adapter_name(), "cachyos");
        assert!(cachy_adapter.can_handle(&cachy_pkg));
        assert!(cachy_adapter.install(&cachy_pkg).is_ok());
    }

    #[test]
    fn test_user_defined_hooks_execution() {
        let mut manager = UniversalPackageManager::new();

        use std::sync::atomic::{AtomicU32, Ordering};
        use std::sync::Arc;

        let pre_install_count = Arc::new(AtomicU32::new(0));
        let post_install_count = Arc::new(AtomicU32::new(0));
        let pre_remove_count = Arc::new(AtomicU32::new(0));
        let post_remove_count = Arc::new(AtomicU32::new(0));

        let pre_i = pre_install_count.clone();
        manager.add_pre_install_hook(Arc::new(CustomPackageHook::new("pre-install-tracker", move |pkg| {
            pre_i.fetch_add(1, Ordering::SeqCst);
            assert_eq!(pkg.name, "hooked-app");
            Ok(())
        })));

        let post_i = post_install_count.clone();
        manager.add_post_install_hook(Arc::new(CustomPackageHook::new("post-install-tracker", move |pkg| {
            post_i.fetch_add(1, Ordering::SeqCst);
            assert_eq!(pkg.name, "hooked-app");
            Ok(())
        })));

        let pre_r = pre_remove_count.clone();
        manager.add_pre_remove_hook(Arc::new(CustomPackageHook::new("pre-remove-tracker", move |pkg| {
            pre_r.fetch_add(1, Ordering::SeqCst);
            assert_eq!(pkg.name, "hooked-app");
            Ok(())
        })));

        let post_r = post_remove_count.clone();
        manager.add_post_remove_hook(Arc::new(CustomPackageHook::new("post-remove-tracker", move |pkg| {
            post_r.fetch_add(1, Ordering::SeqCst);
            assert_eq!(pkg.name, "hooked-app");
            Ok(())
        })));

        let package = UnifiedPackage::new("hooked-app".to_string(), "1.0.0".to_string())
            .with_format(PackageFormat::SigmaPkg);

        manager.add_package(package);

        assert_eq!(pre_install_count.load(Ordering::SeqCst), 0);
        assert_eq!(post_install_count.load(Ordering::SeqCst), 0);

        assert!(manager.install("hooked-app").is_ok());

        assert_eq!(pre_install_count.load(Ordering::SeqCst), 1);
        assert_eq!(post_install_count.load(Ordering::SeqCst), 1);
        assert_eq!(pre_remove_count.load(Ordering::SeqCst), 0);
        assert_eq!(post_remove_count.load(Ordering::SeqCst), 0);

        assert!(manager.remove("hooked-app").is_ok());

        assert_eq!(pre_remove_count.load(Ordering::SeqCst), 1);
        assert_eq!(post_remove_count.load(Ordering::SeqCst), 1);
    }
}
