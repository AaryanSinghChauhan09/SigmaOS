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

/// NixAdapter handles declarative Nix expressions
pub struct NixAdapter {
    pub store_dir: String,
}

impl NixAdapter {
    pub fn new() -> Self {
        Self {
            store_dir: "/nix/store".to_string(),
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
            "[{}] Realizing Nix derivation for {} into {}",
            self.adapter_name(),
            package.name,
            self.store_dir
        );
        Ok(())
    }

    fn remove(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("[{}] Garbage collecting Nix package {}", self.adapter_name(), package.name);
        Ok(())
    }

    fn update(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("[{}] Rebuilding Nix environment for {}", self.adapter_name(), package.name);
        Ok(())
    }
}

/// GentooEbuildAdapter handles Gentoo Portage ebuild compilation
pub struct GentooEbuildAdapter {
    pub portage_dir: String,
}

impl GentooEbuildAdapter {
    pub fn new() -> Self {
        Self {
            portage_dir: "/var/db/repos/gentoo".to_string(),
        }
    }
}

impl PackageFormatAdapter for GentooEbuildAdapter {
    fn format(&self) -> PackageFormat {
        PackageFormat::Ebuild
    }

    fn adapter_name(&self) -> &str {
        "ebuild"
    }

    fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "[{}] Compiling and merging ebuild {} from source repo {}",
            self.adapter_name(),
            package.name,
            self.portage_dir
        );
        Ok(())
    }

    fn remove(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("[{}] Unmerging Gentoo package {}", self.adapter_name(), package.name);
        Ok(())
    }

    fn update(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("[{}] Re-emerging and compiling Gentoo package {}", self.adapter_name(), package.name);
        Ok(())
    }
}

/// AlpineApkAdapter handles Alpine Linux apk package manager
pub struct AlpineApkAdapter {
    pub apk_db_path: String,
}

impl AlpineApkAdapter {
    pub fn new() -> Self {
        Self {
            apk_db_path: "/lib/apk/db/installed".to_string(),
        }
    }
}

impl PackageFormatAdapter for AlpineApkAdapter {
    fn format(&self) -> PackageFormat {
        PackageFormat::Apk
    }

    fn adapter_name(&self) -> &str {
        "apk"
    }

    fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "[{}] Querying apk db at {}. Adding APK package {}",
            self.adapter_name(),
            self.apk_db_path,
            package.name
        );
        Ok(())
    }

    fn remove(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("[{}] Deleting APK package {}", self.adapter_name(), package.name);
        Ok(())
    }

    fn update(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("[{}] Upgrading APK package {}", self.adapter_name(), package.name);
        Ok(())
    }
}

/// SlackwareTxzAdapter handles Slackware pkgtool package manager (`.txz`)
pub struct SlackwareTxzAdapter {
    pub pkg_db_dir: String,
}

impl SlackwareTxzAdapter {
    pub fn new() -> Self {
        Self {
            pkg_db_dir: "/var/log/packages".to_string(),
        }
    }
}

impl PackageFormatAdapter for SlackwareTxzAdapter {
    fn format(&self) -> PackageFormat {
        PackageFormat::Txz
    }

    fn adapter_name(&self) -> &str {
        "pkgtool"
    }

    fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "[{}] Registering installation log at {}. Installing Slackware TXZ package {}",
            self.adapter_name(),
            self.pkg_db_dir,
            package.name
        );
        Ok(())
    }

    fn remove(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("[{}] Removing Slackware package {} via removepkg", self.adapter_name(), package.name);
        Ok(())
    }

    fn update(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("[{}] Upgrading Slackware package {} via upgradepkg", self.adapter_name(), package.name);
        Ok(())
    }
}

/// VoidXbpsAdapter handles Void Linux XBPS package manager
pub struct VoidXbpsAdapter {
    pub xbps_db_dir: String,
}

impl VoidXbpsAdapter {
    pub fn new() -> Self {
        Self {
            xbps_db_dir: "/var/db/xbps".to_string(),
        }
    }
}

impl PackageFormatAdapter for VoidXbpsAdapter {
    fn format(&self) -> PackageFormat {
        PackageFormat::Xbps
    }

    fn adapter_name(&self) -> &str {
        "xbps"
    }

    fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "[{}] Updating XBPS state in {}. Installing package {}",
            self.adapter_name(),
            self.xbps_db_dir,
            package.name
        );
        Ok(())
    }

    fn remove(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("[{}] Removing XBPS package {}", self.adapter_name(), package.name);
        Ok(())
    }

    fn update(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("[{}] Updating XBPS package {}", self.adapter_name(), package.name);
        Ok(())
    }
}

/// CachyosAdapter handles microarchitecture-optimized CachyOS pacman repository formats
pub struct CachyosAdapter {
    pub arch_v_level: String,
}

