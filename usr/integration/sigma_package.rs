// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/integration/sigma_package.rs — Sigma Package Manager
//
// Implements declarative package management supporting Nixpkgs,
// Flatpak, Homebrew, and Chocolatey-style package installation.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Package Types ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PackageManager {
    Nixpkgs,
    Flatpak,
    Homebrew,
    Chocolatey,
}

#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub manager: PackageManager,
    pub description: String,
    pub dependencies: Vec<String>,
    pub installed: bool,
}

#[derive(Debug, Clone)]
pub struct PackageManifest {
    pub name: String,
    pub version: String,
    pub packages: Vec<Package>,
    pub environment: HashMap<String, String>,
}

// ─── Package Manager ─────────────────────────────────────────────────────────

pub struct SigmaPackageManager {
    pub packages: HashMap<String, Package>,
    pub manifests: Vec<PackageManifest>,
    pub current_manager: PackageManager,
}

impl SigmaPackageManager {
    pub fn new() -> Self {
        let mut manager = SigmaPackageManager {
            packages: HashMap::new(),
            manifests: Vec::new(),
            current_manager: PackageManager::Nixpkgs,
        };
        
        manager.init_package_database();
        manager
    }

    /// Initialize package database
    fn init_package_database(&mut self) {
        // Nixpkgs packages
        self.packages.insert("nixpkgs_neovim".to_string(), Package {
            name: "neovim".to_string(),
            version: "0.9.0".to_string(),
            manager: PackageManager::Nixpkgs,
            description: "Vim-fork focused on extensibility and usability".to_string(),
            dependencies: vec!["lua".to_string(), "luajit".to_string()],
            installed: false,
        });

        self.packages.insert("nixpkgs_git".to_string(), Package {
            name: "git".to_string(),
            version: "2.42.0".to_string(),
            manager: PackageManager::Nixpkgs,
            description: "Distributed version control system".to_string(),
            dependencies: vec![],
            installed: false,
        });

        // Flatpak packages
        self.packages.insert("flatpak_libreoffice".to_string(), Package {
            name: "org.libreoffice.LibreOffice".to_string(),
            version: "7.6.0".to_string(),
            manager: PackageManager::Flatpak,
            description: "Complete office suite".to_string(),
            dependencies: vec!["runtime/org.freedesktop.Platform".to_string()],
            installed: false,
        });

        self.packages.insert("flatpak_gimp".to_string(), Package {
            name: "org.gimp.GIMP".to_string(),
            version: "2.10.34".to_string(),
            manager: PackageManager::Flatpak,
            description: "GNU Image Manipulation Program".to_string(),
            dependencies: vec!["runtime/org.freedesktop.Platform".to_string()],
            installed: false,
        });

        // Homebrew packages
        self.packages.insert("homebrew_vim".to_string(), Package {
            name: "vim".to_string(),
            version: "9.0.0".to_string(),
            manager: PackageManager::Homebrew,
            description: "Vi IMproved - enhanced vi editor".to_string(),
            dependencies: vec!["lua".to_string()],
            installed: false,
        });

        self.packages.insert("homebrew_node".to_string(), Package {
            name: "node".to_string(),
            version: "20.0.0".to_string(),
            manager: PackageManager::Homebrew,
            description: "JavaScript runtime built on Chrome's V8 engine".to_string(),
            dependencies: vec![],
            installed: false,
        });

        // Chocolatey packages
        self.packages.insert("choco_vscode".to_string(), Package {
            name: "vscode".to_string(),
            version: "1.85.0".to_string(),
            manager: PackageManager::Chocolatey,
            description: "Visual Studio Code".to_string(),
            dependencies: vec![],
            installed: false,
        });

        self.packages.insert("choco_python".to_string(), Package {
            name: "python".to_string(),
            version: "3.12.0".to_string(),
            manager: PackageManager::Chocolatey,
            description: "Python programming language".to_string(),
            dependencies: vec![],
            installed: false,
        });
    }

    /// Set current package manager
    pub fn set_manager(&mut self, manager: PackageManager) {
        self.current_manager = manager;
    }

    /// Get package by name
    pub fn get_package(&self, name: &str) -> Option<&Package> {
        self.packages.get(name)
    }

    /// Get packages by manager
    pub fn get_packages_by_manager(&self, manager: PackageManager) -> Vec<&Package> {
        self.packages.values()
            .filter(|p| p.manager == manager)
            .collect()
    }

    /// Get all packages
    pub fn get_all_packages(&self) -> Vec<&Package> {
        self.packages.values().collect()
    }

    /// Install package (simulated)
    pub fn install_package(&mut self, name: &str) -> Result<(), String> would be installed from {}", self.get_manager_name(p.manager), p.version);
        
        // Check dependencies
        for dep in &p.dependencies {
            println!("Checking dependency: {}", dep);
            if let Some(dep_pkg) = self.packages.get(&format!("{}_{}", self.get_manager_prefix(p.manager), dep)) {
                if !dep_pkg.installed {
                    println!("Installing dependency: {}", dep);
                }
            }
        }
        
        if let Some(pkg) = self.packages.get_mut(name) {
            pkg.installed = true;
        }
        
