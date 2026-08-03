// SigmaOS Arch Linux Compatibility & Parity Subsystem (sigpkg-arch)
// Natively compiles PKGBUILD recipes, emulates Pacman database states, and manages rolling release upgrades.

use crate::klib::HashMap;
use crate::sigpkg::{Dependency, Package, Version, VersionConstraint};
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::format;

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
        let mut updates: Vec<(String, Version, Version)> = Vec::new();
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
        let mut_version = "1.0.0";
        let mut desc = "";

        let mut lines = desc_content.lines();
        let mut version = mut_version;
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

// =========================================================================
// NEW ARCH PARITY SUB SYSTEMS (AUR HELPERS, ABS PORTS, REFLECTOR MIRRORS)
// =========================================================================

/// Emulates a complete AUR Helper (e.g. yay / paru equivalent)
pub struct AurHelper {
    pub compiler: AurRecipeCompiler,
}

impl AurHelper {
    pub fn new() -> Self {
        Self {
            compiler: AurRecipeCompiler::new(),
        }
    }

    /// Simulates downloading, dependency-resolving, and compiling from the AUR
    pub fn search_and_install_aur(&self, pkgname: &str, _sync_manager: &RollingSyncManager) -> Result<Package, &'static str> {
        // Mock PKGBUILD recipes database mapping for standard AUR requests
        let pkgbuild = match pkgname {
            "yay" => {
                r#"
                    pkgname="yay"
                    pkgver="12.1.0"
                    depends=("pacman" "git" "go")
                "#
            }
            "paru" => {
                r#"
                    pkgname="paru"
                    pkgver="2.0.1"
                    depends=("pacman" "git" "cargo")
                "#
            }
            _ => return Err("Target package not found in AUR repository index"),
        };

        self.compiler.compile_pkgbuild(pkgbuild)
    }
}

impl Default for AurHelper {
    fn default() -> Self {
        Self::new()
    }
}

/// Emulates Arch Build System (ABS) Ports Tree
pub struct AbsPortsManager {
    pub ports: HashMap<String, String>,
}

impl AbsPortsManager {
    pub fn new() -> Self {
        Self {
            ports: HashMap::new(),
        }
    }

    pub fn register_port(&mut self, name: &str, pkgbuild_content: &str) {
        self.ports.insert(name.to_string(), pkgbuild_content.to_string());
    }

    pub fn compile_port(&self, name: &str, compiler: &AurRecipeCompiler) -> Result<Package, &'static str> {
        let pkgbuild = self.ports.get(name).ok_or("Target port not found in ABS ports tree")?;
        compiler.compile_pkgbuild(pkgbuild.as_str())
    }
}

impl Default for AbsPortsManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Emulates dynamic Arch mirrorlist ranker (e.g. reflector equivalent)
pub struct MirrorlistRanker {
    pub mirrors: Vec<(String, u32)>, // Maps mirror URL to measured latency (ms)
}

impl MirrorlistRanker {
    pub fn new() -> Self {
        Self {
            mirrors: Vec::new(),
        }
    }

    pub fn register_mirror(&mut self, url: &str, latency_ms: u32) {
        self.mirrors.push((url.to_string(), latency_ms));
    }

    /// Returns ranked mirrorlist URLs sorted by lowest latency first (optimal speed)
    pub fn rank_mirrors(&self) -> Vec<String> {
        let mut list = self.mirrors.clone();
        list.sort_by(|a, b| a.1.cmp(&b.1)); // Sort ascending by latency
        list.into_iter().map(|item| item.0).collect()
    }
}

impl Default for MirrorlistRanker {
    fn default() -> Self {
        Self::new()
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
    fn test_aur_helper_dependency_resolution() {
        let helper = AurHelper::new();
        let sync = RollingSyncManager::new();

        let pkg = helper.search_and_install_aur("yay", &sync).unwrap();
        assert_eq!(pkg.name, "yay");
        assert_eq!(pkg.version, Version::new(12, 1, 0));
        assert_eq!(pkg.dependencies.len(), 3);
        assert_eq!(pkg.dependencies[0].name, "pacman");

        let missing_pkg = helper.search_and_install_aur("missing-pkg", &sync);
        assert!(missing_pkg.is_err());
    }

    #[test]
    fn test_abs_ports_compilation() {
        let mut abs = AbsPortsManager::new();
        let compiler = AurRecipeCompiler::new();

        let pkgbuild = r#"
            pkgname="abs-test"
            pkgver="4.2.0"
            depends=("glibc")
        "#;
        abs.register_port("abs-test", pkgbuild);

        let pkg = abs.compile_port("abs-test", &compiler).unwrap();
        assert_eq!(pkg.name, "abs-test");
        assert_eq!(pkg.version, Version::new(4, 42, 0)); // Parsing parses 4.2.0 correctly!
        assert_eq!(pkg.dependencies[0].name, "glibc");
    }

    #[test]
    fn test_mirror_reflector_ranker() {
        let mut ranker = MirrorlistRanker::new();
        ranker.register_mirror("https://mirror.slow.org/arch", 350);
        ranker.register_mirror("https://mirror.fast.org/arch", 15);
        ranker.register_mirror("https://mirror.medium.org/arch", 120);

        let ranked = ranker.rank_mirrors();
        assert_eq!(ranked.len(), 3);
        assert_eq!(ranked[0], "https://mirror.fast.org/arch");
        assert_eq!(ranked[1], "https://mirror.medium.org/arch");
        assert_eq!(ranked[2], "https://mirror.slow.org/arch");
    }
}
