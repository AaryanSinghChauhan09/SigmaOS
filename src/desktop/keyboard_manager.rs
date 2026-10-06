// Desktop Keyboard Manager
// Linux Mint & Omarchy inspiration for comprehensive keyboard management

use std::collections::HashMap;

/// Keyboard Layout
#[derive(Debug, Clone)]
pub struct KeyboardLayout {
    pub id: String,
    pub name: String,
    pub language: String,
    pub variant: Option<String>,
}

impl KeyboardLayout {
    pub fn new(id: String, name: String, language: String) -> Self {
        Self {
            id,
            name,
            language,
            variant: None,
        }
    }

    pub fn with_variant(mut self, variant: String) -> Self {
        self.variant = Some(variant);
        self
    }
}

/// Repeat Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepeatMode {
    Off,
    Delayed,
    Immediate,
}

impl RepeatMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            RepeatMode::Off => "off",
            RepeatMode::Delayed => "delayed",
            RepeatMode::Immediate => "immediate",
        }
    }
}

/// Keyboard Device
#[derive(Debug, Clone)]
pub struct KeyboardDevice {
    pub id: String,
    pub name: String,
    pub vendor: String,
    pub product: String,
    pub num_keys: u32,
}

impl KeyboardDevice {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            vendor: String::new(),
            product: String::new(),
            num_keys: 0,
        }
    }

    pub fn with_vendor(mut self, vendor: String) -> Self {
        self.vendor = vendor;
        self
    }

    pub fn with_product(mut self, product: String) -> Self {
        self.product = product;
        self
    }

    pub fn with_num_keys(mut self, num_keys: u32) -> Self {
        self.num_keys = num_keys;
        self
    }
}

/// Keyboard Configuration
#[derive(Debug, Clone)]
pub struct KeyboardConfiguration {
    pub device_id: String,
    pub layout: String,
    pub variant: Option<String>,
    pub repeat_mode: RepeatMode,
    pub repeat_delay: u32,    // milliseconds
    pub repeat_rate: u32,     // repeats per second
    pub numlock_on: bool,
    pub capslock_warning: bool,
}

impl KeyboardConfiguration {
    pub fn new(device_id: String) -> Self {
        Self {
            device_id,
            layout: "us".to_string(),
            variant: None,
            repeat_mode: RepeatMode::Delayed,
            repeat_delay: 500,
            repeat_rate: 30,
            numlock_on: false,
            capslock_warning: true,
        }
    }

    pub fn set_layout(&mut self, layout: String) {
        self.layout = layout;
    }

    pub fn set_variant(&mut self, variant: Option<String>) {
        self.variant = variant;
    }

    pub fn set_repeat_mode(&mut self, mode: RepeatMode) {
        self.repeat_mode = mode;
    }

    pub fn set_repeat_delay(&mut self, delay: u32) {
        self.repeat_delay = delay;
    }

    pub fn set_repeat_rate(&mut self, rate: u32) {
        self.repeat_rate = rate;
    }
}

/// Desktop Keyboard Manager
pub struct DesktopKeyboardManager {
    devices: HashMap<String, KeyboardDevice>,
    configurations: HashMap<String, KeyboardConfiguration>,
    layouts: HashMap<String, KeyboardLayout>,
    current_layout: Option<String>,
    counter: u32,
}

impl DesktopKeyboardManager {
    pub fn new() -> Self {
        let mut manager = Self {
            devices: HashMap::new(),
            configurations: HashMap::new(),
            layouts: HashMap::new(),
            current_layout: None,
            counter: 0,
        };

        // Add default layouts
        manager.add_default_layouts();

        // Add default keyboard
        manager.add_default_keyboard();

        manager
    }

