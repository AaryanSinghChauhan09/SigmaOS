//! SigmaOS Display Brightness Manager
//!
//! Inspired by: Omarchy display brightness (omancy-brightness-*, omancy-display-text-size)
//! Following principles: SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY,
//! LAYERED DESIGN, KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING,
//! SECURE_DEFAULTS, DIRECT_DEVICE_CONTROL, LOW_LEVEL_CONTROL, USER_CONTROL,
/// ENERGY_EFFICIENCY, ACCESSIBILITY.

/// Display device
#[derive(Debug, Clone)]
pub struct DisplayDevice {
    pub id: String,
    pub name: String,
    pub current_brightness: u32,
    pub min_brightness: u32,
    pub max_brightness: u32,
    pub is_built_in: bool,
}

/// Display brightness manager
pub struct DisplayBrightnessManager {
    pub displays: HashMap<String, DisplayDevice>,
    pub active_display: Option<String>,
}

impl DisplayBrightnessManager {
    pub fn new() -> Self {
        Self {
            displays: HashMap::new(),
            active_display: None,
        }
    }

    /// Add a display
    pub fn add_display(&mut self, display: DisplayDevice) {
        let id = display.id.clone();
        self.displays.insert(id, display);
        if self.active_display.is_none() {
            self.active_display = self.displays.keys().next().cloned();
        }
    }

    /// Set active display
    pub fn set_active_display(&mut self, id: &str) -> Result<(), String> {
        if !self.displays.contains_key(id) {
            return Err(format!("Display {} not found", id));
        }
        self.active_display = Some(id.to_string());
        Ok(())
    }

    /// Get active display
    pub fn get_active_display(&self) -> Option<&DisplayDevice> {
        self.active_display
            .as_ref()
            .and_then(|id| self.displays.get(id))
    }

    /// Set brightness for a display
    pub fn set_brightness(&mut self, id: &str, brightness: u32) -> Result<(), String> {
        let display = self
            .displays
            .get_mut(id)
            .ok_or_else(|| format!("Display {} not found", id))?;
        if brightness < display.min_brightness || brightness > display.max_brightness {
            return Err(format!(
                "Brightness must be between {} and {}",
                display.min_brightness, display.max_brightness
            ));
        }
        display.current_brightness = brightness;
        Ok(())
    }

    /// Get brightness for a display
    pub fn get_brightness(&self, id: &str) -> Option<u32> {
        self.displays.get(id).map(|d| d.current_brightness)
    }

    /// Increase brightness
    pub fn increase_brightness(&mut self, id: &str, step: u32) -> Result<(), String> {
        let current = self.get_brightness(id).unwrap_or(0);
        let max = self
            .displays
            .get(id)
            .map(|d| d.max_brightness)
            .unwrap_or(100);
        let new_value = (current + step).min(max);
        self.set_brightness(id, new_value)
    }

    /// Decrease brightness
    pub fn decrease_brightness(&mut self, id: &str, step: u32) -> Result<(), String> {
        let current = self.get_brightness(id).unwrap_or(0);
        let min = self
            .displays
            .get(id)
            .map(|d| d.min_brightness)
            .unwrap_or(0);
        let new_value = current.saturating_sub(step).max(min);
        self.set_brightness(id, new_value)
    }

    /// Get brightness percentage (0-100)
    pub fn get_brightness_percent(&self, id: &str) -> Option<f32> {
        let display = self.displays.get(id)?;
        let range = display.max_brightness - display.min_brightness;
        if range == 0 {
            return Some(100.0);
        }
        let current = display.current_brightness - display.min_brightness;
        Some((current as f32 / range as f32) * 100.0)
    }

    /// List displays
    pub fn list_displays(&self) -> Vec<&DisplayDevice> {
        self.displays.values().collect()
    }

    /// Remove a display
    pub fn remove_display(&mut self, id: &str) -> Result<(), String> {
        self.displays
            .remove(id)
            .ok_or_else(|| format!("Display {} not found", id))?;
        if self.active_display.as_deref() == Some(id) {
            self.active_display = self.displays.keys().next().cloned();
        }
        Ok(())
    }

    /// Display count
    pub fn display_count(&self) -> usize {
        self.displays.len()
    }
}

impl Default for DisplayBrightnessManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_display() {
        let mut mgr = DisplayBrightnessManager::new();
        let display = DisplayDevice {
            id: "eDP-1".to_string(),
            name: "Built-in Display".to_string(),
            current_brightness: 50,
            min_brightness: 1,
            max_brightness: 100,
            is_built_in: true,
        };
        mgr.add_display(display);
        assert_eq!(mgr.display_count(), 1);
        assert!(mgr.get_active_display().is_some());
    }

    #[test]
    fn test_set_brightness() {
        let mut mgr = DisplayBrightnessManager::new();
        mgr.add_display(DisplayDevice {
            id: "eDP-1".to_string(),
            name: "Display".to_string(),
            current_brightness: 50,
            min_brightness: 1,
            max_brightness: 100,
            is_built_in: true,
        });
        assert!(mgr.set_brightness("eDP-1", 80).is_ok());
        assert_eq!(mgr.get_brightness("eDP-1"), Some(80));
    }

    #[test]
    fn test_set_brightness_out_of_range() {
        let mut mgr = DisplayBrightnessManager::new();
        mgr.add_display(DisplayDevice {
            id: "eDP-1".to_string(),
            name: "Display".to_string(),
            current_brightness: 50,
            min_brightness: 1,
            max_brightness: 100,
            is_built_in: true,
        });
        assert!(mgr.set_brightness("eDP-1", 150).is_err());
        assert!(mgr.set_brightness("eDP-1", 0).is_err());
    }

    #[test]
    fn test_increase_decrease() {
        let mut mgr = DisplayBrightnessManager::new();
        mgr.add_display(DisplayDevice {
            id: "eDP-1".to_string(),
            name: "Display".to_string(),
            current_brightness: 50,
            min_brightness: 1,
            max_brightness: 100,
            is_built_in: true,
        });
        assert!(mgr.increase_brightness("eDP-1", 20).is_ok());
        assert_eq!(mgr.get_brightness("eDP-1"), Some(70));
        assert!(mgr.decrease_brightness("eDP-1", 30).is_ok());
        assert_eq!(mgr.get_brightness("eDP-1"), Some(40));
    }

    #[test]
    fn test_brightness_percent() {
        let mut mgr = DisplayBrightnessManager::new();
        mgr.add_display(DisplayDevice {
            id: "eDP-1".to_string(),
            name: "Display".to_string(),
            current_brightness: 50,
            min_brightness: 0,
            max_brightness: 100,
            is_built_in: true,
        });
        assert!((mgr.get_brightness_percent("eDP-1").unwrap() - 50.0).abs() < 0.01);
    }

    #[test]
    fn test_set_active_display() {
        let mut mgr = DisplayBrightnessManager::new();
        mgr.add_display(DisplayDevice {
            id: "eDP-1".to_string(),
            name: "Built-in".to_string(),
            current_brightness: 50,
            min_brightness: 1,
            max_brightness: 100,
            is_built_in: true,
        });
        mgr.add_display(DisplayDevice {
            id: "HDMI-1".to_string(),
            name: "External".to_string(),
            current_brightness: 80,
            min_brightness: 1,
            max_brightness: 100,
            is_built_in: false,
        });
        assert!(mgr.set_active_display("HDMI-1").is_ok());
        assert_eq!(mgr.get_active_display().unwrap().id, "HDMI-1");
    }
}