// SigmaOS Unified Control Center
// Inspired by Linux Mint Cinnamon Control Center, elementaryOS Switchboard, openSUSE YaST2 Control Center, and BSD Security/Sysctl Systems
// - Modular Switchboard plug architecture for dynamic setting category registration
// - Cinnamon Spices management (Applets, Desklets, Extensions, Actions)
// - MintDrivers hardware driver management & switching
// - Timeshift system restore checkpointing
// - OpenBSD pledge/unveil policies & FreeBSD Jail security controls
// - Comprehensive system settings management (Network, Display, Sound, Power, Users, Security, Storage, Printers, System, Spices, Themes, Startup, Drivers, Restoration, BsdJails)

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::vec;

/// Categories for Control Center settings
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ControlCenterCategory {
    Network,
    Display,
    Sound,
    Power,
    Users,
    Security,
    Storage,
    Printers,
    System,
    Spices,
    Themes,
    Startup,
    Drivers,
    Restoration,
    BsdJails,
}

/// Individual setting item in a plug
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemSettingItem {
    pub key: String,
    pub label: String,
    pub value: String,
    pub is_editable: bool,
}

/// Modular Switchboard Plug representation
#[derive(Debug, Clone)]
pub struct SwitchboardPlug {
    pub plug_id: String,
    pub name: String,
    pub description: String,
    pub category: ControlCenterCategory,
    pub settings: Vec<SystemSettingItem>,
}

impl SwitchboardPlug {
    pub fn new(plug_id: &str, name: &str, description: &str, category: ControlCenterCategory) -> Self {
        Self {
            plug_id: plug_id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            category,
            settings: Vec::new(),
        }
    }

    pub fn add_setting(&mut self, key: &str, label: &str, value: &str, is_editable: bool) {
        self.settings.push(SystemSettingItem {
            key: key.to_string(),
            label: label.to_string(),
            value: value.to_string(),
            is_editable,
        });
    }

    pub fn set_value(&mut self, key: &str, new_value: &str) -> bool {
        if let Some(item) = self.settings.iter_mut().find(|i| i.key == key) {
            if item.is_editable {
                item.value = new_value.to_string();
                return true;
            }
        }
        false
    }
}

// =========================================================================
// Cinnamon & Linux/BSD Specific Data Structures
// =========================================================================

/// Cinnamon Spice Component Types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CinnamonSpiceType {
    Applet,
    Desklet,
    Extension,
    Action,
}

/// Cinnamon Spice Metadata
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CinnamonSpiceItem {
    pub id: String,
    pub name: String,
    pub spice_type: CinnamonSpiceType,
    pub enabled: bool,
    pub author: String,
    pub version: String,
}

/// Hardware Driver metadata (Linux Mint Drivers Manager inspired)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MintDriverEntry {
    pub id: String,
    pub name: String,
    pub hardware_class: String,
    pub proprietary: bool,
    pub active: bool,
    pub driver_version: String,
}

/// Timeshift System Restore Checkpoint
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeshiftRestorePoint {
    pub snapshot_id: u32,
    pub label: String,
    pub is_rsync: bool,
    pub timestamp_epoch: u64,
    pub system_hash: u64,
}

/// BSD Security & Sysctl Tuning Profile
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BsdSecurityPolicy {
    pub pledge_rules: String,
    pub unveil_paths: String,
    pub securelevel: i32,
    pub active_jails_count: usize,
}

/// Cinnamon Desktop Customization Styling Settings
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CinnamonDesktopConfig {
    pub panel_height: u32,
    pub menu_layout_compact: bool,
    pub opacity_percent: u32,
    pub gtk_theme: String,
    pub window_effects_enabled: bool,
}

impl Default for CinnamonDesktopConfig {
    fn default() -> Self {
        Self {
            panel_height: 40,
            menu_layout_compact: false,
            opacity_percent: 100,
            gtk_theme: "Mint-Y-Dark".to_string(),
            window_effects_enabled: true,
        }
    }
}

/// Master Unified Control Center Engine
#[derive(Debug, Clone)]
pub struct UnifiedControlCenter {
    pub plugs: BTreeMap<String, SwitchboardPlug>,
    pub dark_mode: bool,
    pub active_category: ControlCenterCategory,
    pub spices: Vec<CinnamonSpiceItem>,
    pub drivers: Vec<MintDriverEntry>,
    pub restore_points: Vec<TimeshiftRestorePoint>,
    pub bsd_policy: BsdSecurityPolicy,
    pub cinnamon_config: CinnamonDesktopConfig,
}

