// Desktop Display Settings Manager
// Linux Mint & Omarchy inspiration for comprehensive display management

use std::collections::HashMap;

/// Desktop Display Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopDisplayMode {
    Extend,
    Mirror,
    Single,
    Disable,
}

impl DesktopDisplayMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            DesktopDisplayMode::Extend => "extend",
            DesktopDisplayMode::Mirror => "mirror",
            DesktopDisplayMode::Single => "single",
            DesktopDisplayMode::Disable => "disable",
        }
    }
}

/// Desktop Refresh Rate
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DesktopRefreshRate {
    pub value: u32,  // Hz
}

impl DesktopRefreshRate {
    pub fn new(value: u32) -> Self {
        Self { value }
    }

    pub fn as_str(&self) -> String {
        format!("{}Hz", self.value)
    }
}

/// Desktop Resolution
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesktopResolution {
    pub width: u32,
    pub height: u32,
}

impl DesktopResolution {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn as_str(&self) -> String {
        format!("{}x{}", self.width, self.height)
    }
}

/// Desktop Display
#[derive(Debug, Clone)]
pub struct DesktopDisplay {
    pub id: String,
    pub name: String,
    pub connected: bool,
    pub enabled: bool,
    pub primary: bool,
    pub resolution: Option<DesktopResolution>,
    pub refresh_rate: Option<DesktopRefreshRate>,
    pub mode: DesktopDisplayMode,
}

impl DesktopDisplay {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            connected: false,
            enabled: false,
            primary: false,
            resolution: None,
            refresh_rate: None,
            mode: DesktopDisplayMode::Extend,
        }
    }

    pub fn set_resolution(&mut self, resolution: DesktopResolution) {
        self.resolution = Some(resolution);
    }

    pub fn set_refresh_rate(&mut self, refresh_rate: DesktopRefreshRate) {
        self.refresh_rate = Some(refresh_rate);
    }
}

/// Desktop Display Settings Manager
pub struct DesktopDisplaySettingsManager {
    displays: HashMap<String, DesktopDisplay>,
    primary_display: Option<String>,
    mode: DesktopDisplayMode,
    counter: u32,
}

impl DesktopDisplaySettingsManager {
    pub fn new() -> Self {
        let mut manager = Self {
            displays: HashMap::new(),
            primary_display: None,
            mode: DesktopDisplayMode::Extend,
            counter: 0,
        };

        // Add default display
        manager.add_default_display();

        manager
    }

    fn add_default_display(&mut self) {
        let mut display = DesktopDisplay::new(
            "display_0".to_string(),
            "eDP-1".to_string(),
        );

        display.connected = true;
        display.enabled = true;
        display.primary = true;
        display.set_resolution(DesktopResolution::new(1920, 1080));
        display.set_refresh_rate(DesktopRefreshRate::new(60));

        let display_id = display.id.clone();
        self.displays.insert(display_id.clone(), display);
        self.primary_display = Some(display_id);
    }

    pub fn add_display(&mut self, display: DesktopDisplay) -> String {
        let id = format!("display_{}", self.counter);
        self.counter += 1;

        let display = DesktopDisplay {
            id: id.clone(),
            ..display
        };

        self.displays.insert(id.clone(), display);
        id
    }

    pub fn remove_display(&mut self, id: &str) -> bool {
        if Some(id.to_string()) == self.primary_display {
            return false;
        }
        self.displays.remove(id).is_some()
    }

    pub fn get_display(&self, id: &str) -> Option<&DesktopDisplay> {
        self.displays.get(id)
    }

    pub fn get_displays(&self) -> Vec<&DesktopDisplay> {
        self.displays.values().collect()
    }

