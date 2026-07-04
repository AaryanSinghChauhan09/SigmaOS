// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/ui/sigma_desktop.rs — Sigma Desktop Environment (KDE/GNOME)
//
// Implements KDE/GNOME-style desktop environment with panel management,
    pub application launcher, window management, and system settings.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Desktop Environment Types ─────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct DesktopApp {
    pub name: String,
    pub exec: String,
    pub icon: String,
    pub category: String,
    pub pinned: bool,
    pub running: bool,
}

#[derive(Debug, Clone)]
pub struct Panel {
    pub position: String,  // top, bottom, left, right
    pub height: u32,
    pub auto_hide: bool,
    pub widgets: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Workspace {
    pub name: String,
    pub index: u32,
    pub active: bool,
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub accent_color: String,
    pub dark_mode: bool,
    pub font: String,
}

// ─── Desktop Manager ─────────────────────────────────────────────────────

pub struct DesktopManager {
    pub apps: Vec<DesktopApp>,
    pub panels: Vec<Panel>,
    pub workspaces: Vec<Workspace>,
    pub current_workspace: u32,
    pub theme: Theme,
    pub desktop_running: bool,
}

impl DesktopManager {
    pub fn new() -> Self {
        let mut manager = DesktopManager {
            apps: Vec::new(),
            panels: Vec::new(),
            workspaces: Vec::new(),
            current_workspace: 0,
            theme: Theme {
                name: "Sigma Dark".to_string(),
                accent_color: "#3b82f6".to_string(),
                dark_mode: true,
                font: "Inter".to_string(),
            },
            desktop_running: true,
        };
        
        manager.init_sample_apps();
        manager.init_sample_panels();
        manager.init_sample_workspaces();
        manager
    }

    /// Initialize sample applications
    fn init_sample_apps(&mut self) {
        self.apps.push(DesktopApp {
            name: "Sigma Terminal".to_string(),
            exec: "sigma-terminal".to_string(),
            icon: "terminal".to_string(),
            category: "System".to_string(),
            pinned: true,
            running: false,
        });

        self.apps.push(DesktopApp {
            name: "Sigma File Manager".to_string(),
            exec: "sigma-files".to_string(),
            icon: "folder".to_string(),
            category: "System".to_string(),
            pinned: true,
            running: false,
        });

        self.apps.push(DesktopApp {
            name: "Sigma Browser".to_string(),
            exec: "sigma-browser".to_string(),
            icon: "browser".to_string(),
            category: "Internet".to_string(),
            pinned: true,
            running: true,
        });

        self.apps.push(DesktopApp {
            name: "Sigma Editor".to_string(),
            exec: "sigma-editor".to_string(),
            icon: "editor".to_string(),
            category: "Development".to_string(),
            pinned: false,
            running: false,
        });

        self.apps.push(DesktopApp {
            name: "Sigma Settings".to_string(),
            exec: "sigma-settings".to_string(),
            icon: "settings".to_string(),
            category: "System".to_string(),
            pinned: false,
            running: false,
        });
    }

    /// Initialize sample panels
    fn init_sample_panels(&mut self) {
        self.panels.push(Panel {
            position: "bottom".to_string(),
            height: 48,
            auto_hide: false,
            widgets: vec!["app_launcher".to_string(), "taskbar".to_string(), "system_tray".to_string(), "clock".to_string()],
        });

        self.panels.push(Panel {
            position: "top".to_string(),
            height: 32,
            auto_hide: true,
            widgets: vec!["menu".to_string(), "workspace_switcher".to_string()],
        });
    }

    /// Initialize sample workspaces
    fn init_sample_workspaces(&mut self) {
        self.workspaces.push(Workspace {
            name: "Workspace 1".to_string(),
            index: 0,
            active: true,
        });

        self.workspaces.push(Workspace {
            name: "Workspace 2".to_string(),
            index: 1,
            active: false,
        });

        self.workspaces.push(Workspace {
            name: "Workspace 3".to_string(),
            index: 2,
            active: false,
        });

        self.workspaces.push(Workspace {
            name: "Workspace 4".to_string(),
            index: 3,
            active: false,
        });
    }

    /// Launch application
    pub fn launch_app(&mut self, app_name: &str) -> Result<(), String> {
        if let Some(app) = self.apps.iter_mut().find(|a| a.name == app_name) {
            app.running = true;
            Ok(())
        } else {
            Err("Application not found".to_string())
        }
    }

    /// Close application
    pub fn close_app(&mut self, app_name: &str) -> Result<(), String> {
        if let Some(app) = self.apps.iter_mut().find(|a| a.name == app_name) {
            app.running = false;
            Ok(())
        } else {
            Err("Application not found".to_string())
        }
    }

    /// Pin application
    pub fn pin_app(&mut self, app_name: &str) -> Result<(), String> {
        if let Some(app) = self.apps.iter_mut().find(|a| a.name == app_name) {
            app.pinned = true;
            Ok(())
        } else {
            Err("Application not found".to_string())
        }
    }

    /// Unpin application
    pub fn unpin_app(&mut self, app_name: &str) -> Result<(), String> {
        if let Some(app) = self.apps.iter_mut().find(|a| a.name == app_name) {
            app.pinned = false;
            Ok(())
        } else {
            Err("Application not found".to_string())
        }
    }

