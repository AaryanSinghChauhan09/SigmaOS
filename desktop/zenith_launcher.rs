// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// desktop/zenith_launcher.rs — Zenith Application Launcher
//
// Implements the application launcher with fuzzy search, category icons,
// and keyboard navigation for SigmaOS desktop environment.
//
// Language: Rust (std for userland services)

use std::collections::HashMap;

// ─── Application Metadata ───────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct AppEntry {
    pub name: String,
    pub exec: String,
    pub icon: String,
    pub category: AppCategory,
    pub description: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppCategory {
    Internet,
    Productivity,
    Games,
    System,
    Multimedia,
    Development,
    Utilities,
}

// ─── Launcher State ───────────────────────────────────────────────────────────

pub struct ZenithLauncher {
    pub apps: HashMap<String, AppEntry>,
    pub visible: bool,
    pub search_query: String,
    pub filtered_apps: Vec<String>,
    pub selected_index: usize,
    pub initialized: bool,
}

impl ZenithLauncher {
    pub fn new() -> Self {
        let mut launcher = ZenithLauncher {
            apps: HashMap::new(),
            visible: false,
            search_query: String::new(),
            filtered_apps: Vec::new(),
            selected_index: 0,
            initialized: false,
        };
        
        launcher.init();
        launcher
    }

    /// Initialize with default applications
    pub fn init(&mut self) {
        // Add default applications
        self.add_app(AppEntry {
            name: "sigma-edit".to_string(),
            exec: "/usr/bin/sigma-edit".to_string(),
            icon: "text-editor".to_string(),
            category: AppCategory::Productivity,
            description: "Text editor".to_string(),
        });

        self.add_app(AppEntry {
            name: "sigma-files".to_string(),
            exec: "/usr/bin/sigma-files".to_string(),
            icon: "folder".to_string(),
            category: AppCategory::System,
            description: "File manager".to_string(),
        });

        self.add_app(AppEntry {
            name: "sigma-terminal".to_string(),
            exec: "/usr/bin/sigma-terminal".to_string(),
            icon: "terminal".to_string(),
            category: AppCategory::System,
            description: "Terminal emulator".to_string(),
        });

        self.add_app(AppEntry {
            name: "sigma-browser".to_string(),
            exec: "/usr/bin/sigma-browser".to_string(),
            icon: "web-browser".to_string(),
            category: AppCategory::Internet,
            description: "Web browser".to_string(),
        });

        self.add_app(AppEntry {
            name: "sigma-mail".to_string(),
            exec: "/usr/bin/sigma-mail".to_string(),
            icon: "email".to_string(),
            category: AppCategory::Internet,
            description: "Email client".to_string(),
        });

        self.add_app(AppEntry {
            name: "sigma-calc".to_string(),
            exec: "/usr/bin/sigma-calc".to_string(),
            icon: "calculator".to_string(),
            category: AppCategory::Utilities,
            description: "Calculator".to_string(),
        });

        self.add_app(AppEntry {
            name: "sigma-calendar".to_string(),
            exec: "/usr/bin/sigma-calendar".to_string(),
            icon: "calendar".to_string(),
            category: AppCategory::Productivity,
            description: "Calendar".to_string(),
        });

        self.add_app(AppEntry {
            name: "sigma-notes".to_string(),
            exec: "/usr/bin/sigma-notes".to_string(),
            icon: "notes".to_string(),
            category: AppCategory::Productivity,
            description: "Note application".to_string(),
        });

        self.add_app(AppEntry {
            name: "sigma-clock".to_string(),
            exec: "/usr/bin/sigma-clock".to_string(),
            icon: "clock".to_string(),
            category: AppCategory::Utilities,
            description: "System clock".to_string(),
        });

        self.add_app(AppEntry {
            name: "sigma-settings".to_string(),
            exec: "/usr/bin/sigma-settings".to_string(),
            icon: "settings".to_string(),
            category: AppCategory::System,
            description: "Settings panel".to_string(),
        });

        self.initialized = true;
    }

    /// Add an application to the launcher
    pub fn add_app(&mut self, app: AppEntry) {
        self.apps.insert(app.name.clone(), app);
    }

    /// Show launcher (triggered by Super key)
    pub fn show(&mut self) {
        self.visible = true;
        self.search_query.clear();
        self.filtered_apps = self.apps.keys().cloned().collect();
        self.selected_index = 0;
    }

    /// Hide launcher
    pub fn hide(&mut self) {
        self.visible = false;
        self.search_query.clear();
        self.filtered_apps.clear();
    }

    /// Toggle visibility
    pub fn toggle(&mut self) {
        if self.visible {
            self.hide();
        } else {
            self.show();
        }
    }

    /// Update search query with fuzzy matching
    pub fn update_search(&mut self, query: &str) {
        self.search_query = query.to_lowercase();
        
        if self.search_query.is_empty() {
            self.filtered_apps = self.apps.keys().cloned().collect();
        } else {
            self.filtered_apps.clear();
            
            for app_name in self.apps.keys() {
                let app_lower = app_name.to_lowercase();
                
                // Fuzzy matching: check if all characters in query appear in order
                if self.fuzzy_match(&self.search_query, &app_lower) {
                    self.filtered_apps.push(app_name.clone());
                }
            }
        }
        
        self.selected_index = 0;
    }

    /// Simple fuzzy matching algorithm
    fn fuzzy_match(&self, query: &str, text: &str) -> bool {
        let mut query_chars = query.chars().peekable();
        let mut text_chars = text.chars();
        
        while let Some(q_char) = query_chars.next() {
            let mut found = false;
            while let Some(t_char) = text_chars.next() {
                if t_char == q_char {
                    found = true;
                    break;
                }
            }
            if !found {
                return false;
            }
        }
        
        true
    }

    /// Select next item
    pub fn select_next(&mut self) {
        if !self.filtered_apps.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.filtered_apps.len();
        }
    }

    /// Select previous item
    pub fn select_previous(&mut self) {
        if !self.filtered_apps.is_empty() {
            self.selected_index = if self.selected_index == 0 {
                self.filtered_apps.len() - 1
            } else {
                self.selected_index - 1
            };
        }
    }

    /// Get selected app
    pub fn get_selected(&self) -> Option<&AppEntry> {
        if self.selected_index < self.filtered_apps.len() {
            let app_name = &self.filtered_apps[self.selected_index];
            self.apps.get(app_name)
        } else {
            None
        }
    }

    /// Launch selected application
    pub fn launch_selected(&self) -> Result<String, String> {
        if let Some(app) = self.get_selected() {
            // In a real implementation, this would fork/exec the application
            Ok(format!("Launching: {}", app.exec))
        } else {
            Err("No application selected".to_string())
        }
    }

    /// Get apps by category
    pub fn get_apps_by_category(&self, category: AppCategory) -> Vec<&AppEntry> {
        self.apps.values()
            .filter(|app| app.category == category)
            .collect()
    }

    /// Get all categories
    pub fn get_categories(&self) -> Vec<AppCategory> {
        use std::collections::HashSet;
        
        let mut categories: HashSet<AppCategory> = HashSet::new();
        for app in self.apps.values() {
            categories.insert(app.category);
        }
        
        vec![
            AppCategory::Internet,
            AppCategory::Productivity,
            AppCategory::Games,
            AppCategory::System,
            AppCategory::Multimedia,
            AppCategory::Development,
            AppCategory::Utilities,
        ]
        .into_iter()
        .filter(|c| categories.contains(c))
        .collect()
    }
}
