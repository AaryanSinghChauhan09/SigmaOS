// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// desktop/zenith_tray.rs — Zenith System Tray
//
// Implements the system tray with time, battery, network status,
// and quick settings for volume, brightness, and Wi-Fi.
//
// Language: Rust (std for userland services)

use std::time::{SystemTime, UNIX_EPOCH};

// ─── Tray Item Types ───────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum TrayItem {
    Clock(ClockItem),
    Battery(BatteryItem),
    Network(NetworkItem),
    Volume(VolumeItem),
    Brightness(BrightnessItem),
}

#[derive(Debug, Clone)]
pub struct ClockItem {
    pub time: String,
    pub date: String,
}

#[derive(Debug, Clone)]
pub struct BatteryItem {
    pub percentage: u8,
    pub charging: bool,
    pub time_remaining: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NetworkItem {
    pub connected: bool,
    pub ssid: Option<String>,
    pub signal_strength: u8, // 0-100
    pub interface: String,
}

#[derive(Debug, Clone)]
pub struct VolumeItem {
    pub level: u8, // 0-100
    pub muted: bool,
}

#[derive(Debug, Clone)]
pub struct BrightnessItem {
    pub level: u8, // 0-100
}

// ─── Quick Settings State ─────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct QuickSettings {
    pub volume: u8,
    pub muted: bool,
    pub brightness: u8,
    pub wifi_enabled: bool,
    pub bluetooth_enabled: bool,
    pub airplane_mode: bool,
}

// ─── System Tray ─────────────────────────────────────────────────────────────

pub struct ZenithTray {
    pub items: Vec<TrayItem>,
    pub quick_settings: QuickSettings,
    pub visible: bool,
    pub initialized: bool,
}

impl ZenithTray {
    pub fn new() -> Self {
        let mut tray = ZenithTray {
            items: Vec::new(),
            quick_settings: QuickSettings {
                volume: 50,
                muted: false,
                brightness: 75,
                wifi_enabled: true,
                bluetooth_enabled: true,
                airplane_mode: false,
            },
            visible: true,
            initialized: false,
        };
        
        tray.init();
        tray
    }

    /// Initialize tray with default items
    pub fn init(&mut self) {
        self.items.push(TrayItem::Clock(ClockItem {
            time: "00:00".to_string(),
            date: "1970-01-01".to_string(),
        }));

        self.items.push(TrayItem::Battery(BatteryItem {
            percentage: 100,
            charging: false,
            time_remaining: None,
        }));

        self.items.push(TrayItem::Network(NetworkItem {
            connected: false,
            ssid: None,
            signal_strength: 0,
            interface: "eth0".to_string(),
        }));

        self.items.push(TrayItem::Volume(VolumeItem {
            level: 50,
            muted: false,
        }));

        self.items.push(TrayItem::Brightness(BrightnessItem {
            level: 75,
        }));

        self.initialized = true;
    }

    /// Update clock time
    pub fn update_clock(&mut self) {
        if let Some(TrayItem::Clock(clock)) = self.items.get_mut(0) {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            
            let hours = (now % 86400) / 3600;
            let minutes = (now % 3600) / 60;
            let days = now / 86400;
            
            // Simple date calculation (epoch-based)
            let year = 1970 + (days / 365) as i32;
            let day_of_year = (days % 365) as u32;
            let month = (day_of_year / 30) + 1;
            let day = (day_of_year % 30) + 1;
            
            clock.time = format!("{:02}:{:02}", hours, minutes);
            clock.date = format!("{}-{:02}-{:02}", year, month, day);
        }
    }

    /// Update battery status
    pub fn update_battery(&mut self, percentage: u8, charging: bool, time_remaining: Option<String>) {
        for item in &mut self.items {
            if let TrayItem::Battery(battery) = item {
                battery.percentage = percentage;
                battery.charging = charging;
                battery.time_remaining = time_remaining;
                break;
            }
        }
    }

    /// Update network status
    pub fn update_network(&mut self, connected: bool, ssid: Option<String>, signal_strength: u8) {
        for item in &mut self.items {
            if let TrayItem::Network(network) = item {
                network.connected = connected;
                network.ssid = ssid;
                network.signal_strength = signal_strength;
                break;
            }
        }
    }

    /// Set volume level
    pub fn set_volume(&mut self, level: u8) {
        self.quick_settings.volume = level.min(100);
        self.quick_settings.muted = level == 0;
        
        for item in &mut self.items {
            if let TrayItem::Volume(volume) = item {
                volume.level = self.quick_settings.volume;
                volume.muted = self.quick_settings.muted;
                break;
            }
        }
    }

    /// Toggle mute
    pub fn toggle_mute(&mut self) {
        self.quick_settings.muted = !self.quick_settings.muted;
        
        for item in &mut self.items {
            if let TrayItem::Volume(volume) = item {
                volume.muted = self.quick_settings.muted;
                break;
            }
        }
    }

    /// Set brightness level
    pub fn set_brightness(&mut self, level: u8) {
        self.quick_settings.brightness = level.min(100);
        
        for item in &mut self.items {
            if let TrayItem::Brightness(brightness) = item {
                brightness.level = self.quick_settings.brightness;
                break;
            }
        }
    }

    /// Toggle Wi-Fi
    pub fn toggle_wifi(&mut self) {
        self.quick_settings.wifi_enabled = !self.quick_settings.wifi_enabled;
    }

    /// Toggle Bluetooth
    pub fn toggle_bluetooth(&mut self) {
        self.quick_settings.bluetooth_enabled = !self.quick_settings.bluetooth_enabled;
    }

    /// Toggle airplane mode
    pub fn toggle_airplane_mode(&mut self) {
        self.quick_settings.airplane_mode = !self.quick_settings.airplane_mode;
        
        // Disable Wi-Fi and Bluetooth when airplane mode is enabled
        if self.quick_settings.airplane_mode {
            self.quick_settings.wifi_enabled = false;
            self.quick_settings.bluetooth_enabled = false;
        }
    }

    /// Get current time string
    pub fn get_time(&self) -> String {
        for item in &self.items {
            if let TrayItem::Clock(clock) = item {
                return clock.time.clone();
            }
        }
        "00:00".to_string()
    }

    /// Get battery percentage
    pub fn get_battery_percentage(&self) -> u8 {
        for item in &self.items {
            if let TrayItem::Battery(battery) = item {
                return battery.percentage;
            }
        }
        0
    }

    /// Get network status
    pub fn get_network_status(&self) -> (bool, Option<String>, u8) {
        for item in &self.items {
            if let TrayItem::Network(network) = item {
                return (network.connected, network.ssid.clone(), network.signal_strength);
            }
        }
        (false, None, 0)
    }

    /// Show/hide tray
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    /// Get tray items for rendering
    pub fn get_items(&self) -> &[TrayItem] {
        &self.items
    }

    /// Get quick settings
    pub fn get_quick_settings(&self) -> &QuickSettings {
        &self.quick_settings
    }
}
