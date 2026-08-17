// SigmaOS Arch Linux Compatibility & Parity Subsystem (sigpkg-arch)
// Natively compiles PKGBUILD recipes, emulates Pacman database states, manages rolling release upgrades,
// and implements ALPM hooks, mkinitcpio initramfs builders, and makepkg source pipelines.

#[cfg(not(test))]
use crate::sigpkg::{Dependency, Package, Version, VersionConstraint};
use std::collections::HashMap;

#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionConstraint {
    Any,
    Exact(Version),
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dependency {
    pub name: String,
    pub version_constraint: VersionConstraint,
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

#[cfg(test)]
impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self { major, minor, patch }
    }

    pub fn parse(s: &str) -> Result<Self, &'static str> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() >= 3 {
            let major = parts[0].parse().unwrap_or(1);
            let minor = parts[1].parse().unwrap_or(0);
            let patch = parts[2].parse().unwrap_or(0);
            Ok(Version { major, minor, patch })
        } else {
            Ok(Version { major: 1, minor: 0, patch: 0 })
        }
    }
}

#[cfg(test)]
#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: Version,
    pub description: String,
    pub dependencies: Vec<Dependency>,
    pub checksum: String,
}

#[cfg(test)]
impl Package {
    pub fn new(
        name: String,
        version: Version,
        description: String,
        dependencies: Vec<Dependency>,
        checksum: String,
    ) -> Self {
        Self {
            name,
            version,
            description,
            dependencies,
            checksum,
        }
    }
}

/// Emulates Arch User Repository (AUR) PKGBUILD recipes parsing and compiling
#[derive(Debug, Clone)]
pub struct AurRecipeCompiler {
    pub build_env_active: bool,
}

impl AurRecipeCompiler {
    pub fn new() -> Self {
        Self {
            build_env_active: true,
        }
    }

    /// Compiles a declarative Arch-style PKGBUILD text into a native S-PKG Package metadata
    pub fn compile_pkgbuild(&self, pkgbuild_content: &str) -> Result<Package, &'static str> {
        let mut pkgname = "";
        let mut pkgver = "1.0.0";
        let mut depends = Vec::new();

        for line in pkgbuild_content.lines() {
            let line = line.trim();
            if line.starts_with("pkgname=") {
                pkgname = line.strip_prefix("pkgname=").unwrap().trim_matches('"');
            } else if line.starts_with("pkgver=") {
                pkgver = line.strip_prefix("pkgver=").unwrap().trim_matches('"');
            } else if line.starts_with("depends=") {
                let dep_str = line
                    .strip_prefix("depends=(")
                    .unwrap()
                    .trim_matches(')')
                    .trim_matches('"');
                for d in dep_str.split_whitespace() {
                    depends.push(Dependency {
                        name: d.replace('\'', "").replace('"', ""),
                        version_constraint: VersionConstraint::Any,
                    });
                }
            }
        }

        if pkgname.is_empty() {
            return Err("PKGBUILD missing mandatory pkgname field");
        }

        let parsed_ver =
            Version::parse(pkgver).map_err(|_| "Invalid version format in PKGBUILD")?;

        Ok(Package::new(
            pkgname.to_string(),
            parsed_ver,
            format!("Compiled AUR Package: {}", pkgname),
            depends,
            "sha256_compiled_mock_hash_value".to_string(),
        ))
    }
}

impl Default for AurRecipeCompiler {
    fn default() -> Self {
        Self::new()
    }
}

/// Rolling Release System Synchronizer
#[derive(Debug, Clone)]
pub struct RollingSyncManager {
    pub installed_packages: HashMap<String, Version>,
    pub remote_repository: HashMap<String, Version>,
}

impl RollingSyncManager {
    pub fn new() -> Self {
        Self {
            installed_packages: HashMap::new(),
            remote_repository: HashMap::new(),
        }
    }

    pub fn register_installed(&mut self, name: &str, version: Version) {
        self.installed_packages.insert(name.to_string(), version);
    }

    pub fn register_remote(&mut self, name: &str, version: Version) {
        self.remote_repository.insert(name.to_string(), version);
    }

    /// Checks for available package updates in the rolling release stream
    pub fn list_pending_rolling_updates(&self) -> Vec<(String, Version, Version)> {
        let mut updates = Vec::new();
        for (pkg_name, installed_ver) in &self.installed_packages {
            if let Some(remote_ver) = self.remote_repository.get(pkg_name.as_str()) {
                if remote_ver > installed_ver {
                    updates.push((pkg_name.clone(), *installed_ver, *remote_ver));
                }
            }
        }
        updates.sort_by(|a, b| a.0.cmp(&b.0));
        updates
    }
}

