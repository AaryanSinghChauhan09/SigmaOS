// Desktop Touchpad Manager
// Linux Mint & Omarchy inspiration for comprehensive touchpad management

use std::collections::HashMap;

/// Tap-to-Click Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TapToClickMode {
    Disabled,
    Enabled,
}

impl TapToClickMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            TapToClickMode::Disabled => "disabled",
            TapToClickMode::Enabled => "enabled",
        }
    }
}

/// Natural Scrolling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NaturalScrolling {
    Disabled,
    Enabled,
}

impl NaturalScrolling {
    pub fn as_str(&self) -> &'static str {
        match self {
            NaturalScrolling::Disabled => "disabled",
            NaturalScrolling::Enabled => "enabled",
        }
    }
}

/// Two-Finger Scrolling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TwoFingerScrolling {
    Disabled,
    Enabled,
}

impl TwoFingerScrolling {
    pub fn as_str(&self) -> &'static str {
        match self {
            TwoFingerScrolling::Disabled => "disabled",
            TwoFingerScrolling::Enabled => "enabled",
        }
    }
}

/// Edge Scrolling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeScrolling {
    Disabled,
    Enabled,
}

impl EdgeScrolling {
    pub fn as_str(&self) -> &'static str {
        match self {
            EdgeScrolling::Disabled => "disabled",
            EdgeScrolling::Enabled => "enabled",
        }
    }
}

/// Palm Detection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PalmDetection {
    Disabled,
    Enabled,
}

impl PalmDetection {
    pub fn as_str(&self) -> &'static str {
        match self {
            PalmDetection::Disabled => "disabled",
            PalmDetection::Enabled => "enabled",
        }
    }
}

/// Touchpad Device
#[derive(Debug, Clone)]
pub struct TouchpadDevice {
    pub id: String,
    pub name: String,
    pub vendor: String,
    pub product: String,
    pub width: u32,    // mm
    pub height: u32,   // mm
    pub num_fingers: u32,
}

impl TouchpadDevice {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            vendor: String::new(),
            product: String::new(),
            width: 0,
            height: 0,
            num_fingers: 0,
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

    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_num_fingers(mut self, num_fingers: u32) -> Self {
        self.num_fingers = num_fingers;
        self
    }
}

/// Touchpad Configuration
#[derive(Debug, Clone)]
pub struct TouchpadConfiguration {
    pub device_id: String,
    pub tap_to_click: TapToClickMode,
    pub natural_scrolling: NaturalScrolling,
    pub two_finger_scrolling: TwoFingerScrolling,
    pub edge_scrolling: EdgeScrolling,
    pub palm_detection: PalmDetection,
    pub sensitivity: u32,       // 0-100
    pub acceleration: f32,     // multiplier
    pub speed: u32,            // 0-100
    pub disable_while_typing: bool,
    pub tap_and_drag: bool,
}

impl TouchpadConfiguration {
    pub fn new(device_id: String) -> Self {
        Self {
            device_id,
            tap_to_click: TapToClickMode::Enabled,
            natural_scrolling: NaturalScrolling::Disabled,
            two_finger_scrolling: TwoFingerScrolling::Enabled,
            edge_scrolling: EdgeScrolling::Disabled,
            palm_detection: PalmDetection::Enabled,
            sensitivity: 50,
            acceleration: 1.0,
            speed: 50,
            disable_while_typing: true,
            tap_and_drag: false,
        }
    }

    pub fn set_tap_to_click(&mut self, mode: TapToClickMode) {
        self.tap_to_click = mode;
    }

    pub fn set_natural_scrolling(&mut self, mode: NaturalScrolling) {
        self.natural_scrolling = mode;
    }

    pub fn set_two_finger_scrolling(&mut self, mode: TwoFingerScrolling) {
        self.two_finger_scrolling = mode;
    }

    pub fn set_edge_scrolling(&mut self, mode: EdgeScrolling) {
        self.edge_scrolling = mode;
    }

    pub fn set_palm_detection(&mut self, mode: PalmDetection) {
        self.palm_detection = mode;
    }

    pub fn set_sensitivity(&mut self, sensitivity: u32) {
        self.sensitivity = sensitivity.min(100);
    }

    pub fn set_acceleration(&mut self, acceleration: f32) {
        self.acceleration = acceleration;
    }

    pub fn set_speed(&mut self, speed: u32) {
        self.speed = speed.min(100);
    }
}

/// Desktop Touchpad Manager
pub struct DesktopTouchpadManager {
    devices: HashMap<String, TouchpadDevice>,
    configurations: HashMap<String, TouchpadConfiguration>,
    counter: u32,
}

impl DesktopTouchpadManager {
    pub fn new() -> Self {
        let mut manager = Self {
            devices: HashMap::new(),
            configurations: HashMap::new(),
            counter: 1000,
        };

        // Add default touchpad
        manager.add_default_touchpad();

        manager
    }

