//! Input Device Manager
//!
//! Input device management inspired by Linux Mint's input settings and Omarchy's
//! input utilities, supporting keyboard, mouse, and touchpad configuration.

use std::collections::HashMap;

/// Input device type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputDeviceType {
    Keyboard,
    Mouse,
    Touchpad,
    Trackball,
    Tablet,
    Gamepad,
}

impl InputDeviceType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "keyboard" => Some(InputDeviceType::Keyboard),
            "mouse" => Some(InputDeviceType::Mouse),
            "touchpad" => Some(InputDeviceType::Touchpad),
            "trackball" => Some(InputDeviceType::Trackball),
            "tablet" => Some(InputDeviceType::Tablet),
            "gamepad" => Some(InputDeviceType::Gamepad),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            InputDeviceType::Keyboard => "Keyboard",
            InputDeviceType::Mouse => "Mouse",
            InputDeviceType::Touchpad => "Touchpad",
            InputDeviceType::Trackball => "Trackball",
            InputDeviceType::Tablet => "Tablet",
            InputDeviceType::Gamepad => "Gamepad",
        }
    }
}

/// Input device
#[derive(Debug, Clone)]
pub struct InputDevice {
    pub id: String,
    pub name: String,
    pub device_type: InputDeviceType,
    pub is_enabled: bool,
}

impl InputDevice {
    pub fn new(id: String, name: String, device_type: InputDeviceType) -> Self {
        Self {
            id,
            name,
            device_type,
            is_enabled: true,
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.is_enabled = enabled;
    }
}

/// Keyboard configuration
#[derive(Debug, Clone)]
pub struct KeyboardConfig {
    pub layout: String,
    pub repeat_delay: u32,
    pub repeat_rate: u32,
    pub caps_lock_behavior: String,
}

impl KeyboardConfig {
    pub fn new() -> Self {
        Self {
            layout: "us".to_string(),
            repeat_delay: 500,
            repeat_rate: 30,
            caps_lock_behavior: "default".to_string(),
        }
    }
}

impl Default for KeyboardConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Mouse configuration
#[derive(Debug, Clone)]
pub struct MouseConfig {
    pub acceleration: f32,
    pub sensitivity: u32,
    pub left_handed: bool,
    pub scroll_speed: u32,
}

impl MouseConfig {
    pub fn new() -> Self {
        Self {
            acceleration: 1.0,
            sensitivity: 5,
            left_handed: false,
            scroll_speed: 5,
        }
    }
}

impl Default for MouseConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Touchpad configuration
#[derive(Debug, Clone)]
pub struct TouchpadConfig {
    pub tap_to_click: bool,
    pub natural_scrolling: bool,
    pub two_finger_scroll: bool,
    pub edge_scrolling: bool,
    pub disable_while_typing: bool,
    pub speed: u32,
}

impl TouchpadConfig {
    pub fn new() -> Self {
        Self {
            tap_to_click: true,
            natural_scrolling: false,
            two_finger_scroll: true,
            edge_scrolling: false,
            disable_while_typing: true,
            speed: 5,
        }
    }
}

impl Default for TouchpadConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Input device manager
#[derive(Debug)]
pub struct InputDeviceManager {
    devices: HashMap<String, InputDevice>,
    keyboard_config: KeyboardConfig,
    mouse_config: MouseConfig,
    touchpad_config: TouchpadConfig,
}

impl InputDeviceManager {
    pub fn new() -> Self {
        let mut manager = Self {
            devices: HashMap::new(),
            keyboard_config: KeyboardConfig::new(),
            mouse_config: MouseConfig::new(),
            touchpad_config: TouchpadConfig::new(),
        };

        // Add default devices
        let keyboard = InputDevice::new(
            "keyboard-0".to_string(),
            "System Keyboard".to_string(),
            InputDeviceType::Keyboard,
        );
        manager.devices.insert("keyboard-0".to_string(), keyboard);

        let mouse = InputDevice::new(
            "mouse-0".to_string(),
            "System Mouse".to_string(),
            InputDeviceType::Mouse,
        );
        manager.devices.insert("mouse-0".to_string(), mouse);

        let touchpad = InputDevice::new(
            "touchpad-0".to_string(),
            "System Touchpad".to_string(),
            InputDeviceType::Touchpad,
        );
        manager.devices.insert("touchpad-0".to_string(), touchpad);

        manager
    }

    /// Add a device
    pub fn add_device(&mut self, device: InputDevice) {
        self.devices.insert(device.id.clone(), device);
    }

    /// Get a device
    pub fn get_device(&self, id: &str) -> Option<&InputDevice> {
        self.devices.get(id)
    }

    /// List all devices
    pub fn list_devices(&self) -> Vec<&InputDevice> {
        self.devices.values().collect()
    }