impl Default for RollingSyncManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Emulates parsing Pacman local database states (/var/lib/pacman/local)
#[derive(Debug, Clone)]
pub struct PacmanDbAdapter {
    pub pacman_db_path: String,
}

impl PacmanDbAdapter {
    pub fn new(db_path: &str) -> Self {
        Self {
            pacman_db_path: db_path.to_string(),
        }
    }

    /// Parses Pacman formatted `/var/lib/pacman/local/pkg/desc` file into S-PKG Package metadata
    pub fn import_legacy_pacman_package(
        &self,
        desc_content: &str,
    ) -> Result<Package, &'static str> {
        let mut name = "";
        let mut version = "1.0.0";
        let mut desc = "";

        let mut lines = desc_content.lines();
        while let Some(line) = lines.next() {
            let line = line.trim();
            if line == "%NAME%" {
                name = lines.next().unwrap_or("").trim();
            } else if line == "%VERSION%" {
                version = lines.next().unwrap_or("").trim();
            } else if line == "%DESC%" {
                desc = lines.next().unwrap_or("").trim();
            }
        }

        if name.is_empty() {
            return Err("Legacy Pacman desc file missing NAME block");
        }

        // Clean any release suffixes like -1 or -arch from version string
        let base_version = version.split('-').next().unwrap_or("1.0.0");
        let parsed_ver =
            Version::parse(base_version).map_err(|_| "Failed to parse legacy version")?;

        Ok(Package::new(
            name.to_string(),
            parsed_ver,
            desc.to_string(),
            Vec::new(),
            "sha256_imported_legacy_hash_value".to_string(),
        ))
    }
}

/// Gentoo-style compile-time USE flag toggle configuration
#[derive(Debug, Clone)]
pub struct GentooUseFlag {
    pub name: String,
    pub is_enabled: bool,
}

/// Gentoo-style declarative ebuild package build recipe descriptor
#[derive(Debug, Clone)]
pub struct GentooEbuildPackage {
    pub name: String,
    pub version: Version,
    pub use_flags: Vec<GentooUseFlag>,
    pub configure_flags: Vec<String>,
}

/// Gentoo Portage-style source-build package compiler & optimizer engine
pub struct PortageEbuildCompiler {
    pub global_use_flags: HashMap<String, bool>,
}

impl PortageEbuildCompiler {
    pub fn new() -> Self {
        Self {
            global_use_flags: HashMap::new(),
        }
    }

    pub fn set_global_use_flag(&mut self, name: &str, enabled: bool) {
        self.global_use_flags.insert(name.to_string(), enabled);
    }

    /// Evaluates if custom USE flags match global feature policies, and dynamically generates
    /// the optimized compiler `./configure` target strings.
    pub fn configure_and_compile(&self, ebuild: &mut GentooEbuildPackage) -> Result<Package, &'static str> {
        let mut active_features = Vec::new();

        for flag in &mut ebuild.use_flags {
            if let Some(&global_state) = self.global_use_flags.get(&flag.name) {
                flag.is_enabled = global_state;
            }
            if flag.is_enabled {
                active_features.push(flag.name.clone());
            }
        }

        for feature in &active_features {
            let config_arg = format!("--enable-{}", feature);
            if !ebuild.configure_flags.contains(&config_arg) {
                ebuild.configure_flags.push(config_arg);
            }
        }

        let description = format!(
            "Compiled Gentoo ebuild package: {} with active features: {:?}",
            ebuild.name, active_features
        );

        Ok(Package::new(
            ebuild.name.clone(),
            ebuild.version.clone(),
            description,
            Vec::new(),
            "sha256_portage_compiled_source_binary".to_string(),
        ))
    }

    /// Portage-style compiler native CPU microarchitecture optimization target level generator
    pub fn get_optimized_target_cpu_level(&self, cpu_features: &[&str]) -> usize {
        let mut level = 1;
        if cpu_features.contains(&"sse4.2") {
            level = 2;
        }
        if cpu_features.contains(&"avx2") {
            level = 3;
        }
        if cpu_features.contains(&"avx512f") {
            level = 4;
        }
        level
    }
}

