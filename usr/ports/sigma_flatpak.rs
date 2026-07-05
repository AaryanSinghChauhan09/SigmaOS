// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/ports/sigma_flatpak.rs — Sigma Flatpak/Sandbox
//
// Implements Flatpak-style sandboxed application distribution with
// runtime management, permissions, portals, and app isolation.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Flatpak Types ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Permission {
    Network,
    X11,
    Wayland,
    PulseAudio,
    Devices,
    SystemBus,
    SessionBus,
    Filesystem,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SandboxLevel {
    Full,
    Host,
    Shared,
}

#[derive(Debug, Clone)]
pub struct FlatpakRef {
    pub id: String,
    pub arch: String,
    pub branch: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Runtime {
    pub ref_: FlatpakRef,
    pub version: String,
    pub installed: bool,
    pub size: u64,
}

#[derive(Debug, Clone)]
pub struct Application {
    pub ref_: FlatpakRef,
    pub name: String,
    pub version: String,
    pub installed: bool,
    pub runtime: String,
    pub permissions: Vec<Permission>,
    pub sandbox_level: SandboxLevel,
    pub size: u64,
    pub launch_command: String,
}

#[derive(Debug, Clone)]
pub struct Remote {
    pub name: String,
    pub url: String,
    pub title: String,
    pub collection_id: String,
    pub enabled: bool,
    pub gpg_verify: bool,
}

#[derive(Debug, Clone)]
pub struct Portal {
    pub name: String,
    pub description: String,
    pub version: String,
}

// ─── Flatpak Manager ─────────────────────────────────────────────────────

pub struct FlatpakManager {
    pub applications: HashMap<String, Application>,
    pub runtimes: HashMap<String, Runtime>,
    pub remotes: HashMap<String, Remote>,
    pub portals: Vec<Portal>,
    pub default_runtime: String,
    pub user_install: bool,
}

impl FlatpakManager {
    pub fn new() -> Self {
        let mut manager = FlatpakManager {
            applications: HashMap::new(),
            runtimes: HashMap::new(),
            remotes: HashMap::new(),
            portals: vec![],
            default_runtime: "org.freedesktop.Platform".to_string(),
            user_install: true,
        };

        manager.init_default_remotes();
        manager.init_default_runtimes();
        manager.init_default_portals();
        manager
    }

    /// Initialize default remotes
    fn init_default_remotes(&mut self) {
        self.remotes.insert("flathub".to_string(), Remote {
            name: "flathub".to_string(),
            url: "https://flathub.org/repo/flathub.flatpakrepo".to_string(),
            title: "Flathub".to_string(),
            collection_id: "org.flathub.Stable".to_string(),
            enabled: true,
            gpg_verify: true,
        });

        self.remotes.insert("flathub-beta".to_string(), Remote {
            name: "flathub-beta".to_string(),
            url: "https://flathub.org/beta-repo/flathub-beta.flatpakrepo".to_string(),
            title: "Flathub Beta".to_string(),
            collection_id: "org.flathub.Beta".to_string(),
            enabled: false,
            gpg_verify: true,
        });
    }

    /// Initialize default runtimes
    fn init_default_runtimes(&mut self) {
        self.runtimes.insert("org.freedesktop.Platform/x86_64/22.08".to_string(), Runtime {
            ref_: FlatpakRef {
                id: "org.freedesktop.Platform".to_string(),
                arch: "x86_64".to_string(),
                branch: "22.08".to_string(),
                name: "org.freedesktop.Platform".to_string(),
            },
            version: "22.08".to_string(),
            installed: true,
            size: 350 * 1024 * 1024,  // 350MB
        });

        self.runtimes.insert("org.gnome.Platform/x86_64/44".to_string(), Runtime {
            ref_: FlatpakRef {
                id: "org.gnome.Platform".to_string(),
                arch: "x86_64".to_string(),
                branch: "44".to_string(),
                name: "org.gnome.Platform".to_string(),
            },
            version: "44".to_string(),
            installed: false,
            size: 400 * 1024 * 1024,  // 400MB
        });
    }

