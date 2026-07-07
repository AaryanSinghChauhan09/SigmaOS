// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/core/package/sigma_nix_modules.rs — Nix-Style Declarative Package Modules
//
// Implements Nix-style declarative package management for SigmaOS.
// Provides reproducible, atomic package configurations and dependencies.
// Inspired by: NixOS modules, Nix flakes, Guix
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
    ConfigError(String),
    DependencyError(String),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

// ── Package Source ─────────────────────────────────────────────────────────
#[derive(Debug, Clone, PartialEq)]
pub enum PackageSource {
    /// Git repository.
    Git(String),
    /// Local path.
    Local(PathBuf),
    /// Remote URL.
    Url(String),
    /// Nix channel.
    Channel(String),
}

// ── Package Dependency ─────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct PackageDependency {
    pub name: String,
    pub version: String,
    pub optional: bool,
}

// ── Package Module ─────────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct PackageModule {
    pub name: String,
    pub version: String,
    pub description: String,
    pub source: PackageSource,
    pub dependencies: Vec<PackageDependency>,
    pub enabled: bool,
    pub config: HashMap<String, String>,
}

impl PackageModule {
    pub fn new(name: String, version: String, source: PackageSource) -> Self {
        Self {
            name,
            version,
            description: String::new(),
            source,
            dependencies: Vec::new(),
            enabled: true,
            config: HashMap::new(),
        }
    }

    pub fn add_dependency(&mut self, dep: PackageDependency) {
        self.dependencies.push(dep);
    }

    pub fn set_config(&mut self, key: String, value: String) {
        self.config.insert(key, value);
    }
}

// ── Module Configuration ───────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct ModuleConfig {
    pub modules: HashMap<String, PackageModule>,
    pub global_config: HashMap<String, String>,
}

impl ModuleConfig {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
            global_config: HashMap::new(),
        }
    }

    pub fn add_module(&mut self, module: PackageModule) {
        self.modules.insert(module.name.clone(), module);
    }

    pub fn get_module(&self, name: &str) -> Option<&PackageModule> {
        self.modules.get(name)
    }

    pub fn enable_module(&mut self, name: &str) -> Result<()> {
        if let Some(module) = self.modules.get_mut(name) {
            module.enabled = true;
            Ok(())
        } else {
            Err(Error::ConfigError(format!("Module '{}' not found", name)))
        }
    }

    pub fn disable_module(&mut self, name: &str) -> Result<()> {
        if let Some(module) = self.modules.get_mut(name) {
            module.enabled = false;
            Ok(())
        } else {
            Err(Error::ConfigError(format!("Module '{}' not found", name)))
        }
    }

    pub fn resolve_dependencies(&self) -> Result<Vec<String>> {
        let mut resolved = Vec::new();
        let mut visited = HashMap::new();

        for (name, module) in &self.modules {
            if module.enabled {
                self.resolve_module_deps(name, &mut resolved, &mut visited, &self.modules)?;
            }
        }

        Ok(resolved)
    }

    fn resolve_module_deps(
        &self,
        name: &str,
        resolved: &mut Vec<String>,
        visited: &mut HashMap<String, bool>,
        modules: &HashMap<String, PackageModule>,
    ) -> Result<()> {
        if visited.contains_key(name) {
            if visited.get(name) == Some(&true) {
                return Err(Error::DependencyError(format!("Circular dependency detected for '{}'", name)));
            }
            return Ok(());
        }

        visited.insert(name.to_string(), true);

        if let Some(module) = modules.get(name) {
            for dep in &module.dependencies {
                if !dep.optional {
                    self.resolve_module_deps(&dep.name, resolved, visited, modules)?;
                }
            }
        }

        resolved.push(name.to_string());
        visited.insert(name.to_string(), false);
        Ok(())
    }

    pub fn set_global_config(&mut self, key: String, value: String) {
        self.global_config.insert(key, value);
    }

    pub fn get_global_config(&self, key: &str) -> Option<&String> {
        self.global_config.get(key)
    }
}

// ── Nix Module Manager ─────────────────────────────────────────────────────
pub struct NixModuleManager {
    pub config: ModuleConfig,
    pub config_path: PathBuf,
    pub state_path: PathBuf,
}

impl NixModuleManager {
    pub fn new(config_path: PathBuf, state_path: PathBuf) -> Self {
        Self {
            config: ModuleConfig::new(),
            config_path,
            state_path,
        }
    }

    pub fn init(&mut self) -> Result<()> {
        fs::create_dir_all(&self.config_path)?;
        fs::create_dir_all(&self.state_path)?;
        
        // Initialize default modules
        self.init_default_modules();
        
        Ok(())
    }