impl Default for PortageEbuildCompiler {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a Debian-style source package targeting sbuild compiler rules
#[derive(Debug, Clone)]
pub struct DebianSbuildPackage {
    pub name: String,
    pub version: Version,
    pub build_depends: Vec<String>,
}

impl RollingSyncManager {
    pub fn is_debian_sbuild_builddeps_satisfied(&self, package: &DebianSbuildPackage) -> bool {
        for dep in &package.build_depends {
            if !self.installed_packages.contains_key(dep) {
                return false;
            }
        }
        true
    }
}

// ==========================================
// 6. ALPM (Arch Linux Package Management) Hooks Engine
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HookWhen {
    PreTransaction,
    PostTransaction,
}

#[derive(Debug, Clone)]
pub struct AlpmHook {
    pub name: String,
    pub when: HookWhen,
    pub target_packages: Vec<String>, // e.g. ["glibc", "*"]
    pub exec_command: String,
}

pub struct AlpmHookManager {
    pub hooks: Vec<AlpmHook>,
}

impl AlpmHookManager {
    pub fn new() -> Self {
        Self { hooks: Vec::new() }
    }

    pub fn register_hook(&mut self, hook: AlpmHook) {
        self.hooks.push(hook);
    }

    /// Executes matching ALPM hooks for an install/upgrade event
    pub fn trigger_hooks(&self, when: HookWhen, modified_packages: &[&str]) -> Vec<String> {
        let mut executed = Vec::new();

        for hook in &self.hooks {
            if hook.when == when {
                let matches = hook.target_packages.iter().any(|target| {
                    target == "*" || modified_packages.contains(&target.as_str())
                });

                if matches {
                    executed.push(hook.exec_command.clone());
                }
            }
        }
        executed
    }
}

impl Default for AlpmHookManager {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 7. Mkinitcpio Initramfs Image Builder
// ==========================================

pub struct MkinitcpioBuilder {
    pub hooks: Vec<String>, // e.g. ["base", "udev", "block", "filesystems"]
}

impl MkinitcpioBuilder {
    pub fn new() -> Self {
        Self {
            hooks: vec![
                "base".to_string(),
                "udev".to_string(),
                "block".to_string(),
                "filesystems".to_string(),
            ],
        }
    }

    pub fn build_initramfs(&self, kernel_ver: &str) -> Result<String, &'static str> {
        if self.hooks.is_empty() {
            return Err("Cannot build initramfs without hooks configured");
        }
        Ok(format!(
            "/boot/initramfs-{}.img [hooks: {:?}]",
            kernel_ver, self.hooks
        ))
    }
}

impl Default for MkinitcpioBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 8. Makepkg Source Build Pipeline Engine
// ==========================================

pub struct MakepkgBuilder;

impl MakepkgBuilder {
    pub fn new() -> Self {
        Self
    }

    /// Validates SHA256 checksums of downloaded sources and builds a `.pkg.tar.zst` binary package
    pub fn build_package_from_source(
        &self,
        pkgname: &str,
        pkgver: &str,
        sources: &[(&str, &str)], // (filename, expected_sha256)
        simulated_file_hashes: &HashMap<String, String>,
    ) -> Result<String, &'static str> {
        for (file, expected_hash) in sources {
            let actual_hash = simulated_file_hashes
                .get(*file)
                .ok_or("Source file missing")?;
            if actual_hash != expected_hash {
                return Err("SHA256 checksum verification failed");
            }
        }

        Ok(format!("{}-{}-1-x86_64.pkg.tar.zst", pkgname, pkgver))
    }
}

impl Default for MakepkgBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 9. Pacman Transaction Database Lock Manager
// ==========================================

pub struct PacmanLockManager {
    pub lock_file_path: String,
    pub is_locked: bool,
    pub holder_pid: Option<u32>,
}

impl PacmanLockManager {
    pub fn new(path: &str) -> Self {
        Self {
            lock_file_path: path.to_string(),
            is_locked: false,
            holder_pid: None,
        }
    }

    /// Acquires transaction database lock (/var/lib/pacman/db.lck)
    pub fn acquire_lock(&mut self, pid: u32) -> Result<(), &'static str> {
        if self.is_locked {
            return Err("Pacman database is locked by another process (db.lck exists)");
        }
        self.is_locked = true;
        self.holder_pid = Some(pid);
        Ok(())
    }

    /// Releases transaction database lock
    pub fn release_lock(&mut self, pid: u32) -> Result<(), &'static str> {
        if !self.is_locked {
            return Ok(());
        }
        if self.holder_pid != Some(pid) {
            return Err("Cannot release lock held by another PID");
        }
        self.is_locked = false;
        self.holder_pid = None;
        Ok(())
    }

    /// Forcefully breaks stale lock if process holding lock no longer exists
    pub fn force_unlock_stale(&mut self) {
        self.is_locked = false;
        self.holder_pid = None;
    }
}