    /// Initialize default portals
    fn init_default_portals(&mut self) {
        self.portals = vec![
            Portal {
                name: "org.freedesktop.portal.FileChooser".to_string(),
                description: "File chooser portal".to_string(),
                version: "1.6".to_string(),
            },
            Portal {
                name: "org.freedesktop.portal.Print".to_string(),
                description: "Print portal".to_string(),
                version: "1.6".to_string(),
            },
            Portal {
                name: "org.freedesktop.portal.Screenshot".to_string(),
                description: "Screenshot portal".to_string(),
                version: "1.6".to_string(),
            },
            Portal {
                name: "org.freedesktop.portal.Settings".to_string(),
                description: "Settings portal".to_string(),
                version: "1.6".to_string(),
            },
        ];
    }

    /// Add a remote
    pub fn add_remote(&mut self, name: String, url: String, title: String) -> Result<Remote, String> {
        if self.remotes.contains_key(&name) {
            return Err("Remote already exists".to_string());
        }

        let remote = Remote {
            name: name.clone(),
            url,
            title,
            collection_id: format!("com.{}.Stable", name),
            enabled: true,
            gpg_verify: true,
        };

        self.remotes.insert(name.clone(), remote.clone());
        Ok(remote)
    }

    /// Remove a remote
    pub fn remove_remote(&mut self, name: &str) -> Result<(), String> {
        if self.remotes.remove(name).is_some() {
            Ok(())
        } else {
            Err("Remote not found".to_string())
        }
    }

    /// Install an application
    pub fn install(&mut self, ref_: String, remote: Option<String>) -> Result<Application, String> {
        let parts: Vec<&str> = ref_.split('/').collect();
        if parts.len() < 3 {
            return Err("Invalid ref format".to_string());
        }

        let app_id = parts[0].to_string();
        let arch = parts[1].to_string();
        let branch = parts[2].to_string();

        if self.applications.contains_key(&app_id) {
            return Err("Application already installed".to_string());
        }

        let app = Application {
            ref_: FlatpakRef {
                id: app_id.clone(),
                arch,
                branch,
                name: app_id.clone(),
            },
            name: app_id.split('.').last().unwrap_or(&app_id).to_string(),
            version: branch.clone(),
            installed: true,
            runtime: self.default_runtime.clone(),
            permissions: vec![
                Permission::X11,
                Permission::Wayland,
                Permission::PulseAudio,
            ],
            sandbox_level: SandboxLevel::Shared,
            size: 100 * 1024 * 1024,  // Simulated size
            launch_command: format!("/usr/bin/flatpak run {}", app_id),
        };

        self.applications.insert(app_id.clone(), app.clone());
        Ok(app)
    }

    /// Uninstall an application
    pub fn uninstall(&mut self, ref_: &str) -> Result<(), String> {
        if let Some(app) = self.applications.get_mut(ref_) {
            app.installed = false;
            Ok(())
        } else {
            Err("Application not found".to_string())
        }
    }

    /// Update an application
    pub fn update(&mut self, ref_: &str) -> Result<Application, String> {
        if let Some(app) = self.applications.get_mut(ref_) {
            app.version = format!("{}.1", app.version);
            Ok(app.clone())
        } else {
            Err("Application not found".to_string())
        }
    }

    /// Add permission to application
    pub fn add_permission(&mut self, app_id: &str, permission: Permission) -> Result<(), String> {
        if let Some(app) = self.applications.get_mut(app_id) {
            if !app.permissions.contains(&permission) {
                app.permissions.push(permission);
            }
            Ok(())
        } else {
            Err("Application not found".to_string())
        }
    }

    /// Remove permission from application
    pub fn remove_permission(&mut self, app_id: &str, permission: Permission) -> Result<(), String> {
        if let Some(app) = self.applications.get_mut(app_id) {
            app.permissions.retain(|p| *p != permission);
            Ok(())
        } else {
            Err("Application not found".to_string())
        }
    }

    /// Set sandbox level for application
    pub fn set_sandbox_level(&mut self, app_id: &str, level: SandboxLevel) -> Result<(), String> {
        if let Some(app) = self.applications.get_mut(app_id) {
            app.sandbox_level = level;
            Ok(())
        } else {
            Err("Application not found".to_string())
        }
    }

