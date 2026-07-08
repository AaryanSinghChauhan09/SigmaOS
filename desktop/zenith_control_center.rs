// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// desktop/zenith_control_center.rs — Zenith Unified Control Center
//
// Implements a unified control center for system settings, providing
// centralized access to all system configuration options
//
// Language: Rust (std for userland services)

use std::collections::HashMap;

// ─── Control Center Panel ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PanelType {
    Network,
    Display,
    Sound,
    Bluetooth,
    WiFi,
    Power,
    Storage,
    Accessibility,
    Security,
    Accounts,
    Updates,
    About,
}

#[derive(Debug, Clone)]
pub struct Panel {
    pub panel_type: PanelType,
    pub name: String,
    pub icon: String,
    pub enabled: bool,
}

// ─── Setting Item ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum SettingType {
    Toggle,
    Slider,
    Select,
    Text,
    Color,
    Action,
}

#[derive(Debug, Clone)]
pub struct SettingItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub setting_type: SettingType,
    pub value: SettingValue,
    pub min: f32,
    pub max: f32,
    pub options: Vec<String>,
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub enum SettingValue {
    Bool(bool),
    Float(f32),
    String(String),
    Color([u8; 4]),
}

// ─── Control Center State ───────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ControlCenterState {
    pub panels: Vec<Panel>,
    pub current_panel: Option<PanelType>,
    pub settings: HashMap<String, SettingItem>,
    pub quick_settings: Vec<SettingItem>,
    pub notifications_enabled: bool,
    pub do_not_disturb: bool,
    pub initialized: bool,
}

impl ControlCenterState {
    pub fn new() -> Self {
        let mut state = ControlCenterState {
            panels: vec![
                Panel {
                    panel_type: PanelType::Network,
                    name: "Network".to_string(),
                    icon: "network".to_string(),
                    enabled: true,
                },
                Panel {
                    panel_type: PanelType::Display,
                    name: "Display".to_string(),
                    icon: "display".to_string(),
                    enabled: true,
                },
                Panel {
                    panel_type: PanelType::Sound,
                    name: "Sound".to_string(),
                    icon: "sound".to_string(),
                    enabled: true,
                },
                Panel {
                    panel_type: PanelType::Bluetooth,
                    name: "Bluetooth".to_string(),
                    icon: "bluetooth".to_string(),
                    enabled: true,
                },
                Panel {
                    panel_type: PanelType::WiFi,
                    name: "WiFi".to_string(),
                    icon: "wifi".to_string(),
                    enabled: true,
                },
                Panel {
                    panel_type: PanelType::Power,
                    name: "Power".to_string(),
                    icon: "power".to_string(),
                    enabled: true,
                },
                Panel {
                    panel_type: PanelType::Storage,
                    name: "Storage".to_string(),
                    icon: "storage".to_string(),
                    enabled: true,
                },
                Panel {
                    panel_type: PanelType::Accessibility,
                    name: "Accessibility".to_string(),
                    icon: "accessibility".to_string(),
                    enabled: true,
                },
                Panel {
                    panel_type: PanelType::Security,
                    name: "Security".to_string(),
                    icon: "security".to_string(),
                    enabled: true,
                },
                Panel {
                    panel_type: PanelType::Accounts,
                    name: "Accounts".to_string(),
                    icon: "accounts".to_string(),
                    enabled: true,
                },
                Panel {
                    panel_type: PanelType::Updates,
                    name: "Updates".to_string(),
                    icon: "updates".to_string(),
                    enabled: true,
                },
                Panel {
                    panel_type: PanelType::About,
                    name: "About".to_string(),
                    icon: "about".to_string(),
                    enabled: true,
                },
            ],
            current_panel: None,
            settings: HashMap::new(),
            quick_settings: vec![
                SettingItem {
                    id: "wifi".to_string(),
                    name: "WiFi".to_string(),
                    description: "Enable or disable WiFi".to_string(),
                    setting_type: SettingType::Toggle,
                    value: SettingValue::Bool(true),
                    min: 0.0,
                    max: 0.0,
                    options: vec![],
                    enabled: true,
                },
                SettingItem {
                    id: "bluetooth".to_string(),
                    name: "Bluetooth".to_string(),
                    description: "Enable or disable Bluetooth".to_string(),
                    setting_type: SettingType::Toggle,
                    value: SettingValue::Bool(true),
                    min: 0.0,
                    max: 0.0,
                    options: vec![],
                    enabled: true,
                },
                SettingItem {
                    id: "do_not_disturb".to_string(),
                    name: "Do Not Disturb".to_string(),
                    description: "Silence notifications".to_string(),
                    setting_type: SettingType::Toggle,
                    value: SettingValue::Bool(false),
                    min: 0.0,
                    max: 0.0,
                    options: vec![],
                    enabled: true,
                },
                SettingItem {
                    id: "brightness".to_string(),
                    name: "Brightness".to_string(),
                    description: "Adjust screen brightness".to_string(),
                    setting_type: SettingType::Slider,
                    value: SettingValue::Float(0.8),
                    min: 0.0,
                    max: 1.0,
                    options: vec![],
                    enabled: true,
                },
                SettingItem {
                    id: "volume".to_string(),
                    name: "Volume".to_string(),
                    description: "Adjust system volume".to_string(),
                    setting_type: SettingType::Slider,
                    value: SettingValue::Float(0.7),
                    min: 0.0,
                    max: 1.0,
                    options: vec![],
                    enabled: true,
                },
            ],
            notifications_enabled: true,
            do_not_disturb: false,
            initialized: false,
        };

        state.init_default_settings();
        state
    }

