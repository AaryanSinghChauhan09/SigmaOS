// SigmaOS Screen Brightness Manager
// Inspired by Linux Mint's brightness settings and Omarchy's display utilities

use std::collections::HashMap;

/// Brightness level type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrightnessType {
    Backlight, // Screen backlight
    Keyboard,  // Keyboard backlight
    Indicator, // Indicator LED
}

impl BrightnessType {
    pub fn as_str(&self) -> &'static str {
        match self {
            BrightnessType::Backlight => "Backlight",
            BrightnessType::Keyboard => "Keyboard",
            BrightnessType::Indicator => "Indicator",
        }
    }
}

/// Brightness device
#[derive(Debug, Clone)]
pub struct BrightnessDevice {
    pub id: String,
    pub name: String,
    pub device_type: BrightnessType,
    pub brightness: u8, // 0-100
    pub max_brightness: u8,
    pub min_brightness: u8,
    pub is_available: bool,
}

impl BrightnessDevice {
    pub fn new(
        id: String,
        name: String,
        device_type: BrightnessType,
        max_brightness: u8,
        min_brightness: u8,
    ) -> Self {
        BrightnessDevice {
            id,
            name,
            device_type,
            brightness: 50,
            max_brightness,
            min_brightness,
            is_available: true,
        }
    }

    pub fn set_brightness(&mut self, brightness: u8) {
        self.brightness = brightness.min(self.max_brightness).max(self.min_brightness);
    }

    pub fn set_available(&mut self, available: bool) {
        self.is_available = available;
    }
}

/// Adaptive brightness mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdaptiveBrightnessMode {
    Disabled,
    Auto,
    Manual,
}

impl AdaptiveBrightnessMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            AdaptiveBrightnessMode::Disabled => "Disabled",
            AdaptiveBrightnessMode::Auto => "Auto",
            AdaptiveBrightnessMode::Manual => "Manual",
        }
    }
}

/// Screen Brightness Manager
pub struct BrightnessManager {
    devices: HashMap<String, BrightnessDevice>,
    adaptive_mode: AdaptiveBrightnessMode,
    ambient_light_level: u8, // 0-100, simulated
    next_device_id: u32,
}

impl BrightnessManager {
    pub fn new() -> Self {
        let mut manager = BrightnessManager {
            devices: HashMap::new(),
            adaptive_mode: AdaptiveBrightnessMode::Disabled,
            ambient_light_level: 50,
            next_device_id: 1,
        };

        // Add default devices
        manager.add_default_devices();

        manager
    }

    fn add_default_devices(&mut self) {
        let backlight = BrightnessDevice::new(
            format!("dev_{}", self.next_device_id),
            "Built-in Display".to_string(),
            BrightnessType::Backlight,
            100,
            0,
        );
        self.devices.insert(backlight.id.clone(), backlight);
        self.next_device_id += 1;

        let keyboard = BrightnessDevice::new(
            format!("dev_{}", self.next_device_id),
            "Keyboard Backlight".to_string(),
            BrightnessType::Keyboard,
            100,
            0,
        );
        self.devices.insert(keyboard.id.clone(), keyboard);
        self.next_device_id += 1;
    }

    pub fn add_device(
        &mut self,
        name: String,
        device_type: BrightnessType,
        max_brightness: u8,
        min_brightness: u8,
    ) -> String {
        let id = format!("dev_{}", self.next_device_id);
        let device = BrightnessDevice::new(
            id.clone(),
            name,
            device_type,
            max_brightness,
            min_brightness,
        );
        self.devices.insert(id.clone(), device);
        self.next_device_id += 1;
        id
    }

    pub fn remove_device(&mut self, id: &str) -> bool {
        self.devices.remove(id).is_some()
    }

    pub fn get_device(&self, id: &str) -> Option<&BrightnessDevice> {
        self.devices.get(id)
    }

    pub fn get_devices(&self) -> Vec<&BrightnessDevice> {
        self.devices.values().collect()
    }

    pub fn get_devices_by_type(&self, device_type: BrightnessType) -> Vec<&BrightnessDevice> {
        self.devices
            .values()
            .filter(|d| d.device_type == device_type)
            .collect()
    }

    pub fn get_backlight_device(&self) -> Option<&BrightnessDevice> {
        self.devices
            .values()
            .find(|d| d.device_type == BrightnessType::Backlight && d.is_available)
    }

    pub fn set_brightness(&mut self, id: &str, brightness: u8) -> bool {
        if let Some(device) = self.devices.get_mut(id) {
            device.set_brightness(brightness);
            true
        } else {
            false
        }
    }

    pub fn get_brightness(&self, id: &str) -> Option<u8> {
        self.devices.get(id).map(|d| d.brightness)
    }

    pub fn increase_brightness(&mut self, id: &str, amount: u8) -> bool {
        if let Some(device) = self.devices.get_mut(id) {
            let new_brightness = device.brightness.saturating_add(amount);
            device.set_brightness(new_brightness);
            true
        } else {
            false
        }
    }

    pub fn decrease_brightness(&mut self, id: &str, amount: u8) -> bool {
        if let Some(device) = self.devices.get_mut(id) {
            let new_brightness = device.brightness.saturating_sub(amount);
            device.set_brightness(new_brightness);
            true
        } else {
            false
        }
    }

    pub fn set_device_available(&mut self, id: &str, available: bool) -> bool {
        if let Some(device) = self.devices.get_mut(id) {
            device.set_available(available);
            true
        } else {
            false
        }
    }