impl UnifiedControlCenter {
    pub fn new() -> Self {
        let mut cc = Self {
            plugs: BTreeMap::new(),
            dark_mode: true,
            active_category: ControlCenterCategory::Network,
            spices: Vec::new(),
            drivers: Vec::new(),
            restore_points: Vec::new(),
            bsd_policy: BsdSecurityPolicy {
                pledge_rules: "stdio rpath wpath cpath inet".to_string(),
                unveil_paths: "/etc /usr /var /tmp".to_string(),
                securelevel: 1,
                active_jails_count: 2,
            },
            cinnamon_config: CinnamonDesktopConfig::default(),
        };

        // Initialize default core system plugs (inspired by elementaryOS Switchboard)
        let mut net_plug = SwitchboardPlug::new(
            "plug_net",
            "Network & Wireless",
            "Manage Wi-Fi, Ethernet, and WireGuard VPNs",
            ControlCenterCategory::Network,
        );
        net_plug.add_setting("wifi_enabled", "Wi-Fi Switch", "true", true);
        net_plug.add_setting("ip_address", "IP Address", "192.168.1.100", false);
        cc.register_plug(net_plug);

        let mut display_plug = SwitchboardPlug::new(
            "plug_display",
            "Display & Scaling",
            "Configure resolution, 4K scaling, and Night Light",
            ControlCenterCategory::Display,
        );
        display_plug.add_setting("resolution", "Resolution", "3840x2160", true);
        display_plug.add_setting("scaling", "Fractional Scaling", "125%", true);
        cc.register_plug(display_plug);

        let mut sec_plug = SwitchboardPlug::new(
            "plug_security",
            "Security & Privacy",
            "OpenBSD pledge/unveil policies and Firewall controls",
            ControlCenterCategory::Security,
        );
        sec_plug.add_setting("firewall_active", "Stateful PF Firewall", "true", true);
        sec_plug.add_setting("securelevel", "BSD Securelevel", "1", true);
        cc.register_plug(sec_plug);

        // Cinnamon Spices Switchboard Plug
        let mut spices_plug = SwitchboardPlug::new(
            "plug_spices",
            "Cinnamon Spices",
            "Manage desktop applets, desklets, extensions, and context actions",
            ControlCenterCategory::Spices,
        );
        spices_plug.add_setting("applets_active", "Active Applets Count", "4", false);
        spices_plug.add_setting("desklets_enabled", "Desklets Engine", "true", true);
        cc.register_plug(spices_plug);

        // MintDrivers Manager Switchboard Plug
        let mut drivers_plug = SwitchboardPlug::new(
            "plug_drivers",
            "Driver Manager",
            "Inspect and activate proprietary or open-source hardware drivers",
            ControlCenterCategory::Drivers,
        );
        drivers_plug.add_setting("gpu_driver", "Primary GPU Driver", "nvidia-open-550", true);
        drivers_plug.add_setting("wifi_driver", "Wireless Card Driver", "broadcom-wl", true);
        cc.register_plug(drivers_plug);

        // Timeshift System Restore Switchboard Plug
        let mut restoration_plug = SwitchboardPlug::new(
            "plug_restoration",
            "System Restoration",
            "Create checkpoints and restore SigmaOS system state via Timeshift",
            ControlCenterCategory::Restoration,
        );
        restoration_plug.add_setting("auto_snapshot", "Daily Timeshift Snapshots", "true", true);
        restoration_plug.add_setting("snapshot_mode", "Snapshot Mode", "rsync", true);
        cc.register_plug(restoration_plug);

        // FreeBSD Jails & OpenBSD Security Switchboard Plug
        let mut bsd_plug = SwitchboardPlug::new(
            "plug_bsd_jails",
            "BSD Jails & Security",
            "Configure FreeBSD isolated jails and OpenBSD pledge sandboxes",
            ControlCenterCategory::BsdJails,
        );
        bsd_plug.add_setting("active_jails", "Running Jails", "2", false);
        bsd_plug.add_setting("pledge_default", "Default Pledge Sandbox", "stdio rpath", true);
        cc.register_plug(bsd_plug);

        // Cinnamon Themes & Desktop Styling Switchboard Plug
        let mut theme_plug = SwitchboardPlug::new(
            "plug_themes",
            "Themes & Appearance",
            "Configure Cinnamon GTK themes, panel dimensions, and window effects",
            ControlCenterCategory::Themes,
        );
        theme_plug.add_setting("gtk_theme", "GTK Theme", "Mint-Y-Dark", true);
        theme_plug.add_setting("panel_height", "Panel Height (px)", "40", true);
        theme_plug.add_setting("window_effects", "Window Animations", "true", true);
        cc.register_plug(theme_plug);

        // Seed default Cinnamon Spices
        cc.spices.push(CinnamonSpiceItem {
            id: "sys_tray".to_string(),
            name: "System Tray".to_string(),
            spice_type: CinnamonSpiceType::Applet,
            enabled: true,
            author: "SigmaOS Core".to_string(),
            version: "1.0.0".to_string(),
        });
        cc.spices.push(CinnamonSpiceItem {
            id: "clock_desklet".to_string(),
            name: "Digital Clock".to_string(),
            spice_type: CinnamonSpiceType::Desklet,
            enabled: true,
            author: "Cinnamon Community".to_string(),
            version: "2.1.0".to_string(),
        });

        // Seed default Mint Drivers
        cc.drivers.push(MintDriverEntry {
            id: "nvidia_drv".to_string(),
            name: "NVIDIA GeForce Proprietary Driver".to_string(),
            hardware_class: "Graphics Controller".to_string(),
            proprietary: true,
            active: true,
            driver_version: "550.54".to_string(),
        });

        // Seed default restore checkpoint
        cc.restore_points.push(TimeshiftRestorePoint {
            snapshot_id: 1,
            label: "Initial Sovereign Clean Install".to_string(),
            is_rsync: true,
            timestamp_epoch: 1700000000,
            system_hash: 0xDEADBEEF,
        });

        cc
    }

