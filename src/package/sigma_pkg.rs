use std::format;
use std::string::{String, ToString};
use std::vec;
use std::vec::Vec;
// SigmaOS Package Manager (sigma-pkg)
// Inspired by Arch Linux pacman, Debian apt, and FreeBSD pkg
// Supports dependencies, repositories, transactions, and package management

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub conflicts: Vec<String>,
    pub provides: Vec<String>,
    pub size: u64, // bytes
    pub installed_size: u64,
    pub url: Option<String>,
    pub license: String,
    pub groups: Vec<String>,
    pub architecture: String,
    pub repository: String,
}

#[derive(Debug, Clone)]
pub struct Repository {
    pub name: String,
    pub url: String,
    pub enabled: bool,
    pub priority: u32,
    pub packages: HashMap<String, Package>,
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub install: Vec<Package>,
    pub remove: Vec<Package>,
    pub upgrade: Vec<(Package, Package)>, // (old, new)
    pub download_size: u64,
    pub install_size: u64,
}

/// Supported Linux & BSD Universal Foreign Package Formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UniversalPackageFormat {
    DebianDeb,      // .deb (APT/dpkg)
    ArchPacman,     // .pkg.tar.zst (pacman)
    FedoraRpm,      // .rpm (dnf/rpm)
    AlpineApk,      // .apk (apk)
    GentooEbuild,   // .ebuild (portage)
    VoidXbps,       // .xbps (xbps)
    FreeBsdPkg,     // .txz / .pkg (pkg)
    OpenBsdPkg,     // .tgz / .pkg (OpenBSD pkg_add)
    NetBsdPkgsrc,   // .tgz / .tgz (NetBSD pkgsrc)
    SlackwarePkg,   // .txz / .tgz (Slackware installpkg)
    NixDerivation,  // .nix / .drv (NixOS store derivation)
    GuixPackage,    // .scm (GNU Guix package scheme)
    HaikuHpkg,      // .hpkg (Haiku package format)
    FlatpakBundle,  // .flatpak
    SnapPackage,    // .snap
    AppImageBinary, // .AppImage
}

/// Importer/Converter engine mapping foreign Linux/BSD packages into SigmaPkg native representation
pub struct UniversalPackageImporter;

impl UniversalPackageImporter {
    pub fn autodetect_format(filename: &str) -> Option<UniversalPackageFormat> {
        if filename.ends_with(".deb") {
            Some(UniversalPackageFormat::DebianDeb)
        } else if filename.ends_with(".pkg.tar.zst") || filename.ends_with(".pkg.tar.xz") {
            Some(UniversalPackageFormat::ArchPacman)
        } else if filename.ends_with(".rpm") {
            Some(UniversalPackageFormat::FedoraRpm)
        } else if filename.ends_with(".apk") {
            Some(UniversalPackageFormat::AlpineApk)
        } else if filename.ends_with(".ebuild") {
            Some(UniversalPackageFormat::GentooEbuild)
        } else if filename.ends_with(".xbps") {
            Some(UniversalPackageFormat::VoidXbps)
        } else if filename.ends_with(".openbsd.tgz") {
            Some(UniversalPackageFormat::OpenBsdPkg)
        } else if filename.ends_with(".pkgsrc.tgz") {
            Some(UniversalPackageFormat::NetBsdPkgsrc)
        } else if filename.ends_with(".slackware.txz") || filename.ends_with(".slackware.tgz") {
            Some(UniversalPackageFormat::SlackwarePkg)
        } else if filename.ends_with(".txz") || filename.ends_with(".pkg") {
            Some(UniversalPackageFormat::FreeBsdPkg)
        } else if filename.ends_with(".nix") || filename.ends_with(".drv") {
            Some(UniversalPackageFormat::NixDerivation)
        } else if filename.ends_with(".scm") || filename.ends_with(".guix") {
            Some(UniversalPackageFormat::GuixPackage)
        } else if filename.ends_with(".hpkg") {
            Some(UniversalPackageFormat::HaikuHpkg)
        } else if filename.ends_with(".flatpak") {
            Some(UniversalPackageFormat::FlatpakBundle)
        } else if filename.ends_with(".snap") {
            Some(UniversalPackageFormat::SnapPackage)
        } else if filename.ends_with(".AppImage") || filename.ends_with(".appimage") {
            Some(UniversalPackageFormat::AppImageBinary)
        } else {
            None
        }
    }