impl Default for PacmanLockManager {
    fn default() -> Self {
        Self::new("/var/lib/pacman/db.lck")
    }
}

// ==========================================
// 10. Parallel Download Engine (ParallelDownloads = N)
// ==========================================

pub struct ParallelDownloadEngine {
    pub max_concurrent_streams: usize,
}

impl ParallelDownloadEngine {
    pub fn new(max_concurrent_streams: usize) -> Self {
        Self {
            max_concurrent_streams: max_concurrent_streams.max(1),
        }
    }

    /// Simulates concurrent downloading of multiple package targets across mirror pools
    pub fn download_packages_parallel(&self, packages: &[&str]) -> Vec<(String, bool, usize)> {
        let mut results = Vec::new();
        for (i, pkg) in packages.iter().enumerate() {
            let stream_id = i % self.max_concurrent_streams;
            results.push((pkg.to_string(), true, stream_id));
        }
        results
    }
}

impl Default for ParallelDownloadEngine {
    fn default() -> Self {
        Self::new(5)
    }
}

// ==========================================
// 11. Reflector Mirror Sorting & Rating Engine
// ==========================================

#[derive(Debug, Clone)]
pub struct ArchMirror {
    pub url: String,
    pub country: String,
    pub latency_ms: u32,
    pub completion_rate: f32,
    pub is_https: bool,
}

pub struct ReflectorEngine {
    pub mirrors: Vec<ArchMirror>,
}

impl ReflectorEngine {
    pub fn new() -> Self {
        Self { mirrors: Vec::new() }
    }

    pub fn add_mirror(&mut self, mirror: ArchMirror) {
        self.mirrors.push(mirror);
    }

    /// Filters and ranks top Arch mirrors by latency, HTTPS protocol, and completion rate
    pub fn get_ranked_mirrorlist(
        &self,
        require_https: bool,
        top_n: usize,
    ) -> Vec<ArchMirror> {
        let mut filtered: Vec<ArchMirror> = self
            .mirrors
            .iter()
            .filter(|m| !require_https || m.is_https)
            .cloned()
            .collect();

        filtered.sort_by(|a, b| {
            a.latency_ms
                .cmp(&b.latency_ms)
                .then_with(|| b.completion_rate.partial_cmp(&a.completion_rate).unwrap_or(core::cmp::Ordering::Equal))
        });

        filtered.into_iter().take(top_n).collect()
    }
}

impl Default for ReflectorEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 12. Archiso Boot & Live SquashFS RAM Engine
// ==========================================

pub struct ArchisoBootEngine {
    pub archisobasedir: String,
    pub archisolabel: String,
    pub copytoram: bool,
}

impl ArchisoBootEngine {
    pub fn new(basedir: &str, label: &str) -> Self {
        Self {
            archisobasedir: basedir.to_string(),
            archisolabel: label.to_string(),
            copytoram: false,
        }
    }

    /// Parses Arch kernel cmdline boot parameters
    pub fn parse_cmdline(&mut self, cmdline: &str) {
        for arg in cmdline.split_whitespace() {
            if arg == "copytoram" {
                self.copytoram = true;
            } else if arg.starts_with("archisobasedir=") {
                self.archisobasedir = arg["archisobasedir=".len()..].to_string();
            } else if arg.starts_with("archisolabel=") {
                self.archisolabel = arg["archisolabel=".len()..].to_string();
            }
        }
    }

    /// Simulates mounting the SquashFS Live ISO filesystem into RAM OverlayFS
    pub fn mount_live_overlay(&self) -> Result<String, &'static str> {
        if self.archisobasedir.is_empty() || self.archisolabel.is_empty() {
            return Err("Missing mandatory archisobasedir or archisolabel parameter");
        }
        let ram_status = if self.copytoram { "RAM Copy Active" } else { "Direct Mount" };
        Ok(format!(
            "Mounted ArchISO [{}/{}] - Status: {}",
            self.archisolabel, self.archisobasedir, ram_status
        ))
    }
}