    /// Switch workspace
    pub fn switch_workspace(&mut self, index: u32) -> Result<(), String> {
        if let Some(workspace) = self.workspaces.iter_mut().find(|w| w.index == index) {
            for w in &mut self.workspaces {
                w.active = false;
            }
            workspace.active = true;
            self.current_workspace = index;
            Ok(())
        } else {
            Err("Workspace not found".to_string())
        }
    }

    /// Add workspace
    pub fn add_workspace(&mut self) {
        let index = self.workspaces.len() as u32;
        self.workspaces.push(Workspace {
            name: format!("Workspace {}", index + 1),
            index,
            active: false,
        });
    }

    /// Remove workspace
    pub fn remove_workspace(&mut self, index: u32) -> Result<(), String> {
        if self.workspaces.len() <= 1 {
            return Err("Cannot remove last workspace".to_string());
        }
        
        if let Some(pos) = self.workspaces.iter().position(|w| w.index == index) {
            if self.workspaces[pos].active {
                self.switch_workspace(0)?;
            }
            self.workspaces.remove(pos);
            Ok(())
        } else {
            Err("Workspace not found".to_string())
        }
    }

    /// Set theme
    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }

    /// Toggle dark mode
    pub fn toggle_dark_mode(&mut self) {
        self.theme.dark_mode = !self.theme.dark_mode;
    }

    /// Get running apps
    pub fn get_running_apps(&self) -> Vec<&DesktopApp> {
        self.apps.iter().filter(|a| a.running).collect()
    }

    /// Get pinned apps
    pub fn get_pinned_apps(&self) -> Vec<&DesktopApp> {
        self.apps.iter().filter(|a| a.pinned).collect()
    }

    /// Get apps by category
    pub fn get_apps_by_category(&self, category: &str) -> Vec<&DesktopApp> {
        self.apps.iter().filter(|a| a.category == category).collect()
    }

    /// Toggle desktop
    pub fn toggle_desktop(&mut self) {
        self.desktop_running = !self.desktop_running;
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────

fn main() {
    let mut manager = DesktopManager::new();
    
    println!("Sigma Desktop Environment v0.1 - KDE/GNOME Style");
    
    loop {
        println!("\n--- Desktop Status ---");
        println!("Desktop: {}", if manager.desktop_running { "RUNNING" } else { "STOPPED" });
        println!("Theme: {} ({})", manager.theme.name, if manager.theme.dark_mode { "Dark" } else { "Light" });
        println!("Workspace: {}", manager.current_workspace + 1);
        println!("Running Apps: {}", manager.get_running_apps().len());
        println!("Pinned Apps: {}", manager.get_pinned_apps().len());
        
        println!("\nCommands: launch <app>, close <app>, pin <app>, unpin <app>, switch <workspace>, add_workspace, remove_workspace <index>, apps, running, pinned, category <cat>, toggle_dark, toggle, quit");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "launch" => {
                if let Some(arg) = parts.get(1) {
                    match manager.launch_app(arg) {
                        Ok(_) => println!("Application launched"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "close" => {
                if let Some(arg) = parts.get(1) {
                    match manager.close_app(arg) {
                        Ok(_) => println!("Application closed"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "pin" => {
                if let Some(arg) = parts.get(1) {
                    match manager.pin_app(arg) {
                        Ok(_) => println!("Application pinned"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "unpin" => {
                if let Some(arg) = parts.get(1) {
                    match manager.unpin_app(arg) {
                        Ok(_) => println!("Application unpinned"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "switch" => {
                if let Some(arg) = parts.get(1) {
                    if let Ok(index) = arg.parse::<u32>() {
                        match manager.switch_workspace(index) {
                            Ok(_) => println!("Switched to workspace {}", index + 1),
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                }
            }
            "add_workspace" => {
                manager.add_workspace();
                println!("Workspace added");
            }
            "remove_workspace" => {
                if let Some(arg) = parts.get(1) {
                    if let Ok(index) = arg.parse::<u32>() {
                        match manager.remove_workspace(index) {
                            Ok(_) => println!("Workspace removed"),
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                }
            }
            "apps" => {
                println!("--- All Applications ---");
                for app in &manager.apps {
                    let running = if app.running { "[RUNNING]" } else { "" };
                    let pinned = if app.pinned { "[PINNED]" } else { "" };
                    println!("{} - {} {} {} ({})", app.name, app.icon, running, pinned, app.category);
                }
            }
            "running" => {
                println!("--- Running Applications ---");
                for app in manager.get_running_apps() {
                    println!("{} - {}", app.name, app.exec);
                }
            }
            "pinned" => {
                println!("--- Pinned Applications ---");
                for app in manager.get_pinned_apps() {
                    println!("{} - {}", app.name, app.icon);
                }
            }
            "category" => {
                if let Some(arg) = parts.get(1) {
                    println!("--- Applications in {} ---", arg);
                    for app in manager.get_apps_by_category(arg) {
                        println!("{} - {}", app.name, app.exec);
                    }
                }
            }
            "toggle_dark" => {
                manager.toggle_dark_mode();
                println!("Dark mode: {}", manager.theme.dark_mode);
            }
            "toggle" => {
                manager.toggle_desktop();
                println!("Desktop {}", if manager.desktop_running { "started" } else { "stopped" });
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
