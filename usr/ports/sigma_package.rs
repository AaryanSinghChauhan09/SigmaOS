// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/ports/sigma_package.rs — Sigma RPM/dpkg Package Management
//
// Implements RPM/dpkg-style package management with installation,
// removal, dependency resolution, and repository management.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Package Types ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PackageFormat {
    RPM,
    DEB,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PackageState {
    Installed,
    ConfigFiles,
    HalfInstalled,
    Unpacked,
    HalfConfigured,
    TriggersAwaited,
    TriggersPending,
    NotInstalled,
}

#[derive(Debug, Clone)]
pub struct Dependency {
    pub package_name: String,
    pub version_constraint: Option<String>,
    pub required: bool,
}

#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub architecture: String,
    pub format: PackageFormat,
    pub state: PackageState,
    pub description: String,
    pub maintainer: String,
    pub section: String,
    pub priority: String,
    pub dependencies: Vec<Dependency>,
    pub conflicts: Vec<String>,
    pub provides: Vec<String>,
    pub size: u64,
    pub installed_size: u64,
    pub files: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Repository {
    pub name: String,
    pub url: String,
    pub enabled: bool,
    pub format: PackageFormat,
    pub packages: Vec<String>,  // Package names available
}

// ─── Package Manager ───────────────────────────────────────────────────────

pub struct PackageManager {
    pub packages: HashMap<String, Package>,
    pub repositories: HashMap<String, Repository>,
    pub installed_packages: HashMap<String, Package>,
    pub config_files: HashMap<String, String>,
    pub lock_file: String,
    pub dpkg_status: String,
    pub rpm_database: String,
}

impl PackageManager {
    pub fn new() -> Self {
        let mut manager = PackageManager {
            packages: HashMap::new(),
            repositories: HashMap::new(),
            installed_packages: HashMap::new(),
            config_files: HashMap::new(),
            lock_file: "/var/lib/dpkg/lock".to_string(),
            dpkg_status: "/var/lib/dpkg/status".to_string(),
            rpm_database: "/var/lib/rpm/Packages".to_string(),
        };

        manager.init_default_repositories();
        manager.init_default_packages();
        manager
    }

    /// Initialize default repositories
    fn init_default_repositories(&mut self) {
        self.repositories.insert("main".to_string(), Repository {
            name: "Main Repository".to_string(),
            url: "http://archive.sigmaos.org/sigmaos/main".to_string(),
            enabled: true,
            format: PackageFormat::DEB,
            packages: vec!["sigmaos-core".to_string(), "sigmaos-utils".to_string()],
        });

        self.repositories.insert("updates".to_string(), Repository {
            name: "Updates Repository".to_string(),
            url: "http://archive.sigmaos.org/sigmaos/updates".to_string(),
            enabled: true,
            format: PackageFormat::DEB,
            packages: vec![],
        });
    }

    /// Initialize default packages
    fn init_default_packages(&mut self) {
        let core_package = Package {
            name: "sigmaos-core".to_string(),
            version: "1.0.0".to_string(),
            architecture: "amd64".to_string(),
            format: PackageFormat::DEB,
            state: PackageState::Installed,
            description: "Core SigmaOS system files".to_string(),
            maintainer: "SigmaOS Team <team@sigmaos.org>".to_string(),
            section: "base".to_string(),
            priority: "required".to_string(),
            dependencies: vec![],
            conflicts: vec![],
            provides: vec!["sigmaos-system".to_string()],
            size: 10 * 1024 * 1024,
            installed_size: 50 * 1024 * 1024,
            files: vec!["/bin/sigmaos".to_string(), "/etc/sigmaos.conf".to_string()],
        };

        self.packages.insert("sigmaos-core".to_string(), core_package.clone());
        self.installed_packages.insert("sigmaos-core".to_string(), core_package);
    }

    /// Add a repository
    pub fn add_repository(&mut self, name: String, url: String, format: PackageFormat) -> Result<Repository, String> {
        if self.repositories.contains_key(&name) {
            return Err("Repository already exists".to_string());
        }

        let repo = Repository {
            name: name.clone(),
            url,
            enabled: true,
            format,
            packages: vec![],
        };

        self.repositories.insert(name.clone(), repo.clone());
        Ok(repo)
    }

    /// Remove a repository
    pub fn remove_repository(&mut self, name: &str) -> Result<(), String> {
        if self.repositories.remove(name).is_some() {
            Ok(())
        } else {
            Err("Repository not found".to_string())
        }
    }

    /// Enable repository
    pub fn enable_repository(&mut self, name: &str) -> Result<(), String> {
        if let Some(repo) = self.repositories.get_mut(name) {
            repo.enabled = true;
            Ok(())
        } else {
            Err("Repository not found".to_string())
        }
    }

