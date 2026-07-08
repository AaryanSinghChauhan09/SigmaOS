// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// installer/sigma_persona_bundle.rs — Persona Bundle Installer
//
// Implements persona bundle installation for SigmaOS.
// Provides curated software bundles for different user personas.
// Inspired by: Ubuntu flavors, Fedora Spins, KDE/GNOME variants
// Language: Rust (std available for userland tools)

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

// ── Types ─────────────────────────────────────────────────────────────────────
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    BundleError(String),
    ConfigError(String),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

// ── Persona Type ───────────────────────────────────────────────────────────
#[derive(Debug, Clone, PartialEq)]
pub enum Persona {
    /// Developer persona (IDEs, compilers, tools).
    Developer,
    /// Creative persona (graphics, audio, video).
    Creative,
    /// Gaming persona (Steam, Lutris, game launchers).
    Gaming,
    /// Minimalist persona (lightweight tools only).
    Minimal,
    /// Server persona (headless, server tools).
    Server,
    /// Education persona (learning tools, documentation).
    Education,
    /// Custom persona (user-defined).
    Custom(String),
}

// ── Package ───────────────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: String,
    pub size_mb: u32,
    pub dependencies: Vec<String>,
}

// ── Persona Bundle ───────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct PersonaBundle {
    pub persona: Persona,
    pub name: String,
    pub description: String,
    pub packages: Vec<Package>,
    pub total_size_mb: u32,
}

impl PersonaBundle {
    pub fn new(persona: Persona, name: String, description: String) -> Self {
        Self {
            persona,
            name,
            description,
            packages: Vec::new(),
            total_size_mb: 0,
        }
    }

    pub fn add_package(&mut self, package: Package) {
        self.total_size_mb += package.size_mb;
        self.packages.push(package);
    }
}

// ── Bundle Manager ─────────────────────────────────────────────────────────
pub struct BundleManager {
    pub bundles: HashMap<String, PersonaBundle>,
    pub repo_path: PathBuf,
    pub installed_bundles: Vec<String>,
}

impl BundleManager {
    pub fn new(repo_path: PathBuf) -> Self {
        Self {
            bundles: HashMap::new(),
            repo_path,
            installed_bundles: Vec::new(),
        }
    }

    // ── Public API ────────────────────────────────────────────────────────────