impl CachyosAdapter {
    pub fn new() -> Self {
        Self {
            arch_v_level: "x86_64-v3".to_string(),
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
            "[{}] Target microarchitecture optimization: {}. Installing cachyos package {}",
            self.adapter_name(),
            self.arch_v_level,
            package.name
        );
        Ok(())
    }

    fn remove(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("[{}] Removing cachyos package {}", self.adapter_name(), package.name);
        Ok(())
    }

    fn update(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("[{}] Refreshing and upgrading CachyOS package {}", self.adapter_name(), package.name);
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
        // Optimize: scan string once for first comparison operator boundary rather than 5 separate finds.
        if let Some(idx) = dep_str.find(|c| c == '>' || c == '<' || c == '=') {
            let name = dep_str[..idx].trim().to_string();
            let constraint_str = &dep_str[idx..];
            let constraint = SemVerConstraint::parse(constraint_str);
            (name, constraint)
        } else {
            (dep_str.trim().to_string(), SemVerConstraint::Any)
        }
    }

    pub fn resolve_dependencies(&self, package_name: &str) -> Result<Vec<String>, PackageError> {
        let mut resolved = Vec::new();
        // Optimize: keep parsed name and constraints in the stack to avoid redundant parsing operations on pop.
        let mut to_visit = vec![(package_name.to_string(), SemVerConstraint::Any)];
        let mut visited = std::collections::HashSet::new();

        while let Some((name, constraint)) = to_visit.pop() {
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
                    let (dep_name, dep_constraint) = Self::parse_dependency(dep);
                    if !visited.contains(&dep_name) {
                        to_visit.push((dep_name, dep_constraint));
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

pub type UserHook = std::sync::Arc<dyn Fn(&mut UnifiedPackage) -> Result<(), PackageError> + Send + Sync>;

#[derive(Debug, Clone)]
pub struct Checkpoint {
    pub id: usize,
    pub installed_keys: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TransactionHistory {
    pub checkpoints: Vec<Checkpoint>,
    pub next_id: usize,
}

impl TransactionHistory {
    pub fn new() -> Self {
        Self {
            checkpoints: Vec::new(),
            next_id: 1,
        }
    }

    pub fn create_checkpoint(&mut self, installed: &HashMap<String, UnifiedPackage>) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        let keys = installed.keys().cloned().collect();
        self.checkpoints.push(Checkpoint { id, installed_keys: keys });
        id
    }

    pub fn get_checkpoint(&self, id: usize) -> Option<&Checkpoint> {
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
    pub global_hooks: Vec<UserHook>,
    pub transaction_history: TransactionHistory,
}

impl UniversalPackageManager {
    pub fn new() -> Self {
        let mut manager = Self {
            packages: HashMap::new(),
            adapters: HashMap::new(),
            resolver: DependencyResolver::new(),
            installed_packages: HashMap::new(),
            metadata_cache: LocalMetadataCache::new(),
            global_hooks: Vec::new(),
            transaction_history: TransactionHistory::new(),
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
        self.adapters.insert(PackageFormat::Ebuild, Box::new(GentooEbuildAdapter::new()));
        self.adapters.insert(PackageFormat::Apk, Box::new(AlpineApkAdapter::new()));
        self.adapters.insert(PackageFormat::Txz, Box::new(SlackwareTxzAdapter::new()));
        self.adapters.insert(PackageFormat::Xbps, Box::new(VoidXbpsAdapter::new()));
        self.adapters.insert(PackageFormat::Cachyos, Box::new(CachyosAdapter::new()));
    }

    /// Dynamic polymorphic registration of custom format adapters
    pub fn register_adapter(&mut self, format: PackageFormat, adapter: Box<dyn PackageFormatAdapter>) {
        self.adapters.insert(format, adapter);
    }

    /// Add a manager-level global user-defined verification/hook function (UDF)
    pub fn add_global_hook<F>(&mut self, hook: F)
    where
        F: Fn(&mut UnifiedPackage) -> Result<(), PackageError> + Send + Sync + 'static,
    {
        self.global_hooks.push(std::sync::Arc::new(hook));
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
            if let Some(package_ref) = self.packages.get(&dep_name) {
                let mut package = package_ref.clone();

                // Run global user-defined verification/hook functions (UDF)
                for hook in &self.global_hooks {
                    if let Err(e) = hook(&mut package) {
                        self.rollback_transaction(&installed_in_this_transaction);
                        return Err(e);
                    }
                }

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
                        match adapter.install(&package) {
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

                let mut installed = package;
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
    fn test_nix_adapter_flow() {
        let adapter = NixAdapter::new();
        assert_eq!(adapter.format(), PackageFormat::Nix);
        assert_eq!(adapter.adapter_name(), "nix");

        let package = UnifiedPackage::new("hello-nix".to_string(), "2.12.0".to_string())
            .with_format(PackageFormat::Nix);

        assert!(adapter.can_handle(&package));
        assert!(adapter.install(&package).is_ok());
        assert!(adapter.update(&package).is_ok());
        assert!(adapter.remove(&package).is_ok());
    }

    #[test]
    fn test_gentoo_ebuild_adapter_flow() {
        let adapter = GentooEbuildAdapter::new();
        assert_eq!(adapter.format(), PackageFormat::Ebuild);
        assert_eq!(adapter.adapter_name(), "ebuild");

        let package = UnifiedPackage::new("sys-apps/util-linux".to_string(), "2.39.0".to_string())
            .with_format(PackageFormat::Ebuild);

        assert!(adapter.can_handle(&package));
        assert!(adapter.install(&package).is_ok());
        assert!(adapter.update(&package).is_ok());
        assert!(adapter.remove(&package).is_ok());
    }

    #[test]
    fn test_alpine_apk_adapter_flow() {
        let adapter = AlpineApkAdapter::new();
        assert_eq!(adapter.format(), PackageFormat::Apk);
        assert_eq!(adapter.adapter_name(), "apk");

        let package = UnifiedPackage::new("musl".to_string(), "1.2.4".to_string())
            .with_format(PackageFormat::Apk);

        assert!(adapter.can_handle(&package));
        assert!(adapter.install(&package).is_ok());
        assert!(adapter.update(&package).is_ok());
        assert!(adapter.remove(&package).is_ok());
    }

    #[test]
    fn test_slackware_txz_adapter_flow() {
        let adapter = SlackwareTxzAdapter::new();
        assert_eq!(adapter.format(), PackageFormat::Txz);
        assert_eq!(adapter.adapter_name(), "pkgtool");

        let package = UnifiedPackage::new("slack-app".to_string(), "15.0.0".to_string())
            .with_format(PackageFormat::Txz);

        assert!(adapter.can_handle(&package));
        assert!(adapter.install(&package).is_ok());
        assert!(adapter.update(&package).is_ok());
        assert!(adapter.remove(&package).is_ok());
    }

    #[test]
    fn test_void_xbps_adapter_flow() {
        let adapter = VoidXbpsAdapter::new();
        assert_eq!(adapter.format(), PackageFormat::Xbps);
        assert_eq!(adapter.adapter_name(), "xbps");

        let package = UnifiedPackage::new("void-runit".to_string(), "2.1.2".to_string())
            .with_format(PackageFormat::Xbps);

        assert!(adapter.can_handle(&package));
        assert!(adapter.install(&package).is_ok());
        assert!(adapter.update(&package).is_ok());
        assert!(adapter.remove(&package).is_ok());
    }

    #[test]
    fn test_cachyos_adapter_flow() {
        let adapter = CachyosAdapter::new();
        assert_eq!(adapter.format(), PackageFormat::Cachyos);
        assert_eq!(adapter.adapter_name(), "cachyos");

        let package = UnifiedPackage::new("cachy-kernel".to_string(), "6.8.0".to_string())
            .with_format(PackageFormat::Cachyos);

        assert!(adapter.can_handle(&package));
        assert!(adapter.install(&package).is_ok());
        assert!(adapter.update(&package).is_ok());
        assert!(adapter.remove(&package).is_ok());
    }

    #[test]
    fn test_udf_global_hooks() {
        let mut manager = UniversalPackageManager::new();

        // Add a global verification hook UDF
        manager.add_global_hook(|package: &mut UnifiedPackage| -> Result<(), PackageError> {
            if package.name == "malicious-package" {
                return Err(PackageError::InstallationFailed("Blocked malicious package via UDF".to_string()));
            }
            package.checksum = "UDF_VERIFIED_CHECKSUM_TOKEN_99".to_string();
            Ok(())
        });

        let package = UnifiedPackage::new("safe-pkg".to_string(), "1.0.0".to_string())
            .with_format(PackageFormat::SigmaPkg);
        manager.add_package(package);

        // Installing safe-pkg should trigger UDF hook and set custom verified checksum
        assert!(manager.install("safe-pkg").is_ok());
        let installed = manager.installed_packages.get("safe-pkg").unwrap();
        assert_eq!(installed.checksum, "UDF_VERIFIED_CHECKSUM_TOKEN_99");

        // Installing malicious-package should be blocked by the global hook
        let malicious = UnifiedPackage::new("malicious-package".to_string(), "1.0.0".to_string())
            .with_format(PackageFormat::SigmaPkg);
        manager.add_package(malicious);
        assert!(manager.install("malicious-package").is_err());
    }
}
