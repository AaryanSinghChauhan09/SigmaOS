// SigmaOS Arch Linux Compatibility & Parity Subsystem (sigpkg-arch)
// Natively compiles PKGBUILD recipes, emulates Pacman database states, and manages rolling release upgrades.

#[cfg(not(test))]
use crate::sigpkg::{Dependency, Package, Version, VersionConstraint};

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

#[cfg(test)]
impl Version {
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self { major, minor, patch }
    }

    pub fn parse(v_str: &str) -> Result<Self, &'static str> {
        let mut parts = v_str.split('.');
        let major = parts.next().ok_or("err")?.parse::<u64>().map_err(|_| "err")?;
        let minor = parts.next().ok_or("err")?.parse::<u64>().map_err(|_| "err")?;
        let patch = parts.next().ok_or("err")?.parse::<u64>().map_err(|_| "err")?;
        Ok(Self::new(major, minor, patch))
    }
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VersionConstraint {
    Exact(Version),
    GreaterThan(Version),
    LessThan(Version),
    GreaterOrEqual(Version),
    LessOrEqual(Version),
    Any,
}

#[cfg(test)]
#[derive(Debug, Clone)]
pub struct Dependency {
    pub name: String,
    pub version_constraint: VersionConstraint,
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
    pub fn new(name: String, version: Version, description: String, dependencies: Vec<Dependency>, checksum: String) -> Self {
        Self { name, version, description, dependencies, checksum }
    }
}

use std::collections::HashMap;

/// Debian-style Sbuild Source Build Dependency Representation
#[derive(Debug, Clone)]
pub struct DebianSbuildPackage {
    pub name: String,
    pub build_depends: Vec<String>,
}

impl DebianSbuildPackage {
    pub fn new(name: &str, build_deps: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            build_depends: build_deps,
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
            if let Some(remote_ver) = self.remote_repository.get(pkg_name) {
                if remote_ver > installed_ver {
                    updates.push((pkg_name.clone(), *installed_ver, *remote_ver));
                }
            }
        }
        updates.sort_by(|a, b| a.0.cmp(&b.0));
        updates
    }

    /// Verifies if a Debian sbuild environment has all required build dependencies satisfied
    pub fn is_debian_sbuild_builddeps_satisfied(
        &self,
        sbuild: &DebianSbuildPackage,
    ) -> bool {
        for dep in &sbuild.build_depends {
            if !self.installed_packages.contains_key(dep) {
                return false;
            }
        }
        true
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

#[cfg(test)]
mod tests {
    use super::*;

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

        // Remotes (Rolling upgrades)
        sync.register_remote("bash", Version::new(5, 2, 0)); // Newer version
        sync.register_remote("curl", Version::new(7, 85, 0)); // Equal version

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
    fn test_debian_sbuild_resolver() {
        let mut sync = RollingSyncManager::new();
        sync.register_installed("gcc", Version::new(11, 2, 0));
        sync.register_installed("make", Version::new(4, 3, 0));

        let sbuild1 = DebianSbuildPackage::new(
            "sigma-core",
            vec!["gcc".to_string(), "make".to_string()],
        );
        assert!(sync.is_debian_sbuild_builddeps_satisfied(&sbuild1));

        let sbuild2 = DebianSbuildPackage::new(
            "sigma-core",
            vec!["gcc".to_string(), "clang".to_string()], // clang not installed
        );
        assert!(!sync.is_debian_sbuild_builddeps_satisfied(&sbuild2));
    }
}