    fn add_default_touchpad(&mut self) {
        let device = TouchpadDevice::new(
            "touchpad_0".to_string(),
            "SynPS/2 Synaptics TouchPad".to_string(),
        )
        .with_vendor("Synaptics".to_string())
        .with_product("TouchPad".to_string())
        .with_size(100, 60)
        .with_num_fingers(5);

        let device_id = device.id.clone();
        self.devices.insert(device_id.clone(), device);

        let config = TouchpadConfiguration::new(device_id.clone());
        self.configurations.insert(device_id, config);
    }

    pub fn add_device(&mut self, device: TouchpadDevice) -> String {
        let id = format!("touchpad_{}", self.counter);
        self.counter += 1;

        let device = TouchpadDevice {
            id: id.clone(),
            ..device
        };

        let device_id = device.id.clone();
        self.devices.insert(device_id.clone(), device.clone());

        let config = TouchpadConfiguration::new(device_id.clone());
        self.configurations.insert(device_id, config);

        id
    }

    pub fn remove_device(&mut self, id: &str) -> bool {
        self.configurations.remove(id);
        self.devices.remove(id).is_some()
    }

    pub fn get_device(&self, id: &str) -> Option<&TouchpadDevice> {
        self.devices.get(id)
    }

    pub fn get_devices(&self) -> Vec<&TouchpadDevice> {
        self.devices.values().collect()
    }

    pub fn get_configuration(&self, id: &str) -> Option<&TouchpadConfiguration> {
        self.configurations.get(id)
    }

    pub fn get_configurations(&self) -> Vec<&TouchpadConfiguration> {
        self.configurations.values().collect()
    }

    pub fn update_configuration(&mut self, id: &str, config: TouchpadConfiguration) -> bool {
        if self.devices.contains_key(id) {
            self.configurations.insert(id.to_string(), config);
            true
        } else {
            false
        }
    }

    pub fn set_tap_to_click(&mut self, id: &str, mode: TapToClickMode) -> bool {
        if let Some(config) = self.configurations.get_mut(id) {
            config.set_tap_to_click(mode);
            true
        } else {
            false
        }
    }

    pub fn set_natural_scrolling(&mut self, id: &str, mode: NaturalScrolling) -> bool {
        if let Some(config) = self.configurations.get_mut(id) {
            config.set_natural_scrolling(mode);
            true
        } else {
            false
        }
    }

    pub fn set_two_finger_scrolling(&mut self, id: &str, mode: TwoFingerScrolling) -> bool {
        if let Some(config) = self.configurations.get_mut(id) {
            config.set_two_finger_scrolling(mode);
            true
        } else {
            false
        }
    }

    pub fn set_edge_scrolling(&mut self, id: &str, mode: EdgeScrolling) -> bool {
        if let Some(config) = self.configurations.get_mut(id) {
            config.set_edge_scrolling(mode);
            true
        } else {
            false
        }
    }

    pub fn set_palm_detection(&mut self, id: &str, mode: PalmDetection) -> bool {
        if let Some(config) = self.configurations.get_mut(id) {
            config.set_palm_detection(mode);
            true
        } else {
            false
        }
    }

    pub fn set_sensitivity(&mut self, id: &str, sensitivity: u32) -> bool {
        if let Some(config) = self.configurations.get_mut(id) {
            config.set_sensitivity(sensitivity);
            true
        } else {
            false
        }
    }

    pub fn set_speed(&mut self, id: &str, speed: u32) -> bool {
        if let Some(config) = self.configurations.get_mut(id) {
            config.set_speed(speed);
            true
        } else {
            false
        }
    }

    pub fn disable_while_typing(&mut self, id: &str, disable: bool) -> bool {
        if let Some(config) = self.configurations.get_mut(id) {
            config.disable_while_typing = disable;
            true
        } else {
            false
        }
    }

    pub fn tap_and_drag(&mut self, id: &str, enable: bool) -> bool {
        if let Some(config) = self.configurations.get_mut(id) {
            config.tap_and_drag = enable;
            true
        } else {
            false
        }
    }

    pub fn get_statistics(&self) -> TouchpadManagerStatistics {
        TouchpadManagerStatistics {
            total_devices: self.devices.len(),
            total_configurations: self.configurations.len(),
            tap_to_click_enabled: self.configurations
                .values()
                .filter(|c| c.tap_to_click == TapToClickMode::Enabled)
                .count(),
            natural_scrolling_enabled: self.configurations
                .values()
                .filter(|c| c.natural_scrolling == NaturalScrolling::Enabled)
                .count(),
        }
    }
}

impl Default for DesktopTouchpadManager {
    fn default() -> Self {
        Self::new()
    }
}