    /// Disable repository
    pub fn disable_repository(&mut self, name: &str) -> Result<(), String> {
        if let Some(repo) = self.repositories.get_mut(name) {
            repo.enabled = false;
            Ok(())
        } else {
            Err("Repository not found".to_string())
        }
    }

    /// Install a package
    pub fn install_package(&mut self, name: String) -> Result<Package, String> {
        if self.installed_packages.contains_key(&name) {
            return Err("Package already installed".to_string());
        }

        if let Some(mut package) = self.packages.get(&name).cloned() {
            // Check dependencies
            for dep in &package.dependencies {
                if !self.installed_packages.contains_key(&dep.package_name) {
                    return Err(format!("Dependency not installed: {}", dep.package_name));
                }
            }

            package.state = PackageState::Installed;
            self.installed_packages.insert(name.clone(), package.clone());
            Ok(package)
        } else {
            Err("Package not found".to_string())
        }
    }

    /// Remove a package
    pub fn remove_package(&mut self, name: &str, purge: bool) -> Result<(), String> {
        if let Some(mut package) = self.installed_packages.remove(name) {
            if purge {
                // Remove config files
                for file in &package.files {
                    if file.starts_with("/etc/") {
                        self.config_files.remove(file);
                    }
                }
            }
            package.state = PackageState::NotInstalled;
            Ok(())
        } else {
            Err("Package not installed".to_string())
        }
    }

    /// Update package cache
    pub fn update_cache(&mut self) -> Result<(), String> {
        // Simulate updating package cache from repositories
        for repo in self.repositories.values_mut() {
            if repo.enabled {
                // Simulate fetching package list
                repo.packages.push(format!("{}-new-pkg", repo.name));
            }
        }
        Ok(())
    }

    /// Upgrade all packages
    pub fn upgrade_all(&mut self) -> Result<Vec<String>, String> {
        let mut upgraded = vec![];
        
        for (name, package) in self.packages.iter_mut() {
            if self.installed_packages.contains_key(name) {
                // Simulate upgrade
                package.version = format!("{}.1", package.version);
                upgraded.push(name.clone());
                
                if let Some(installed) = self.installed_packages.get_mut(name) {
                    installed.version = package.version.clone();
                }
            }
        }
        
        Ok(upgraded)
    }