    /// Initialize default persona bundles.
    pub fn init_default_bundles(&mut self) {
        // Developer bundle
        let mut dev_bundle = PersonaBundle::new(
            Persona::Developer,
            "developer".to_string(),
            "Development tools and IDEs".to_string(),
        );
        dev_bundle.add_package(Package {
            name: "sigma-ide".to_string(),
            version: "1.0.0".to_string(),
            description: "SigmaOS native IDE".to_string(),
            size_mb: 150,
            dependencies: vec!["sigma-gcc".to_string(), "sigma-rust".to_string()],
        });
        dev_bundle.add_package(Package {
            name: "sigma-gcc".to_string(),
            version: "13.0.0".to_string(),
            description: "GNU Compiler Collection".to_string(),
            size_mb: 300,
            dependencies: vec![],
        });
        dev_bundle.add_package(Package {
            name: "sigma-rust".to_string(),
            version: "1.75.0".to_string(),
            description: "Rust toolchain".to_string(),
            size_mb: 200,
            dependencies: vec![],
        });
        self.bundles.insert("developer".to_string(), dev_bundle);

        // Creative bundle
        let mut creative_bundle = PersonaBundle::new(
            Persona::Creative,
            "creative".to_string(),
            "Creative tools for graphics, audio, and video".to_string(),
        );
        creative_bundle.add_package(Package {
            name: "sigma-gimp".to_string(),
            version: "2.10.0".to_string(),
            description: "Image editor".to_string(),
            size_mb: 250,
            dependencies: vec![],
        });
        creative_bundle.add_package(Package {
            name: "sigma-inkscape".to_string(),
            version: "1.3.0".to_string(),
            description: "Vector graphics editor".to_string(),
            size_mb: 180,
            dependencies: vec![],
        });
        creative_bundle.add_package(Package {
            name: "sigma-audacity".to_string(),
            version: "3.4.0".to_string(),
            description: "Audio editor".to_string(),
            size_mb: 120,
            dependencies: vec![],
        });
        self.bundles.insert("creative".to_string(), creative_bundle);

        // Gaming bundle
        let mut gaming_bundle = PersonaBundle::new(
            Persona::Gaming,
            "gaming".to_string(),
            "Gaming platform and launchers".to_string(),
        );
        gaming_bundle.add_package(Package {
            name: "sigma-steam".to_string(),
            version: "1.0.0".to_string(),
            description: "Steam client".to_string(),
            size_mb: 400,
            dependencies: vec!["sigma-vulkan".to_string()],
        });
        gaming_bundle.add_package(Package {
            name: "sigma-lutris".to_string(),
            version: "0.5.0".to_string(),
            description: "Game launcher".to_string(),
            size_mb: 150,
            dependencies: vec![],
        });
        self.bundles.insert("gaming".to_string(), gaming_bundle);

        // Minimal bundle
        let mut minimal_bundle = PersonaBundle::new(
            Persona::Minimal,
            "minimal".to_string(),
            "Minimal desktop environment".to_string(),
        );
        minimal_bundle.add_package(Package {
            name: "sigma-terminal".to_string(),
            version: "1.0.0".to_string(),
            description: "Terminal emulator".to_string(),
            size_mb: 20,
            dependencies: vec![],
        });
        minimal_bundle.add_package(Package {
            name: "sigma-file-manager".to_string(),
            version: "1.0.0".to_string(),
            description: "File manager".to_string(),
            size_mb: 30,
            dependencies: vec![],
        });
        self.bundles.insert("minimal".to_string(), minimal_bundle);

        // Server bundle
        let mut server_bundle = PersonaBundle::new(
            Persona::Server,
            "server".to_string(),
            "Server tools and services".to_string(),
        );
        server_bundle.add_package(Package {
            name: "sigma-ssh".to_string(),
            version: "9.0.0".to_string(),
            description: "SSH server".to_string(),
            size_mb: 10,
            dependencies: vec![],
        });
        server_bundle.add_package(Package {
            name: "sigma-nginx".to_string(),
            version: "1.24.0".to_string(),
            description: "Web server".to_string(),
            size_mb: 50,
            dependencies: vec![],
        });
        server_bundle.add_package(Package {
            name: "sigma-docker".to_string(),
            version: "24.0.0".to_string(),
            description: "Container runtime".to_string(),
            size_mb: 200,
            dependencies: vec![],
        });
        self.bundles.insert("server".to_string(), server_bundle);

        // Education bundle
        let mut edu_bundle = PersonaBundle::new(
            Persona::Education,
            "education".to_string(),
            "Educational tools and documentation".to_string(),
        );
        edu_bundle.add_package(Package {
            name: "sigma-docs".to_string(),
            version: "1.0.0".to_string(),
            description: "SigmaOS documentation".to_string(),
            size_mb: 100,
            dependencies: vec![],
        });
        edu_bundle.add_package(Package {
            name: "sigma-tutorials".to_string(),
            version: "1.0.0".to_string(),
            description: "Interactive tutorials".to_string(),
            size_mb: 150,
            dependencies: vec![],
        });
        self.bundles.insert("education".to_string(), edu_bundle);
    }

    /// Install a persona bundle.
    pub fn install_bundle(&mut self, bundle_name: &str) -> Result<()> {
        if !self.bundles.contains_key(bundle_name) {
            return Err(Error::BundleError(format!("Bundle '{}' not found", bundle_name)));
        }

        let bundle = self.bundles.get(bundle_name).unwrap().clone();

        println!("Installing bundle: {}", bundle.name);
        println!("Description: {}", bundle.description);
        println!("Total size: {} MB", bundle.total_size_mb);
        println!("Packages: {}", bundle.packages.len());

        // Install each package
        for package in &bundle.packages {
            println!("  Installing {} ({})...", package.name, package.size_mb);
            self.install_package(package)?;
        }

        self.installed_bundles.push(bundle_name.to_string());
        println!("Bundle '{}' installed successfully", bundle_name);
        Ok(())
    }

