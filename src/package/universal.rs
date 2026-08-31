// SigmaOS Universal Package Manager
// Unified system absorbing apt, yum, pacman, snap, flatpak, zypper, dnf, appimages

#[cfg(not(test))]
use crate::klib::HashMap;
use crate::runtime::node_distribution::{
    LibcFlavor, NodeBinaryDistroEngine, NodeBinaryPackage, NodeReleaseStream, NodeTargetArch,
};

#[cfg(test)]
use crate::klib::HashMap;
use crate::runtime::node_distribution::{
    LibcFlavor, NodeBinaryDistroEngine, NodeBinaryPackage, NodeReleaseStream, NodeTargetArch,
};

/// Package format type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackagePriority {
    Essential,
    Required,
    Important,
    Standard,
    Optional,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackageFormat {
    Deb,      // apt/dpkg
    Rpm,      // yum/dnf/zypper
    Pacman,   // pacman/pkgbuild
    Snap,     // snap/squashfs
    Flatpak,  // flatpak sandbox
    AppImage, // AppImage single-file container
    SigmaPkg, // native SigmaOS format
    Air,      // Adobe AIR (.air)
    Bottle,   // Homebrew Bottle (.bottle)
    Ipa,      // iOS App (.ipa)
    Ports,    // BSD Ports (.ports)
    Pkg,      // macOS / BSD / Solaris PKG (.pkg)
    Aab,      // Android App Bundle (.aab)
    Apk,      // Android Package / Alpine Package (.apk)
    Eopkg,    // Solus eopkg (.eopkg)
    Nixpkg,   // Nix store package (.nixpkg)
    Ebuild,   // Gentoo ebuild (.ebuild / .portage)
    TarGz,    // Compressed Tar (.tar.gz, .tgz)
    Xz,       // Compressed XZ archive (.xz, .tar.xz)
    App,      // macOS App bundle (.app)
    Hap,      // HarmonyOS Ability Package (.hap)
    Pisi,     // Pardus / Solus PiSi (.PiSi)
    Superdeb, // Deepin Superdeb (.superdeb)
    Lzm,      // Slax Linux Module (.lzm)
    Pup,      // Puppy Linux Package (.pup)
    Pet,      // Puppy Extra Tarball (.pet)
    Tar,      // Plain tarball (.tar)
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
#[derive(Debug, Clone, PartialEq, Eq)]
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

// =========================================================================
// Advanced Packaging Format Manifest Subsystems
// =========================================================================

/// Description of Debian / APT Control Manifest (.deb / dpkg parity)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AptDebManifest {
    pub package: String,
    pub version: String,
    pub architecture: String,
    pub maintainer: String,
    pub depends: Vec<String>,
    pub description: String,
}

/// Description of Arch Linux PKGBUILD Manifest (pacman parity)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacmanPkgbuild {
    pub pkgname: String,
    pub pkgver: String,
    pub pkgdesc: String,
    pub arch: Vec<String>,
    pub depends: Vec<String>,
    pub makedepends: Vec<String>,
    pub source_urls: Vec<String>,
}

/// Description of Snapcraft YAML Manifest (Ubuntu Snap squashfs parity)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapcraftManifest {
    pub name: String,
    pub version: String,
    pub summary: String,
    pub confinement: String, // e.g. "strict", "classic", "devmode"
    pub plugs: Vec<String>,
    pub slots: Vec<String>,
}

/// Description of Flatpak Metadata Manifest (Flatpak Sandboxed Sandbox parity)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlatpakManifest {
    pub id: String,
    pub runtime: String,
    pub runtime_version: String,
    pub sdk: String,
    pub command: String,
    pub finish_args: Vec<String>, // Sandboxing constraints e.g. "--share=network", "--filesystem=host"
}

/// APT Repository Source Configuration (sources.list / apt-get parity)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AptRepoConfig {
    pub sourcelist_url: String,
    pub suite: String,
    pub components: Vec<String>,
    pub trust_anchor: Option<String>,
}

/// DNF Repository Configuration (.repo files / dnf/yum parity)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnfRepoConfig {
    pub repoid: String,
    pub baseurl: String,
    pub gpgcheck: bool,
    pub enabled: bool,
}