    fn add_default_layouts(&mut self) {
        let us = KeyboardLayout::new(
            "layout_0".to_string(),
            "English (US)".to_string(),
            "en".to_string(),
        );

        let gb = KeyboardLayout::new(
            "layout_1".to_string(),
            "English (UK)".to_string(),
            "en".to_string(),
        );

        let de = KeyboardLayout::new(
            "layout_2".to_string(),
            "German".to_string(),
            "de".to_string(),
        );

        let fr = KeyboardLayout::new(
            "layout_3".to_string(),
            "French".to_string(),
            "fr".to_string(),
        );

        let es = KeyboardLayout::new(
            "layout_4".to_string(),
            "Spanish".to_string(),
            "es".to_string(),
        );

        let jp = KeyboardLayout::new(
            "layout_5".to_string(),
            "Japanese".to_string(),
            "ja".to_string(),
        )
        .with_variant("jp106".to_string());

        self.layouts.insert(us.id.clone(), us);
        self.layouts.insert(gb.id.clone(), gb);
        self.layouts.insert(de.id.clone(), de);
        self.layouts.insert(fr.id.clone(), fr);
        self.layouts.insert(es.id.clone(), es);
        self.layouts.insert(jp.id.clone(), jp);

        self.current_layout = Some("layout_0".to_string());
    }

    fn add_default_keyboard(&mut self) {
        let device = KeyboardDevice::new(
            "keyboard_0".to_string(),
            "AT Translated Set 2 keyboard".to_string(),
        )
        .with_vendor("Generic".to_string())
        .with_product("Keyboard".to_string())
        .with_num_keys(104);

        let device_id = device.id.clone();
        self.devices.insert(device_id.clone(), device);

        let mut config = KeyboardConfiguration::new(device_id.clone());
        config.layout = "us".to_string();
        self.configurations.insert(device_id, config);
    }

    pub fn add_device(&mut self, device: KeyboardDevice) -> String {
        let id = format!("keyboard_{}", self.counter);
        self.counter += 1;

        let device = KeyboardDevice {
            id: id.clone(),
            ..device
        };

        let device_id = device.id.clone();
        self.devices.insert(device_id.clone(), device.clone());

        let config = KeyboardConfiguration::new(device_id.clone());
        self.configurations.insert(device_id, config);

        id
    }

    pub fn remove_device(&mut self, id: &str) -> bool {
        self.configurations.remove(id);
        self.devices.remove(id).is_some()
    }

    pub fn get_device(&self, id: &str) -> Option<&KeyboardDevice> {
        self.devices.get(id)
    }

    pub fn get_devices(&self) -> Vec<&KeyboardDevice> {
        self.devices.values().collect()
    }

    pub fn get_configuration(&self, id: &str) -> Option<&KeyboardConfiguration> {
        self.configurations.get(id)
    }

    pub fn get_configurations(&self) -> Vec<&KeyboardConfiguration> {
        self.configurations.values().collect()
    }

    pub fn update_configuration(&mut self, id: &str, config: KeyboardConfiguration) -> bool {
        if self.devices.contains_key(id) {
            self.configurations.insert(id.to_string(), config);
            true
        } else {
            false
        }
    }

    pub fn add_layout(&mut self, layout: KeyboardLayout) -> String {
        let id = format!("layout_{}", self.counter);
        self.counter += 1;

        let layout = KeyboardLayout {
            id: id.clone(),
            ..layout
        };

        self.layouts.insert(id.clone(), layout);
        id
    }

    pub fn remove_layout(&mut self, id: &str) -> bool {
        // Don't remove if it's the current layout
        if Some(id.to_string()) == self.current_layout {
            return false;
        }

        self.layouts.remove(id).is_some()
    }

    pub fn get_layout(&self, id: &str) -> Option<&KeyboardLayout> {
        self.layouts.get(id)
    }

    pub fn get_layouts(&self) -> Vec<&KeyboardLayout> {
        self.layouts.values().collect()
    }