    /// Install a single package.
    fn install_package(&self, package: &Package) -> Result<()> {
        // In production: use package manager to install
        // For now, simulate installation
        let package_path = self.repo_path.join(&package.name);
        fs::create_dir_all(&package_path)?;
        Ok(())
    }

    /// Remove a persona bundle.
    pub fn remove_bundle(&mut self, bundle_name: &str) -> Result<()> {
        if !self.installed_bundles.contains(&bundle_name.to_string()) {
            return Err(Error::BundleError(format!("Bundle '{}' not installed", bundle_name)));
        }

        let bundle = self.bundles.get(bundle_name).unwrap();

        println!("Removing bundle: {}", bundle_name);

        // Remove each package
        for package in &bundle.packages {
            println!("  Removing {}...", package.name);
            self.remove_package(package)?;
        }

        self.installed_bundles.retain(|b| b != bundle_name);
        println!("Bundle '{}' removed successfully", bundle_name);
        Ok(())
    }

    /// Remove a single package.
    fn remove_package(&self, package: &Package) -> Result<()> {
        let package_path = self.repo_path.join(&package.name);
        if package_path.exists() {
            fs::remove_dir_all(&package_path)?;
        }
        Ok(())
    }

    /// List available bundles.
    pub fn list_bundles(&self) -> Vec<&PersonaBundle> {
        self.bundles.values().collect()
    }

    /// List installed bundles.
    pub fn list_installed(&self) -> Vec<String> {
        self.installed_bundles.clone()
    }

    /// Get bundle by name.
    pub fn get_bundle(&self, name: &str) -> Option<&PersonaBundle> {
        self.bundles.get(name)
    }

    /// Create custom bundle.
    pub fn create_custom_bundle(&mut self, name: String, description: String) {
        let bundle = PersonaBundle::new(Persona::Custom(name.clone()), name.clone(), description);
        self.bundles.insert(name, bundle);
    }

    /// Add package to bundle.
    pub fn add_package_to_bundle(&mut self, bundle_name: &str, package: Package) -> Result<()> {
        if let Some(bundle) = self.bundles.get_mut(bundle_name) {
            bundle.add_package(package);
            Ok(())
        } else {
            Err(Error::BundleError(format!("Bundle '{}' not found", bundle_name)))
        }
    }
}

// ── CLI Interface ─────────────────────────────────────────────────────────────
pub fn run_persona_bundle(args: Vec<String>) -> Result<()> {
    if args.len() < 2 {
        eprintln!("Usage: sigma-persona-bundle <command> [args]");
        eprintln!("Commands: list, install, remove, create, add-package, status");
        std::process::exit(1);
    }

    let repo_path = PathBuf::from("/var/lib/sigmaos/persona-bundles");
    let mut manager = BundleManager::new(repo_path);
    manager.init_default_bundles();

    match args[1].as_str() {
        "list" => {
            println!("Available Persona Bundles:");
            for bundle in manager.list_bundles() {
                println!("  {} - {} ({} MB, {} packages)",
                    bundle.name,
                    bundle.description,
                    bundle.total_size_mb,
                    bundle.packages.len()
                );
            }
        }
        "install" => {
            if args.len() < 3 {
                eprintln!("Usage: sigma-persona-bundle install <bundle-name>");
                std::process::exit(1);
            }
            manager.install_bundle(&args[2])?;
        }
        "remove" => {
            if args.len() < 3 {
                eprintln!("Usage: sigma-persona-bundle remove <bundle-name>");
                std::process::exit(1);
            }
            manager.remove_bundle(&args[2])?;
        }
        "status" => {
            println!("Installed Bundles:");
            for bundle_name in manager.list_installed() {
                if let Some(bundle) = manager.get_bundle(&bundle_name) {
                    println!("  {} - {}", bundle.name, bundle.description);
                }
            }
        }
        "create" => {
            if args.len() < 4 {
                eprintln!("Usage: sigma-persona-bundle create <name> <description>");
                std::process::exit(1);
            }
            manager.create_custom_bundle(args[2].clone(), args[3].clone());
            println!("Created custom bundle: {}", args[2]);
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            std::process::exit(1);
        }
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Err(e) = run_persona_bundle(args) {
        eprintln!("Error: {:?}", e);
        std::process::exit(1);
    }
}
