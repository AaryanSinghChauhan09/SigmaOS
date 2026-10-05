// SigmaOS Quick Settings Manager
// Inspired by Linux Mint's quick settings and Omarchy's control center utilities

use std::collections::HashMap;

/// Quick setting type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopQuickSettingType {
    Toggle,
    Slider,
    Action,
    Menu,
}

impl DesktopQuickSettingType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DesktopQuickSettingType::Toggle => "Toggle",
            DesktopQuickSettingType::Slider => "Slider",
            DesktopQuickSettingType::Action => "Action",
            DesktopQuickSettingType::Menu => "Menu",
        }
    }
}

/// Quick setting
#[derive(Debug, Clone)]
pub struct DesktopQuickSetting {
    pub id: String,
    pub name: String,
    pub setting_type: DesktopQuickSettingType,
    pub icon: Option<String>,
    pub is_enabled: bool,
    pub value: Option<i32>, // For sliders (0-100)
    pub description: Option<String>,
}

impl DesktopQuickSetting {
    pub fn new(id: String, name: String, setting_type: DesktopQuickSettingType) -> Self {
        DesktopQuickSetting {
            id,
            name,
            setting_type,
            icon: None,
            is_enabled: true,
            value: None,
            description: None,
        }
    }

    pub fn set_icon(&mut self, icon: String) {
        self.icon = Some(icon);
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.is_enabled = enabled;
    }

    pub fn set_value(&mut self, value: i32) {
        self.value = Some(value.clamp(0, 100));
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }
}

/// Quick Settings Manager
pub struct DesktopQuickSettingsManager {
    settings: HashMap<String, DesktopQuickSetting>,
    next_setting_id: u32,
}

impl DesktopQuickSettingsManager {
    pub fn new() -> Self {
        let mut manager = DesktopQuickSettingsManager {
            settings: HashMap::new(),
            next_setting_id: 1,
        };

        // Add default quick settings
        manager.add_default_settings();

        manager
    }

    fn add_default_settings(&mut self) {
        // WiFi toggle
        let mut wifi = DesktopQuickSetting::new(
            format!("setting_{}", self.next_setting_id),
            "WiFi".to_string(),
            DesktopQuickSettingType::Toggle,
        );
        wifi.set_icon("network-wireless".to_string());
        wifi.set_enabled(true);
        wifi.set_description("Wireless network".to_string());
        self.settings.insert(wifi.id.clone(), wifi);
        self.next_setting_id += 1;

        // Bluetooth toggle
        let mut bluetooth = DesktopQuickSetting::new(
            format!("setting_{}", self.next_setting_id),
            "Bluetooth".to_string(),
            DesktopQuickSettingType::Toggle,
        );
        bluetooth.set_icon("bluetooth".to_string());
        bluetooth.set_enabled(false);
        bluetooth.set_description("Bluetooth connectivity".to_string());
        self.settings.insert(bluetooth.id.clone(), bluetooth);
        self.next_setting_id += 1;

        // Airplane mode toggle
        let mut airplane = DesktopQuickSetting::new(
            format!("setting_{}", self.next_setting_id),
            "Airplane Mode".to_string(),
            DesktopQuickSettingType::Toggle,
        );
        airplane.set_icon("airplane-mode".to_string());
        airplane.set_enabled(false);
        airplane.set_description("Disable wireless connections".to_string());
        self.settings.insert(airplane.id.clone(), airplane);
        self.next_setting_id += 1;

        // Brightness slider
        let mut brightness = DesktopQuickSetting::new(
            format!("setting_{}", self.next_setting_id),
            "Brightness".to_string(),
            DesktopQuickSettingType::Slider,
        );
        brightness.set_icon("display-brightness".to_string());
        brightness.set_value(75);
        brightness.set_description("Screen brightness".to_string());
        self.settings.insert(brightness.id.clone(), brightness);
        self.next_setting_id += 1;

        // Volume slider
        let mut volume = DesktopQuickSetting::new(
            format!("setting_{}", self.next_setting_id),
            "Volume".to_string(),
            DesktopQuickSettingType::Slider,
        );
        volume.set_icon("audio-volume-high".to_string());
        volume.set_value(50);
        volume.set_description("Output volume".to_string());
        self.settings.insert(volume.id.clone(), volume);
        self.next_setting_id += 1;

        // Do Not Disturb toggle
        let mut dnd = DesktopQuickSetting::new(
            format!("setting_{}", self.next_setting_id),
            "Do Not Disturb".to_string(),
            DesktopQuickSettingType::Toggle,
        );
        dnd.set_icon("notifications-disabled".to_string());
        dnd.set_enabled(false);
        dnd.set_description("Suppress notifications".to_string());
        self.settings.insert(dnd.id.clone(), dnd);
        self.next_setting_id += 1;

        // Night Light toggle
        let mut night_light = DesktopQuickSetting::new(
            format!("setting_{}", self.next_setting_id),
            "Night Light".to_string(),
            DesktopQuickSettingType::Toggle,
        );
        night_light.set_icon("night-light".to_string());
        night_light.set_enabled(false);
        night_light.set_description("Reduce blue light".to_string());
        self.settings.insert(night_light.id.clone(), night_light);
        self.next_setting_id += 1;

        // Settings action
        let mut settings = DesktopQuickSetting::new(
            format!("setting_{}", self.next_setting_id),
            "Settings".to_string(),
            DesktopQuickSettingType::Action,
        );
        settings.set_icon("settings".to_string());
        settings.set_description("System settings".to_string());
        self.settings.insert(settings.id.clone(), settings);
        self.next_setting_id += 1;
    }

