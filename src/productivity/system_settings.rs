// SigmaOS Unified System Settings and Preferences Manager (S-CONTROL)
// Inspired by GNOME/KDE Control Centers, Windows Control Panel, and BSD rc.conf settings.
// Manages accounts, network, bluetooth, backup, drivers, visual effects, firewall, font, input methods, and touch preferences.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserAccount {
    pub username: String,
    pub full_name: String,
    pub shell_path: String,
    pub language: String,
    pub input_method: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisplayPreference {
    pub resolution: (u32, u32),
    pub refresh_rate_hz: u32,
    pub visual_effects_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputDeviceSettings {
    pub mouse_sensitivity_level: u32, // 1 to 10 scale
    pub touchpad_natural_scrolling: bool,
    pub touch_preferences_enabled: bool,
}

/// Unified Control Center Settings Database
pub struct UnifiedSettingsManager {
    pub accounts: HashMap<String, UserAccount>,
    pub display: DisplayPreference,
    pub input_settings: InputDeviceSettings,
    pub desktop_background: String,
    pub bluetooth_enabled: bool,
    pub bluetooth_paired_devices: Vec<String>,
    pub firewall_block_all: bool,
    pub system_font: String,
    pub last_backup_timestamp: u64,
}

impl UnifiedSettingsManager {
    pub fn new() -> Self {
        let mut usm = Self {
            accounts: HashMap::new(),
            display: DisplayPreference {
                resolution: (1920, 1080),
                refresh_rate_hz: 60,
                visual_effects_enabled: true,
            },
            input_settings: InputDeviceSettings {
                mouse_sensitivity_level: 5,
                touchpad_natural_scrolling: true,
                touch_preferences_enabled: false,
            },
            desktop_background: "default_sovereign.jpg".to_string(),
            bluetooth_enabled: false,
            bluetooth_paired_devices: Vec::new(),
            firewall_block_all: false,
            system_font: "Sovereign Sans".to_string(),
            last_backup_timestamp: 0,
        };

        // Seed default admin account
        usm.register_account("admin", "Sovereign Administrator", "/bin/sigma-sh", "en_IN", "us-qwerty");
        usm
    }

    pub fn register_account(&mut self, username: &str, full_name: &str, shell: &str, lang: &str, input_method: &str) {
        self.accounts.insert(
            username.to_string(),
            UserAccount {
                username: username.to_string(),
                full_name: full_name.to_string(),
                shell_path: shell.to_string(),
                language: lang.to_string(),
                input_method: input_method.to_string(),
            },
        );
    }

    pub fn change_desktop_background(&mut self, bg_name: &str) {
        self.desktop_background = bg_name.to_string();
    }

    pub fn set_display_resolution(&mut self, width: u32, height: u32, hz: u32) {
        self.display.resolution = (width, height);
        self.display.refresh_rate_hz = hz;
    }

    pub fn set_visual_effects(&mut self, enabled: bool) {
        self.display.visual_effects_enabled = enabled;
    }

    pub fn toggle_bluetooth(&mut self, enabled: bool) {
        self.bluetooth_enabled = enabled;
        if !enabled {
            self.bluetooth_paired_devices.clear();
        }
    }

    pub fn pair_bluetooth_device(&mut self, device_name: &str) -> bool {
        if self.bluetooth_enabled {
            self.bluetooth_paired_devices.push(device_name.to_string());
            true
        } else {
            false
        }
    }

    pub fn configure_firewall(&mut self, block_all: bool) {
        self.firewall_block_all = block_all;
    }

    pub fn trigger_backup(&mut self, current_time: u64) -> bool {
        self.last_backup_timestamp = current_time;
        true
    }

    pub fn set_system_font(&mut self, font_name: &str) {
        self.system_font = font_name.to_string();
    }
}

impl Default for UnifiedSettingsManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_settings_manager_lifecycle() {
        let mut manager = UnifiedSettingsManager::new();

        // 1. Account preference updates
        manager.register_account("ravi", "Ravi Kumar", "/bin/zsh", "hi_IN", "hindi-inscript");
        let ravi = manager.accounts.get("ravi").unwrap();
        assert_eq!(ravi.full_name, "Ravi Kumar");
        assert_eq!(ravi.language, "hi_IN");

        // 2. Display and visual effects
        manager.set_display_resolution(2560, 1440, 120);
        assert_eq!(manager.display.resolution, (2560, 1440));
        assert_eq!(manager.display.refresh_rate_hz, 120);
        manager.set_visual_effects(false);
        assert!(!manager.display.visual_effects_enabled);

        // 3. Desktop Backgrounds
        manager.change_desktop_background("india_valley.png");
        assert_eq!(manager.desktop_background, "india_valley.png");

        // 4. Bluetooth pairing
        assert!(!manager.pair_bluetooth_device("Sovereign Buds")); // Disabled by default
        manager.toggle_bluetooth(true);
        assert!(manager.pair_bluetooth_device("Sovereign Buds"));
        assert_eq!(manager.bluetooth_paired_devices[0], "Sovereign Buds");

        // 5. Fonts and Firewall
        manager.set_system_font("Sovereign Mono");
        assert_eq!(manager.system_font, "Sovereign Mono");
        manager.configure_firewall(true);
        assert!(manager.firewall_block_all);

        // 6. Backup Tool
        assert!(manager.trigger_backup(1716000000));
        assert_eq!(manager.last_backup_timestamp, 1716000000);
    }
}
