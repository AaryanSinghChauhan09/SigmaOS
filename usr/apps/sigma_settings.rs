// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/apps/sigma_settings.rs — Sigma-Settings Settings Panel
//
// Implements a settings panel with system configuration,
// appearance settings, network configuration, and user preferences.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Setting Category ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SettingCategory {
    Appearance,
    Network,
    Sound,
    Display,
    System,
    Privacy,
    Accessibility,
}

// ─── Setting Item ───────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum SettingValue {
    Bool(bool),
    String(String),
    Number(i32),
    Select(Vec<String>, usize), // Options, selected index
}

#[derive(Debug, Clone)]
pub struct SettingItem {
    pub key: String,
    pub name: String,
    pub description: String,
    pub value: SettingValue,
}

// ─── Settings Application State ───────────────────────────────────────────────

pub struct SettingsApp {
    pub settings: HashMap<String, SettingItem>,
    pub current_category: SettingCategory,
    pub modified: bool,
}

impl SettingsApp {
    pub fn new() -> Self {
        let mut settings = SettingsApp {
            settings: HashMap::new(),
            current_category: SettingCategory::Appearance,
            modified: false,
        };
        
        settings.init_default_settings();
        settings
    }

    /// Initialize default settings
    fn init_default_settings(&mut self) {
        // Appearance settings
        self.settings.insert("theme".to_string(), SettingItem {
            key: "theme".to_string(),
            name: "Theme".to_string(),
            description: "Select desktop theme".to_string(),
            value: SettingValue::Select(vec!["Default".to_string(), "Dark".to_string(), "Light".to_string()], 0),
        });
        
        self.settings.insert("font_size".to_string(), SettingItem {
            key: "font_size".to_string(),
            name: "Font Size".to_string(),
            description: "System font size in points".to_string(),
            value: SettingValue::Number(12),
        });
        
        self.settings.insert("animations".to_string(), SettingItem {
            key: "animations".to_string(),
            name: "Animations".to_string(),
            description: "Enable UI animations".to_string(),
            value: SettingValue::Bool(true),
        });
        
        // Network settings
        self.settings.insert("wifi_enabled".to_string(), SettingItem {
            key: "wifi_enabled".to_string(),
            name: "Wi-Fi".to_string(),
            description: "Enable Wi-Fi adapter".to_string(),
            value: SettingValue::Bool(true),
        });
        
        self.settings.insert("bluetooth_enabled".to_string(), SettingItem {
            key: "bluetooth_enabled".to_string(),
            name: "Bluetooth".to_string(),
            description: "Enable Bluetooth adapter".to_string(),
            value: SettingValue::Bool(true),
        });
        
        self.settings.insert("airplane_mode".to_string(), SettingItem {
            key: "airplane_mode".to_string(),
            name: "Airplane Mode".to_string(),
            description: "Disable all wireless connections".to_string(),
            value: SettingValue::Bool(false),
        });
        
        // Sound settings
        self.settings.insert("volume".to_string(), SettingItem {
            key: "volume".to_string(),
            name: "Master Volume".to_string(),
            description: "System volume level (0-100)".to_string(),
            value: SettingValue::Number(75),
        });
        
        self.settings.insert("mute".to_string(), SettingItem {
            key: "mute".to_string(),
            name: "Mute".to_string(),
            description: "Mute all sounds".to_string(),
            value: SettingValue::Bool(false),
        });
        
        // Display settings
        self.settings.insert("brightness".to_string(), SettingItem {
            key: "brightness".to_string(),
            name: "Brightness".to_string(),
            description: "Screen brightness level (0-100)".to_string(),
            value: SettingValue::Number(75),
        });
        
        self.settings.insert("resolution".to_string(), SettingItem {
            key: "resolution".to_string(),
            name: "Resolution".to_string(),
            description: "Display resolution".to_string(),
            value: SettingValue::Select(vec!["1920x1080".to_string(), "2560x1440".to_string(), "3840x2160".to_string()], 0),
        });
        
        // System settings
        self.settings.insert("auto_update".to_string(), SettingItem {
            key: "auto_update".to_string(),
            name: "Auto Update".to_string(),
            description: "Automatically install system updates".to_string(),
            value: SettingValue::Bool(true),
        });
        
        self.settings.insert("timezone".to_string(), SettingItem {
            key: "timezone".to_string(),
            name: "Timezone".to_string(),
            description: "System timezone offset from UTC".to_string(),
            value: SettingValue::Number(0),
        });
        
        // Privacy settings
        self.settings.insert("telemetry".to_string(), SettingItem {
            key: "telemetry".to_string(),
            name: "Telemetry".to_string(),
            description: "Send anonymous usage data".to_string(),
            value: SettingValue::Bool(false),
        });
        
        self.settings.insert("location_services".to_string(), SettingItem {
            key: "location_services".to_string(),
            name: "Location Services".to_string(),
            description: "Allow apps to access location".to_string(),
            value: SettingValue::Bool(false),
        });
        
        // Accessibility settings
        self.settings.insert("high_contrast".to_string(), SettingItem {
            key: "high_contrast".to_string(),
            name: "High Contrast".to_string(),
            description: "Enable high contrast theme".to_string(),
            value: SettingValue::Bool(false),
        });
        
        self.settings.insert("screen_magnifier".to_string(), SettingItem {
            key: "screen_magnifier".to_string(),
            name: "Screen Magnifier".to_string(),
            description: "Enable screen magnification".to_string(),
            value: SettingValue::Bool(false),
        });
        
        self.settings.insert("screen_reader".to_string(), SettingItem {
            key: "screen_reader".to_string(),
            name: "Screen Reader".to_string(),
            description: "Enable screen reader".to_string(),
            value: SettingValue::Bool(false),
        });
    }