    /// List all applications
    pub fn list_applications(&self, installed_only: bool) -> Vec<&Application> {
        self.applications.values()
            .filter(|app| !installed_only || app.installed)
            .collect()
    }

    /// List all runtimes
    pub fn list_runtimes(&self) -> Vec<&Runtime> {
        self.runtimes.values().collect()
    }

    /// List all remotes
    pub fn list_remotes(&self) -> Vec<&Remote> {
        self.remotes.values().collect()
    }

    /// List all portals
    pub fn list_portals(&self) -> &Vec<Portal> {
        &self.portals
    }

    /// Search for applications
    pub fn search(&self, query: &str) -> Vec<&Application> {
        let query_lower = query.to_lowercase();
        self.applications.values()
            .filter(|app| {
                app.name.to_lowercase().contains(&query_lower) ||
                app.ref_.id.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    /// Get application info
    pub fn get_info(&self, ref_: &str) -> Option<&Application> {
        self.applications.get(ref_)
    }

    /// Install runtime
    pub fn install_runtime(&mut self, ref_: String) -> Result<Runtime, String> {
        let parts: Vec<&str> = ref_.split('/').collect();
        if parts.len() < 3 {
            return Err("Invalid ref format".to_string());
        }

        let runtime_id = parts[0].to_string();
        let arch = parts[1].to_string();
        let branch = parts[2].to_string();

        let runtime = Runtime {
            ref_: FlatpakRef {
                id: runtime_id.clone(),
                arch,
                branch,
                name: runtime_id.clone(),
            },
            version: branch.clone(),
            installed: true,
            size: 300 * 1024 * 1024,
        };

        self.runtimes.insert(ref_.clone(), runtime.clone());
        Ok(runtime)
    }

    /// Get statistics
    pub fn get_statistics(&self) -> HashMap<String, u32> {
        let mut stats = HashMap::new();
        stats.insert("total_applications".to_string(), self.applications.len() as u32);
        stats.insert("installed_applications".to_string(), self.applications.values().filter(|a| a.installed).count() as u32);
        stats.insert("runtimes".to_string(), self.runtimes.len() as u32);
        stats.insert("remotes".to_string(), self.remotes.len() as u32);
        stats.insert("portals".to_string(), self.portals.len() as u32);
        
        stats
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────

fn main() {
    let mut flatpak = FlatpakManager::new();
    
    println!("Sigma Flatpak v0.1 - Sandbox Application Distribution");
    
    loop {
        println!("\n--- Flatpak Commands ---");
        println!("list [installed]  - List applications");
        println!("search <query>    - Search applications");
        println!("info <ref>        - Get application info");
        println!("install <ref> [remote] - Install application");
        println!("uninstall <ref>   - Uninstall application");
        println!("update <ref>      - Update application");
        println!("runtimes          - List runtimes");
        println!("install_runtime <ref> - Install runtime");
        println!("remotes           - List remotes");
        println!("add_remote <name> <url> <title> - Add remote");
        println!("remove_remote <name> - Remove remote");
        println!("portals           - List portals");
        println!("add_perm <app> <perm> - Add permission");
        println!("remove_perm <app> <perm> - Remove permission");
        println!("sandbox <app> <level> - Set sandbox level");
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
            "list" => {
                let installed_only = parts.get(1).map(|s| *s == "installed").unwrap_or(false);
                println!("--- Applications ---");
                for app in flatpak.list_applications(installed_only) {
                    println!("{} - {} - {} - {} MB - {:?}", 
                        app.ref_.id, app.name, app.version, app.size / (1024 * 1024), app.sandbox_level);
                }
            }
            "search" => {
                if let Some(query) = parts.get(1) {
                    println!("--- Search Results ---");
                    for app in flatpak.search(query) {
                        println!("{} - {} - {}", app.ref_.id, app.name, app.version);
                    }
                }
            }
            "info" => {
                if let Some(ref_) = parts.get(1) {
                    if let Some(app) = flatpak.get_info(ref_) {
                        println!("--- Application Info ---");
                        println!("ID: {}", app.ref_.id);
                        println!("Name: {}", app.name);
                        println!("Version: {}", app.version);
                        println!("Runtime: {}", app.runtime);
                        println!("Installed: {}", app.installed);
                        println!("Size: {} MB", app.size / (1024 * 1024));
                        println!("Sandbox: {:?}", app.sandbox_level);
                        println!("Permissions: {:?}", app.permissions);
                    }
                }
            }
            "install" => {
                if parts.len() >= 2 {
                    let ref_ = parts[1].to_string();
                    let remote = parts.get(2).map(|r| r.to_string());
                    match flatpak.install(ref_, remote) {
                        Ok(_) => println!("Application installed"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "uninstall" => {
                if let Some(ref_) = parts.get(1) {
                    match flatpak.uninstall(ref_) {
                        Ok(_) => println!("Application uninstalled"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "update" => {
                if let Some(ref_) = parts.get(1) {
                    match flatpak.update(ref_) {
                        Ok(_) => println!("Application updated"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "runtimes" => {
                println!("--- Runtimes ---");
                for runtime in flatpak.list_runtimes() {
                    println!("{} - {} - {} - {} MB - {}", 
                        runtime.ref_.id, runtime.version, runtime.ref_.arch, 
                        runtime.size / (1024 * 1024), 
                        if runtime.installed { "installed" } else { "not installed" });
                }
            }
            "install_runtime" => {
                if parts.len() >= 2 {
                    let ref_ = parts[1].to_string();
                    match flatpak.install_runtime(ref_) {
                        Ok(_) => println!("Runtime installed"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "remotes" => {
                println!("--- Remotes ---");
                for remote in flatpak.list_remotes() {
                    println!("{} - {} - {} - {}", 
                        remote.name, remote.title, remote.url, 
                        if remote.enabled { "enabled" } else { "disabled" });
                }
            }
            "add_remote" => {
                if parts.len() >= 4 {
                    let name = parts[1].to_string();
                    let url = parts[2].to_string();
                    let title = parts[3].to_string();
                    match flatpak.add_remote(name, url, title) {
                        Ok(_) => println!("Remote added"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "remove_remote" => {
                if let Some(name) = parts.get(1) {
                    match flatpak.remove_remote(name) {
                        Ok(_) => println!("Remote removed"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "portals" => {
                println!("--- Portals ---");
                for portal in flatpak.list_portals() {
                    println!("{} - {} - {}", portal.name, portal.description, portal.version);
                }
            }
            "add_perm" => {
                if parts.len() >= 3 {
                    let app = parts[1];
                    let perm = match parts[2] {
                        "network" => Permission::Network,
                        "x11" => Permission::X11,
                        "wayland" => Permission::Wayland,
                        "pulseaudio" => Permission::PulseAudio,
                        "devices" => Permission::Devices,
                        "systembus" => Permission::SystemBus,
                        "sessionbus" => Permission::SessionBus,
                        "filesystem" => Permission::Filesystem,
                        _ => {
                            println!("Unknown permission");
                            continue;
                        }
                    };
                    match flatpak.add_permission(app, perm) {
                        Ok(_) => println!("Permission added"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "remove_perm" => {
                if parts.len() >= 3 {
                    let app = parts[1];
                    let perm = match parts[2] {
                        "network" => Permission::Network,
                        "x11" => Permission::X11,
                        "wayland" => Permission::Wayland,
                        "pulseaudio" => Permission::PulseAudio,
                        "devices" => Permission::Devices,
                        "systembus" => Permission::SystemBus,
                        "sessionbus" => Permission::SessionBus,
                        "filesystem" => Permission::Filesystem,
                        _ => {
                            println!("Unknown permission");
                            continue;
                        }
                    };
                    match flatpak.remove_permission(app, perm) {
                        Ok(_) => println!("Permission removed"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "sandbox" => {
                if parts.len() >= 3 {
                    let app = parts[1];
                    let level = match parts[2] {
                        "full" => SandboxLevel::Full,
                        "host" => SandboxLevel::Host,
                        "shared" => SandboxLevel::Shared,
                        _ => {
                            println!("Unknown level");
                            continue;
                        }
                    };
                    match flatpak.set_sandbox_level(app, level) {
                        Ok(_) => println!("Sandbox level set"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "stats" => {
                println!("--- Statistics ---");
                for (key, value) in flatpak.get_statistics() {
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