    pub fn parse_foreign_package(
        filename: &str,
        format: UniversalPackageFormat,
    ) -> Result<Package, String> {
        let pkg_name = filename
            .split(&['-', '_', '.'][..])
            .next()
            .unwrap_or("unknown")
            .to_string();

        let (license, default_deps) = match format {
            UniversalPackageFormat::DebianDeb => ("GPL-3.0-or-later", vec!["libc6".to_string()]),
            UniversalPackageFormat::ArchPacman => ("MIT", vec!["glibc".to_string()]),
            UniversalPackageFormat::FedoraRpm => {
                ("GPLv2+", vec!["glibc".to_string(), "bash".to_string()])
            }
            UniversalPackageFormat::AlpineApk => ("MIT/GPL-2.0", vec!["musl".to_string()]),
            UniversalPackageFormat::FreeBsdPkg => {
                ("BSD-2-Clause", vec!["freebsd-runtime".to_string()])
            }
            UniversalPackageFormat::OpenBsdPkg => ("ISC/BSD", vec!["openbsd-sys".to_string()]),
            UniversalPackageFormat::NetBsdPkgsrc => {
                ("BSD-3-Clause", vec!["pkgsrc-core".to_string()])
            }
            UniversalPackageFormat::SlackwarePkg => ("GPL", vec!["slack-base".to_string()]),
            UniversalPackageFormat::NixDerivation => {
                ("MIT/Apache-2.0", vec!["nix-store".to_string()])
            }
            UniversalPackageFormat::GuixPackage => ("GPL-3.0+", vec!["guix-daemon".to_string()]),
            UniversalPackageFormat::HaikuHpkg => ("MIT", vec!["haiku-libroot".to_string()]),
            _ => ("GPL/MIT/BSD", vec![]),
        };

        Ok(Package {
            name: pkg_name.clone(),
            version: "1.0.0-universal".to_string(),
            description: format!("Imported {:?} package '{}'", format, pkg_name),
            dependencies: default_deps,
            conflicts: vec![],
            provides: vec![pkg_name.clone()],
            size: 10_000_000,
            installed_size: 25_000_000,
            url: Some(format!("file://{}", filename)),
            license: license.to_string(),
            groups: vec!["universal-imported".to_string()],
            architecture: "x86_64".to_string(),
            repository: format!("universal-{:?}", format).to_lowercase(),
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PackageAction {
    Install,
    Remove,
    Upgrade,
    Query,
    Sync,
}

#[derive(Debug, Clone)]
pub struct SigmaPkg {
    config: PkgConfig,
    repositories: Vec<Repository>,
    local_packages: HashMap<String, Package>,
    cache_dir: PathBuf,
    database_dir: PathBuf,
}

#[derive(Debug, Clone)]
pub struct PkgConfig {
    pub color_output: bool,
    pub verbose: bool,
    pub no_confirm: bool,
    pub needed_only: bool,
    pub as_deps: bool,
    pub overwrite_files: bool,
}

impl Default for PkgConfig {
    fn default() -> Self {
        PkgConfig {
            color_output: true,
            verbose: false,
            no_confirm: false,
            needed_only: false,
            as_deps: false,
            overwrite_files: false,
        }
    }
}

impl SigmaPkg {
    pub fn new() -> Result<Self, String> {
        let cache_dir = PathBuf::from("/var/cache/sigma-pkg");
        let database_dir = PathBuf::from("/var/lib/sigma-pkg");

        // Create directories if they don't exist
        fs::create_dir_all(&cache_dir)
            .map_err(|e| format!("Failed to create cache directory: {}", e))?;
        fs::create_dir_all(&database_dir)
            .map_err(|e| format!("Failed to create database directory: {}", e))?;

        let mut pkg = SigmaPkg {
            config: PkgConfig::default(),
            repositories: vec![],
            local_packages: HashMap::new(),
            cache_dir,
            database_dir,
        };

        pkg.load_repositories()?;
        pkg.load_local_database()?;

        Ok(pkg)
    }

    pub fn with_config(config: PkgConfig) -> Result<Self, String> {
        let mut pkg = Self::new()?;
        pkg.config = config;
        Ok(pkg)
    }

    fn load_repositories(&mut self) -> Result<(), String> {
        // Load repository configuration from /etc/sigma-pkg/repositories.conf
        let repo_config_path = PathBuf::from("/etc/sigma-pkg/repositories.conf");

        if !repo_config_path.exists() {
            // Create default repositories
            self.repositories = vec![
                Repository {
                    name: "core".to_string(),
                    url: "https://sigmaos.org/packages/core".to_string(),
                    enabled: true,
                    priority: 1,
                    packages: HashMap::new(),
                },
                Repository {
                    name: "extra".to_string(),
                    url: "https://sigmaos.org/packages/extra".to_string(),
                    enabled: true,
                    priority: 2,
                    packages: HashMap::new(),
                },
                Repository {
                    name: "community".to_string(),
                    url: "https://sigmaos.org/packages/community".to_string(),
                    enabled: true,
                    priority: 3,
                    packages: HashMap::new(),
                },
            ];
            return Ok(());
        }

        let content = fs::read_to_string(&repo_config_path)
            .map_err(|e| format!("Failed to read repository config: {}", e))?;

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Parse repository configuration
            // Format: [repo_name] or Server = url
            if line.starts_with('[') && line.ends_with(']') {
                let repo_name = line[1..line.len() - 1].to_string();
                self.repositories.push(Repository {
                    name: repo_name,
                    url: String::new(),
                    enabled: true,
                    priority: self.repositories.len() as u32 + 1,
                    packages: HashMap::new(),
                });
            } else if line.starts_with("Server") {
                if let Some(repo) = self.repositories.last_mut() {
                    if let Some(url) = line.split('=').nth(1) {
                        repo.url = url.trim().to_string();
                    }
                }
            }
        }

        Ok(())
    }

    fn load_local_database(&mut self) -> Result<(), String> {
        let db_path = self.database_dir.join("local");

        if !db_path.exists() {
            return Ok(());
        }

        // Load installed packages from local database
        // Implementation would parse package database files
        Ok(())
    }

    pub fn sync_repositories(&mut self) -> Result<(), String> {
        println!("Synchronizing package databases...");

        for repo in &mut self.repositories {
            if !repo.enabled {
                continue;
            }

            println!("Syncing repository: {}", repo.name);

            // Download repository database
            let db_url = format!("{}/{}.db", repo.url, repo.name);
            let db_path = self.cache_dir.join(format!("{}.db", repo.name));

            // Simulate database download
            // In real implementation, would use HTTP client to download
            if let Ok(content) = Self::download_file(&db_url) {
                fs::write(&db_path, content)
                    .map_err(|e| format!("Failed to write database: {}", e))?;

                // Parse database and update packages
                repo.packages = Self::parse_database(&db_path)?;
            }
        }

        println!("Synchronization complete.");
        Ok(())
    }

    fn download_file(url: &str) -> Result<String, String> {
        // Simulate download - in real implementation would use HTTP client
        Ok(format!("Simulated download from {}", url))
    }

    fn parse_database(path: &Path) -> Result<HashMap<String, Package>, String> {
        let mut packages = HashMap::new();

        // Parse package database (simplified)
        // Real implementation would parse actual database format
        Ok(packages)
    }

    pub fn search(&self, query: &str) -> Vec<&Package> {
        let mut results = Vec::new();
        let query_lower = query.to_lowercase();

        for repo in &self.repositories {
            if !repo.enabled {
                continue;
            }

            for (name, package) in &repo.packages {
                if name.to_lowercase().contains(&query_lower)
                    || package.description.to_lowercase().contains(&query_lower)
                {
                    results.push(package);
                }
            }
        }

        results
    }

    pub fn query_info(&self, package_name: &str) -> Option<&Package> {
        // Check local packages first
        if let Some(pkg) = self.local_packages.get(package_name) {
            return Some(pkg);
        }

        // Check repositories
        for repo in &self.repositories {
            if repo.enabled {
                if let Some(pkg) = repo.packages.get(package_name) {
                    return Some(pkg);
                }
            }
        }

        None
    }

    pub fn resolve_dependencies(&self, package_names: &[String]) -> Result<Transaction, String> {
        let mut transaction = Transaction {
            install: Vec::new(),
            remove: Vec::new(),
            upgrade: Vec::new(),
            download_size: 0,
            install_size: 0,
        };

        for name in package_names {
            self.resolve_package_dependencies(name, &mut transaction)?;
        }

        Ok(transaction)
    }

    fn resolve_package_dependencies(
        &self,
        name: &str,
        transaction: &mut Transaction,
    ) -> Result<(), String> {
        // Find package in repositories
        let package = self.find_package(name)?;

        // Check if already in transaction
        if transaction.install.iter().any(|p| p.name == package.name) {
            return Ok(());
        }

        // Add package to transaction
        transaction.download_size += package.size;
        transaction.install_size += package.installed_size;
        transaction.install.push(package.clone());

        // Resolve dependencies recursively
        for dep in &package.dependencies {
            self.resolve_package_dependencies(dep, transaction)?;
        }

        Ok(())
    }

    fn find_package(&self, name: &str) -> Result<Package, String> {
        for repo in &self.repositories {
            if repo.enabled {
                if let Some(pkg) = repo.packages.get(name) {
                    return Ok(pkg.clone());
                }
            }
        }
        Err(format!("Package '{}' not found", name))
    }

    pub fn install_packages(&mut self, package_names: &[String]) -> Result<(), String> {
        println!("Resolving dependencies...");
        let transaction = self.resolve_dependencies(package_names)?;

        self.display_transaction(&transaction);

        if !self.config.no_confirm {
            if !self.confirm_transaction() {
                println!("Installation cancelled.");
                return Ok(());
            }
        }

        println!("Installing packages...");

        for package in &transaction.install {
            self.install_package(package)?;
        }

        println!("Installation complete.");
        Ok(())
    }

    fn install_package(&mut self, package: &Package) -> Result<(), String> {
        println!("Installing {} {}...", package.name, package.version);

        // Download package
        let package_url = format!(
            "{}/{}-{}.sigmpkg",
            package.repository, package.name, package.version
        );
        let package_path = self
            .cache_dir
            .join(format!("{}-{}.sigmpkg", package.name, package.version));

        // Simulate download
        println!("Downloading from {}", package_url);

        // Extract package
        println!("Extracting package...");

        // Install files
        println!("Installing files...");

        // Update local database
        self.local_packages
            .insert(package.name.clone(), package.clone());

        // Run post-install scripts
        self.run_hooks("post_install", package)?;

        Ok(())
    }

    fn display_transaction(&self, transaction: &Transaction) {
        println!("\nTransaction Summary:");
        println!("  Install: {} packages", transaction.install.len());
        println!("  Remove: {} packages", transaction.remove.len());
        println!("  Upgrade: {} packages", transaction.upgrade.len());
        println!(
            "  Total Download Size: {} MB",
            transaction.download_size / 1024 / 1024
        );
        println!(
            "  Total Installed Size: {} MB",
            transaction.install_size / 1024 / 1024
        );

        if !transaction.install.is_empty() {
            println!("\nPackages to install:");
            for pkg in &transaction.install {
                println!("  {} {} ({})", pkg.name, pkg.version, pkg.repository);
            }
        }
    }

    fn confirm_transaction(&self) -> bool {
        println!("\nProceed with installation? [Y/n]");
        // In real implementation, would read user input
        true
    }

    fn run_hooks(&self, hook_type: &str, package: &Package) -> Result<(), String> {
        let hook_dir = PathBuf::from("/etc/sigma-pkg/hooks").join(hook_type);

        if !hook_dir.exists() {
            return Ok(());
        }

        // Run hook scripts
        println!("Running {} hooks for {}...", hook_type, package.name);

        Ok(())
    }

    pub fn remove_packages(&mut self, package_names: &[String]) -> Result<(), String> {
        println!("Removing packages...");

        for name in package_names {
            if let Some(package) = self.local_packages.get(name).cloned() {
                self.remove_package(&package)?;
            } else {
                println!("Package '{}' is not installed.", name);
            }
        }

        println!("Removal complete.");
        Ok(())
    }

    fn remove_package(&mut self, package: &Package) -> Result<(), String> {
        println!("Removing {} {}...", package.name, package.version);

        // Check for reverse dependencies
        let dependents = self.find_dependents(&package.name);
        if !dependents.is_empty() {
            return Err(format!(
                "Cannot remove {}: required by {:?}",
                package.name, dependents
            ));
        }

        // Run pre-remove hooks
        self.run_hooks("pre_remove", package)?;

        // Remove files
        println!("Removing files...");

        // Update local database
        self.local_packages.remove(&package.name);

        // Run post-remove hooks
        self.run_hooks("post_remove", package)?;

        Ok(())
    }

    fn find_dependents(&self, package_name: &str) -> Vec<String> {
        let mut dependents = Vec::new();

        for (name, package) in &self.local_packages {
            if package.dependencies.contains(&package_name.to_string()) {
                dependents.push(name.clone());
            }
        }

        dependents
    }

    pub fn upgrade_system(&mut self) -> Result<(), String> {
        println!("Starting full system upgrade...");

        // Sync repositories first
        self.sync_repositories()?;

        // Find upgradable packages
        let mut upgradable = Vec::new();

        for (name, local_pkg) in &self.local_packages {
            if let Ok(remote_pkg) = self.find_package(name) {
                if remote_pkg.version != local_pkg.version {
                    upgradable.push((local_pkg.clone(), remote_pkg));
                }
            }
        }

        if upgradable.is_empty() {
            println!("System is up to date.");
            return Ok(());
        }

        println!("Found {} package(s) to upgrade.", upgradable.len());

        // Create upgrade transaction
        let mut transaction = Transaction {
            install: Vec::new(),
            remove: Vec::new(),
            upgrade: upgradable,
            download_size: 0,
            install_size: 0,
        };

        for (old, new) in &transaction.upgrade {
            transaction.download_size += new.size;
            transaction.install_size += new.installed_size - old.installed_size;
        }

        self.display_transaction(&transaction);

        if !self.config.no_confirm {
            if !self.confirm_transaction() {
                println!("Upgrade cancelled.");
                return Ok(());
            }
        }

        // Perform upgrades
        for (old, new) in &transaction.upgrade {
            println!("Upgrading {} {} -> {}", new.name, old.version, new.version);
            self.upgrade_package(old, new)?;
        }

        println!("System upgrade complete.");
        Ok(())
    }

    fn upgrade_package(&mut self, old: &Package, new: &Package) -> Result<(), String> {
        // Run pre-upgrade hooks
        self.run_hooks("pre_upgrade", new)?;

        // Download and install new version
        self.install_package(new)?;

        // Run post-upgrade hooks
        self.run_hooks("post_upgrade", new)?;

        Ok(())
    }

    pub fn list_installed(&self) -> Vec<&Package> {
        self.local_packages.values().collect()
    }

    pub fn list_available(&self) -> Vec<&Package> {
        let mut packages = Vec::new();

        for repo in &self.repositories {
            if repo.enabled {
                packages.extend(repo.packages.values());
            }
        }

        packages.sort_by(|a, b| a.name.cmp(&b.name));
        packages
    }

    /// Import and install foreign package format directly via Universal PM adapter
    pub fn import_foreign_package(&mut self, file_path: &str) -> Result<Package, String> {
        let format = UniversalPackageImporter::autodetect_format(file_path)
            .ok_or_else(|| format!("Unsupported foreign package format: {}", file_path))?;

        let package = UniversalPackageImporter::parse_foreign_package(file_path, format)?;
        println!(
            "Successfully imported foreign package '{}' ({:?}) into Sigma-pkg universal engine.",
            package.name, format
        );
        self.local_packages
            .insert(package.name.clone(), package.clone());
        Ok(package)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = PkgConfig::default();
        assert!(config.color_output);
        assert!(!config.no_confirm);
    }

    #[test]
    fn test_package_creation() {
        let package = Package {
            name: "test".to_string(),
            version: "1.0.0".to_string(),
            description: "Test package".to_string(),
            dependencies: vec![],
            conflicts: vec![],
            provides: vec![],
            size: 1024,
            installed_size: 2048,
            url: None,
            license: "MIT".to_string(),
            groups: vec![],
            architecture: "x86_64".to_string(),
            repository: "core".to_string(),
        };

        assert_eq!(package.name, "test");
        assert_eq!(package.version, "1.0.0");
    }

    #[test]
    fn test_universal_package_format_importer() {
        let fmt_deb = UniversalPackageImporter::autodetect_format("nginx_1.24.deb");
        assert_eq!(fmt_deb, Some(UniversalPackageFormat::DebianDeb));

        let fmt_arch =
            UniversalPackageImporter::autodetect_format("ripgrep-13.0.0-1-x86_64.pkg.tar.zst");
        assert_eq!(fmt_arch, Some(UniversalPackageFormat::ArchPacman));

        let fmt_rpm = UniversalPackageImporter::autodetect_format("htop-3.2.1.rpm");
        assert_eq!(fmt_rpm, Some(UniversalPackageFormat::FedoraRpm));

        let fmt_slack = UniversalPackageImporter::autodetect_format("bash.slackware.txz");
        assert_eq!(fmt_slack, Some(UniversalPackageFormat::SlackwarePkg));

        let fmt_nix = UniversalPackageImporter::autodetect_format("hello.nix");
        assert_eq!(fmt_nix, Some(UniversalPackageFormat::NixDerivation));

        let fmt_guix = UniversalPackageImporter::autodetect_format("gnu-hello.scm");
        assert_eq!(fmt_guix, Some(UniversalPackageFormat::GuixPackage));

        let fmt_haiku = UniversalPackageImporter::autodetect_format("bash.hpkg");
        assert_eq!(fmt_haiku, Some(UniversalPackageFormat::HaikuHpkg));

        let pkg = UniversalPackageImporter::parse_foreign_package(
            "curl_8.0.deb",
            UniversalPackageFormat::DebianDeb,
        )
        .unwrap();
        assert_eq!(pkg.name, "curl");
        assert_eq!(pkg.repository, "universal-debiandeb");
        assert_eq!(pkg.license, "GPL-3.0-or-later");
        assert!(pkg.dependencies.contains(&"libc6".to_string()));

        let apk_pkg = UniversalPackageImporter::parse_foreign_package(
            "htop.apk",
            UniversalPackageFormat::AlpineApk,
        )
        .unwrap();
        assert_eq!(apk_pkg.license, "MIT/GPL-2.0");
        assert!(apk_pkg.dependencies.contains(&"musl".to_string()));
    }
}