    pub fn set_current_layout(&mut self, id: &str) -> bool {
        if self.layouts.contains_key(id) {
            self.current_layout = Some(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn get_current_layout(&self) -> Option<&KeyboardLayout> {
        self.current_layout
            .as_ref()
            .and_then(|id| self.layouts.get(id))
    }

    pub fn set_layout_for_device(&mut self, device_id: &str, layout: String) -> bool {
        if let Some(config) = self.configurations.get_mut(device_id) {
            config.set_layout(layout);
            true
        } else {
            false
        }
    }

    pub fn set_repeat_mode(&mut self, id: &str, mode: RepeatMode) -> bool {
        if let Some(config) = self.configurations.get_mut(id) {
            config.set_repeat_mode(mode);
            true
        } else {
            false
        }
    }

    pub fn set_repeat_delay(&mut self, id: &str, delay: u32) -> bool {
        if let Some(config) = self.configurations.get_mut(id) {
            config.set_repeat_delay(delay);
            true
        } else {
            false
        }
    }

    pub fn set_repeat_rate(&mut self, id: &str, rate: u32) -> bool {
        if let Some(config) = self.configurations.get_mut(id) {
            config.set_repeat_rate(rate);
            true
        } else {
            false
        }
    }

    pub fn set_numlock(&mut self, id: &str, on: bool) -> bool {
        if let Some(config) = self.configurations.get_mut(id) {
            config.numlock_on = on;
            true
        } else {
            false
        }
    }

    pub fn set_capslock_warning(&mut self, id: &str, warning: bool) -> bool {
        if let Some(config) = self.configurations.get_mut(id) {
            config.capslock_warning = warning;
            true
        } else {
            false
        }
    }

    pub fn search_layouts(&self, query: &str) -> Vec<&KeyboardLayout> {
        self.layouts
            .values()
            .filter(|l| {
                l.name.to_lowercase().contains(&query.to_lowercase())
                    || l.language.to_lowercase().contains(&query.to_lowercase())
            })
            .collect()
    }

    pub fn get_statistics(&self) -> KeyboardManagerStatistics {
        KeyboardManagerStatistics {
            total_devices: self.devices.len(),
            total_layouts: self.layouts.len(),
            current_layout_set: self.current_layout.is_some(),
        }
    }
}

impl Default for DesktopKeyboardManager {
    fn default() -> Self {
        Self::new()
    }
}

/// KeyboardManagerStatistics
#[derive(Debug, Clone, Copy)]
pub struct KeyboardManagerStatistics {
    pub total_devices: usize,
    pub total_layouts: usize,
    pub current_layout_set: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_manager_state() {
        let manager = DesktopKeyboardManager::new();
        let stats = manager.get_statistics();

        assert_eq!(stats.total_devices, 1);
        assert!(stats.total_layouts >= 6);
        assert!(stats.current_layout_set);
    }

    #[test]
    fn test_add_device() {
        let mut manager = DesktopKeyboardManager::new();
        let initial_count = manager.get_devices().len();

        let device = KeyboardDevice::new(
            "custom".to_string(),
            "Custom Keyboard".to_string(),
        );

        let id = manager.add_device(device);
        assert!(manager.get_device(&id).is_some());
        assert_eq!(manager.get_devices().len(), initial_count + 1);
    }

    #[test]
    fn test_remove_device() {
        let mut manager = DesktopKeyboardManager::new();

        let device = KeyboardDevice::new(
            "custom".to_string(),
            "Custom Keyboard".to_string(),
        );

        let id = manager.add_device(device);
        assert!(manager.remove_device(&id));
        assert!(manager.get_device(&id).is_none());
    }

    #[test]
    fn test_add_layout() {
        let mut manager = DesktopKeyboardManager::new();
        let initial_count = manager.get_layouts().len();

        let layout = KeyboardLayout::new(
            "custom".to_string(),
            "Custom Layout".to_string(),
            "custom".to_string(),
        );

        let id = manager.add_layout(layout);
        assert!(manager.get_layout(&id).is_some());
        assert_eq!(manager.get_layouts().len(), initial_count + 1);
    }

    #[test]
    fn test_remove_layout() {
        let mut manager = DesktopKeyboardManager::new();

        let layout = KeyboardLayout::new(
            "custom".to_string(),
            "Custom Layout".to_string(),
            "custom".to_string(),
        );

        let id = manager.add_layout(layout);
        assert!(manager.remove_layout(&id));
        assert!(manager.get_layout(&id).is_none());
    }

    #[test]
    fn test_remove_current_layout() {
        let mut manager = DesktopKeyboardManager::new();

        // Try to remove current layout (should fail)
        let current = manager.get_current_layout().unwrap();
        let current_id = current.id.clone();
        let result = manager.remove_layout(&current_id);
        assert!(!result);
    }

    #[test]
    fn test_set_current_layout() {
        let mut manager = DesktopKeyboardManager::new();
        let layouts = manager.get_layouts();

        if layouts.len() > 1 {
            let new_layout_id = layouts[1].id.clone();
            assert!(manager.set_current_layout(&new_layout_id));
            assert_eq!(
                manager.get_current_layout().unwrap().id,
                new_layout_id
            );
        }
    }

    #[test]
    fn test_set_layout_for_device() {
        let mut manager = DesktopKeyboardManager::new();
        let device_id = "keyboard_0";

        assert!(manager.set_layout_for_device(device_id, "gb".to_string()));
        let config = manager.get_configuration(device_id).unwrap();
        assert_eq!(config.layout, "gb");
    }

    #[test]
    fn test_set_repeat_mode() {
        let mut manager = DesktopKeyboardManager::new();
        let device_id = "keyboard_0";

        assert!(manager.set_repeat_mode(device_id, RepeatMode::Immediate));
        let config = manager.get_configuration(device_id).unwrap();
        assert_eq!(config.repeat_mode, RepeatMode::Immediate);
    }

    #[test]
    fn test_set_repeat_delay() {
        let mut manager = DesktopKeyboardManager::new();
        let device_id = "keyboard_0";

        assert!(manager.set_repeat_delay(device_id, 300));
        let config = manager.get_configuration(device_id).unwrap();
        assert_eq!(config.repeat_delay, 300);
    }

    #[test]
    fn test_set_repeat_rate() {
        let mut manager = DesktopKeyboardManager::new();
        let device_id = "keyboard_0";

        assert!(manager.set_repeat_rate(device_id, 50));
        let config = manager.get_configuration(device_id).unwrap();
        assert_eq!(config.repeat_rate, 50);
    }

    #[test]
    fn test_set_numlock() {
        let mut manager = DesktopKeyboardManager::new();
        let device_id = "keyboard_0";

        assert!(manager.set_numlock(device_id, true));
        let config = manager.get_configuration(device_id).unwrap();
        assert!(config.numlock_on);
    }

    #[test]
    fn test_set_capslock_warning() {
        let mut manager = DesktopKeyboardManager::new();
        let device_id = "keyboard_0";

        assert!(manager.set_capslock_warning(device_id, false));
        let config = manager.get_configuration(device_id).unwrap();
        assert!(!config.capslock_warning);
    }

    #[test]
    fn test_search_layouts() {
        let manager = DesktopKeyboardManager::new();
        let results = manager.search_layouts("English");

        assert!(!results.is_empty());
        for layout in results {
            assert!(layout.name.to_lowercase().contains("english")
                || layout.language.to_lowercase().contains("en"));
        }
    }

    #[test]
    fn test_layout_variant() {
        let mut manager = DesktopKeyboardManager::new();

        let layout = KeyboardLayout::new(
            "custom".to_string(),
            "Custom".to_string(),
            "custom".to_string(),
        )
        .with_variant("dvorak".to_string());

        let id = manager.add_layout(layout);
        let retrieved = manager.get_layout(&id).unwrap();
        assert_eq!(retrieved.variant, Some("dvorak".to_string()));
    }
}