/// AppImage Single-File Executable Container metadata (AppImage runtime parity)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppImageRuntime {
    pub app_name: String,
    pub signature_offset: u64,
    pub squashfs_offset: u64,
    pub embedded_icon_path: String,
}

/// Universal Package Adapter representing modern Linux distros packaging formats
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
        Ok(())
    }

    fn remove(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "[{}] Purging DEB package {}",
            self.adapter_name, package.name
        );
        Ok(())
    }

    fn update(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "[{}] Refreshing and updating DEB package {}",
            self.adapter_name, package.name
        );
        Ok(())
    }

    /// Dynamically parses and enforces Flatpak/Snap sandboxing policy constraints onto SigmaOS sandboxes
    pub fn translate_flatpak_sandbox_policy(&self, manifest: &FlatpakManifest) -> Vec<String> {
        let mut enforced_pledges = Vec::new();
        for arg in &manifest.finish_args {
            if arg.contains("--share=network") {
                enforced_pledges.push(String::from("network"));
            } else if arg.contains("--share=ipc") {
                enforced_pledges.push(String::from("ipc"));
            } else if arg.contains("--filesystem=host") {
                enforced_pledges.push(String::from("unveil_all"));
            }
        }
        enforced_pledges
    }

    /// Translates Snap squashfs confinement settings to native capability restrictions
    pub fn translate_snap_confinement(&self, manifest: &SnapcraftManifest) -> &'static str {
        match manifest.confinement.as_str() {
            "strict" => "strict_pledge_sandbox",
            "classic" => "unrestricted_legacy",
            _ => "devmode_permissive",
        }
    }

    /// Simulates mounting the AppImage's internal squashfs payload region
    pub fn mount_appimage_squashfs(
        &self,
        appimage: &AppImageRuntime,
    ) -> Result<String, PackageError> {
        if appimage.squashfs_offset == 0 {
            return Err(PackageError::InstallationFailed(String::from(
                "Invalid squashfs offset inside AppImage payload",
            )));
        }
        Ok(format!("/tmp/.mount_{}_squashfs", appimage.app_name))
    }

    /// Simulates querying APT repository sources
    pub fn query_apt_repository(&self, config: &AptRepoConfig) -> bool {
        config.enabled_components().len() > 0 && !config.sourcelist_url.is_empty()
    }

    /// Simulates querying DNF repository sources
    pub fn query_dnf_repository(&self, config: &DnfRepoConfig) -> bool {
        config.enabled && !config.baseurl.is_empty()
    }
}

pub trait PackageFormatAdapter {
    fn format(&self) -> PackageFormat;
    fn adapter_name(&self) -> &str {
        "unknown"
    }
    fn parse_manifest(&self, _raw_data: &[u8]) -> Result<UnifiedPackage, &'static str> {
        Err("Not implemented")
    }
    fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
    fn remove(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
    fn update(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
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

        let mut pkg =
            UnifiedPackage::new(name.clone(), version.clone()).with_format(PackageFormat::Deb);
        for dep in dependencies {
            pkg = pkg.with_dependency(dep);
        }
        Ok(pkg)
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
            "Installing {} using {} adapter",
            package.name,
            self.adapter_name()
        );
        Ok(())
    }

    fn remove(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "Removing {} using {} adapter",
            package.name,
            self.adapter_name()
        );
        Ok(())
    }

    fn update(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "Updating {} using {} adapter",
            package.name,
            self.adapter_name()
        );
        Ok(())
    }
}

impl AptRepoConfig {
    pub fn enabled_components(&self) -> &[String] {
        &self.components
    }
}