        Ok(())
    }

    /// Uninstall package (simulated)
    pub fn uninstall_package(&mut self, name: &str) -> Result<(), String> {
        if let Some(pkg) = self.packages.get(name) {
            if !pkg.installed {
                return Err("Package not installed".to_string());
            }
            
            println!("Uninstalling {} {} from {}", pkg.name, pkg.version, self.get_manager_name(pkg.manager));
            
            if let Some(pkg) = self.packages.get_mut(name) {
                pkg.installed = false;
            }
            
            Ok(())
        } else {
            Err("Package not found".to_string())
        }
    }

    /// Search packages
    pub fn search_packages(&self, query: &str) -> Vec<&Package> {
        self.packages.values()
            .filter(|p| {
                p.name.to_lowercase().contains(&query.to_lowercase()) ||
                p.description.to_lowercase().contains(&query.to_lowercase())
            })
            .collect()
    }

    /// Create manifest
    pub fn create_manifest(&mut self, name: String, version: String, package_names: Vec<String>) -> PackageManifest {
        let mut packages = Vec::new();
        let mut environment = HashMap::new();
        
        environment.insert("PATH".to_string(), "/usr/local/bin:/usr/bin".to_string());
        environment.insert("LD_LIBRARY_PATH".to_string(), "/usr/local/lib".to_string());
        
        for pkg_name in package_names {
            if let Some(pkg) = self.packages.get(&pkg_name) {
                packages.push(pkg.clone());
            }
        }
        
        let manifest = PackageManifest {
            name,
            version,
            packages,
            environment,
        };
        
        self.manifests.push(manifest.clone());
        manifest
    }

    /// Get manager name
    fn get_manager_name(&self, manager: PackageManager) -> &str {
        match manager {
            PackageManager::Nixpkgs => "Nixpkgs",
            PackageManager::Flatpak => "Flatpak",
            PackageManager::Homebrew => "Homebrew",
            PackageManager::Chocolatey => "Chocolatey",
        }
    }

    /// Get manager prefix
    fn get_manager_prefix(&self, manager: PackageManager) -> &str {
        match manager {
            PackageManager::Nixpkgs => "nixpkgs",
            PackageManager::Flatpak => "flatpak",
            PackageManager::Homebrew => "homebrew",
            PackageManager::Chocolatey => "choco",
        }
    }

    /// Get all manifests
    pub fn get_manifests(&self) -> &[PackageManifest] {
        &self.manifests
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────--

fn main() {
    let mut manager = SigmaPackageManager::new();
    
    println!("Sigma Package Manager v0.1 - Nixpkgs/Flatpak/Homebrew/Chocolatey");
    
    loop {
        println!("\n--- Current Manager: {} ---", manager.get_manager_name(manager.current_manager));
        println!("\nCommands: manager <type>, search <query>, install <name>, uninstall <name>, list, manifest <name> <version> <packages>, manifests, quit");
        println!("Managers: nixpkgs, flatpak, homebrew, chocolatey");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "manager" => {
                if let Some(arg) = parts.get(1) {
                    let pkg_manager = match *arg {
                        "nixpkgs" => PackageManager::Nixpkgs,
                        "flatpak" => PackageManager::Flatpak,
                        "homebrew" => PackageManager::Homebrew,
                        "chocolatey" => PackageManager::Chocolatey,
                        _ => {
                            println!("Unknown package manager");
                            continue;
                        }
                    };
                    manager.set_manager(pkg_manager);
                    println!("Package manager changed to {}", manager.get_manager_name(pkg_manager));
                }
            }
            "search" => {
                if parts.len() >= 2 {
                    let query = parts[1..].join(" ");
                    let results = manager.search_packages(&query);
                    println!("--- Search Results for '{}' ---", query);
                    for pkg in results {
                        let status = if pkg.installed { "[INSTALLED]" } else { "" };
                        println!("{} {} ({}) {} - {}", pkg.name, pkg.version, manager.get_manager_name(pkg.manager), status, pkg.description);
                    }
                }
            }
            "install" => {
                if let Some(arg) = parts.get(1) {
                    match manager.install_package(arg) {
                        Ok(_) => println!("Package installed successfully"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "uninstall" => {
                if let Some(arg) = parts.get(1) {
                    match manager.uninstall_package(arg) {
                        Ok(_) => println!("Package uninstalled successfully"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "list" => {
                println!("--- All Packages ---");
                for pkg in manager.get_all_packages() {
                    let status = if pkg.installed { "[INSTALLED]" } else { "" };
                    println!("{} {} ({}) {} - {}", pkg.name, pkg.version, manager.get_manager_name(pkg.manager), status, pkg.description);
                }
            }
            "manifest" => {
                if parts.len() >= 3 {
                    let name = parts[1].to_string();
                    let version = parts[2].to_string();
                    let package_names: Vec<String> = parts[3..].iter().map(|s| s.to_string()).collect();
                    let manifest = manager.create_manifest(name, version, package_names);
                    println!("Manifest created: {}", manifest.name);
                }
            }
            "manifests" => {
                println!("--- All Manifests ---");
                for manifest in manager.get_manifests() {
                    println!("{} ({})", manifest.name, manifest.version);
                    println!("  Packages: {}", manifest.packages.iter().map(|p| p.name.clone()).collect::<Vec<_>>().join(", "));
                }
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