    /// List devices by type
    pub fn list_by_type(&self, device_type: InputDeviceType) -> Vec<&InputDevice> {
        self.devices.values()
            .filter(|d| d.device_type == device_type)
            .collect()
    }

    /// Enable a device
    pub fn enable(&mut self, id: &str) -> Result<(), String> {
        let device = self.devices.get_mut(id)
            .ok_or_else(|| format!("Device {} not found", id))?;

        device.set_enabled(true);
        Ok(())
    }

    /// Disable a device
    pub fn disable(&mut self, id: &str) -> Result<(), String> {
        let device = self.devices.get_mut(id)
            .ok_or_else(|| format!("Device {} not found", id))?;

        device.set_enabled(false);
        Ok(())
    }

    /// Get keyboard config
    pub fn get_keyboard_config(&self) -> &KeyboardConfig {
        &self.keyboard_config
    }

    /// Set keyboard config
    pub fn set_keyboard_config(&mut self, config: KeyboardConfig) {
        self.keyboard_config = config;
    }

    /// Get mouse config
    pub fn get_mouse_config(&self) -> &MouseConfig {
        &self.mouse_config
    }

    /// Set mouse config
    pub fn set_mouse_config(&mut self, config: MouseConfig) {
        self.mouse_config = config;
    }

    /// Get touchpad config
    pub fn get_touchpad_config(&self) -> &TouchpadConfig {
        &self.touchpad_config
    }

    /// Set touchpad config
    pub fn set_touchpad_config(&mut self, config: TouchpadConfig) {
        self.touchpad_config = config;
    }

    /// Get statistics
    pub fn get_statistics(&self) -> InputDeviceStatistics {
        let total_devices = self.devices.len();
        let enabled_count = self.devices.values()
            .filter(|d| d.is_enabled)
            .count();
        let keyboard_count = self.devices.values()
            .filter(|d| d.device_type == InputDeviceType::Keyboard)
            .count();
        let mouse_count = self.devices.values()
            .filter(|d| d.device_type == InputDeviceType::Mouse)
            .count();
        let touchpad_count = self.devices.values()
            .filter(|d| d.device_type == InputDeviceType::Touchpad)
            .count();

        InputDeviceStatistics {
            total_devices,
            enabled_count,
            keyboard_count,
            mouse_count,
            touchpad_count,
        }
    }
}

impl Default for InputDeviceManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Input device statistics
#[derive(Debug, Clone)]
pub struct InputDeviceStatistics {
    pub total_devices: usize,
    pub enabled_count: usize,
    pub keyboard_count: usize,
    pub mouse_count: usize,
    pub touchpad_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_device_type_from_str() {
        assert_eq!(InputDeviceType::from_str("keyboard"), Some(InputDeviceType::Keyboard));
        assert_eq!(InputDeviceType::from_str("mouse"), Some(InputDeviceType::Mouse));
    }

    #[test]
    fn test_input_device_creation() {
        let device = InputDevice::new(
            "test".to_string(),
            "Test Device".to_string(),
            InputDeviceType::Keyboard,
        );
        assert_eq!(device.name, "Test Device");
    }

    #[test]
    fn test_input_device_manager_creation() {
        let manager = InputDeviceManager::new();
        assert!(manager.get_device("keyboard-0").is_some());
    }

    #[test]
    fn test_add_device() {
        let mut manager = InputDeviceManager::new();
        let device = InputDevice::new(
            "test".to_string(),
            "Test".to_string(),
            InputDeviceType::Mouse,
        );
        manager.add_device(device);
        assert!(manager.get_device("test").is_some());
    }

    #[test]
    fn test_enable_disable() {
        let mut manager = InputDeviceManager::new();
        manager.disable("keyboard-0").ok();
        assert!(!manager.get_device("keyboard-0").unwrap().is_enabled);
        manager.enable("keyboard-0").ok();
        assert!(manager.get_device("keyboard-0").unwrap().is_enabled);
    }

    #[test]
    fn test_list_by_type() {
        let manager = InputDeviceManager::new();
        let keyboards = manager.list_by_type(InputDeviceType::Keyboard);
        assert!(keyboards.len() >= 1);
    }

    #[test]
    fn test_keyboard_config() {
        let manager = InputDeviceManager::new();
        let config = manager.get_keyboard_config();
        assert_eq!(config.layout, "us");
    }

    #[test]
    fn test_mouse_config() {
        let manager = InputDeviceManager::new();
        let config = manager.get_mouse_config();
        assert_eq!(config.acceleration, 1.0);
    }

    #[test]
    fn test_touchpad_config() {
        let manager = InputDeviceManager::new();
        let config = manager.get_touchpad_config();
        assert!(config.tap_to_click);
    }

    #[test]
    fn test_statistics() {
        let manager = InputDeviceManager::new();
        let stats = manager.get_statistics();
        assert!(stats.total_devices >= 3);
    }
}