    pub fn get_adaptive_mode(&self) -> AdaptiveBrightnessMode {
        self.adaptive_mode
    }

    pub fn set_adaptive_mode(&mut self, mode: AdaptiveBrightnessMode) {
        self.adaptive_mode = mode;
    }

    pub fn get_ambient_light_level(&self) -> u8 {
        self.ambient_light_level
    }

    pub fn set_ambient_light_level(&mut self, level: u8) {
        self.ambient_light_level = level.min(100);

        // Auto-adjust backlight if in auto mode
        if self.adaptive_mode == AdaptiveBrightnessMode::Auto {
            if let Some(backlight_id) = self.get_backlight_device().map(|d| d.id.clone()) {
                // Higher ambient light = higher brightness
                let target_brightness = self.ambient_light_level;
                self.set_brightness(&backlight_id, target_brightness);
            }
        }
    }

    pub fn adjust_for_ambient_light(&mut self) {
        if self.adaptive_mode == AdaptiveBrightnessMode::Auto {
            self.set_ambient_light_level(self.ambient_light_level);
        }
    }

    pub fn get_statistics(&self) -> BrightnessStatistics {
        BrightnessStatistics {
            device_count: self.devices.len(),
            adaptive_mode: self.adaptive_mode,
            ambient_light_level: self.ambient_light_level,
        }
    }
}

impl Default for BrightnessManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Brightness statistics
#[derive(Debug, Clone, Copy)]
pub struct BrightnessStatistics {
    pub device_count: usize,
    pub adaptive_mode: AdaptiveBrightnessMode,
    pub ambient_light_level: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brightness_manager_initialization() {
        let manager = BrightnessManager::new();
        assert_eq!(manager.get_devices().len(), 2);
        assert_eq!(
            manager.get_adaptive_mode(),
            AdaptiveBrightnessMode::Disabled
        );
    }

    #[test]
    fn test_add_device() {
        let mut manager = BrightnessManager::new();
        let id = manager.add_device(
            "External Monitor".to_string(),
            BrightnessType::Backlight,
            100,
            0,
        );
        assert!(manager.get_device(&id).is_some());
        assert_eq!(manager.get_devices().len(), 3);
    }

    #[test]
    fn test_remove_device() {
        let mut manager = BrightnessManager::new();
        let id = manager.add_device(
            "External Monitor".to_string(),
            BrightnessType::Backlight,
            100,
            0,
        );
        assert!(manager.remove_device(&id));
        assert!(!manager.get_device(&id).is_some());
        assert_eq!(manager.get_devices().len(), 2);
    }

    #[test]
    fn test_set_brightness() {
        let mut manager = BrightnessManager::new();
        let id = manager.add_device(
            "External Monitor".to_string(),
            BrightnessType::Backlight,
            100,
            0,
        );
        assert!(manager.set_brightness(&id, 75));
        assert_eq!(manager.get_brightness(&id), Some(75));
    }

    #[test]
    fn test_brightness_clamping() {
        let mut manager = BrightnessManager::new();
        let id = manager.add_device(
            "External Monitor".to_string(),
            BrightnessType::Backlight,
            100,
            10,
        );
        manager.set_brightness(&id, 150); // Above max
        assert_eq!(manager.get_brightness(&id), Some(100));
        manager.set_brightness(&id, 5); // Below min
        assert_eq!(manager.get_brightness(&id), Some(10));
    }

    #[test]
    fn test_increase_brightness() {
        let mut manager = BrightnessManager::new();
        let id = manager.add_device(
            "External Monitor".to_string(),
            BrightnessType::Backlight,
            100,
            0,
        );
        manager.set_brightness(&id, 50);
        assert!(manager.increase_brightness(&id, 20));
        assert_eq!(manager.get_brightness(&id), Some(70));
    }

    #[test]
    fn test_decrease_brightness() {
        let mut manager = BrightnessManager::new();
        let id = manager.add_device(
            "External Monitor".to_string(),
            BrightnessType::Backlight,
            100,
            0,
        );
        manager.set_brightness(&id, 50);
        assert!(manager.decrease_brightness(&id, 20));
        assert_eq!(manager.get_brightness(&id), Some(30));
    }

    #[test]
    fn test_adaptive_mode() {
        let mut manager = BrightnessManager::new();
        manager.set_adaptive_mode(AdaptiveBrightnessMode::Auto);
        assert_eq!(manager.get_adaptive_mode(), AdaptiveBrightnessMode::Auto);
    }

    #[test]
    fn test_ambient_light_auto_adjust() {
        let mut manager = BrightnessManager::new();
        manager.set_adaptive_mode(AdaptiveBrightnessMode::Auto);
        manager.set_ambient_light_level(80);

        if let Some(backlight) = manager.get_backlight_device() {
            assert_eq!(backlight.brightness, 80);
        }
    }

    #[test]
    fn test_get_devices_by_type() {
        let manager = BrightnessManager::new();
        let backlights = manager.get_devices_by_type(BrightnessType::Backlight);
        assert_eq!(backlights.len(), 1);

        let keyboards = manager.get_devices_by_type(BrightnessType::Keyboard);
        assert_eq!(keyboards.len(), 1);
    }

    #[test]
    fn test_statistics() {
        let manager = BrightnessManager::new();
        let stats = manager.get_statistics();
        assert_eq!(stats.device_count, 2);
        assert_eq!(stats.adaptive_mode, AdaptiveBrightnessMode::Disabled);
    }
}