    pub fn add_setting(&mut self, name: String, setting_type: DesktopQuickSettingType) -> String {
        let id = format!("setting_{}", self.next_setting_id);
        let setting = DesktopQuickSetting::new(id.clone(), name, setting_type);
        self.settings.insert(id.clone(), setting);
        self.next_setting_id += 1;
        id
    }

    pub fn remove_setting(&mut self, id: &str) -> bool {
        self.settings.remove(id).is_some()
    }

    pub fn get_setting(&self, id: &str) -> Option<&DesktopQuickSetting> {
        self.settings.get(id)
    }

    pub fn get_settings(&self) -> Vec<&DesktopQuickSetting> {
        self.settings.values().collect()
    }

    pub fn get_settings_by_type(&self, setting_type: DesktopQuickSettingType) -> Vec<&DesktopQuickSetting> {
        self.settings
            .values()
            .filter(|s| s.setting_type == setting_type)
            .collect()
    }

    pub fn get_enabled_settings(&self) -> Vec<&DesktopQuickSetting> {
        self.settings
            .values()
            .filter(|s| s.is_enabled)
            .collect()
    }

    pub fn set_setting_icon(&mut self, id: &str, icon: String) -> bool {
        if let Some(setting) = self.settings.get_mut(id) {
            setting.set_icon(icon);
            true
        } else {
            false
        }
    }

    pub fn set_setting_enabled(&mut self, id: &str, enabled: bool) -> bool {
        if let Some(setting) = self.settings.get_mut(id) {
            setting.set_enabled(enabled);
            true
        } else {
            false
        }
    }

    pub fn toggle_setting(&mut self, id: &str) -> bool {
        if let Some(setting) = self.settings.get_mut(id) {
            if setting.setting_type == DesktopQuickSettingType::Toggle {
                setting.set_enabled(!setting.is_enabled);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn set_setting_value(&mut self, id: &str, value: i32) -> bool {
        if let Some(setting) = self.settings.get_mut(id) {
            if setting.setting_type == DesktopQuickSettingType::Slider {
                setting.set_value(value);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn set_setting_description(&mut self, id: &str, description: String) -> bool {
        if let Some(setting) = self.settings.get_mut(id) {
            setting.set_description(description);
            true
        } else {
            false
        }
    }

    pub fn get_statistics(&self) -> DesktopQuickSettingsStatistics {
        DesktopQuickSettingsStatistics {
            total_settings: self.settings.len(),
            toggle_settings: self.get_settings_by_type(DesktopQuickSettingType::Toggle).len(),
            slider_settings: self.get_settings_by_type(DesktopQuickSettingType::Slider).len(),
            enabled_settings: self.get_enabled_settings().len(),
        }
    }
}

impl Default for DesktopQuickSettingsManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Quick Settings statistics
#[derive(Debug, Clone, Copy)]
pub struct DesktopQuickSettingsStatistics {
    pub total_settings: usize,
    pub toggle_settings: usize,
    pub slider_settings: usize,
    pub enabled_settings: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_settings_manager_initialization() {
        let manager = DesktopQuickSettingsManager::new();
        assert_eq!(manager.get_settings().len(), 8);
    }

    #[test]
    fn test_add_setting() {
        let mut manager = DesktopQuickSettingsManager::new();
        let id = manager.add_setting("Location".to_string(), DesktopQuickSettingType::Toggle);
        assert!(manager.get_setting(&id).is_some());
        assert_eq!(manager.get_settings().len(), 9);
    }

    #[test]
    fn test_remove_setting() {
        let mut manager = DesktopQuickSettingsManager::new();
        let id = manager.add_setting("Location".to_string(), DesktopQuickSettingType::Toggle);
        assert!(manager.remove_setting(&id));
        assert!(!manager.get_setting(&id).is_some());
        assert_eq!(manager.get_settings().len(), 8);
    }

    #[test]
    fn test_set_setting_enabled() {
        let mut manager = DesktopQuickSettingsManager::new();
        let id = manager.add_setting("Location".to_string(), DesktopQuickSettingType::Toggle);
        assert!(manager.set_setting_enabled(&id, false));
        assert!(!manager.get_setting(&id).unwrap().is_enabled);
    }

    #[test]
    fn test_toggle_setting() {
        let mut manager = DesktopQuickSettingsManager::new();
        let id = manager.add_setting("Location".to_string(), DesktopQuickSettingType::Toggle);
        let initial_state = manager.get_setting(&id).unwrap().is_enabled;
        assert!(manager.toggle_setting(&id));
        assert_ne!(manager.get_setting(&id).unwrap().is_enabled, initial_state);
    }

    #[test]
    fn test_set_setting_value() {
        let mut manager = DesktopQuickSettingsManager::new();
        let id = manager.add_setting("Custom Slider".to_string(), DesktopQuickSettingType::Slider);
        assert!(manager.set_setting_value(&id, 80));
        assert_eq!(manager.get_setting(&id).unwrap().value, Some(80));
    }

    #[test]
    fn test_get_settings_by_type() {
        let manager = DesktopQuickSettingsManager::new();
        let toggles = manager.get_settings_by_type(DesktopQuickSettingType::Toggle);
        assert!(toggles.len() > 0);

        let sliders = manager.get_settings_by_type(DesktopQuickSettingType::Slider);
        assert!(sliders.len() > 0);
    }

    #[test]
    fn test_statistics() {
        let manager = DesktopQuickSettingsManager::new();
        let stats = manager.get_statistics();
        assert_eq!(stats.total_settings, 8);
        assert!(stats.toggle_settings > 0);
        assert!(stats.slider_settings > 0);
    }
}