    pub fn set_primary_display(&mut self, id: &str) -> bool {
        if !self.displays.contains_key(id) {
            return false;
        }

        // Unset primary from previous
        if let Some(prev_id) = &self.primary_display {
            if let Some(display) = self.displays.get_mut(prev_id) {
                display.primary = false;
            }
        }

        // Set new primary
        if let Some(display) = self.displays.get_mut(id) {
            display.primary = true;
            self.primary_display = Some(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn get_primary_display(&self) -> Option<&DesktopDisplay> {
        self.primary_display
            .as_ref()
            .and_then(|id| self.displays.get(id))
    }

    pub fn enable_display(&mut self, id: &str) -> bool {
        if let Some(display) = self.displays.get_mut(id) {
            display.enabled = true;
            true
        } else {
            false
        }
    }

    pub fn disable_display(&mut self, id: &str) -> bool {
        if let Some(display) = self.displays.get_mut(id) {
            display.enabled = false;
            true
        } else {
            false
        }
    }

    pub fn set_display_mode(&mut self, mode: DesktopDisplayMode) {
        self.mode = mode;
    }

    pub fn get_display_mode(&self) -> DesktopDisplayMode {
        self.mode
    }

    pub fn set_display_resolution(&mut self, id: &str, resolution: DesktopResolution) -> bool {
        if let Some(display) = self.displays.get_mut(id) {
            display.set_resolution(resolution);
            true
        } else {
            false
        }
    }

    pub fn set_display_refresh_rate(&mut self, id: &str, refresh_rate: DesktopRefreshRate) -> bool {
        if let Some(display) = self.displays.get_mut(id) {
            display.set_refresh_rate(refresh_rate);
            true
        } else {
            false
        }
    }

    pub fn get_connected_displays(&self) -> Vec<&DesktopDisplay> {
        self.displays
            .values()
            .filter(|d| d.connected)
            .collect()
    }

    pub fn get_enabled_displays(&self) -> Vec<&DesktopDisplay> {
        self.displays
            .values()
            .filter(|d| d.enabled)
            .collect()
    }

    pub fn get_statistics(&self) -> DesktopDisplaySettingsManagerStatistics {
        DesktopDisplaySettingsManagerStatistics {
            total_displays: self.displays.len(),
            connected_displays: self.get_connected_displays().len(),
            enabled_displays: self.get_enabled_displays().len(),
            primary_display_set: self.primary_display.is_some(),
            mode: self.mode.as_str().to_string(),
        }
    }
}

impl Default for DesktopDisplaySettingsManager {
    fn default() -> Self {
        Self::new()
    }
}

/// DesktopDisplaySettingsManagerStatistics
#[derive(Debug, Clone)]
pub struct DesktopDisplaySettingsManagerStatistics {
    pub total_displays: usize,
    pub connected_displays: usize,
    pub enabled_displays: usize,
    pub primary_display_set: bool,
    pub mode: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_manager_state() {
        let manager = DesktopDisplaySettingsManager::new();
        let stats = manager.get_statistics();

        assert_eq!(stats.total_displays, 1);
        assert_eq!(stats.connected_displays, 1);
        assert_eq!(stats.enabled_displays, 1);
        assert!(stats.primary_display_set);
        assert_eq!(stats.mode, "extend");
    }

    #[test]
    fn test_add_display() {
        let mut manager = DesktopDisplaySettingsManager::new();
        let initial_count = manager.get_displays().len();

        let display = DesktopDisplay::new("HDMI-1".to_string(), "HDMI-1".to_string());

        let id = manager.add_display(display);
        assert!(manager.get_display(&id).is_some());
        assert_eq!(manager.get_displays().len(), initial_count + 1);
    }

    #[test]
    fn test_remove_display() {
        let mut manager = DesktopDisplaySettingsManager::new();

        let display = DesktopDisplay::new("HDMI-1".to_string(), "HDMI-1".to_string());

        let id = manager.add_display(display);
        assert!(manager.remove_display(&id));
        assert!(manager.get_display(&id).is_none());
    }

    #[test]
    fn test_remove_primary_display() {
        let mut manager = DesktopDisplaySettingsManager::new();

        // Try to remove primary display (should fail)
        let primary_id = manager.get_primary_display().unwrap().id.clone();
        let result = manager.remove_display(&primary_id);
        assert!(!result);
    }

    #[test]
    fn test_set_primary_display() {
        let mut manager = DesktopDisplaySettingsManager::new();

        let display = DesktopDisplay::new("HDMI-1".to_string(), "HDMI-1".to_string());

        let id = manager.add_display(display);
        assert!(manager.set_primary_display(&id));

        let primary = manager.get_primary_display().unwrap();
        assert_eq!(primary.id, id);
    }

    #[test]
    fn test_enable_disable_display() {
        let mut manager = DesktopDisplaySettingsManager::new();

        let display = DesktopDisplay::new("HDMI-1".to_string(), "HDMI-1".to_string());

        let id = manager.add_display(display);
        assert!(manager.enable_display(&id));

        let display = manager.get_display(&id).unwrap();
        assert!(display.enabled);

        assert!(manager.disable_display(&id));

        let display = manager.get_display(&id).unwrap();
        assert!(!display.enabled);
    }

    #[test]
    fn test_display_mode() {
        let mut manager = DesktopDisplaySettingsManager::new();

        manager.set_display_mode(DesktopDisplayMode::Mirror);
        assert_eq!(manager.get_display_mode(), DesktopDisplayMode::Mirror);
    }

    #[test]
    fn test_display_resolution() {
        let mut manager = DesktopDisplaySettingsManager::new();
        let display_id = "display_0";

        assert!(manager.set_display_resolution(
            display_id,
            DesktopResolution::new(2560, 1440),
        ));

        let display = manager.get_display(display_id).unwrap();
        assert_eq!(display.resolution.as_ref().unwrap().width, 2560);
        assert_eq!(display.resolution.as_ref().unwrap().height, 1440);
    }

    #[test]
    fn test_display_refresh_rate() {
        let mut manager = DesktopDisplaySettingsManager::new();
        let display_id = "display_0";

        assert!(manager.set_display_refresh_rate(
            display_id,
            DesktopRefreshRate::new(144),
        ));

        let display = manager.get_display(display_id).unwrap();
        assert_eq!(display.refresh_rate.as_ref().unwrap().value, 144);
    }

    #[test]
    fn test_connected_displays() {
        let mut manager = DesktopDisplaySettingsManager::new();

        let mut display = DesktopDisplay::new("HDMI-1".to_string(), "HDMI-1".to_string());
        display.connected = true;

        let id = manager.add_display(display);

        let connected = manager.get_connected_displays();
        assert_eq!(connected.len(), 2);
    }

    #[test]
    fn test_enabled_displays() {
        let mut manager = DesktopDisplaySettingsManager::new();

        let mut display = DesktopDisplay::new("HDMI-1".to_string(), "HDMI-1".to_string());
        display.enabled = true;

        let id = manager.add_display(display);

        let enabled = manager.get_enabled_displays();
        assert_eq!(enabled.len(), 2);
    }
}
