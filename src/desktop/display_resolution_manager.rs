// SigmaOS Desktop Display Resolution Manager
// Inspired by Linux Mint's display settings and Omarchy's display utilities

use std::collections::HashMap;

/// Resolution
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

impl Resolution {
    pub fn new(width: u32, height: u32) -> Self {
        Resolution { width, height }
    }

    pub fn as_str(&self) -> String {
        format!("{}x{}", self.width, self.height)
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }
}

/// Refresh rate
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RefreshRate {
    pub hz: u32,
}

impl RefreshRate {
    pub fn new(hz: u32) -> Self {
        RefreshRate { hz }
    }

    pub fn as_str(&self) -> String {
        format!("{} Hz", self.hz)
    }
}

/// Display mode
#[derive(Debug, Clone)]
pub struct DisplayMode {
    pub id: String,
    pub resolution: Resolution,
    pub refresh_rate: RefreshRate,
    pub is_preferred: bool,
}

impl DisplayMode {
    pub fn new(
        id: String,
        resolution: Resolution,
        refresh_rate: RefreshRate,
    ) -> Self {
        DisplayMode {
            id,
            resolution,
            refresh_rate,
            is_preferred: false,
        }
    }

    pub fn set_preferred(&mut self, preferred: bool) {
        self.is_preferred = preferred;
    }
}

/// Display
#[derive(Debug, Clone)]
pub struct Display {
    pub id: String,
    pub name: String,
    pub is_primary: bool,
    pub is_enabled: bool,
    pub current_mode: Option<String>,
    pub available_modes: Vec<DisplayMode>,
    pub position: (i32, i32),
}

impl Display {
    pub fn new(
        id: String,
        name: String,
    ) -> Self {
        Display {
            id,
            name,
            is_primary: false,
            is_enabled: true,
            current_mode: None,
            available_modes: Vec::new(),
            position: (0, 0),
        }
    }

    pub fn set_primary(&mut self, primary: bool) {
        self.is_primary = primary;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.is_enabled = enabled;
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.position = (x, y);
    }

    pub fn add_mode(&mut self, mode: DisplayMode) {
        self.available_modes.push(mode);
    }

    pub fn set_current_mode(&mut self, mode_id: String) {
        self.current_mode = Some(mode_id);
    }
}

/// Display Resolution Manager
pub struct DisplayResolutionManager {
    displays: HashMap<String, Display>,
    next_display_id: u32,
}

impl DisplayResolutionManager {
    pub fn new() -> Self {
        let mut manager = DisplayResolutionManager {
            displays: HashMap::new(),
            next_display_id: 1,
        };

        manager.add_default_display();
        manager
    }

    fn add_default_display(&mut self) {
        let id = format!("display_{}", self.next_display_id);
        self.next_display_id += 1;

        let mut display = Display::new(
            id.clone(),
            "eDP-1".to_string(),
        );
        display.set_primary(true);

        // Add common resolutions
        let mode1_id = format!("mode_{}_1", id);
        let mut mode1 = DisplayMode::new(
            mode1_id.clone(),
            Resolution::new(1920, 1080),
            RefreshRate::new(60),
        );
        mode1.set_preferred(true);
        display.add_mode(mode1);
        display.set_current_mode(mode1_id);

        let mode2_id = format!("mode_{}_2", id);
        let mode2 = DisplayMode::new(
            mode2_id,
            Resolution::new(2560, 1440),
            RefreshRate::new(60),
        );
        display.add_mode(mode2);

        let mode3_id = format!("mode_{}_3", id);
        let mode3 = DisplayMode::new(
            mode3_id,
            Resolution::new(3840, 2160),
            RefreshRate::new(60),
        );
        display.add_mode(mode3);

        self.displays.insert(id, display);
    }

    pub fn add_display(&mut self, name: String) -> String {
        let id = format!("display_{}", self.next_display_id);
        self.next_display_id += 1;

        let display = Display::new(id.clone(), name);
        self.displays.insert(id.clone(), display);
        id
    }

    pub fn remove_display(&mut self, id: &str) -> bool {
        self.displays.remove(id).is_some()
    }

    pub fn get_display(&self, id: &str) -> Option<&Display> {
        self.displays.get(id)
    }

    pub fn get_displays(&self) -> Vec<&Display> {
        self.displays.values().collect()
    }

    pub fn get_primary_display(&self) -> Option<&Display> {
        self.displays.values().find(|d| d.is_primary)
    }