    fn init_default_settings(&mut self) {
        // Network settings
        self.settings.insert(
            "network_wifi_enabled".to_string(),
            SettingItem {
                id: "network_wifi_enabled".to_string(),
                name: "WiFi".to_string(),
                description: "Enable WiFi connection".to_string(),
                setting_type: SettingType::Toggle,
                value: SettingValue::Bool(true),
                min: 0.0,
                max: 0.0,
                options: vec![],
                enabled: true,
            },
        );

        // Display settings
        self.settings.insert(
            "display_brightness".to_string(),
            SettingItem {
                id: "display_brightness".to_string(),
                name: "Brightness".to_string(),
                description: "Screen brightness level".to_string(),
                setting_type: SettingType::Slider,
                value: SettingValue::Float(0.8),
                min: 0.0,
                max: 1.0,
                options: vec![],
                enabled: true,
            },
        );

        self.settings.insert(
            "display_night_light".to_string(),
            SettingItem {
                id: "display_night_light".to_string(),
                name: "Night Light".to_string(),
                description: "Reduce blue light at night".to_string(),
                setting_type: SettingType::Toggle,
                value: SettingValue::Bool(false),
                min: 0.0,
                max: 0.0,
                options: vec![],
                enabled: true,
            },
        );

        // Sound settings
        self.settings.insert(
            "sound_volume".to_string(),
            SettingItem {
                id: "sound_volume".to_string(),
                name: "Volume".to_string(),
                description: "System volume level".to_string(),
                setting_type: SettingType::Slider,
                value: SettingValue::Float(0.7),
                min: 0.0,
                max: 1.0,
                options: vec![],
                enabled: true,
            },
        );

        self.settings.insert(
            "sound_mute".to_string(),
            SettingItem {
                id: "sound_mute".to_string(),
                name: "Mute".to_string(),
                description: "Mute all sounds".to_string(),
                setting_type: SettingType::Toggle,
                value: SettingValue::Bool(false),
                min: 0.0,
                max: 0.0,
                options: vec![],
                enabled: true,
            },
        );

        // Power settings
        self.settings.insert(
            "power_battery_saver".to_string(),
            SettingItem {
                id: "power_battery_saver".to_string(),
                name: "Battery Saver".to_string(),
                description: "Extend battery life".to_string(),
                setting_type: SettingType::Toggle,
                value: SettingValue::Bool(false),
                min: 0.0,
                max: 0.0,
                options: vec![],
                enabled: true,
            },
        );
    }

    /// Initialize control center
    pub fn init(&mut self) {
        self.initialized = true;
    }

    /// Open panel
    pub fn open_panel(&mut self, panel_type: PanelType) {
        self.current_panel = Some(panel_type);
    }

    /// Close panel
    pub fn close_panel(&mut self) {
        self.current_panel = None;
    }

    /// Get current panel
    pub fn get_current_panel(&self) -> Option<PanelType> {
        self.current_panel
    }

    /// Get panel settings
    pub fn get_panel_settings(&self, panel_type: PanelType) -> Vec<SettingItem> {
        match panel_type {
            PanelType::Network => {
                vec![
                    self.settings.get("network_wifi_enabled").cloned().unwrap(),
                ]
            }
            PanelType::Display => {
                vec![
                    self.settings.get("display_brightness").cloned().unwrap(),
                    self.settings.get("display_night_light").cloned().unwrap(),
                ]
            }
            PanelType::Sound => {
                vec![
                    self.settings.get("sound_volume").cloned().unwrap(),
                    self.settings.get("sound_mute").cloned().unwrap(),
                ]
            }
            PanelType::Power => {
                vec![
                    self.settings.get("power_battery_saver").cloned().unwrap(),
                ]
            }
            _ => vec![],
        }
    }

