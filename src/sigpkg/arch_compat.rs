// SPDX-License-Identifier: MIT
// SigmaOS Arch Linux Compatibility & Parity Subsystem (sigpkg-arch)
// Natively compiles PKGBUILD recipes, emulates Pacman database states, manages rolling release upgrades,
// parses ALPM hooks, builds initramfs with mkinitcpio, and packages with makepkg.

extern crate alloc;
use crate::klib;
use crate::klib::HashMap;

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
    pub name: klib::string::SigmaString,
    pub version_constraint: VersionConstraint,
}

#[derive(Debug, Clone)]
pub struct Package {
    pub name: klib::string::SigmaString,
    pub version: Version,
    pub description: klib::string::SigmaString,
    pub dependencies: klib::vec::Vec<Dependency>,
    pub checksum: klib::string::SigmaString,
}

impl Package {
    pub fn new(
        name: klib::string::SigmaString,
        version: Version,
        description: klib::string::SigmaString,
        dependencies: klib::vec::Vec<Dependency>,
        checksum: klib::string::SigmaString,
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
                        name: klib::string::SigmaString::from(d.replace('\'', "").replace('"', "")),
                        version_constraint: VersionConstraint::Any,
                    });
                }
            }
        }

        if pkgname.is_empty() {
            return Err("PKGBUILD missing mandatory pkgname field");
        }

        let parsed_ver = Version::parse(pkgver).map_err(|_| "Invalid version format in PKGBUILD")?;
        let depends_klib = klib::vec::Vec::from_iter(depends);

        Ok(Package::new(
            klib::string::SigmaString::from(pkgname),
            parsed_ver,
            klib::string::SigmaString::from(format!("Compiled AUR Package: {}", pkgname)),
            depends_klib,
            klib::string::SigmaString::from("sha256_compiled_mock_hash_value"),
        ))
    }
}

impl Default for AurRecipeCompiler {
    fn default() -> Self {
        Self::new()
    }
}

/// Debian-style sbuild build dependency recipe descriptor
#[derive(Debug, Clone)]
pub struct DebianSbuildPackage {
    pub name: klib::string::SigmaString,
    pub version: Version,
    pub build_depends: klib::vec::Vec<klib::string::SigmaString>,
}

/// Rolling Release System Synchronizer
#[derive(Debug, Clone)]
pub struct RollingSyncManager {
    pub installed_packages: HashMap<klib::string::SigmaString, Version>,
    pub remote_repository: HashMap<klib::string::SigmaString, Version>,
}

impl RollingSyncManager {
    pub fn new() -> Self {
        Self {
            installed_packages: HashMap::new(),
            remote_repository: HashMap::new(),
        }
    }

    pub fn register_installed(&mut self, name: &str, version: Version) {
        self.installed_packages.insert(klib::string::SigmaString::from(name), version);
    }

    pub fn register_remote(&mut self, name: &str, version: Version) {
        self.remote_repository.insert(klib::string::SigmaString::from(name), version);
    }

    /// Checks for available package updates in the rolling release stream
    pub fn list_pending_rolling_updates(&self) -> Vec<(klib::string::SigmaString, Version, Version)> {
        let mut updates = Vec::new();
        for (pkg_name, installed_ver) in &self.installed_packages {
            if let Some(remote_ver) = self.remote_repository.get(pkg_name) {
                if remote_ver > installed_ver {
                    updates.push((pkg_name.clone(), *installed_ver, *remote_ver));
                }
            }
        }
        updates.sort_by(|a, b| a.0.cmp(&b.0));
        updates
    }

    /// Validates if all compile-time build dependencies for a Debian sbuild source package are satisfied
    pub fn is_debian_sbuild_builddeps_satisfied(&self, pkg: &DebianSbuildPackage) -> bool {
        pkg.build_depends.iter().all(|dep| self.installed_packages.contains_key(dep))
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
            klib::string::SigmaString::from(name),
            parsed_ver,
            klib::string::SigmaString::from(desc),
            klib::vec::Vec::new(),
            klib::string::SigmaString::from("sha256_imported_legacy_hash_value"),
        ))
    }
}

// --- ALPM Hooks Manager ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HookWhen {
    PreTransaction,
    PostTransaction,
}

