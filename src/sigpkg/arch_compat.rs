// SPDX-License-Identifier: MIT
// SigmaOS Arch Linux Compatibility & Parity Subsystem (sigpkg-arch)
// Natively compiles PKGBUILD recipes, emulates Pacman database states, manages rolling release upgrades,
// parses ALPM hooks, builds initramfs with mkinitcpio, and packages with makepkg.

extern crate alloc;
use alloc::{format, vec, string::ToString, string::String, vec::Vec};
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

/// Debian-style sbuild build dependency recipe descriptor
#[derive(Debug, Clone)]
pub struct DebianSbuildPackage {
    pub name: String,
    pub version: Version,
    pub build_depends: Vec<String>,
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
            name.to_string(),
            parsed_ver,
            desc.to_string(),
            Vec::new(),
            "sha256_imported_legacy_hash_value".to_string(),
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
    pub name: String,
    pub when: HookWhen,
    pub target_pattern: String,
    pub exec_cmd: String,
}

#[derive(Debug, Clone)]
pub struct AlpmHookManager {
    pub hooks: Vec<AlpmHook>,
}

impl AlpmHookManager {
    pub fn new() -> Self {
        Self { hooks: Vec::new() }
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
            name: name.to_string(),
            when,
            target_pattern,
            exec_cmd,
        });

        Ok(())
    }

    pub fn trigger_hooks(&self, when: HookWhen, changed_file: &str) -> Vec<String> {
        let mut triggered_cmds = Vec::new();
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
    pub hooks: Vec<String>,
    pub compression: String,
}

impl DebianSbuildPackage {
    pub fn new(name: &str, build_depends: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            version: Version::new(1, 0, 0),
            build_depends,
        }
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

        let (pkg_file, pkg_data): (klib::string::SigmaString, klib::vec::Vec<u8>) = builder.build_package_archive(source_bytes).unwrap();
        assert_eq!(pkg_file, "ripgrep-13.0.0-x86_64.pkg.tar.zst");
        assert!(pkg_data.len() > source_bytes.len());
    }
}