// =========================================================================
// Existing Package management & dependency resolver
// =========================================================================

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

    pub fn resolve_dependencies(
        &self,
        package_name: &str,
    ) -> Result<std::vec::Vec<String>, PackageError> {
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageCheckpoint {
    pub checkpoint_id: usize,
    pub installed_keys: Vec<String>,
}

/// Transactional history tracker for SigmaPkg/UniversalPackageManager rollbacks
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionalHistory {
    pub checkpoints: Vec<PackageCheckpoint>,
    pub next_checkpoint_id: usize,
}

impl TransactionalHistory {
    pub fn new() -> Self {
        Self {
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
    pub user_hooks: Vec<alloc::sync::Arc<dyn PackageHook>>,
    pub node_distro_engine: NodeBinaryDistroEngine,
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
            user_hooks: Vec::new(),
            node_distro_engine: NodeBinaryDistroEngine::new(),
        };

        manager.add_default_adapters();
        manager
    }

    /// Register and install a Node.js binary distribution runtime into the isolated store
    pub fn install_node_runtime(
        &mut self,
        package: &NodeBinaryPackage,
        bytes: &[u8],
        npm_version: &str,
    ) -> Result<String, PackageError> {
        let store_path = self
            .node_distro_engine
            .install_to_store(package, bytes, npm_version)
            .map_err(|e| PackageError::InstallationFailed(e.to_string()))?;

        let mut pkg = UnifiedPackage::new(format!("nodejs-{}", package.version), package.version.clone())
            .with_format(PackageFormat::SigmaPkg)
            .with_provides("nodejs".to_string());
        pkg.installed = true;

        self.installed_packages
            .insert(format!("nodejs-{}", package.version), pkg);
        Ok(store_path)
    }

    /// Registers a user-defined lifecycle hook
    pub fn add_user_hook(&mut self, hook: alloc::sync::Arc<dyn PackageHook>) {
        self.user_hooks.push(hook);

    /// Triggers user-defined hooks matching the requested lifecycle stage
    pub fn trigger_user_hooks(&self, timing: HookTiming, package: &UnifiedPackage) -> Result<(), PackageError> {
        for hook in &self.user_hooks {
            if hook.timing() == timing {
                hook.execute(package)?;
            }
        }
        Ok(())

    /// Installs a package file directly by inferring format from filename
    pub fn install_from_file(&mut self, filepath: &str) -> Result<(), PackageError> {
        let format = PackageFormat::from_filename(filepath).ok_or_else(|| {
            PackageError::InstallationFailed(format!("Unsupported file format extension for file: {}", filepath))
        })?;

        let file_name = filepath.split('/').last().unwrap_or(filepath);
        let pkg_name = file_name.split('.').next().unwrap_or(file_name);

        let package = UnifiedPackage::new(pkg_name.to_string(), "1.0.0".to_string())
            .with_format(format);

        self.add_package(package);
        self.install(pkg_name)

    fn add_default_adapters(&mut self) {
        let apt_adapter = PackageAdapter::new(PackageFormat::Deb, "apt".to_string());
        let yum_adapter = PackageAdapter::new(PackageFormat::Rpm, "yum".to_string());
        let pacman_adapter = PackageAdapter::new(PackageFormat::Pacman, "pacman".to_string());
        let snap_adapter = PackageAdapter::new(PackageFormat::Snap, "snap".to_string());
        let flatpak_adapter = PackageAdapter::new(PackageFormat::Flatpak, "flatpak".to_string());
        let appimage_adapter = PackageAdapter::new(PackageFormat::AppImage, "appimage".to_string());
        let sigpkg_adapter = PackageAdapter::new(PackageFormat::SigmaPkg, "sigpkg".to_string());

        self.adapters.insert(PackageFormat::Deb, apt_adapter);
        self.adapters.insert(PackageFormat::Rpm, yum_adapter);
        self.adapters.insert(PackageFormat::Pacman, pacman_adapter);
        self.adapters.insert(PackageFormat::Snap, snap_adapter);
        self.adapters.insert(
            PackageFormat::Deb,
            PackageAdapter::new(PackageFormat::Deb, String::from("AptDeb")),
        );
        self.adapters.insert(
            PackageFormat::Rpm,
            PackageAdapter::new(PackageFormat::Rpm, String::from("YumRpm")),
        );
        self.adapters.insert(
            PackageFormat::Pacman,
            PackageAdapter::new(PackageFormat::Pacman, String::from("Pacman")),
        );
        self.adapters.insert(
            PackageFormat::Snap,
            PackageAdapter::new(PackageFormat::Snap, String::from("Snap")),
        );
        self.adapters.insert(
            PackageFormat::Flatpak,
            PackageAdapter::new(PackageFormat::Flatpak, String::from("Flatpak")),
        );
        self.adapters.insert(
            PackageFormat::AppImage,
            appimage_adapter,
        );
        self.adapters.insert(
            PackageFormat::SigmaPkg,
            PackageAdapter::new(PackageFormat::SigmaPkg, String::from("SigmaPkg")),
        );
        self.adapters.insert(PackageFormat::Air, PackageAdapter::new(PackageFormat::Air, "air".to_string()));
        self.adapters.insert(PackageFormat::Bottle, PackageAdapter::new(PackageFormat::Bottle, "bottle".to_string()));
        self.adapters.insert(PackageFormat::Ipa, PackageAdapter::new(PackageFormat::Ipa, "ipa".to_string()));
        self.adapters.insert(PackageFormat::Ports, PackageAdapter::new(PackageFormat::Ports, "ports".to_string()));
        self.adapters.insert(PackageFormat::Pkg, PackageAdapter::new(PackageFormat::Pkg, "pkg".to_string()));
        self.adapters.insert(PackageFormat::Aab, PackageAdapter::new(PackageFormat::Aab, "aab".to_string()));
        self.adapters.insert(PackageFormat::Apk, PackageAdapter::new(PackageFormat::Apk, "apk".to_string()));
        self.adapters.insert(PackageFormat::Eopkg, PackageAdapter::new(PackageFormat::Eopkg, "eopkg".to_string()));
        self.adapters.insert(PackageFormat::Nixpkg, PackageAdapter::new(PackageFormat::Nixpkg, "nixpkg".to_string()));
        self.adapters.insert(PackageFormat::Ebuild, PackageAdapter::new(PackageFormat::Ebuild, "ebuild".to_string()));
        self.adapters.insert(PackageFormat::TarGz, PackageAdapter::new(PackageFormat::TarGz, "targz".to_string()));
        self.adapters.insert(PackageFormat::Xz, PackageAdapter::new(PackageFormat::Xz, "xz".to_string()));
        self.adapters.insert(PackageFormat::App, PackageAdapter::new(PackageFormat::App, "app".to_string()));
        self.adapters.insert(PackageFormat::Hap, PackageAdapter::new(PackageFormat::Hap, "hap".to_string()));
        self.adapters.insert(PackageFormat::Pisi, PackageAdapter::new(PackageFormat::Pisi, "pisi".to_string()));
        self.adapters.insert(PackageFormat::Superdeb, PackageAdapter::new(PackageFormat::Superdeb, "superdeb".to_string()));
        self.adapters.insert(PackageFormat::Lzm, PackageAdapter::new(PackageFormat::Lzm, "lzm".to_string()));
        self.adapters.insert(PackageFormat::Pup, PackageAdapter::new(PackageFormat::Pup, "pup".to_string()));
        self.adapters.insert(PackageFormat::Pet, PackageAdapter::new(PackageFormat::Pet, "pet".to_string()));
        self.adapters.insert(PackageFormat::Tar, PackageAdapter::new(PackageFormat::Tar, "tar".to_string()));
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
            let package_opt = self.packages.get(&dep_name).cloned();
            if let Some(package) = package_opt {
                // Find appropriate adapter
                for format in &package.formats {
                    if let Some(adapter) = self.adapters.get(format) {
                        let adapter: &PackageAdapter = adapter;
                        adapter.install(&package)?;
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
        let package_opt = self.installed_packages.get(package_name).cloned();
        if let Some(package) = package_opt {
            for format in &package.formats {
                if let Some(adapter) = self.adapters.get(format) {
                    let adapter: &PackageAdapter = adapter;
                    adapter.remove(&package)?;
                    break;
                }
            }
            self.installed_packages.remove(package_name);
        }
        Ok(())
    }

    pub fn update(&mut self, package_name: &str) -> Result<(), PackageError> {
        let package_opt = self.installed_packages.get(package_name).cloned();
        if let Some(package) = package_opt {
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

/// Package errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageError {
    PackageNotFound(String),
    DependencyNotFound(String),
    AdapterNotFound,
    InstallationFailed(String),
    ConflictDetected(Vec<(String, String)>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeatureType {
    Binary,
    Library,
    Source,
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

pub trait PackageAdapterTrait {
    fn adapter_name(&self) -> &str;
}

/// Universal multi-format package metadata parser and handler supporting
/// Linux, BSD, macOS, Android, and HarmonyOS package formats
pub struct UniversalPackageManifestParser;

impl UniversalPackageManifestParser {
    pub fn detect_format_from_filename(filename: &str) -> Option<PackageFormat> {
        let name = filename.to_lowercase();
        if name.ends_with(".deb") || name.ends_with(".superdeb") {
            Some(PackageFormat::Deb)
        } else if name.ends_with(".rpm") {
            Some(PackageFormat::Rpm)
        } else if name.ends_with(".apk") {
            Some(PackageFormat::Apk)
        } else if name.ends_with(".pkg.tar.xz") || name.ends_with(".pkg.tar.zst") {
            Some(PackageFormat::Pacman)
        } else if name.ends_with(".snap") {
            Some(PackageFormat::Snap)
        } else if name.ends_with(".flatpak") {
            Some(PackageFormat::Flatpak)
        } else if name.ends_with(".appimage") {
            Some(PackageFormat::AppImage)
        } else if name.ends_with(".ebuild") || name.ends_with(".portage") {
            Some(PackageFormat::Ebuild)
        } else if name.ends_with(".nixpkg") || name.ends_with(".nix") {
            Some(PackageFormat::Nixpkg)
        } else if name.ends_with(".eopkg") {
            Some(PackageFormat::Eopkg)
        } else if name.ends_with(".ports") {
            Some(PackageFormat::Ports)
        } else if name.ends_with(".pkg") {
            Some(PackageFormat::Pkg)
        } else if name.ends_with(".ipa") {
            Some(PackageFormat::Ipa)
        } else if name.ends_with(".aab") {
            Some(PackageFormat::Aab)
        } else if name.ends_with(".hap") {
            Some(PackageFormat::Hap)
        } else if name.ends_with(".pisi") {
            Some(PackageFormat::Pisi)
        } else if name.ends_with(".lzm") {
            Some(PackageFormat::Lzm)
        } else if name.ends_with(".pup") {
            Some(PackageFormat::Pup)
        } else if name.ends_with(".pet") {
            Some(PackageFormat::Pet)
        } else if name.ends_with(".tar.gz") || name.ends_with(".tgz") {
            Some(PackageFormat::TarGz)
        } else if name.ends_with(".tar.xz") || name.ends_with(".xz") {
            Some(PackageFormat::Xz)
        } else if name.ends_with(".tar") {
            Some(PackageFormat::Tar)
        } else if name.ends_with(".app") {
            Some(PackageFormat::App)
        } else {
            None
        }
    }

    pub fn parse_manifest_auto(filename: &str, raw_data: &[u8]) -> Result<UnifiedPackage, &'static str> {
        let fmt = Self::detect_format_from_filename(filename)
            .ok_or("UniversalManifestParser: Unsupported or unrecognized package extension")?;

        let pkg_name = filename
            .split('.')
            .next()
            .unwrap_or("unknown")
            .to_string();

        let mut pkg = UnifiedPackage::new(pkg_name, "1.0.0".to_string()).with_format(fmt);
        if !raw_data.is_empty() {
            pkg = pkg.with_provides("universal_binary".to_string());
        }
        Ok(pkg)
    }
}

/// Linux & BSD Distro Inspired Rollback Mechanics
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistroRollbackType {
    NixOsGeneration,       // NixOS atomic generation profile rollback
    FreeBsdZfsBootEnv,     // FreeBSD ZFS boot environment (bectl / beadm) rollback
    OpenSuseSnapper,       // openSUSE Snapper CoW snapshot rollback
    FedoraRpmOstree,       // Fedora Silverblue / rpm-ostree deployment rollback
    AlpineApkCache,        // Alpine Linux local apk tarball cache rollback
}

#[derive(Debug, Clone)]
pub struct SovereignRollbackSnapshot {
    pub snapshot_id: usize,
    pub rollback_type: DistroRollbackType,
    pub label: String,
    pub installed_packages_state: Vec<String>,
    pub timestamp_sec: u64,
}

pub struct SovereignPackageRollbackEngine {
    pub snapshots: Vec<SovereignRollbackSnapshot>,
    pub active_snapshot_id: Option<usize>,
    pub next_snapshot_id: usize,
}

impl SovereignPackageRollbackEngine {
    pub fn new() -> Self {
        Self {
            snapshots: Vec::new(),
            active_snapshot_id: None,
            next_snapshot_id: 1,
        }
    }

    pub fn create_distro_snapshot(
        &mut self,
        rollback_type: DistroRollbackType,
        label: &str,
        installed_packages: &[String],
        now_sec: u64,
    ) -> usize {
        let id = self.next_snapshot_id;
        self.next_snapshot_id += 1;

        let snap = SovereignRollbackSnapshot {
            snapshot_id: id,
            rollback_type,
            label: label.to_string(),
            installed_packages_state: installed_packages.to_vec(),
            timestamp_sec: now_sec,
        };

        self.snapshots.push(snap);
        self.active_snapshot_id = Some(id);
        id
    }

    pub fn rollback(&mut self, snapshot_id: usize) -> Result<Vec<String>, &'static str> {
        let snap = self.snapshots.iter().find(|s| s.snapshot_id == snapshot_id).ok_or("Rollback Engine: Snapshot not found")?;
        self.active_snapshot_id = Some(snapshot_id);
        Ok(snap.installed_packages_state.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_creation() {
        let manager = UniversalPackageManager::new();
        assert!(manager.adapters.len() >= 25);
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

    #[test]
    fn test_distro_packaging_manifests_and_sandboxing() {
        let adapter = PackageAdapter::new(PackageFormat::Flatpak, "flatpak".to_string());

        // 1. Test Flatpak sandboxing policy translation
        let flat_manifest = FlatpakManifest {
            id: "org.gnome.Gimp".to_string(),
            runtime: "org.gnome.Platform".to_string(),
            runtime_version: "44".to_string(),
            sdk: "org.gnome.Sdk".to_string(),
            command: "gimp".to_string(),
            finish_args: {
                let mut v = Vec::new();
                v.push("--share=network".to_string());
                v.push("--share=ipc".to_string());
                v.push("--filesystem=host".to_string());
                v
            },
        };

        let enforced_pledges = adapter.translate_flatpak_sandbox_policy(&flat_manifest);
        assert_eq!(enforced_pledges.len(), 3);
        assert!(enforced_pledges.contains(&"network".to_string()));
        assert!(enforced_pledges.contains(&"ipc".to_string()));
        assert!(enforced_pledges.contains(&"unveil_all".to_string()));

        // 2. Test Snapcraft confinement translation
        let snap_adapter = PackageAdapter::new(PackageFormat::Snap, "snap".to_string());
        let snap_manifest = SnapcraftManifest {
            name: "gimp".to_string(),
            version: "2.10.30".to_string(),
            summary: "GNU Image Manipulation Program".to_string(),
            confinement: "strict".to_string(),
            plugs: {
                let mut v = Vec::new();
                v.push("network".to_string());
                v
            },
            slots: Vec::new(),
        };

        let confinement_rule = snap_adapter.translate_snap_confinement(&snap_manifest);
        assert_eq!(confinement_rule, "strict_pledge_sandbox");

        // 3. Verify general manifest definitions compile (AptDebManifest and PacmanPkgbuild)
        let _deb = AptDebManifest {
            package: "curl".to_string(),
            version: "7.81.0".to_string(),
            architecture: "amd64".to_string(),
            maintainer: "Debian Curl Maintainers".to_string(),
            depends: {
                let mut v = Vec::new();
                v.push("libcurl4".to_string());
                v
            },
            description: "command line tool for transferring data with URLs".to_string(),
        };

        let _pkgbuild = PacmanPkgbuild {
            pkgname: "curl".to_string(),
            pkgver: "7.81.0".to_string(),
            pkgdesc: "command line tool for transferring data with URLs".to_string(),
            arch: {
                let mut v = Vec::new();
                v.push("x86_64".to_string());
                v
            },
            depends: {
                let mut v = Vec::new();
                v.push("openssl".to_string());
                v
            },
            makedepends: {
                let mut v = Vec::new();
                v.push("git".to_string());
                v
            },
            source_urls: {
                let mut v = Vec::new();
                v.push("https://curl.se/download/curl-7.81.0.tar.gz".to_string());
                v
            },
        };
    }

    #[test]
    fn test_comprehensive_packaging_systems() {
        let appimage_adapter = PackageAdapter::new(PackageFormat::AppImage, "appimage".to_string());

        // 1. Test AppImage metadata runtime & mount mock
        let app_runtime = AppImageRuntime {
            app_name: "Blender".to_string(),
            signature_offset: 1024,
            squashfs_offset: 2048,
            embedded_icon_path: "/usr/share/icons/blender.png".to_string(),
        };
        let mount_path = appimage_adapter
            .mount_appimage_squashfs(&app_runtime)
            .unwrap();
        assert_eq!(mount_path, "/tmp/.mount_Blender_squashfs");

        // AppImage mount offset error checking
        let bad_app = AppImageRuntime {
            app_name: "BadApp".to_string(),
            signature_offset: 0,
            squashfs_offset: 0,
            embedded_icon_path: String::new(),
        };
        assert!(appimage_adapter.mount_appimage_squashfs(&bad_app).is_err());

        // 2. Test APT repository source query checks
        let apt_adapter = PackageAdapter::new(PackageFormat::Deb, "apt".to_string());
        let apt_config = AptRepoConfig {
            sourcelist_url: "deb https://deb.debian.org/debian".to_string(),
            suite: "bookworm".to_string(),
            components: {
                let mut v = Vec::new();
                v.push("main".to_string());
                v.push("contrib".to_string());
                v
            },
            trust_anchor: None,
        };
        assert!(apt_adapter.query_apt_repository(&apt_config));

        // 3. Test DNF repository configuration query checks
        let dnf_adapter = PackageAdapter::new(PackageFormat::Rpm, "dnf".to_string());
        let dnf_config = DnfRepoConfig {
            repoid: "fedora".to_string(),
            baseurl: "https://mirrors.fedoraproject.org/".to_string(),
            gpgcheck: true,
            enabled: true,
        };
        assert!(dnf_adapter.query_dnf_repository(&dnf_config));
    }

    #[test]
    fn test_universal_package_manifest_parser() {
        assert_eq!(
            UniversalPackageManifestParser::detect_format_from_filename("nginx.deb"),
            Some(PackageFormat::Deb)
        );
        assert_eq!(
            UniversalPackageManifestParser::detect_format_from_filename("app.flatpak"),
            Some(PackageFormat::Flatpak)
        );

        let pkg = UniversalPackageManifestParser::parse_manifest_auto("tool.apk", b"payload").unwrap();
        assert_eq!(pkg.name, "tool");
        assert_eq!(pkg.formats[0], PackageFormat::Apk);
    }

    #[test]
    fn test_distro_package_rollback_engine() {
        let mut engine = SovereignPackageRollbackEngine::new();
        let pkgs = vec!["nginx".to_string(), "curl".to_string()];

        let snap_id = engine.create_distro_snapshot(
            DistroRollbackType::NixOsGeneration,
            "NixOS Gen 101",
            &pkgs,
            1700000000,
        );

        assert_eq!(snap_id, 1);
        let restored = engine.rollback(snap_id).unwrap();
        assert_eq!(restored, pkgs);
    }

    #[test]
    fn test_universal_package_manager_node_runtime_integration() {
        let mut manager = UniversalPackageManager::new();
        let bytes = vec![0x42u8; 120];
        let mut hash = [0u8; 32];
        let mut state: u64 = 0xcbf29ce484222325;
        for (i, &b) in bytes.iter().enumerate() {
            state ^= b as u64;
            state = state.wrapping_mul(0x100000001b3);
            hash[i % 32] ^= (state >> ((i % 8) * 8)) as u8;
        }

        let node_pkg = NodeBinaryPackage::new(
            "v20.11.0",
            NodeReleaseStream::Lts,
            NodeTargetArch::X86_64,
            LibcFlavor::Musl,
            "https://dist.sigmaos.org/node/v20.11.0.tar.xz",
            hash,
            [0u8; 64],
            120,
        );

        let path = manager.install_node_runtime(&node_pkg, &bytes, "10.2.4").unwrap();
        assert!(path.starts_with("/sovereign/store/node-v20.11.0-"));

        let installed_pkg = manager.installed_packages.get("nodejs-v20.11.0").unwrap();
        assert_eq!(installed_pkg.version, "v20.11.0");
        assert!(installed_pkg.installed);
    }
}