    pub fn get_enabled_displays(&self) -> Vec<&Display> {
        self.displays
            .values()
            .filter(|d| d.is_enabled)
            .collect()
    }

    pub fn set_primary_display(&mut self, id: &str) -> bool {
        if self.displays.contains_key(id) {
            // Deselect current primary
            for display in self.displays.values_mut() {
                display.set_primary(false);
            }
            // Set new primary
            if let Some(display) = self.displays.get_mut(id) {
                display.set_primary(true);
            }
            true
        } else {
            false
        }
    }

    pub fn set_display_enabled(&mut self, id: &str, enabled: bool) -> bool {
        if let Some(display) = self.displays.get_mut(id) {
            display.set_enabled(enabled);
            true
        } else {
            false
        }
    }

    pub fn set_display_position(&mut self, id: &str, x: i32, y: i32) -> bool {
        if let Some(display) = self.displays.get_mut(id) {
            display.set_position(x, y);
            true
        } else {
            false
        }
    }

    pub fn add_display_mode(&mut self, display_id: &str, mode: DisplayMode) -> bool {
        if let Some(display) = self.displays.get_mut(display_id) {
            display.add_mode(mode);
            true
        } else {
            false
        }
    }

    pub fn set_display_mode(&mut self, display_id: &str, mode_id: String) -> bool {
        if let Some(display) = self.displays.get_mut(display_id) {
            display.set_current_mode(mode_id);
            true
        } else {
            false
        }
    }

    pub fn get_statistics(&self) -> DisplayResolutionStatistics {
        DisplayResolutionStatistics {
            total_displays: self.displays.len(),
            enabled_displays: self.get_enabled_displays().len(),
            primary_display: self.get_primary_display().is_some(),
        }
    }
}

impl Default for DisplayResolutionManager {
    fn default() -> Self {
        Self::new()
    }
}

/// DisplayResolutionStatistics
#[derive(Debug, Clone, Copy)]
pub struct DisplayResolutionStatistics {
    pub total_displays: usize,
    pub enabled_displays: usize,
    pub primary_display: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_resolution_manager_initialization() {
        let manager = DisplayResolutionManager::new();
        assert_eq!(manager.get_displays().len(), 1);
        assert!(manager.get_primary_display().is_some());
    }

    #[test]
    fn test_add_display() {
        let mut manager = DisplayResolutionManager::new();
        let id = manager.add_display("HDMI-1".to_string());
        assert!(manager.get_display(&id).is_some());
        assert_eq!(manager.get_displays().len(), 2);
    }

    #[test]
    fn test_remove_display() {
        let mut manager = DisplayResolutionManager::new();
        let id = manager.add_display("HDMI-1".to_string());
        assert!(manager.remove_display(&id));
        assert_eq!(manager.get_displays().len(), 1);
    }

    #[test]
    fn test_set_primary_display() {
        let mut manager = DisplayResolutionManager::new();
        let id = manager.add_display("HDMI-1".to_string());
        assert!(manager.set_primary_display(&id));
        assert_eq!(manager.get_primary_display().unwrap().id, id);
    }

    #[test]
    fn test_set_display_enabled() {
        let mut manager = DisplayResolutionManager::new();
        let displays = manager.get_displays();
        if let Some(display) = displays.first() {
            assert!(manager.set_display_enabled(&display.id, false));
            assert!(!manager.get_display(&display.id).unwrap().is_enabled);
        }
    }

    #[test]
    fn test_set_display_position() {
        let mut manager = DisplayResolutionManager::new();
        let displays = manager.get_displays();
        if let Some(display) = displays.first() {
            assert!(manager.set_display_position(&display.id, 100, 100));
            assert_eq!(manager.get_display(&display.id).unwrap().position, (100, 100));
        }
    }

    #[test]
    fn test_add_display_mode() {
        let mut manager = DisplayResolutionManager::new();
        let displays = manager.get_displays();
        if let Some(display) = displays.first() {
            let mode = DisplayMode::new(
                "test_mode".to_string(),
                Resolution::new(1280, 720),
                RefreshRate::new(60),
            );
            assert!(manager.add_display_mode(&display.id, mode));
        }
    }

    #[test]
    fn test_resolution_aspect_ratio() {
        let res = Resolution::new(1920, 1080);
        assert!((res.aspect_ratio() - 1.7777).abs() < 0.01);
    }

    #[test]
    fn test_statistics() {
        let manager = DisplayResolutionManager::new();
        let stats = manager.get_statistics();
        assert_eq!(stats.total_displays, 1);
    }
}
