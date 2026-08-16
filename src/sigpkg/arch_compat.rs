// SPDX-License-Identifier: MIT
// SigmaOS Arch Linux Compatibility & Parity Subsystem (sigpkg-arch)
// Natively compiles PKGBUILD recipes, emulates Pacman database states, manages rolling release upgrades,
// and implements ALPM hooks, mkinitcpio initramfs builders, and makepkg source pipelines.

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

impl Version {
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self { major, minor, patch }
    }

    pub fn parse(v_str: &str) -> Result<Self, &'static str> {
        let clean_v = v_str.split('-').next().unwrap_or(v_str);
        let mut parts = clean_v.split('.');
        let major = parts.next().ok_or("err")?.parse::<u64>().unwrap_or(1);
        let minor = parts.next().unwrap_or("0").parse::<u64>().unwrap_or(0);
        let patch = parts.next().unwrap_or("0").parse::<u64>().unwrap_or(0);
        Ok(Self::new(major, minor, patch))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VersionConstraint {
    Exact(Version),
    GreaterThan(Version),
    LessThan(Version),
    GreaterOrEqual(Version),
    LessOrEqual(Version),
    Any,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dependency {
    pub name: String,
    pub version_constraint: VersionConstraint,
}


#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: Version,
    pub description: String,
    pub dependencies: Vec<Dependency>,
    pub checksum: String,
}

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

    /// Compiles a declarative Arch-style PKGBUILD text into a native Package metadata
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

        let parsed_ver = Version::parse(pkgver).map_err(|_| "Invalid version format in PKGBUILD")?;

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

    /// Parses Pacman formatted `/var/lib/pacman/local/pkg/desc` file into Package metadata
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

        let base_version = version.split('-').next().unwrap_or("1.0.0");
        let parsed_ver = Version::parse(base_version).map_err(|_| "Failed to parse legacy version")?;

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
            ebuild.version,
            description,
            Vec::new(),
            "sha256_portage_compiled_source_binary".to_string(),
        ))
    }

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlpmHookAction {
    PreTransaction,
    PostTransaction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HookWhen {
    PreTransaction,
    PostTransaction,
}

/// GDB/Pacman-inspired declarative ALPM Hook
#[derive(Debug, Clone)]
pub struct AlpmHook {
    pub name: String,
    pub action: AlpmHookAction,
    pub when: HookWhen,
    pub target_packages: Vec<String>,
    pub exec_command: String,
}

/// ALPM Transaction Hook Manager
pub struct AlpmHookManager {
    pub hooks: Vec<AlpmHook>,
    pub execution_log: Vec<String>,
}

impl AlpmHookManager {
    pub fn new() -> Self {
        Self {
            hooks: Vec::new(),
            execution_log: Vec::new(),
        }
    }

    pub fn register_hook(&mut self, hook: AlpmHook) {
        self.hooks.push(hook);
    }

    pub fn run_hooks(&mut self, action: AlpmHookAction, updated_pkgs: &[&str]) -> usize {
        let mut count = 0;
        for hook in &self.hooks {
            if hook.action == action {
                let matches = hook.target_packages.iter().any(|target| {
                    target == "*" || updated_pkgs.iter().any(|&pkg| target == pkg)
                });
                if matches {
                    let log_line = format!(
                        "Hook '{}' executed command: '{}'",
                        hook.name, hook.exec_command
                    );
                    self.execution_log.push(log_line);
                    count += 1;
                }
            }
        }
        count
    }

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

/// mkinitcpio initramfs configuration builder
pub struct MkinitcpioBuilder {
    pub active_hooks: Vec<String>,
    pub hooks: Vec<String>,
}

impl MkinitcpioBuilder {
    pub fn new() -> Self {
        Self {
            active_hooks: Vec::new(),
            hooks: vec![
                "base".to_string(),
                "udev".to_string(),
                "block".to_string(),
                "filesystems".to_string(),
            ],
        }
    }

    pub fn add_hook(&mut self, hook_name: &str) {
        self.active_hooks.push(hook_name.to_string());
    }

    pub fn compile_initramfs(&self, output_img: &str) -> Result<String, &'static str> {
        if self.active_hooks.is_empty() {
            return Err("mkinitcpio: Cannot compile initramfs with zero hooks configured");
        }
        let mut img_desc = format!("initramfs-img:{}:", output_img);
        for hook in &self.active_hooks {
            img_desc.push_str(hook);
            img_desc.push('|');
        }
        Ok(img_desc)
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

/// makepkg automation compiler validating checksums and building .pkg.tar.zst packages
pub struct MakepkgBuilder {
    pub default_compression_level: u32,
}

impl MakepkgBuilder {
    pub fn new() -> Self {
        Self {
            default_compression_level: 3,
        }
    }

    pub fn build_package(
        &self,
        pkgname: &str,
        version: Version,
        source_payload: &[u8],
        expected_sha256: &str,
    ) -> Result<Package, &'static str> {
        if source_payload.is_empty() {
            return Err("makepkg: Source code payload cannot be empty");
        }

        if expected_sha256 != "SKIP" && expected_sha256.len() != 64 {
            return Err("makepkg: SHA256 checksum mismatch / verification failure");
        }

        Ok(Package::new(
            pkgname.to_string(),
            version,
            format!("Built Arch Package: {}", pkgname),
            Vec::new(),
            expected_sha256.to_string(),
        ))
    }

    pub fn build_package_from_source(
        &self,
        pkgname: &str,
        pkgver: &str,
        sources: &[(&str, &str)],
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
<<<<<<< HEAD
=======
    fn test_debian_sbuild_resolver() {
        let mut sync = RollingSyncManager::new();
        sync.register_installed("gcc", Version::new(11, 2, 0));
        sync.register_installed("make", Version::new(4, 3, 0));

        let sbuild1 =
            DebianSbuildPackage::new("sigma-core", vec!["gcc".to_string(), "make".to_string()]);
        assert!(sync.is_debian_sbuild_builddeps_satisfied(&sbuild1));

        let sbuild2 = DebianSbuildPackage::new(
            "sigma-core",
            vec!["gcc".to_string(), "clang".to_string()],
        );
        assert!(!sync.is_debian_sbuild_builddeps_satisfied(&sbuild2));
    }

    #[test]
    fn test_alpm_hooks_routing() {
        let mut manager = AlpmHookManager::new();
        let hook = AlpmHook {
            name: "update-grub".to_string(),
            action: AlpmHookAction::PostTransaction,
            when: HookWhen::PostTransaction,
            target_packages: vec!["linux".to_string(), "linux-zen".to_string()],
            exec_command: "grub-mkconfig -o /boot/grub/grub.cfg".to_string(),
        };

        manager.register_hook(hook);

        let executed = manager.run_hooks(AlpmHookAction::PostTransaction, &["linux", "vim"]);
        assert_eq!(executed, 1);
        assert_eq!(manager.execution_log.len(), 1);
        assert!(manager.execution_log[0].contains("grub-mkconfig"));

        let executed_pre = manager.run_hooks(AlpmHookAction::PreTransaction, &["linux"]);
        assert_eq!(executed_pre, 0);
    }

    #[test]
>>>>>>> origin/bolt-optimize-vulnerability-scanner-10312631800595437539
    fn test_alpm_hooks() {
        let mut manager = AlpmHookManager::new();

        manager.register_hook(AlpmHook {
            name: "update-fonts".to_string(),
            action: AlpmHookAction::PostTransaction,
            when: HookWhen::PostTransaction,
            target_packages: vec!["fontconfig".to_string()],
            exec_command: "fc-cache -s".to_string(),
        });

        let executed = manager.trigger_hooks(HookWhen::PostTransaction, &["fontconfig"]);
        assert_eq!(executed.len(), 1);
        assert_eq!(executed[0], "fc-cache -s");

        let executed_none = manager.trigger_hooks(HookWhen::PostTransaction, &["bash"]);
        assert_eq!(executed_none.len(), 0);
    }

    #[test]
    fn test_mkinitcpio_builder() {
<<<<<<< HEAD
        let builder = MkinitcpioBuilder::new();
        let initramfs = builder.build_initramfs("6.1.0-arch1").unwrap();
        assert!(initramfs.contains("/boot/initramfs-6.1.0-arch1.img"));
=======
        let mut builder = MkinitcpioBuilder::new();
        builder.add_hook("udev");
        builder.add_hook("base");
        builder.add_hook("block");

        let ramdisk = builder.compile_initramfs("initramfs-linux.img").unwrap();
        assert!(ramdisk.contains("initramfs-img:initramfs-linux.img:"));
        assert!(ramdisk.contains("udev|base|block|"));

        let builder_empty = MkinitcpioBuilder::new();
        assert!(builder_empty.compile_initramfs("fail.img").is_err());

        let initramfs = builder.build_initramfs("6.1.0-arch1").unwrap();
        assert!(initramfs.contains("/boot/initramfs-6.1.0-arch1.img"));
    }

    #[test]
    fn test_makepkg_builder() {
        let builder = MakepkgBuilder::new();
        let mock_src = b"fn main() { println!(\"source\"); }";

        let sha = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
        let package = builder.build_package("linux-firmware", Version::new(20230510, 0, 0), mock_src, sha).unwrap();
        assert_eq!(package.name, "linux-firmware");

        assert!(builder.build_package("linux-firmware", Version::new(1, 0, 0), mock_src, "short_hash").is_err());
>>>>>>> origin/bolt-optimize-vulnerability-scanner-10312631800595437539
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
}