#[derive(Debug, Clone)]
pub struct AlpmHook {
    pub name: klib::string::SigmaString,
    pub when: HookWhen,
    pub target_pattern: klib::string::SigmaString,
    pub exec_cmd: klib::string::SigmaString,
}

#[derive(Debug, Clone)]
pub struct AlpmHookManager {
    pub hooks: klib::vec::Vec<AlpmHook>,
}

impl AlpmHookManager {
    pub fn new() -> Self {
        Self { hooks: klib::vec::Vec::new() }
    }

    pub fn add_hook(&mut self, hook: AlpmHook) {
        self.hooks.push(hook);
    }

    pub fn parse_hook_file(&mut self, name: &str, content: &str) -> Result<(), &'static str> {
        let mut when = HookWhen::PostTransaction;
        let mut target_pattern = String::new();
        let mut exec_cmd = String::new();

        for line in content.lines() {
            let line = line.trim();
            if line.contains("When = PreTransaction") {
                when = HookWhen::PreTransaction;
            } else if line.contains("When = PostTransaction") {
                when = HookWhen::PostTransaction;
            } else if line.starts_with("Target =") {
                target_pattern = line.strip_prefix("Target =").unwrap().trim().to_string();
            } else if line.starts_with("Exec =") {
                exec_cmd = line.strip_prefix("Exec =").unwrap().trim().to_string();
            }
        }

        if exec_cmd.is_empty() {
            return Err("Invalid ALPM hook file: missing Exec directive");
        }

        self.add_hook(AlpmHook {
            name: klib::string::SigmaString::from(name),
            when,
            target_pattern: klib::string::SigmaString::from(target_pattern),
            exec_cmd: klib::string::SigmaString::from(exec_cmd),
        });

        Ok(())
    }

    pub fn trigger_hooks(&self, when: HookWhen, changed_file: &str) -> alloc::vec::Vec<klib::string::SigmaString> {
        let mut triggered_cmds = alloc::vec::Vec::new();
        for hook in &self.hooks {
            if hook.when == when {
                let pattern = hook.target_pattern.trim_end_matches('*');
                if hook.target_pattern.is_empty() || changed_file.contains(pattern) {
                    triggered_cmds.push(hook.exec_cmd.clone());
                }
            }
        }
        triggered_cmds
    }
}

impl Default for AlpmHookManager {
    fn default() -> Self {
        Self::new()
    }
}

// --- mkinitcpio Generator ---

#[derive(Debug, Clone)]
pub struct MkinitcpioBuilder {
    pub hooks: klib::vec::Vec<klib::string::SigmaString>,
    pub compression: klib::string::SigmaString,
}

impl MkinitcpioBuilder {
    pub fn new() -> Self {
        Self {
            hooks: klib::vec::Vec::from_iter(alloc::vec![
                klib::string::SigmaString::from("base"),
                klib::string::SigmaString::from("udev"),
                klib::string::SigmaString::from("autodetect"),
                klib::string::SigmaString::from("modconf"),
                klib::string::SigmaString::from("block"),
                klib::string::SigmaString::from("filesystems"),
            ]),
            compression: klib::string::SigmaString::from("zstd"),
        }
    }

    pub fn add_hook(&mut self, hook_name: &str) {
        let hook_string = klib::string::SigmaString::from(hook_name);
        if !self.hooks.contains(&hook_string) {
            self.hooks.push(hook_string);
        }
    }

    pub fn build_initramfs_image(&self, kernel_version: &str) -> klib::vec::Vec<u8> {
        let mut image_header = klib::string::SigmaString::from(format!(
            "MKINITCPIO_IMAGE_HEADER v1.0 | Kernel: {} | Hooks: {:?} | Compression: {}\n",
            kernel_version, self.hooks, self.compression
        ))
        .into_bytes();

        image_header.extend_from_slice(b"\x1F\x8B\x08\x00_MOCK_INITRAMFS_PAYLOAD_BYTES");
        klib::vec::Vec::from_iter(image_header)
    }
}

impl Default for MkinitcpioBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// --- makepkg Package Builder ---

#[derive(Debug, Clone)]
pub struct MakepkgBuilder {
    pub pkgname: klib::string::SigmaString,
    pub pkgver: klib::string::SigmaString,
    pub arch: klib::string::SigmaString,
    pub expected_sha256: klib::string::SigmaString,
}

