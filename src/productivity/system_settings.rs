use std::string::{String, ToString};
use std::vec::Vec;
// SigmaOS Unified System Settings and Preferences Manager (S-CONTROL)
// Inspired by GNOME/KDE Control Centers, Windows Control Panel, and BSD rc.conf settings.
// Manages accounts, network, bluetooth, backup, drivers, visual effects, firewall, font, input methods, and touch preferences.

use std::collections::BTreeMap;

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

/// openSUSE YaST inspired control module manager
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YastModuleControl {
    pub active_modules: BTreeMap<String, bool>,
}

impl YastModuleControl {
    pub fn new() -> Self {
        let mut modules = BTreeMap::new();
        modules.insert("hardware_setup".to_string(), true);
        modules.insert("network_services".to_string(), true);
        modules.insert("security_audit".to_string(), true);
        modules.insert("storage_partitioner".to_string(), true);
        Self { active_modules: modules }
    }

    pub fn set_module_status(&mut self, module: &str, enabled: bool) {
        self.active_modules.insert(module.to_string(), enabled);
    }
}

impl Default for YastModuleControl {
    fn default() -> Self {
        Self::new()
    }
}

/// Gentoo eselect profile and USE flag manager
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EselectProfileManager {
    pub active_profile: String,
    pub global_use_flags: Vec<String>,
}

impl EselectProfileManager {
    pub fn new() -> Self {
        Self {
            active_profile: "default/linux/amd64/23.0/desktop/systemd".to_string(),
            global_use_flags: vec!["X".to_string(), "wayland".to_string(), "vulkan".to_string()],
        }
    }

    pub fn set_profile(&mut self, profile_name: &str) {
        self.active_profile = profile_name.to_string();
    }

    pub fn toggle_use_flag(&mut self, flag: &str, enable: bool) {
        if enable {
            if !self.global_use_flags.contains(&flag.to_string()) {
                self.global_use_flags.push(flag.to_string());
            }
        } else {
            self.global_use_flags.retain(|f| f != flag);
        }
    }
}

impl Default for EselectProfileManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Ubuntu/Fedora driver manager settings
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DriverManagerSettings {
    pub proprietary_drivers_allowed: bool,
    pub active_gpu_driver: String,
}

impl DriverManagerSettings {
    pub fn new() -> Self {
        Self {
            proprietary_drivers_allowed: true,
            active_gpu_driver: "amdgpu".to_string(),
        }
    }

    pub fn switch_gpu_driver(&mut self, driver_name: &str) {
        self.active_gpu_driver = driver_name.to_string();
    }
}

impl Default for DriverManagerSettings {
    fn default() -> Self {
        Self::new()
    }
}

/// BSD /etc/rc.conf service configuration settings
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BsdRcConfSettings {
    pub service_flags: BTreeMap<String, String>,
}

impl BsdRcConfSettings {
    pub fn new() -> Self {
        let mut flags = BTreeMap::new();
        flags.insert("sshd_enable".to_string(), "YES".to_string());
        flags.insert("ntpd_enable".to_string(), "YES".to_string());
        flags.insert("pf_enable".to_string(), "YES".to_string());
        Self { service_flags: flags }
    }

    pub fn set_service_enable(&mut self, service: &str, enable: bool) {
        let key = format!("{}_enable", service);
        let val = if enable { "YES" } else { "NO" };
        self.service_flags.insert(key, val.to_string());
    }

    pub fn is_service_enabled(&self, service: &str) -> bool {
        let key = format!("{}_enable", service);
        self.service_flags.get(&key).map(|v| v == "YES").unwrap_or(false)
    }
}

impl Default for BsdRcConfSettings {
    fn default() -> Self {
        Self::new()
    }
}

/// Unified Control Center Settings Database
pub struct UnifiedSettingsManager {
    pub accounts: BTreeMap<String, UserAccount>,
    pub display: DisplayPreference,
    pub input_settings: InputDeviceSettings,
    pub yast_control: YastModuleControl,
    pub eselect_manager: EselectProfileManager,
    pub driver_settings: DriverManagerSettings,
    pub rc_conf: BsdRcConfSettings,
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
            accounts: BTreeMap::new(),
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
            yast_control: YastModuleControl::new(),
            eselect_manager: EselectProfileManager::new(),
            driver_settings: DriverManagerSettings::new(),
            rc_conf: BsdRcConfSettings::new(),
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

    #[test]
    fn test_yast_module_control() {
        let mut yast = YastModuleControl::new();
        assert_eq!(yast.active_modules.get("hardware_setup"), Some(&true));
        yast.set_module_status("bluetooth_daemon", true);
        assert_eq!(yast.active_modules.get("bluetooth_daemon"), Some(&true));
    }

    #[test]
    fn test_eselect_profile_manager() {
        let mut eselect = EselectProfileManager::new();
        assert!(eselect.global_use_flags.contains(&"wayland".to_string()));
        eselect.toggle_use_flag("cuda", true);
        assert!(eselect.global_use_flags.contains(&"cuda".to_string()));
        eselect.toggle_use_flag("cuda", false);
        assert!(!eselect.global_use_flags.contains(&"cuda".to_string()));

        eselect.set_profile("default/linux/amd64/23.0/hardened");
        assert_eq!(eselect.active_profile, "default/linux/amd64/23.0/hardened");
    }

    #[test]
    fn test_driver_manager_settings() {
        let mut driver_mgr = DriverManagerSettings::new();
        assert_eq!(driver_mgr.active_gpu_driver, "amdgpu");
        driver_mgr.switch_gpu_driver("nvidia");
        assert_eq!(driver_mgr.active_gpu_driver, "nvidia");
    }

    #[test]
    fn test_bsd_rc_conf_settings() {
        let mut rc = BsdRcConfSettings::new();
        assert!(rc.is_service_enabled("sshd"));
        assert!(rc.is_service_enabled("pf"));
        assert!(!rc.is_service_enabled("nginx"));

        rc.set_service_enable("nginx", true);
        assert!(rc.is_service_enabled("nginx"));

        rc.set_service_enable("sshd", false);
        assert!(!rc.is_service_enabled("sshd"));
    }
}