    /// Update setting value
    pub fn update_setting(&mut self, id: &str, value: SettingValue) -> bool {
        if let Some(setting) = self.settings.get_mut(id) {
            setting.value = value;
            return true;
        }
        false
    }

    /// Get setting value
    pub fn get_setting(&self, id: &str) -> Option<SettingValue> {
        self.settings.get(id).map(|s| s.value.clone())
    }

    /// Update quick setting
    pub fn update_quick_setting(&mut self, id: &str, value: SettingValue) -> bool {
        for setting in &mut self.quick_settings {
            if setting.id == id {
                setting.value = value;
                return true;
            }
        }
        false
    }

    /// Get quick setting
    pub fn get_quick_setting(&self, id: &str) -> Option<SettingValue> {
        self.quick_settings.iter().find(|s| s.id == id).map(|s| s.value.clone())
    }

    /// Toggle Do Not Disturb
    pub fn toggle_do_not_disturb(&mut self) {
        self.do_not_disturb = !self.do_not_disturb;
        self.update_quick_setting("do_not_disturb", SettingValue::Bool(self.do_not_disturb));
    }

    /// Enable/disable notifications
    pub fn set_notifications_enabled(&mut self, enabled: bool) {
        self.notifications_enabled = enabled;
    }

    /// Check if notifications are enabled
    pub fn is_notifications_enabled(&self) -> bool {
        self.notifications_enabled
    }

    /// Check if Do Not Disturb is enabled
    pub fn is_do_not_disturb(&self) -> bool {
        self.do_not_disturb
    }

    /// Get all panels
    pub fn get_panels(&self) -> Vec<Panel> {
        self.panels.clone()
    }

    /// Get quick settings
    pub fn get_quick_settings(&self) -> Vec<SettingItem> {
        self.quick_settings.clone()
    }

    /// Search settings
    pub fn search_settings(&self, query: &str) -> Vec<SettingItem> {
        let query_lower = query.to_lowercase();
        self.settings
            .values()
            .filter(|s| {
                s.name.to_lowercase().contains(&query_lower)
                    || s.description.to_lowercase().contains(&query_lower)
            })
            .cloned()
            .collect()
    }

    /// Reset to defaults
    pub fn reset_to_defaults(&mut self) {
        self.settings.clear();
        self.init_default_settings();
        self.quick_settings = vec![
            SettingItem {
                id: "wifi".to_string(),
                name: "WiFi".to_string(),
                description: "Enable or disable WiFi".to_string(),
                setting_type: SettingType::Toggle,
                value: SettingValue::Bool(true),
                min: 0.0,
                max: 0.0,
                options: vec![],
                enabled: true,
            },
            SettingItem {
                id: "bluetooth".to_string(),
                name: "Bluetooth".to_string(),
                description: "Enable or disable Bluetooth".to_string(),
                setting_type: SettingType::Toggle,
                value: SettingValue::Bool(true),
                min: 0.0,
                max: 0.0,
                options: vec![],
                enabled: true,
            },
            SettingItem {
                id: "do_not_disturb".to_string(),
                name: "Do Not Disturb".to_string(),
                description: "Silence notifications".to_string(),
                setting_type: SettingType::Toggle,
                value: SettingValue::Bool(false),
                min: 0.0,
                max: 0.0,
                options: vec![],
                enabled: true,
            },
            SettingItem {
                id: "brightness".to_string(),
                name: "Brightness".to_string(),
                description: "Adjust screen brightness".to_string(),
                setting_type: SettingType::Slider,
                value: SettingValue::Float(0.8),
                min: 0.0,
                max: 1.0,
                options: vec![],
                enabled: true,
            },
            SettingItem {
                id: "volume".to_string(),
                name: "Volume".to_string(),
                description: "Adjust system volume".to_string(),
                setting_type: SettingType::Slider,
                value: SettingValue::Float(0.7),
                min: 0.0,
                max: 1.0,
                options: vec![],
                enabled: true,
            },
        ];
    }

    /// Export settings to JSON
    pub fn export_json(&self) -> String {
        // In real implementation, generate JSON representation
        r#"{"panels":{}, "settings":{}}"#.to_string()
    }

    /// Import settings from JSON
    pub fn import_json(&mut self, json: &str) -> bool {
        // In real implementation, parse JSON and apply settings
        true
    }
}

// ─── Control Center Manager ───────────────────────────────────────────────────

pub struct ControlCenterManager {
    pub state: ControlCenterState,
    pub visible: bool,
}

impl ControlCenterManager {
    pub fn new() -> Self {
        ControlCenterManager {
            state: ControlCenterState::new(),
            visible: false,
        }
    }

    pub fn init(&mut self) {
        self.state.init();
    }

    pub fn show(&mut self) {
        self.visible = true;
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }

    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }
}