impl Default for ArchisoBootEngine {
    fn default() -> Self {
        Self::new("arch", "ARCH_202603")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debian_sbuild_package_builddeps() {
        let mut sync = RollingSyncManager::new();
        sync.register_installed("gcc", Version::new(12, 2, 0));
        sync.register_installed("make", Version::new(4, 3, 0));

        let source_pkg = DebianSbuildPackage {
            name: "coreutils".to_string(),
            version: Version::new(9, 1, 0),
            build_depends: vec!["gcc".to_string(), "make".to_string()],
        };

        assert!(sync.is_debian_sbuild_builddeps_satisfied(&source_pkg));

        let source_pkg_missing = DebianSbuildPackage {
            name: "coreutils".to_string(),
            version: Version::new(9, 1, 0),
            build_depends: vec!["gcc".to_string(), "make".to_string(), "libc-dev".to_string()],
        };
        assert!(!sync.is_debian_sbuild_builddeps_satisfied(&source_pkg_missing));
    }

    #[test]
    fn test_gentoo_portage_compiler() {
        let mut compiler = PortageEbuildCompiler::new();
        compiler.set_global_use_flag("vulkan", true);
        compiler.set_global_use_flag("x11", false);

        let mut ebuild = GentooEbuildPackage {
            name: "mpv-player".to_string(),
            version: Version::new(0, 35, 0),
            use_flags: vec![
                GentooUseFlag { name: "vulkan".to_string(), is_enabled: false },
                GentooUseFlag { name: "x11".to_string(), is_enabled: true },
            ],
            configure_flags: Vec::new(),
        };

        let pkg = compiler.configure_and_compile(&mut ebuild).unwrap();
        assert_eq!(pkg.name, "mpv-player");
        assert!(pkg.description.contains("vulkan"));
        assert!(!pkg.description.contains("x11"));

        assert!(ebuild.configure_flags.contains(&"--enable-vulkan".to_string()));
        assert!(!ebuild.configure_flags.contains(&"--enable-x11".to_string()));

        assert_eq!(compiler.get_optimized_target_cpu_level(&["sse4.2", "avx2"]), 3);
        assert_eq!(compiler.get_optimized_target_cpu_level(&["avx512f", "avx2"]), 4);
        assert_eq!(compiler.get_optimized_target_cpu_level(&[]), 1);
    }

    #[test]
    fn test_aur_pkgbuild_compiler() {
        let compiler = AurRecipeCompiler::new();
        let pkgbuild = r#"
            pkgname="neo-vim"
            pkgver="0.9.1"
            depends=("libc" "libuv" "libmsgpack")
        "#;

        let package = compiler.compile_pkgbuild(pkgbuild).unwrap();
        assert_eq!(package.name, "neo-vim");
        assert_eq!(package.version, Version::new(0, 9, 1));
        assert_eq!(package.dependencies.len(), 3);
        assert_eq!(package.dependencies[0].name, "libc");
    }

    #[test]
    fn test_rolling_upgrade_sync() {
        let mut sync = RollingSyncManager::new();
        sync.register_installed("bash", Version::new(5, 1, 0));
        sync.register_installed("curl", Version::new(7, 85, 0));

        sync.register_remote("bash", Version::new(5, 2, 0));
        sync.register_remote("curl", Version::new(7, 85, 0));

        let pending = sync.list_pending_rolling_updates();
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].0, "bash");
        assert_eq!(pending[0].1, Version::new(5, 1, 0));
        assert_eq!(pending[0].2, Version::new(5, 2, 0));
    }

    #[test]
    fn test_legacy_pacman_db_import() {
        let adapter = PacmanDbAdapter::new("/var/lib/pacman");
        let desc = r#"
            %NAME%
            pacman-contrib

            %VERSION%
            1.8.0-1

            %DESC%
            Contrib utilities for pacman package manager
        "#;

        let imported = adapter.import_legacy_pacman_package(desc).unwrap();
        assert_eq!(imported.name, "pacman-contrib");
        assert_eq!(imported.version, Version::new(1, 8, 0));
        assert_eq!(
            imported.description,
            "Contrib utilities for pacman package manager"
        );
    }

    #[test]
    fn test_alpm_hooks() {
        let mut manager = AlpmHookManager::new();

        manager.register_hook(AlpmHook {
            name: "update-fonts".to_string(),
            when: HookWhen::PostTransaction,
            target_packages: vec!["fontconfig".to_string()],
            exec_command: "fc-cache -s".to_string(),
        });

        // Trigger post-transaction hooks when fontconfig is updated
        let executed = manager.trigger_hooks(HookWhen::PostTransaction, &["fontconfig"]);
        assert_eq!(executed.len(), 1);
        assert_eq!(executed[0], "fc-cache -s");

        // Trigger when unrelated package is updated -> 0 hooks executed
        let executed_none = manager.trigger_hooks(HookWhen::PostTransaction, &["bash"]);
        assert_eq!(executed_none.len(), 0);
    }

    #[test]
    fn test_mkinitcpio_builder() {
        let builder = MkinitcpioBuilder::new();
        let initramfs = builder.build_initramfs("6.1.0-arch1").unwrap();
        assert!(initramfs.contains("/boot/initramfs-6.1.0-arch1.img"));
    }

    #[test]
    fn test_makepkg_pipeline() {
        let builder = MakepkgBuilder::new();
        let mut hashes = HashMap::new();
        hashes.insert("v1.0.tar.gz".to_string(), "abc123hash".to_string());

        let sources = [("v1.0.tar.gz", "abc123hash")];
        let pkg_file = builder
            .build_package_from_source("htop", "3.2.0", &sources, &hashes)
            .unwrap();
        assert_eq!(pkg_file, "htop-3.2.0-1-x86_64.pkg.tar.zst");
    }

    #[test]
    fn test_pacman_lock_manager() {
        let mut lock_mgr = PacmanLockManager::default();
        assert!(!lock_mgr.is_locked);

        // Acquire lock
        assert!(lock_mgr.acquire_lock(1001).is_ok());
        assert!(lock_mgr.is_locked);

        // Attempting to acquire again fails
        assert!(lock_mgr.acquire_lock(1002).is_err());

        // Releasing from different PID fails
        assert!(lock_mgr.release_lock(1002).is_err());

        // Releasing from holding PID succeeds
        assert!(lock_mgr.release_lock(1001).is_ok());
        assert!(!lock_mgr.is_locked);

        // Force unlock stale lock
        lock_mgr.acquire_lock(2000).unwrap();
        lock_mgr.force_unlock_stale();
        assert!(!lock_mgr.is_locked);
    }

    #[test]
    fn test_parallel_download_engine() {
        let engine = ParallelDownloadEngine::new(3);
        let packages = vec!["linux", "glibc", "systemd", "pacman", "gcc"];
        let downloads = engine.download_packages_parallel(&packages);

        assert_eq!(downloads.len(), 5);
        assert_eq!(downloads[0].0, "linux");
        assert_eq!(downloads[0].2, 0); // stream 0
        assert_eq!(downloads[3].2, 0); // stream 0 (3 % 3)
        assert_eq!(downloads[4].2, 1); // stream 1 (4 % 3)
    }

    #[test]
    fn test_reflector_mirror_engine() {
        let mut reflector = ReflectorEngine::new();
        reflector.add_mirror(ArchMirror {
            url: "http://mirror1.arch.org".to_string(),
            country: "US".to_string(),
            latency_ms: 120,
            completion_rate: 0.99,
            is_https: false,
        });
        reflector.add_mirror(ArchMirror {
            url: "https://mirror2.arch.org".to_string(),
            country: "DE".to_string(),
            latency_ms: 45,
            completion_rate: 1.0,
            is_https: true,
        });
        reflector.add_mirror(ArchMirror {
            url: "https://mirror3.arch.org".to_string(),
            country: "JP".to_string(),
            latency_ms: 85,
            completion_rate: 0.98,
            is_https: true,
        });

        let ranked_https = reflector.get_ranked_mirrorlist(true, 2);
        assert_eq!(ranked_https.len(), 2);
        assert_eq!(ranked_https[0].url, "https://mirror2.arch.org");
        assert_eq!(ranked_https[1].url, "https://mirror3.arch.org");
    }

    #[test]
    fn test_archiso_boot_engine() {
        let mut engine = ArchisoBootEngine::default();
        engine.parse_cmdline("archisobasedir=arch_custom archisolabel=SIGMA_LIVE copytoram");

        assert_eq!(engine.archisobasedir, "arch_custom");
        assert_eq!(engine.archisolabel, "SIGMA_LIVE");
        assert!(engine.copytoram);

        let status = engine.mount_live_overlay().unwrap();
        assert!(status.contains("SIGMA_LIVE/arch_custom"));
        assert!(status.contains("RAM Copy Active"));
    }
}