/// TouchpadManagerStatistics
#[derive(Debug, Clone, Copy)]
pub struct TouchpadManagerStatistics {
    pub total_devices: usize,
    pub total_configurations: usize,
    pub tap_to_click_enabled: usize,
    pub natural_scrolling_enabled: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_manager_state() {
        let manager = DesktopTouchpadManager::new();
        let stats = manager.get_statistics();

        assert_eq!(stats.total_devices, 1);
        assert_eq!(stats.total_configurations, 1);
        assert!(stats.tap_to_click_enabled >= 1);
    }

    #[test]
    fn test_add_device() {
        let mut manager = DesktopTouchpadManager::new();
        let initial_count = manager.get_devices().len();

        let device = TouchpadDevice::new(
            "custom".to_string(),
            "Custom Touchpad".to_string(),
        );

        let id = manager.add_device(device);
        assert!(manager.get_device(&id).is_some());
        assert_eq!(manager.get_devices().len(), initial_count + 1);
    }

    #[test]
    fn test_remove_device() {
        let mut manager = DesktopTouchpadManager::new();

        let device = TouchpadDevice::new(
            "custom".to_string(),
            "Custom Touchpad".to_string(),
        );

        let id = manager.add_device(device);
        assert!(manager.remove_device(&id));
        assert!(manager.get_device(&id).is_none());
    }

    #[test]
    fn test_set_tap_to_click() {
        let mut manager = DesktopTouchpadManager::new();
        let device_id = "touchpad_0";

        assert!(manager.set_tap_to_click(device_id, TapToClickMode::Disabled));
        let config = manager.get_configuration(device_id).unwrap();
        assert_eq!(config.tap_to_click, TapToClickMode::Disabled);
    }

    #[test]
    fn test_set_natural_scrolling() {
        let mut manager = DesktopTouchpadManager::new();
        let device_id = "touchpad_0";

        assert!(manager.set_natural_scrolling(device_id, NaturalScrolling::Enabled));
        let config = manager.get_configuration(device_id).unwrap();
        assert_eq!(config.natural_scrolling, NaturalScrolling::Enabled);
    }

    #[test]
    fn test_set_two_finger_scrolling() {
        let mut manager = DesktopTouchpadManager::new();
        let device_id = "touchpad_0";

        assert!(manager.set_two_finger_scrolling(device_id, TwoFingerScrolling::Disabled));
        let config = manager.get_configuration(device_id).unwrap();
        assert_eq!(config.two_finger_scrolling, TwoFingerScrolling::Disabled);
    }

    #[test]
    fn test_set_edge_scrolling() {
        let mut manager = DesktopTouchpadManager::new();
        let device_id = "touchpad_0";

        assert!(manager.set_edge_scrolling(device_id, EdgeScrolling::Enabled));
        let config = manager.get_configuration(device_id).unwrap();
        assert_eq!(config.edge_scrolling, EdgeScrolling::Enabled);
    }

    #[test]
    fn test_set_palm_detection() {
        let mut manager = DesktopTouchpadManager::new();
        let device_id = "touchpad_0";

        assert!(manager.set_palm_detection(device_id, PalmDetection::Disabled));
        let config = manager.get_configuration(device_id).unwrap();
        assert_eq!(config.palm_detection, PalmDetection::Disabled);
    }

    #[test]
    fn test_set_sensitivity() {
        let mut manager = DesktopTouchpadManager::new();
        let device_id = "touchpad_0";

        assert!(manager.set_sensitivity(device_id, 75));
        let config = manager.get_configuration(device_id).unwrap();
        assert_eq!(config.sensitivity, 75);
    }

    #[test]
    fn test_set_speed() {
        let mut manager = DesktopTouchpadManager::new();
        let device_id = "touchpad_0";

        assert!(manager.set_speed(device_id, 80));
        let config = manager.get_configuration(device_id).unwrap();
        assert_eq!(config.speed, 80);
    }

    #[test]
    fn test_disable_while_typing() {
        let mut manager = DesktopTouchpadManager::new();
        let device_id = "touchpad_0";

        assert!(manager.disable_while_typing(device_id, false));
        let config = manager.get_configuration(device_id).unwrap();
        assert!(!config.disable_while_typing);
    }

    #[test]
    fn test_tap_and_drag() {
        let mut manager = DesktopTouchpadManager::new();
        let device_id = "touchpad_0";

        assert!(manager.tap_and_drag(device_id, true));
        let config = manager.get_configuration(device_id).unwrap();
        assert!(config.tap_and_drag);
    }

    #[test]
    fn test_update_configuration() {
        let mut manager = DesktopTouchpadManager::new();
        let device_id = "touchpad_0";

        let mut config = TouchpadConfiguration::new(device_id.to_string());
        config.set_sensitivity(90);
        config.set_speed(85);

        assert!(manager.update_configuration(device_id, config));
        let new_config = manager.get_configuration(device_id).unwrap();
        assert_eq!(new_config.sensitivity, 90);
        assert_eq!(new_config.speed, 85);
    }
}