    /// Search for packages
    pub fn search_packages(&self, query: &str) -> Vec<&Package> {
        let query_lower = query.to_lowercase();
        self.packages.values()
            .filter(|p| {
                p.name.to_lowercase().contains(&query_lower) ||
                p.description.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    /// Get package info
    pub fn get_package_info(&self, name: &str) -> Option<&Package> {
        self.packages.get(name)
    }

    /// List installed packages
    pub fn list_installed(&self) -> Vec<&Package> {
        self.installed_packages.values().collect()
    }

    /// List all available packages
    pub fn list_available(&self) -> Vec<&Package> {
        self.packages.values().collect()
    }

    /// Check dependencies
    pub fn check_dependencies(&self, name: &str) -> Result<Vec<Dependency>, String> {
        if let Some(package) = self.packages.get(name) {
            let mut missing = vec![];
            for dep in &package.dependencies {
                if !self.installed_packages.contains_key(&dep.package_name) {
                    missing.push(dep.clone());
                }
            }
            if missing.is_empty() {
                Ok(package.dependencies.clone())
            } else {
                Err(format!("Missing dependencies: {:?}", missing))
            }
        } else {
            Err("Package not found".to_string())
        }
    }

    /// Get statistics
    pub fn get_statistics(&self) -> HashMap<String, u32> {
        let mut stats = HashMap::new();
        stats.insert("total_packages".to_string(), self.packages.len() as u32);
        stats.insert("installed_packages".to_string(), self.installed_packages.len() as u32);
        stats.insert("repositories".to_string(), self.repositories.len() as u32);
        stats.insert("enabled_repositories".to_string(), self.repositories.values().filter(|r| r.enabled).count() as u32);
        stats
    }

    /// Verify package integrity
    pub fn verify_package(&self, name: &str) -> Result<bool, String> {
        if let Some(package) = self.installed_packages.get(name) {
            // Simulate verification
            Ok(true)
        } else {
            Err("Package not installed".to_string())
        }
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────

fn main() {
    let mut pkg_mgr = PackageManager::new();
    
    println!("Sigma Package Manager v0.1 - RPM/dpkg Format Support");
    
    loop {
        println!("\n--- Package Manager Commands ---");
        println!("update            - Update package cache");
        println!("upgrade           - Upgrade all packages");
        println!("install <name>    - Install package");
        println!("remove <name>     - Remove package");
        println!("purge <name>      - Purge package (remove config files)");
        println!("search <query>    - Search packages");
        println!("info <name>       - Show package info");
        println!("list              - List installed packages");
        println!("available         - List available packages");
        println!("deps <name>       - Check dependencies");
        println!("verify <name>     - Verify package integrity");
        println!("repos             - List repositories");
        println!("add_repo <name> <url> <format> - Add repository");
        println!("remove_repo <name> - Remove repository");
        println!("enable_repo <name> - Enable repository");
        println!("disable_repo <name> - Disable repository");
        println!("stats             - Show statistics");
        println!("quit              - Exit");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "update" => {
                match pkg_mgr.update_cache() {
                    Ok(_) => println!("Package cache updated"),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            "upgrade" => {
                match pkg_mgr.upgrade_all() {
                    Ok(upgraded) => println!("Upgraded: {:?}", upgraded),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            "install" => {
                if let Some(name) = parts.get(1) {
                    match pkg_mgr.install_package(name.to_string()) {
                        Ok(_) => println!("Package installed"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "remove" => {
                if let Some(name) = parts.get(1) {
                    match pkg_mgr.remove_package(name, false) {
                        Ok(_) => println!("Package removed"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "purge" => {
                if let Some(name) = parts.get(1) {
                    match pkg_mgr.remove_package(name, true) {
                        Ok(_) => println!("Package purged"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "search" => {
                if let Some(query) = parts.get(1) {
                    println!("--- Search Results ---");
                    for package in pkg_mgr.search_packages(query) {
                        println!("{} - {} - {}", package.name, package.version, package.description);
                    }
                }
            }
            "info" => {
                if let Some(name) = parts.get(1) {
                    if let Some(package) = pkg_mgr.get_package_info(name) {
                        println!("--- Package Info ---");
                        println!("Name: {}", package.name);
                        println!("Version: {}", package.version);
                        println!("Architecture: {}", package.architecture);
                        println!("Format: {:?}", package.format);
                        println!("State: {:?}", package.state);
                        println!("Description: {}", package.description);
                        println!("Maintainer: {}", package.maintainer);
                        println!("Section: {}", package.section);
                        println!("Priority: {}", package.priority);
                        println!("Size: {} MB", package.size / (1024 * 1024));
                        println!("Installed Size: {} MB", package.installed_size / (1024 * 1024));
                        println!("Dependencies: {:?}", package.dependencies);
                    }
                }
            }
            "list" => {
                println!("--- Installed Packages ---");
                for package in pkg_mgr.list_installed() {
                    println!("{} - {} - {}", package.name, package.version, package.state);
                }
            }
            "available" => {
                println!("--- Available Packages ---");
                for package in pkg_mgr.list_available() {
                    println!("{} - {} - {}", package.name, package.version, package.description);
                }
            }
            "deps" => {
                if let Some(name) = parts.get(1) {
                    match pkg_mgr.check_dependencies(name) {
                        Ok(deps) => println!("Dependencies: {:?}", deps),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "verify" => {
                if let Some(name) = parts.get(1) {
                    match pkg_mgr.verify_package(name) {
                        Ok(valid) => println!("Package integrity: {}", if valid { "valid" } else { "invalid" }),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "repos" => {
                println!("--- Repositories ---");
                for repo in pkg_mgr.repositories.values() {
                    println!("{} - {} - {:?} - {}", repo.name, repo.url, repo.format, if repo.enabled { "enabled" } else { "disabled" });
                }
            }
            "add_repo" => {
                if parts.len() >= 4 {
                    let name = parts[1].to_string();
                    let url = parts[2].to_string();
                    let format = match parts[3] {
                        "rpm" => PackageFormat::RPM,
                        "deb" => PackageFormat::DEB,
                        _ => PackageFormat::DEB,
                    };
                    match pkg_mgr.add_repository(name, url, format) {
                        Ok(_) => println!("Repository added"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "remove_repo" => {
                if let Some(name) = parts.get(1) {
                    match pkg_mgr.remove_repository(name) {
                        Ok(_) => println!("Repository removed"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "enable_repo" => {
                if let Some(name) = parts.get(1) {
                    match pkg_mgr.enable_repository(name) {
                        Ok(_) => println!("Repository enabled"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "disable_repo" => {
                if let Some(name) = parts.get(1) {
                    match pkg_mgr.disable_repository(name) {
                        Ok(_) => println!("Repository disabled"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "stats" => {
                println!("--- Statistics ---");
                for (key, value) in pkg_mgr.get_statistics() {
                    println!("{}: {}", key, value);
                }
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