    /// Switch category
    pub fn switch_category(&mut self, category: SettingCategory) {
        self.current_category = category;
    }

    /// Get settings for current category
    pub fn get_category_settings(&self) -> Vec<&SettingItem> {
        let category_keys = match self.current_category {
            SettingCategory::Appearance => vec!["theme", "font_size", "animations"],
            SettingCategory::Network => vec!["wifi_enabled", "bluetooth_enabled", "airplane_mode"],
            SettingCategory::Sound => vec!["volume", "mute"],
            SettingCategory::Display => vec!["brightness", "resolution"],
            SettingCategory::System => vec!["auto_update", "timezone"],
            SettingCategory::Privacy => vec!["telemetry", "location_services"],
            SettingCategory::Accessibility => vec!["high_contrast", "screen_magnifier", "screen_reader"],
        };
        
        category_keys.iter()
            .filter_map(|key| self.settings.get(*key))
            .collect()
    }

    /// Update setting value
    pub fn update_setting(&mut self, key: &str, value: SettingValue) -> Result<(), String> {
        if let Some(setting) = self.settings.get_mut(key) {
            setting.value = value;
            self.modified = true;
            Ok(())
        } else {
            Err(format!("Setting '{}' not found", key))
        }
    }

    /// Get setting value
    pub fn get_setting(&self, key: &str) -> Option<&SettingValue> {
        self.settings.get(key).map(|s| &s.value)
    }

    /// Reset to defaults
    pub fn reset_to_defaults(&mut self) {
        self.settings.clear();
        self.init_default_settings();
        self.modified = true;
    }

    /// Save settings (stub implementation)
    pub fn save(&mut self) -> Result<(), String> {
        // In a real implementation, this would write to a config file
        self.modified = false;
        Ok(())
    }

    /// Check if modified
    pub fn is_modified(&self) -> bool {
        self.modified
    }

    /// Get category name
    pub fn get_category_name(&self) -> &str {
        match self.current_category {
            SettingCategory::Appearance => "Appearance",
            SettingCategory::Network => "Network",
            SettingCategory::Sound => "Sound",
            SettingCategory::Display => "Display",
            SettingCategory::System => "System",
            SettingCategory::Privacy => "Privacy",
            SettingCategory::Accessibility => "Accessibility",
        }
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────--

fn main() {
    let mut settings = SettingsApp::new();
    
    println!("Sigma-Settings v0.1 - Settings Panel");
    println!("Current category: {}", settings.get_category_name());
    
    loop {
        println!("\n--- {} Settings ---", settings.get_category_name());
        for setting in settings.get_category_settings() {
            let value_str = match &setting.value {
                SettingValue::Bool(b) => if *b { "ON" } else { "OFF" }.to_string(),
                SettingValue::String(s) => s.clone(),
                SettingValue::Number(n) => n.to_string(),
                SettingValue::Select(options, idx) => {
                    if *idx < options.len() {
                        options[*idx].clone()
                    } else {
                        "Unknown".to_string()
                    }
                }
            };
            println!("{}: {} ({})", setting.name, value_str, setting.description);
        }
        
        println!("\nCategories: appearance, network, sound, display, system, privacy, accessibility");
        println!("Commands: category <name>, set <key> <value>, reset, save, quit");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "category" => {
                if let Some(arg) = parts.get(1) {
                    let category = match *arg {
                        "appearance" => SettingCategory::Appearance,
                        "network" => SettingCategory::Network,
                        "sound" => SettingCategory::Sound,
                        "display" => SettingCategory::Display,
                        "system" => SettingCategory::System,
                        "privacy" => SettingCategory::Privacy,
                        "accessibility" => SettingCategory::Accessibility,
                        _ => {
                            println!("Unknown category");
                            continue;
                        }
                    };
                    settings.switch_category(category);
                    println!("Switched to: {}", arg);
                }
            }
            "set" => {
                if parts.len() >= 3 {
                    let key = parts[1];
                    let value_str = parts[2];
                    
                    // Try to parse value based on current setting type
                    if let Some(current_value) = settings.get_setting(key) {
                        let new_value = match current_value {
                            SettingValue::Bool(_) => {
                                SettingValue::Bool(value_str == "true" || value_str == "on" || value_str == "1")
                            }
                            SettingValue::Number(_) => {
                                if let Ok(n) = value_str.parse::<i32>() {
                                    SettingValue::Number(n)
                                } else {
                                    println!("Invalid number");
                                    continue;
                                }
                            }
                            SettingValue::String(_) => SettingValue::String(value_str.to_string()),
                            SettingValue::Select(options, _) => {
                                if let Ok(idx) = value_str.parse::<usize>() {
                                    SettingValue::Select(options.clone(), idx)
                                } else {
                                    println!("Invalid index");
                                    continue;
                                }
                            }
                        };
                        
                        match settings.update_setting(key, new_value) {
                            Ok(_) => println!("Setting updated"),
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    } else {
                        println!("Setting not found: {}", key);
                    }
                }
            }
            "reset" => {
                settings.reset_to_defaults();
                println!("Reset to defaults");
            }
            "save" => {
                match settings.save() {
                    Ok(_) => println!("Settings saved"),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            "quit" | "exit" => {
                if settings.is_modified() {
                    println!("Unsaved changes! Use 'save' first or 'force' to quit.");
                } else {
                    break;
                }
            }
            "force" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