    fn init_default_modules(&mut self) {
        // Core module
        let mut core = PackageModule::new(
            "sigma-core".to_string(),
            "1.0.0".to_string(),
            PackageSource::Channel("sigmaos-stable".to_string()),
        );
        core.description = "Core SigmaOS system packages".to_string();
        core.add_dependency(PackageDependency {
            name: "sigma-kernel".to_string(),
            version: "1.0.0".to_string(),
            optional: false,
        });
        self.config.add_module(core);

        // Desktop module
        let mut desktop = PackageModule::new(
            "sigma-desktop".to_string(),
            "1.0.0".to_string(),
            PackageSource::Channel("sigmaos-stable".to_string()),
        );
        desktop.description = "Desktop environment packages".to_string();
        desktop.add_dependency(PackageDependency {
            name: "sigma-core".to_string(),
            version: "1.0.0".to_string(),
            optional: false,
        });
        desktop.add_dependency(PackageDependency {
            name: "sigma-wayland".to_string(),
            version: "1.0.0".to_string(),
            optional: false,
        });
        self.config.add_module(desktop);

        // Development module
        let mut dev = PackageModule::new(
            "sigma-development".to_string(),
            "1.0.0".to_string(),
            PackageSource::Channel("sigmaos-stable".to_string()),
        );
        dev.description = "Development tools and compilers".to_string();
        dev.add_dependency(PackageDependency {
            name: "sigma-core".to_string(),
            version: "1.0.0".to_string(),
            optional: false,
        });
        self.config.add_module(dev);
    }

    pub fn load_config(&mut self) -> Result<()> {
        let config_file = self.config_path.join("modules.nix");
        if config_file.exists() {
            let file = File::open(&config_file)?;
            let reader = BufReader::new(file);
            
            for line in reader.lines() {
                let line = line?;
                if line.starts_with("#") || line.trim().is_empty() {
                    continue;
                }
                // Parse module configuration
                // In production: use proper Nix parser
            }
        }
        Ok(())
    }

    pub fn save_config(&self) -> Result<()> {
        let config_file = self.config_path.join("modules.nix");
        let mut file = File::create(&config_file)?;
        
        writeln!(file, "# SigmaOS Nix-Style Module Configuration")?;
        writeln!(file, "# Auto-generated - do not edit manually")?;
        writeln!(file)?;
        
        for (key, value) in &self.config.global_config {
            writeln!(file, "global.{} = \"{}\"", key, value)?;
        }
        
        writeln!(file)?;
        writeln!(file, "# Modules")?;
        
        for (name, module) in &self.config.modules {
            writeln!(file, "[module.{}]", name)?;
            writeln!(file, "  enabled = {}", module.enabled)?;
            writeln!(file, "  version = \"{}\"", module.version)?;
            writeln!(file, "  source = {:?}", module.source)?;
            
            for dep in &module.dependencies {
                writeln!(file, "  depends.{} = \"{}\"", dep.name, dep.version)?;
            }
            
            for (key, value) in &module.config {
                writeln!(file, "  config.{} = \"{}\"", key, value)?;
            }
            
            writeln!(file)?;
        }
        
        Ok(())
    }

    pub fn apply_config(&self) -> Result<()> {
        let deps = self.config.resolve_dependencies()?;
        
        println!("Applying configuration with {} modules:", deps.len());
        for dep in &deps {
            println!("  - {}", dep);
        }
        
        // In production: apply configuration to system
        Ok(())
    }

    pub fn list_modules(&self) -> Vec<&PackageModule> {
        self.config.modules.values().collect()
    }

    pub fn list_enabled_modules(&self) -> Vec<&PackageModule> {
        self.config.modules.values().filter(|m| m.enabled).collect()
    }
}

// ── CLI Interface ─────────────────────────────────────────────────────────────
pub fn run_nix_modules(args: Vec<String>) -> Result<()> {
    if args.len() < 2 {
        eprintln!("Usage: sigma-nix-modules <command> [args]");
        eprintln!("Commands: init, list, enable, disable, apply, config");
        std::process::exit(1);
    }

    let config_path = PathBuf::from("/etc/sigmaos/modules");
    let state_path = PathBuf::from("/var/lib/sigmaos/modules");
    let mut manager = NixModuleManager::new(config_path, state_path);

    match args[1].as_str() {
        "init" => {
            manager.init()?;
            println!("Nix module system initialized");
        }
        "list" => {
            manager.load_config()?;
            println!("Available Modules:");
            for module in manager.list_modules() {
                println!("  {} {} - {} [{}]",
                    module.name,
                    module.version,
                    module.description,
                    if module.enabled { "enabled" } else { "disabled" }
                );
            }
        }
        "enable" => {
            if args.len() < 3 {
                eprintln!("Usage: sigma-nix-modules enable <module-name>");
                std::process::exit(1);
            }
            manager.load_config()?;
            manager.enable_module(&args[2])?;
            manager.save_config()?;
            println!("Module '{}' enabled", args[2]);
        }
        "disable" => {
            if args.len() < 3 {
                eprintln!("Usage: sigma-nix-modules disable <module-name>");
                std::process::exit(1);
            }
            manager.load_config()?;
            manager.disable_module(&args[2])?;
            manager.save_config()?;
            println!("Module '{}' disabled", args[2]);
        }
        "apply" => {
            manager.load_config()?;
            manager.apply_config()?;
            println!("Configuration applied");
        }
        "config" => {
            if args.len() < 4 {
                eprintln!("Usage: sigma-nix-modules config <key> <value>");
                std::process::exit(1);
            }
            manager.load_config()?;
            manager.config.set_global_config(args[2].clone(), args[3].clone());
            manager.save_config()?;
            println!("Global config set: {} = {}", args[2], args[3]);
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
    if let Err(e) = run_nix_modules(args) {
        eprintln!("Error: {:?}", e);
        std::process::exit(1);
    }
}
