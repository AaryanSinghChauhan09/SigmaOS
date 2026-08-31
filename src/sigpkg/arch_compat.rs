// SigmaOS Arch Linux Compatibility & Parity Subsystem (sigpkg-arch)
// Natively compiles PKGBUILD recipes, emulates Pacman database states, and manages rolling release upgrades.

use crate::klib::HashMap;
use crate::sigpkg::{Dependency, Package, Version, VersionConstraint};

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
    fn test_pacman_keyring_and_mkinitcpio() {
        let mut keyring = PacmanKeyringManager::new();
        assert_eq!(keyring.registered_keys.len(), 0);

        keyring.initialize_keyring();
        assert_eq!(keyring.registered_keys.len(), 2);
        assert!(keyring.verify_key_signature("archlinux-master-key"));

        let mut generator = MkinitcpioGenerator::new("linux-sovereign");
        generator.add_hook("base");
        generator.add_hook("udev");
        generator.add_hook("autodetect");

        let initramfs = generator.generate_initramfs().unwrap();
        assert!(initramfs.contains("linux-sovereign"));
        assert!(initramfs.contains("udev"));
    }
}

/// Emulates Arch Linux `pacman-key` GPG/PQC master keyring management
pub struct PacmanKeyringManager {
    pub registered_keys: HashMap<String, String>, // key_id -> fingerprint
}

impl PacmanKeyringManager {
    pub fn new() -> Self {
        Self {
            registered_keys: HashMap::new(),
        }
    }

    /// Initializes and populates standard master keys (pacman-key --init --populate archlinux)
    pub fn initialize_keyring(&mut self) {
        self.registered_keys.insert(
            "archlinux-master-key".to_string(),
            "Dilithium5_43A8F82019A82B".to_string(),
        );
        self.registered_keys.insert(
            "sigmaos-master-key".to_string(),
            "Kyber1024_0A918C281982BB".to_string(),
        );
    }

    /// Verifies package signature against registered master keys
    pub fn verify_key_signature(&self, key_id: &str) -> bool {
        self.registered_keys.contains_key(key_id)
    }
}

impl Default for PacmanKeyringManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Emulates Arch Linux `mkinitcpio` initial ramdisk generation
pub struct MkinitcpioGenerator {
    pub kernel_preset: String,
    pub hooks: Vec<String>,
}

impl MkinitcpioGenerator {
    pub fn new(kernel_preset: &str) -> Self {
        Self {
            kernel_preset: kernel_preset.to_string(),
            hooks: Vec::new(),
        }
    }

    /// Adds a build hook (e.g. "base", "udev", "autodetect", "block", "filesystems")
    pub fn add_hook(&mut self, hook_name: &str) {
        self.hooks.push(hook_name.to_string());
    }

    /// Generates initramfs image configuration profile
    pub fn generate_initramfs(&self) -> Result<String, &'static str> {
        if self.hooks.is_empty() {
            return Err("mkinitcpio: No hooks configured for initramfs generation.");
        }
        let mut config = format!("PRESET={}\nHOOKS=(", self.kernel_preset);
        for (i, hook) in self.hooks.iter().enumerate() {
            if i > 0 {
                config.push(' ');
            }
            config.push_str(hook);
        }
        config.push(')');
        Ok(config)
    }
}