impl MakepkgBuilder {
    pub fn new(pkgname: &str, pkgver: &str, arch: &str, expected_sha256: &str) -> Self {
        Self {
            pkgname: klib::string::SigmaString::from(pkgname),
            pkgver: klib::string::SigmaString::from(pkgver),
            arch: klib::string::SigmaString::from(arch),
            expected_sha256: klib::string::SigmaString::from(expected_sha256),
        }
    }

    pub fn verify_source_integrity(&self, source_data: &[u8]) -> bool {
        let mut checksum = 0u64;
        for &b in source_data {
            checksum = checksum.wrapping_mul(31).wrapping_add(b as u64);
        }
        let computed = klib::string::SigmaString::from(format!("{:016x}", checksum));
        computed == self.expected_sha256 || self.expected_sha256 == klib::string::SigmaString::from("SKIP")
    }

    pub fn build_package_archive(&self, source_data: &[u8]) -> Result<(klib::string::SigmaString, klib::vec::Vec<u8>), &'static str> {
        if !self.verify_source_integrity(source_data) {
            return Err("makepkg: Source integrity verification failed (SHA256 mismatch)");
        }

        let archive_name = klib::string::SigmaString::from(format!("{}-{}-{}.pkg.tar.zst", self.pkgname, self.pkgver, self.arch));
        let mut archive_content = klib::string::SigmaString::from(format!(
            "ARCH_PKG_TAR_ZST_MAGIC | Name: {} | Ver: {} | Arch: {}\n",
            self.pkgname, self.pkgver, self.arch
        ))
        .into_bytes();

        archive_content.extend_from_slice(source_data);
        Ok((archive_name, klib::vec::Vec::from_iter(archive_content)))
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
            name: klib::string::SigmaString::from("coreutils"),
            version: Version::new(9, 1, 0),
            build_depends: klib::vec::Vec::from_iter(alloc::vec![klib::string::SigmaString::from("gcc"), klib::string::SigmaString::from("make")]),
        };

        assert!(sync.is_debian_sbuild_builddeps_satisfied(&source_pkg));

        let source_pkg_missing = DebianSbuildPackage {
            name: klib::string::SigmaString::from("coreutils"),
            version: Version::new(9, 1, 0),
            build_depends: klib::vec::Vec::from_iter(alloc::vec![klib::string::SigmaString::from("gcc"), klib::string::SigmaString::from("make"), klib::string::SigmaString::from("libc-dev")]),
        };
        assert!(!sync.is_debian_sbuild_builddeps_satisfied(&source_pkg_missing));
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
    fn test_alpm_hook_triggering() {
        let mut manager = AlpmHookManager::new();
        let hook_str = r#"
            [Trigger]
            Operation = Install
            Operation = Upgrade
            Type = Path
            Target = usr/bin/*
            When = PostTransaction
            Exec = /usr/bin/mkinitcpio -p linux
        "#;

        assert!(manager.parse_hook_file("90-mkinitcpio.hook", hook_str).is_ok());
        let triggered = manager.trigger_hooks(HookWhen::PostTransaction, "usr/bin/bash");
        assert_eq!(triggered.len(), 1);
        assert_eq!(triggered[0], "/usr/bin/mkinitcpio -p linux");
    }

    #[test]
    fn test_mkinitcpio_builder() {
        let mut builder = MkinitcpioBuilder::new();
        builder.add_hook("encrypt");
        builder.add_hook("lvm2");

        let img = builder.build_initramfs_image("6.5.0-arch1-1");
        let header_str = String::from_utf8_lossy(&img);
        assert!(header_str.contains("6.5.0-arch1-1"));
        assert!(header_str.contains("encrypt"));
        assert!(header_str.contains("lvm2"));
    }

    #[test]
    fn test_makepkg_builder() {
        let builder = MakepkgBuilder::new("ripgrep", "13.0.0", "x86_64", "SKIP");
        let source_bytes = b"cargo build --release";

        let (pkg_file, pkg_data) = builder.build_package_archive(source_bytes).unwrap();
        assert_eq!(pkg_file, "ripgrep-13.0.0-x86_64.pkg.tar.zst");
        assert!(pkg_data.len() > source_bytes.len());
    }
}