    pub fn register_plug(&mut self, plug: SwitchboardPlug) {
        self.plugs.insert(plug.plug_id.clone(), plug);
    }

    pub fn get_plugs_by_category(&self, category: ControlCenterCategory) -> Vec<&SwitchboardPlug> {
        self.plugs.values().filter(|p| p.category == category).collect()
    }

    pub fn update_setting(&mut self, plug_id: &str, key: &str, value: &str) -> bool {
        if let Some(plug) = self.plugs.get_mut(plug_id) {
            plug.set_value(key, value)
        } else {
            false
        }
    }

    /// Search for setting items across all registered Switchboard plugs
    pub fn search_settings(&self, query: &str) -> Vec<(&SwitchboardPlug, &SystemSettingItem)> {
        let q = query.to_lowercase();
        let mut results = Vec::new();
        for plug in self.plugs.values() {
            for item in &plug.settings {
                if item.key.to_lowercase().contains(&q)
                    || item.label.to_lowercase().contains(&q)
                    || item.value.to_lowercase().contains(&q)
                {
                    results.push((plug, item));
                }
            }
        }
        results
    }

    /// Export current Control Center settings into a Key-Value text configuration string
    pub fn export_configuration(&self) -> String {
        let mut out = String::new();
        out.push_str("[ControlCenter]\n");
        out.push_str(&format!("dark_mode={}\n", self.dark_mode));
        out.push_str(&format!("gtk_theme={}\n", self.cinnamon_config.gtk_theme));
        out.push_str(&format!("panel_height={}\n", self.cinnamon_config.panel_height));
        for (plug_id, plug) in &self.plugs {
            out.push_str(&format!("\n[{}]\n", plug_id));
            for item in &plug.settings {
                out.push_str(&format!("{}={}\n", item.key, item.value));
            }
        }
        out
    }

