// SigmaOS Unified Control Center
// Inspired by elementaryOS Switchboard, openSUSE YaST2 Control Center, and GNOME Settings
// - Modular Switchboard plug architecture for dynamic setting category registration
// - Comprehensive system settings management (Network, Display, Sound, Power, Users, Security, Storage, Printers, System)
// - Hardware profiles, display resolution scaling, power governor toggles, and security policy management

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::vec;

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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemSettingItem {
    pub key: String,
    pub label: String,
    pub value: String,
    pub is_editable: bool,
}

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

/// Master Unified Control Center Engine
#[derive(Debug, Clone)]
pub struct UnifiedControlCenter {
    pub plugs: BTreeMap<String, SwitchboardPlug>,
    pub dark_mode: bool,
    pub active_category: ControlCenterCategory,
}

impl UnifiedControlCenter {
    pub fn new() -> Self {
        let mut cc = Self {
            plugs: BTreeMap::new(),
            dark_mode: true,
            active_category: ControlCenterCategory::Network,
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
}