    /// Import settings from a Key-Value text configuration string
    pub fn import_configuration(&mut self, config: &str) -> usize {
        let mut updated = 0;
        let mut current_section = String::new();
        for line in config.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with('[') && trimmed.ends_with(']') {
                current_section = trimmed[1..trimmed.len() - 1].to_string();
                continue;
            }
            if let Some((k, v)) = trimmed.split_once('=') {
                let key = k.trim();
                let val = v.trim();
                if current_section != "ControlCenter" && !current_section.is_empty() {
                    if self.update_setting(&current_section, key, val) {
                        updated += 1;
                    }
                }
            }
        }
        updated
    }

    /// Toggle a Cinnamon Spice applet/desklet state
    pub fn toggle_spice(&mut self, spice_id: &str, enable: bool) -> bool {
        if let Some(spice) = self.spices.iter_mut().find(|s| s.id == spice_id) {
            spice.enabled = enable;
            true
        } else {
            false
        }
    }

    /// Toggle active hardware driver in MintDrivers
    pub fn toggle_driver(&mut self, driver_id: &str, active: bool) -> bool {
        if let Some(driver) = self.drivers.iter_mut().find(|d| d.id == driver_id) {
            driver.active = active;
            true
        } else {
            false
        }
    }

    /// Create a new Timeshift system restore checkpoint
    pub fn create_restore_checkpoint(&mut self, label: &str, is_rsync: bool, system_hash: u64) -> u32 {
        let next_id = (self.restore_points.len() as u32) + 1;
        self.restore_points.push(TimeshiftRestorePoint {
            snapshot_id: next_id,
            label: label.to_string(),
            is_rsync,
            timestamp_epoch: 1700000000 + (next_id as u64 * 3600),
            system_hash,
        });
        next_id
    }
}

impl Default for UnifiedControlCenter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_control_center_category_plug_dispatch() {
        let cc = UnifiedControlCenter::new();
        let net_plugs = cc.get_plugs_by_category(ControlCenterCategory::Network);
        assert_eq!(net_plugs.len(), 1);
        assert_eq!(net_plugs[0].name, "Network & Wireless");

        let spice_plugs = cc.get_plugs_by_category(ControlCenterCategory::Spices);
        assert_eq!(spice_plugs.len(), 1);
        assert_eq!(spice_plugs[0].name, "Cinnamon Spices");
    }

    #[test]
    fn test_system_settings_toggle() {
        let mut cc = UnifiedControlCenter::new();
        assert!(cc.update_setting("plug_display", "scaling", "150%"));

        let display_plug = cc.plugs.get("plug_display").unwrap();
        let scaling_item = display_plug.settings.iter().find(|i| i.key == "scaling").unwrap();
        assert_eq!(scaling_item.value, "150%");
    }

    #[test]
    fn test_plug_registration() {
        let mut cc = UnifiedControlCenter::new();
        let mut sound_plug = SwitchboardPlug::new(
            "plug_sound",
            "Sound & Audio",
            "PipeWire audio devices and volume",
            ControlCenterCategory::Sound,
        );
        sound_plug.add_setting("master_volume", "Master Volume", "80%", true);

        cc.register_plug(sound_plug);
        assert_eq!(cc.get_plugs_by_category(ControlCenterCategory::Sound).len(), 1);
    }

    #[test]
    fn test_cinnamon_spices_and_mint_drivers() {
        let mut cc = UnifiedControlCenter::new();
        assert_eq!(cc.spices.len(), 2);
        assert!(cc.toggle_spice("clock_desklet", false));
        assert!(!cc.spices.iter().find(|s| s.id == "clock_desklet").unwrap().enabled);

        assert_eq!(cc.drivers.len(), 1);
        assert!(cc.toggle_driver("nvidia_drv", false));
        assert!(!cc.drivers.iter().find(|d| d.id == "nvidia_drv").unwrap().active);
    }

    #[test]
    fn test_timeshift_restore_checkpointing() {
        let mut cc = UnifiedControlCenter::new();
        let id = cc.create_restore_checkpoint("Pre-Update Snapshot", true, 0xCAFEBABE);
        assert_eq!(id, 2);
        assert_eq!(cc.restore_points.len(), 2);
        assert_eq!(cc.restore_points[1].system_hash, 0xCAFEBABE);
    }

    #[test]
    fn test_search_and_config_import_export() {
        let mut cc = UnifiedControlCenter::new();
        let results = cc.search_settings("firewall");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].1.key, "firewall_active");

        let exported = cc.export_configuration();
        assert!(exported.contains("[plug_net]"));
        assert!(exported.contains("wifi_enabled=true"));

        let imported_count = cc.import_configuration("[plug_net]\nwifi_enabled=false\n");
        assert_eq!(imported_count, 1);
        let net_plug = cc.plugs.get("plug_net").unwrap();
        let item = net_plug.settings.iter().find(|i| i.key == "wifi_enabled").unwrap();
        assert_eq!(item.value, "false");
    }
}
